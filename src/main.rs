use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.dictionaryapi.dev/api/v2/entries/en/hello")
        .await?
        .text()
        .await?;
    let data = serde_json::from_str::<Value>(&response)?;

    let meanings = &data[0]["meanings"];
    let meanings = meanings.as_array().unwrap();

    let mut defs = Vec::new();
    for meaning in meanings {
        defs.push(meaning["definitions"][0]["definition"].to_string())
    }

    for def in defs.iter_mut() {
        *def = def.replace('\\', "");
    }

    for def in defs {
        println!("{}", def)
    }

    Ok(())
}
