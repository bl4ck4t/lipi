use uuid::Uuid;
use std::fs;

pub fn get_thread_id() -> Uuid {
    let data = fs::read_to_string(".lipi/config.json")
        .expect("Run `lipi init` first");

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    Uuid::parse_str(json["thread_id"].as_str().unwrap()).unwrap()
}