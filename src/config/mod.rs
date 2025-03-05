use dotenv::dotenv;
use std::env;

pub struct Config {
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
    pub rabbitmq_queue: String,
    pub rabbitmq_url: String,   
    pub mongodb_database: String,
    pub mongodb_username: String,
    pub mongodb_password: String,
    pub mongodb_url: String,
    
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            rabbitmq_url: env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set"),
            rabbitmq_user: env::var("RABBITMQ_USER").expect("RABBITMQ_USER must be set"),
            rabbitmq_password: env::var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set"),
            rabbitmq_queue: env::var("RABBITMQ_QUEUE").expect("RABBITMQ_QUEUE must be set"),
            mongodb_database: env::var("MONGO_DB_DATABASE").expect("MONGO_DB_DATABASE must be set"),
            mongodb_username: env::var("MONGO_DB_USERNAME").expect("MONGO_DB_USERNAME must be set"),
            mongodb_password: env::var("MONGO_DB_PASSWORD").expect("MONGO_DB_PASSWORD must be set"),
            mongodb_url: env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set"),
        }
    }
}