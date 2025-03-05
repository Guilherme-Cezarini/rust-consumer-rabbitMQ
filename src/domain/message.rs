use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub company_id: String,
    pub campaign_id: String,
    pub created_at: String,
    pub updated_at: String,
}