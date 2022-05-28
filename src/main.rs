#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.dictionaryapi.dev/api/v2/entries/en/hello")
        .await?
        .text()
        .await?;
    println!("{}", response);

    Ok(())
}
