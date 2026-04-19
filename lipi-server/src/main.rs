use axum::{Router, routing::post};
use std::sync::Arc;

mod routes;
mod domain;
mod infrastructure;

use infrastructure::backboard_engine::BackboardEngine;

#[tokio::main]
async fn main() {
    let engine = Arc::new(BackboardEngine);

    let app = Router::new()
        .route("/v1/logs", post(routes::logs::ingest_log))
        .route("/v1/generate", post(routes::generate::generate))
        .with_state(engine);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running");

    axum::serve(listener, app).await.unwrap();
}