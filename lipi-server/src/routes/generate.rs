use axum::{Json, extract::State};
use std::sync::Arc;

use lipi_contracts::{GenerateRequest, GenerateResponse};
use crate::domain::KnowledgeEngine;

pub async fn generate(
    State(engine): State<Arc<dyn KnowledgeEngine>>,
    Json(req): Json<GenerateRequest>,
) -> Json<GenerateResponse> {
    let output = engine.generate_summary(req.thread_id).await;

    Json(GenerateResponse { output })
}