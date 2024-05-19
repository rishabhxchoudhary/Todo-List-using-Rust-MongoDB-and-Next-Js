
use axum::{routing::get, Router};

async fn handler() -> String {
    "get all tasks".to_owned()
}

pub fn create_route() -> Router<()> {
    Router::new().route("/", get(handler))
}