mod listings;
mod types;
mod ws;

use std::sync::Arc;

pub use listings::*;
pub use types::*;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

use crate::web::State;

pub fn api(state: Arc<State>) -> BoxedFilter<(impl Reply,)> {
    warp::path("api")
        .and(ws::ws(state.clone()).or(listings(state.clone())))
        .boxed()
}
