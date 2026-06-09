use std::{convert::Infallible, sync::Arc};

use warp::http::StatusCode;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

use crate::{api, web::State};

pub fn listings(state: Arc<State>) -> BoxedFilter<(impl Reply,)> {
    async fn logic(state: Arc<State>) -> Result<warp::reply::Response, Infallible> {
        let listings = crate::mongo::get_current_listings(state.collection()).await;

        match listings {
            Ok(listings) => {
                let listings: Vec<api::ApiReadableListingContainer> =
                    listings.into_iter().map(|listing| listing.into()).collect();
                Ok(warp::reply::json(&listings).into_response())
            }
            Err(_) => Ok(warp::reply::with_status(
                warp::reply(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()),
        }
    }

    warp::get()
        .and(warp::path("listings"))
        .and(warp::path::end())
        .and_then(move || logic(state.clone()))
        .boxed()
}
