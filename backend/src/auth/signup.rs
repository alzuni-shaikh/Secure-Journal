use anyhow::Result;
use dialoguer::Input;
use crate::db::DbPool;
use colored::Colorize;
use serde::Deserialize;
use std::time::Duration;
use rpassword::read_password;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

use crate::auth::validate::validate_creds;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

pub async fn signup_flow(db: &DbPool) -> Result<()> {
    // Ask username
    let username = Input::<String>::new()
        .with_prompt("Choose a Username")
        .interact()?;

    // Ask password
    println!("Choose a Password:");
    let password = read_password()?;

    // Validate password rules
    if let Err(e) = validate_creds(&username, &password) {
        println!("{}", format!("{}", e).bright_red());
        return Ok(());
    }

    // Spinner: check if user exists
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Checking if username is available..");
    spinner.enable_steady_tick(Duration::from_millis(50));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    let existing = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&username)
        .fetch_optional(db)
        .await?;

    spinner.finish_and_clear();

    if existing.is_some() {
        println!("{}", "Username already exists. Please choose another one!!".bright_red());
        return Ok(());
    }

    // Confirm password
    println!("Confirm password:");
    let confirm_pass = read_password()?;
    if confirm_pass != password {
        println!("{}", "Passwords do not match :(".bright_red());
        return Ok(());
    }

    // Hashing Animation
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}% - {msg}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    bar.set_message("Hashing Password Securely..");

    for i in 0..100 {
        bar.set_position(i);
        std::thread::sleep(Duration::from_millis(20));
    }

    // Hash password using Argon2
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    bar.finish_with_message("Password Hashed Successfully..");

    // Insert new user
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Creating your account :)");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&username)
        .bind(&hashed)
        .execute(db)
        .await?;

    spinner.finish_and_clear();

    println!("{}", "Account created successfully.".green());
    Ok(())
}
