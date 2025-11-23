mod auth;
mod common;
mod db;
mod helpers;
mod models;

use auth::api::{login_api, signup_api};
use axum::extract::{Json, State};
use common::utils::main_menu;
use db::DbPool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct AppState {
    pub db: DbPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = db::connect().await?;
    main_menu(&db).await;
    Ok(())
}

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    ok: bool,
    message: String,
}

async fn api_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match login_api(&state.db, &payload.username, &payload.password).await {
        Ok(Some(user)) => Json(AuthResponse {
            ok: true,
            message: format!("Logged in as {}", user.username),
        }),
        Ok(None) => Json(AuthResponse {
            ok: false,
            message: "Invalid credentials".into(),
        }),
        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Error: {}", e),
        }),
    }
}

async fn api_signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match signup_api(&state.db, &payload.username, &payload.password).await {
        Ok(()) => Json(AuthResponse {
            ok: true,
            message: "Signup successful".into(),
        }),
        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Error: {}", e),
        }),
    }
}
