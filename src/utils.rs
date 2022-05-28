use anyhow::Result;
use serde_json::Value;

pub async fn get_data(url: &str) -> Result<Value> {
    let response = reqwest::get(url).await?.error_for_status()?.text().await?;

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

pub fn format_defs(defs: Vec<String>) -> Vec<String> {
    let mut defs = defs;

    for def in defs.iter_mut() {
        def.remove(0); // Leading "
        *def = def.replace("\\\"", "\""); // Useless escapes
        def.remove(def.len() - 1); // Trailing "
    }

    defs
}