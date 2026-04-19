use reqwest::Client;
use uuid::Uuid;
use chrono::Utc;
use lipi_contracts::*;
use crate::config::get_thread_id;

pub async fn run(message: String) {
    let log = LogEntry {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        source: LogSource::Manual,
        content: message,
    };

    let req = LogRequest {
        thread_id: get_thread_id(),
        log,
    };

    let client = Client::new();

    client.post("http://localhost:3000/v1/logs")
        .json(&req)
        .send()
        .await
        .unwrap();

    println!("Logged");
}