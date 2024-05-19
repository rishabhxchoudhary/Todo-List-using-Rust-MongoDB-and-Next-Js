use mongodb::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_client: Arc<Client>,
}

impl AppState {
    pub fn new(client: Client) -> Self {
        Self {
            db_client: Arc::new(client),
        }
    }
}