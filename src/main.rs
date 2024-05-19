mod db;
mod state;
mod routes;

use routes::create_route;


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = db::mongo::init().await?;
    let app_state = state::AppState::new(client);
    println!("Connected to MongoDB");

    let app = create_route();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
