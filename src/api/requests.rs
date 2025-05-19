use super::types::{FactionData, Metadata};
use std::error::Error;

async fn get_api(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .get(url)
        .header(reqwest::header::ORIGIN, "https://infinitytheuniverse.com")
        .send()
        .await
}

pub async fn fetch_metadata() -> Result<Metadata, Box<dyn Error>> {
    let url = "https://api.corvusbelli.com/army/infinity/en/metadata";
    Ok(get_api(url).await?.json().await?)
}

pub async fn fetch_faction_data(id: u64) -> Result<FactionData, Box<dyn Error>> {
    let url = format!("https://api.corvusbelli.com/army/units/en/{id}");
    Ok(get_api(&url).await?.json().await?)
}
