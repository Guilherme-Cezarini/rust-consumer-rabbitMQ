mod config;
mod domain;
mod infrastructure;
mod application;
use log::{info};
use config::Config;
use infrastructure::{mongo_repository::MongoRepository, rabbitmq_consumer::RabbitMQConsumer};
use application::message_service::MessageService;
use env_logger::Env;
use tokio;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    info!("Starting RabbitMQ to MongoDB consumer...");
    let config = Config::new();
    let mongodb_uri = format!(
        "mongodb://{}:{}@{}",
        &config.mongodb_username, &config.mongodb_password, &config.mongodb_url
    );
    
    let mongo_repo = MongoRepository::new(&mongodb_uri, &config.mongodb_database).await;
    let message_service = MessageService::new(mongo_repo);

    let rabbitmq_full_uri = format!(
        "amqp://{}:{}@{}",
        &config.rabbitmq_user, &config.rabbitmq_password, &config.rabbitmq_url
    );

    let consumer = RabbitMQConsumer::new(rabbitmq_full_uri, config.rabbitmq_queue);
    consumer.consume(|message| {
        let service = message_service.clone();
        tokio::spawn(async move {
            info!("Processing message: {:?}", message);
            service.handle_message(message).await;
        });
    }).await;
}