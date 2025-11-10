use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
    id: Option<surrealdb::RecordId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JournalEntry {
    id: Option<surrealdb::RecordId>,
    user: String,
    title: String,
    content: String,
}
