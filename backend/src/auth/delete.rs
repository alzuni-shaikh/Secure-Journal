use anyhow::{Ok, Result};
use colored::*;
use dialoguer::Confirm;

use crate::models::models::User;
use crate::db::DbPool;

pub async fn delete_user(db: &DbPool, user: &User) -> Result<()> {
    let confirm = Confirm::new()
        .with_prompt(format!(
            "Are you sure you wanna delete '{}' and all their entries?",
            user.username
        ))
        .default(false)
        .interact()
        .unwrap();

    if !confirm {
        println!("{}", "Deletion cancelled..".yellow());
        return Ok(());
    }

    // Delete journal entries belonging to the user
    sqlx::query("DELETE FROM entries WHERE user_id = ?")
        .bind(user.id)
        .execute(db)
        .await?;

    // Delete the user
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user.id)
        .execute(db)
        .await?;

    println!(
        "{}",
        "User and all their Entries are deleted successfully!".green()
    );

    Ok(())
}
