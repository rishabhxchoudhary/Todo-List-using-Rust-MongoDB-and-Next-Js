
use axum::extract::Path;
use axum::Json;
use axum::{routing::{get,post}, Router, extract::State};
use crate::models::models::Todo;
use crate::state::AppState;
use crate::controllers::controllers;

async fn get_all_tasks(State(state) : State<AppState>) -> Json<Vec<Todo>> {
    let tasks: Vec<Todo> = controllers::get_all_tasks(state).await;
    Json(tasks)
}

async fn get_task_by_id(State(state) : State<AppState>, Path(id) : Path<String>) -> Json<Todo>  {
    let task = controllers::get_task_by_id(state,id).await;
    match task {
        Some(task) => Json(task),
        None => Json(Todo {
            id: None,
            title: "Task not found".to_string(),
            description: "Task not found".to_string()
        })
    }
}

async fn create_task(State(state) : State<AppState>, Json(todo) : Json<Todo>) {
    controllers::create_task(state,todo).await;
}

pub fn create_route(app_state: AppState) -> Router<()> {
    Router::new()
    .route("/", get(get_all_tasks))
    .route("/todo/:id", get(get_task_by_id))
    .route("/todo",post(create_task))
    .with_state(app_state)
}