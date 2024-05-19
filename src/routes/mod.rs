
use axum::Json;
use axum::{routing::get, Router, extract::State};
use crate::models::models::Todo;
use crate::state::AppState;
use crate::controllers::controllers;

async fn handler(State(state) : State<AppState>) -> Json<Vec<Todo>> {
    let tasks: Vec<Todo> = controllers::get_all_tasks(state).await;
    Json(tasks)
}

pub fn create_route(app_state: AppState) -> Router<()> {
    Router::new()
    .route("/", get(handler))
    .with_state(app_state)
}