use mongodb::{bson::doc, Collection};
use crate::{models::models::Todo, state::AppState};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;

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

pub async fn get_task_by_id(state : AppState, id: String) -> Option<Todo> {
    let client : &mongodb::Client = &state.db_client;
    let collection: Collection<Todo> = client.database("todoList").collection("tasks");
    if let Ok(object_id) = ObjectId::parse_str(&id) {
        let todo = collection.find_one(doc! { "_id": object_id }, None).await.unwrap();
        todo
    } else {
        None
    }
}

pub async fn create_task(state : AppState, todo: Todo) {
    let client : &mongodb::Client = &state.db_client;
    let collection: Collection<Todo> = client.database("todoList").collection("tasks");
    let _ = collection.insert_one(todo, None).await;
}