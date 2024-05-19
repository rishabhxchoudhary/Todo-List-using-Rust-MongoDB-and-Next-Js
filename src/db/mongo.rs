use mongodb::Client;

const URI : &str = "mongodb+srv://root:root@cluster0.xvtxl5v.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

pub async fn init() -> mongodb::error::Result<Client> {
    let client = Client::with_uri_str(URI).await?;
    Ok(client)
}