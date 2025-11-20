use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub password: Option<String>
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct JournalEntry {
    pub id: Option<i64>,
    pub user: String, 
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
