use axum::Json;
use lipi_contracts::{GenerateRequest, GenerateResponse};

pub async fn generate(Json(_req): Json<GenerateRequest>) -> Json<GenerateResponse> {
    Json(GenerateResponse {
        output: "Generated summary (stub)".to_string(),
    })
}