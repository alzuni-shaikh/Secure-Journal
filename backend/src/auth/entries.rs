use anyhow::{Ok, Result};
use colored::*;
use dialoguer::Input;
use chrono::Utc;

use crate::models::models::{JournalEntry, User};
use crate::db::DbPool;

use sqlx::Row;

// ---------------------------
// CREATE NEW ENTRY
// ---------------------------
pub async fn new_entry(db: &DbPool, user: &User) -> Result<()> {
    let title = Input::<String>::new()
        .with_prompt("Title")
        .interact()
        .unwrap();

    let content = Input::<String>::new()
        .with_prompt("Content")
        .interact()
        .unwrap();

    let tags_input = Input::<String>::new()
        .with_prompt("Tags (comma seperated)")
        .allow_empty(true)
        .interact()
        .unwrap();

    let tags: Vec<String> = tags_input
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let now = Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(&tags)?;

    sqlx::query(
        "INSERT INTO entries (user_id, title, content, tags, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(user.id)
    .bind(&title)
    .bind(&content)
    .bind(&tags_json)
    .bind(&now)
    .bind(&now)
    .execute(db)
    .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("{}", "Journal entry has been saved!".green());

    Ok(())
}

// ---------------------------
// DELETE ENTRY
// ---------------------------
pub async fn delete_entry(db: &DbPool, user: &User) -> Result<()> {
    let rows = sqlx::query(
        "SELECT id, title, content, tags, created_at, updated_at
         FROM entries WHERE user_id = ?"
    )
    .bind(user.id)
    .fetch_all(db)
    .await?;

    let mut entries: Vec<JournalEntry> = Vec::new();
    for row in rows {
        entries.push(JournalEntry {
            id: Some(row.get::<i64, _>("id")),
            user: user.username.clone(),
            title: row.get("title"),
            content: row.get("content"),
            tags: serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    if entries.is_empty() {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        println!("{}", "No entries to delete".red());
        return Ok(());
    }

    println!("Your journal entries: ");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("Enter the number of entires to delete: ")
        .interact()
        .unwrap();

    if index == 0 || index > entries.len() {
        println!("{}", "Invalid entry number".red());
        return Ok(());
    }

    let entry_to_delete = &entries[index - 1];

    sqlx::query("DELETE FROM entries WHERE id = ?")
        .bind(entry_to_delete.id.unwrap())
        .execute(db)
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("{}", "Journal entry deleted successfully!".green());

    Ok(())
}

// ---------------------------
// LIST USERS
// ---------------------------
pub async fn list_users(db: &DbPool) -> Result<()> {
    let rows = sqlx::query("SELECT username FROM users")
        .fetch_all(db)
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("{}", "Registered users: ".bright_green());

    for row in rows {
        let username: String = row.get("username");
        println!("- {:?}", username);
    }

    Ok(())
}

// ---------------------------
// LIST ENTRIES FOR USER
// ---------------------------
pub async fn list_entries(db: &DbPool, user: &User) -> Result<()> {
    let rows = sqlx::query(
        "SELECT id, title, content, tags, created_at, updated_at
         FROM entries WHERE user_id = ?"
    )
    .bind(user.id)
    .fetch_all(db)
    .await?;

    if rows.is_empty() {
        println!("{}", format!("No entires found for {}", user.username).red());
        return Ok(());
    }

    println!("Your journal entires: ");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    for (i, row) in rows.iter().enumerate() {
        println!(
            "\n{}. {} - {}\n   created at: {}\n   tags: {}\n",
            i + 1,
            row.get::<String, _>("title"),
            row.get::<String, _>("content"),
            row.get::<String, _>("created_at"),
            {
                let tags: Vec<String> =
                    serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default();
                if tags.is_empty() {
                    "(none)".to_string()
                } else {
                    tags.join(", ")
                }
            }
        );
    }

    Ok(())
}

// ---------------------------
// UPDATE ENTRY
// ---------------------------
pub async fn update_entry(db: &DbPool, user: &User) -> Result<()> {
    let rows = sqlx::query(
        "SELECT id, title, content, tags, created_at, updated_at
         FROM entries WHERE user_id = ?"
    )
    .bind(user.id)
    .fetch_all(db)
    .await?;

    if rows.is_empty() {
        println!("{}", "no entries to update..".red());
        return Ok(());
    }

    let mut entries: Vec<JournalEntry> = Vec::new();
    for row in rows {
        entries.push(JournalEntry {
            id: Some(row.get::<i64, _>("id")),
            user: user.username.clone(),
            title: row.get("title"),
            content: row.get("content"),
            tags: serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    println!("your journal entries: ");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("enter the num of entries you wanna update: ")
        .interact()
        .unwrap();

    if index == 0 || index > entries.len() {
        println!("{}", "invalid entry number..".red());
        return Ok(());
    }

    let entry = &entries[index - 1];

    let new_content = Input::<String>::new()
        .with_prompt("new content")
        .interact()
        .unwrap();

    let new_tags = Input::<String>::new()
        .with_prompt("new tags( comma seperated)")
        .allow_empty(true)
        .interact()
        .unwrap();

    let tags: Vec<String> = new_tags
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let updated_at = Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(&tags)?;

    sqlx::query(
        "UPDATE entries
         SET content = ?, tags = ?, updated_at = ?
         WHERE id = ?"
    )
    .bind(&new_content)
    .bind(&tags_json)
    .bind(&updated_at)
    .bind(entry.id.unwrap())
    .execute(db)
    .await?;

    println!("{}", "entry updated successfully..".bright_green());

    Ok(())
}

// ---------------------------
// GET ENTRIES FOR USER
// ---------------------------
pub async fn get_entries_for_user(db: &DbPool, user: &User) -> Result<Vec<JournalEntry>> {
    let rows = sqlx::query(
        "SELECT id, title, content, tags, created_at, updated_at
         FROM entries WHERE user_id = ?"
    )
    .bind(user.id)
    .fetch_all(db)
    .await?;

    let mut entries = Vec::new();

    for row in rows {
        entries.push(JournalEntry {
            id: Some(row.get::<i64, _>("id")),
            user: user.username.clone(),
            title: row.get("title"),
            content: row.get("content"),
            tags: serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(entries)
}
