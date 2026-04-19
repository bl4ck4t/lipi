use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: LogSource,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum LogSource {
    Manual,
    Git,
}

#[derive(Serialize, Deserialize)]
pub struct LogRequest {
    pub thread_id: Uuid,
    pub log: LogEntry,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest {
    pub thread_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResponse {
    pub output: String,
}