use crate::domain::message::Message;
use crate::infrastructure::mongo_repository::MongoRepository;

#[derive(Clone)]
pub struct MessageService {
    repo: MongoRepository,
}

impl MessageService {
    pub fn new(repo: MongoRepository) -> Self {
        MessageService { repo }
    }

    pub async fn handle_message(&self, message: Message) {
        self.repo.save_message(message).await.expect("Failed to save message");
    }
}