use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable};
use crate::domain::message::Message;
use futures_lite::stream::StreamExt;
use log::{info};
use serde_json;


pub struct RabbitMQConsumer {
    uri: String,
    queue_name: String,
}

impl RabbitMQConsumer {
    pub fn new(uri: String, queue_name: String) -> Self {
        RabbitMQConsumer { uri, queue_name }
    }

    pub async fn consume(&self, callback: impl Fn(Message)) {
        info!("Connecting to RabbitMQ at {}", self.uri);
        let conn = Connection::connect(&self.uri, ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");
        info!("Connected to RabbitMQ. Creating channel...");
        let channel = conn.create_channel().await.expect("Failed to create a channel");

        info!("Declaring queue: {}", self.queue_name);
        let _queue = channel.queue_declare(
            &self.queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await.expect("Failed to declare queue");

        info!("Starting consumer for queue: {}", self.queue_name);
        let mut consumer = channel.basic_consume(
            &self.queue_name, // Passando o nome da fila como &str
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await.expect("Failed to create consumer");

        info!("Waiting for messages...");
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                info!("Received a message from RabbitMQ");
                let message: Message = serde_json::from_slice(&delivery.data).expect("Failed to parse message");
                callback(message);
                delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack");
            }
        }
    }
}