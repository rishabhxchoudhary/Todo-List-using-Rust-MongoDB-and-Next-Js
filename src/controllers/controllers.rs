use mongodb::Collection;
use crate::{models::models::Todo, state::AppState};
use futures::TryStreamExt;

pub async fn get_all_tasks(state : AppState) -> Vec<Todo> {
    let client : &mongodb::Client = &state.db_client;
    let collection: Collection<Todo> = client.database("todoList").collection("tasks");
    let mut cursor = collection.find(None,None).await.unwrap();
    let mut tasks = Vec::new();
    while let Some(doc) = cursor.try_next().await.unwrap() {
        tasks.push(doc);
    }
    tasks
}