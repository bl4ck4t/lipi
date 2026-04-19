use crate::domain::KnowledgeEngine;
use async_trait::async_trait;
use lipi_contracts::LogEntry;
use uuid::Uuid;

pub struct BackboardEngine;

#[async_trait]
impl KnowledgeEngine for BackboardEngine {
    async fn store_log(&self, thread_id: Uuid, log: LogEntry) {
        println!(
            "[Backboard] storing log → thread={} content={}",
            thread_id, log.content
        );

        // TODO: actual API call
    }

    async fn generate_summary(&self, thread_id: Uuid) -> String {
        println!("[Backboard] generating summary → thread={}", thread_id);

        // TODO: actual API call
        "Generated from Backboard (stub)".to_string()
    }
}