use crate::db::DbPool;

use anyhow::{Result, anyhow};
use bcrypt::{hash, DEFAULT_COST};

pub async fn signup_api(pool: &DbPool, username: &str, password: &str) -> Result<()> {
    let hashed = hash(password, DEFAULT_COST)
        .map_err(|e| anyhow!("bcrypt hashing failed: {}", e))?;

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) 
         VALUES (?, ?)"
    )
    .bind(username)
    .bind(&hashed)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),

        Err(e) => {
            Err(anyhow!("Failed to create user: {}", e))
        }
    }
}
