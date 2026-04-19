use axum::Json;
use lipi_contracts::LogRequest;

pub async fn ingest_log(Json(payload): Json<LogRequest>) -> &'static str {
    println!(
        "[LOG] thread_id={} content={}",
        payload.thread_id,
        payload.log.content
    );

    "ok"
}