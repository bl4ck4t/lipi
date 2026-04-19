use axum::{Json, extract::State};
use std::sync::Arc;

use lipi_contracts::LogRequest;
use crate::domain::KnowledgeEngine;

pub async fn ingest_log(
    State(engine): State<Arc<dyn KnowledgeEngine>>,
    Json(payload): Json<LogRequest>,
) -> &'static str {
    engine
        .store_log(payload.thread_id, payload.log)
        .await;

    "ok"
}