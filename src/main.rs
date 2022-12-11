mod cli;
mod utils;

use clap::Parser;
use cli::Args;
use color_eyre::eyre::Result;
use colored::Colorize;
use reqwest::Client;
use serde_json::Value;
use utils::{get_data, remove_tags};

use crate::utils::Definition;

const BASE_URL: &str = "http://api.wordnik.com/v4";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let key = if let Some(key) = args.use_key {
        key
    } else if let Ok(key) = std::env::var("WORDNIK_API_KEY") {
        key
    } else {
        include_str!("../api_key.txt").into()
    };

    let client = Client::new();

    let random_word = if args.random {
        let data = get_data::<Value>(
            &client,
            &format!("{}/words.json/randomWord?api_key={}", BASE_URL, key),
        )
        .await?;
        let word = data["text"].to_string();
        println!("Got \"{}\"", word.purple());
        word
    } else {
        "".into()
    };
    let word = &args.word.unwrap_or(random_word);

    let url = format!(
        "{}/word.json/{}/definitions?api_key={}",
        BASE_URL, word, key
    );

    let defs: Vec<Definition> = get_data(&client, &url).await?;

    for (i, def) in defs
        .iter()
        .filter(|d| d.text().is_some())
        .enumerate()
        .take(args.max)
    {
        let text = remove_tags(&def.text().unwrap());

        println!(
            "{} {} - {}",
            format!("{}.", i + 1).cyan().bold(),
            def.part_of_speech().magenta(),
            text
        )
    }

    Ok(())
}
