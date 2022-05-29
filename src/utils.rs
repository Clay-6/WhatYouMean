use anyhow::Result;
use serde_json::Value;

pub async fn get_data(url: &str) -> Result<Value> {
    let response = reqwest::get(url).await?.error_for_status()?.text().await?;

    let data = serde_json::from_str::<Value>(&response)?;

    Ok(data)
}

pub fn get_info(data: &Value, key: &str) -> Vec<String> {
    let meanings = &data[0]["meanings"];
    let meanings = meanings.as_array().unwrap();

    let mut info = Vec::new();
    for meaning in meanings {
        info.push(meaning["definitions"][0][key].to_string())
    }

    info
}

pub fn format_info(defs: Vec<String>) -> Vec<String> {
    let mut defs = defs;

    for def in defs.iter_mut() {
        def.remove(0); // Leading "
        *def = def.replace("\\\"", "\""); // Useless escapes
        def.remove(def.len() - 1); // Trailing "
    }

    defs
}

pub fn get_word_types(data: &Value) -> Vec<String> {
    let meanings = &data[0]["meanings"];
    let meanings = meanings.as_array().unwrap();

    let mut types = Vec::new();
    for meaning in meanings {
        types.push(meaning["partOfSpeech"].to_string())
    }

    types
}
