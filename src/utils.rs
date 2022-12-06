use color_eyre::eyre::Result;
use serde_json::Value;

pub async fn get_data(client: &reqwest::Client, url: &str) -> Result<Value> {
    let res = client.get(url).send().await?.error_for_status()?;

    Ok(serde_json::from_str(&res.text().await?)?)
}
