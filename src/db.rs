use mongodb::{Client, Database};

use crate::env::MONGODB_URI;

pub async fn get_db() -> Database {
    let client = Client::with_uri_str(MONGODB_URI)
        .await
        .expect("Failed to initialize client.");
    client.database("mytestrustserver")
}
