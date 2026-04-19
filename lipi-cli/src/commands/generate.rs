use reqwest::Client;
use lipi_contracts::*;
use crate::config::get_thread_id;

pub async fn run() {
    let req = GenerateRequest {
        thread_id: get_thread_id(),
    };

    let client = Client::new();

    let res = client.post("http://localhost:3000/v1/generate")
        .json(&req)
        .send()
        .await
        .unwrap()
        .json::<GenerateResponse>()
        .await
        .unwrap();

    println!("{}", res.output);
}