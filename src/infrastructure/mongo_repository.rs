use mongodb::{Client, Database};
use crate::domain::message::Message;

#[derive(Clone)]
pub struct MongoRepository {
    db: Database,
}

impl MongoRepository {
    pub async fn new(uri: &str, db_name: &str) -> Self {
        let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB");
        let db = client.database(db_name);
        MongoRepository { db }
    }

    pub async fn save_message(&self, message: Message) -> Result<(), mongodb::error::Error> {
        let collection = self.db.collection("messages");
        collection.insert_one(message, None).await?;
        Ok(())
    }
}