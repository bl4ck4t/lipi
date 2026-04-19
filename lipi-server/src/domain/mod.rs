use async_trait::async_trait;
use lipi_contracts::LogEntry;
use uuid::Uuid;

#[async_trait]
pub trait KnowledgeEngine: Send + Sync {
    async fn store_log(&self, thread_id: Uuid, log: LogEntry);

    async fn generate_summary(&self, thread_id: Uuid) -> String;
}