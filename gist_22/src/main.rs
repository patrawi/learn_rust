use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JokeData {
    id: u32,
    r#type: String,
    punchline: String,
    setup: String,
}
async fn fetch(url: &str) -> anyhow::Result<JokeData> {
    let json = reqwest::get(url).await?.json().await?;
    Ok(json)
}
#[tokio::main]
async fn main() {
    let json = fetch("https://official-joke-api.appspot.com/random_joke").await;
    println!("{json:#?}");
}
