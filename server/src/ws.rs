use std::cell::Cell;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::task::{AbortHandle, JoinSet};
use warp::ws::{Message, WebSocket};

use crate::listing::PartyFinderListing;
use crate::web::State;

pub struct WsApiClient {
    tasks: JoinSet<TaskOutput>,
    outbound: tokio::sync::mpsc::Sender<ApiMessage>,
    inbound: tokio::sync::broadcast::Sender<ApiMessage>, // Receive handles are gotten by subscribing to this
    state: Arc<State>,
    listings_chan: Cell<Option<AbortHandle>>,
}

enum TaskOutput {
    Run(Box<dyn FnOnce(&mut WsApiClient) -> () + Send>),
    Continue,
    Quit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ApiMessage {
    Subscribe { channel: String },
    Subscribed { channel: String },
    Listings { listings: Arc<[PartyFinderListing]> },
    Err { message: String },
}

impl WsApiClient {
    pub fn new(web_socket: WebSocket, state: Arc<State>) -> Self {
        let (send_outbound, mut recv_outbound) = tokio::sync::mpsc::channel(8);
        let (send_inbound, _) = tokio::sync::broadcast::channel(8);
        let (mut tx, mut rx) = web_socket.split();

        let mut set = JoinSet::new();
        // receive task
        let outbound = send_outbound.clone();
        let inbound = send_inbound.clone();
        set.spawn(async move {
            while let Some(Ok(msg)) = rx.next().await { // give up if there's an error (as far as I can tell they're fatal anyways)
                if let Ok(text) = msg.to_str() { // only a close message has no to_str
                    match serde_json::from_str::<ApiMessage>(text) {
                        Ok(msg) => {
                            if inbound.send(msg).is_err() {
                                // `send` fails if all receivers are dropped
                                return TaskOutput::Quit;
                            }
                        }
                        Err(_) => {
                            let _ = outbound.send(ApiMessage::Err { message: "the request was not understood".to_string() }).await;
                        }
                    }
                }
            }

            TaskOutput::Quit
        });
        // send task
        set.spawn(async move {
            while let Some(msg) = recv_outbound.recv().await {
                let Ok(json) = serde_json::to_string(&msg) else {
                    eprintln!("failed to serialize outbound message: {:#?}", msg);
                    continue;
                };

                if tx.send(Message::text(json)).await.is_err() {
                    break; // can't send. fatal. die
                }
            }

            TaskOutput::Quit
        });

        let mut client = WsApiClient {
            tasks: set,
            outbound: send_outbound,
            inbound: send_inbound,
            state,
            listings_chan: Cell::new(None),
        };
        client.add_subscriber_handler();
        client
    }

    fn add_subscriber_handler(&mut self) {
        let (tx, mut rx) = (self.outbound.clone(), self.inbound.subscribe());
        self.tasks.spawn(async move {
            loop {
                let Ok(ApiMessage::Subscribe { channel}) = rx.recv().await else { continue };
                let task = match channel.as_str() {
                    "listings" => Some(|client: &mut WsApiClient| client.setup_listings_subscription()),
                    _ => None
                };

                if let Some(subscriber) = task {
                    return TaskOutput::Run(Box::new(move |ws| {
                        subscriber(ws);
                        // TODO: honestly, this function is being ran in an async context,
                        // and `Run` _could_ expose that but it's difficult
                        ws.queue_send(ApiMessage::Subscribed { channel });
                        ws.add_subscriber_handler(); // reinstate the handler after
                    }));
                } else {
                    let _ = tx.send(ApiMessage::Err { message: "no such channel to subscribe to".to_string() }).await;
                }
            }
        });
    }

    pub fn setup_listings_subscription(&mut self) {
        let mut rx = self.state.listings_channel.subscribe();
        let send = self.outbound.clone();

        let listings_ah = self.tasks.spawn(async move {
            while let Ok(listings) = rx.recv().await {
                if send.send(ApiMessage::Listings { listings }).await.is_err() {
                    break;
                }
            }

            TaskOutput::Continue
        });
        self.listings_chan.set(Some(listings_ah));
    }

    fn queue_send(&mut self, msg: ApiMessage) {
        let tx = self.outbound.clone();
        self.tasks.spawn(async move {
            let _ = tx.send(msg).await;
            TaskOutput::Continue
        });
    }

    pub async fn run(&mut self) {
        loop {
            let next = self.tasks.join_next().await;
            match next {
                Some(Ok(output)) => {
                    match output {
                        TaskOutput::Run(f) => {
                            f(self);
                        }
                        TaskOutput::Continue => {}
                        TaskOutput::Quit => {
                            return;
                        }
                    }
                }
                Some(Err(_)) => {
                    eprintln!("failed to poll task in ws");
                    return;
                }
                None => {
                    return; // no tasks -> nothing will happen!
                }
            }
        }
    }
}
