use serde_json::Value;

pub async fn get_data(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let data = serde_json::from_str::<Value>(&response)?;

    Ok(data)
}

pub fn get_definitions(data: &Value) -> Vec<String> {
    let meanings = &data[0]["meanings"];
    let meanings = meanings.as_array().unwrap();

    let mut defs = Vec::new();
    for meaning in meanings {
        defs.push(meaning["definitions"][0]["definition"].to_string())
    }

    defs
}
