mod cli;
mod utils;

use clap::Parser;
use cli::Args;
use color_eyre::eyre::Result;
use reqwest::Client;
use utils::get_data;

const BASE_URL: &str = "http://api.wordnik.com/v4";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let key = &args.use_key.unwrap_or(std::env::var("WORDNIK_API_KEY")?);

    let client = Client::new();
    let url = format!(
        "{}/word.json/{}/definitions?api_key={}",
        BASE_URL, args.word, key
    );

    let data = get_data(&client, &url).await?;

    println!("{data:?}");

    Ok(())
}
