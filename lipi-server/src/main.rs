use axum::{Router, routing::post};
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/logs", post(routes::logs::ingest_log))
        .route("/v1/generate", post(routes::generate::generate));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Lipi server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}