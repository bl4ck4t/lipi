use std::fs;
use uuid::Uuid;
use chrono::Utc;

pub fn run() {
    if std::path::Path::new(".lipi/config.json").exists() {
        println!("Lipi already initialized");
        return;
    }

    let thread_id = Uuid::new_v4();

    let config = serde_json::json!({
        "thread_id": thread_id,
        "created_at": Utc::now()
    });

    fs::create_dir_all(".lipi").unwrap();
    fs::write(".lipi/config.json", config.to_string()).unwrap();

    println!("Initialized Lipi");
}