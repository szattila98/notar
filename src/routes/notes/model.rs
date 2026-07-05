use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: serde_json::Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}
