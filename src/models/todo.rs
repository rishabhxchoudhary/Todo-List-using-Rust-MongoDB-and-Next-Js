use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}