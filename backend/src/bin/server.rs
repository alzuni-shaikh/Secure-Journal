use secure_journal::db;
use secure_journal::api::{api_signup, api_login};
use secure_journal::AppState;

use std::sync::Arc;
use axum::{Router, routing::post};
use tower_http::cors::{CorsLayer, Any};
use hyper::Server;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::init_db().await?;
    let app_state = Arc::new(AppState { db });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/signup", post(api_signup))
        .route("/api/login", post(api_login))
        .with_state(app_state)
        .layer(cors);

    let addr = ([0, 0, 0, 0], 3000).into();

    println!("Server running on http://{}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
