use anyhow::Result;
use std::fs;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};

use crate::models::models::User;

pub async fn connect() -> Result<Surreal<Db>> {
    // Create an in-memory SurrealDB instance
    let db = Surreal::new::<Mem>(()).await?;

    // Use namespace and database
    db.use_ns("app").use_db("journal").await?;

    println!("Connected to in-memory SurrealDB");

    // Try loading previous data from file if exists
    if let Ok(data) = fs::read_to_string("db_backup.json") {
        db.import(&data).await?;
        println!("Loaded previous data from db_backup.json");
    }

    Ok(db)
}

pub async fn save_users(users: &[User]) -> Result<()> {
    let data = serde_json::to_string_pretty(&users)?;
    fs::write("users.json", data)?;
    println!("Users saved to users.json");
    Ok(())
}
pub async fn load_users() -> Result<Vec<User>> {
    let data = fs::read_to_string("users.json").unwrap_or_else(|_| "[]".into());
    let users: Vec<User> = serde_json::from_str(&data)?;
    Ok(users)
}
