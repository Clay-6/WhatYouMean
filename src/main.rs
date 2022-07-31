mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use colored::Colorize as _;
use serde_json::Value;
use utils::*;

const HOST: &str = "wordsapiv1.p.rapidapi.com";

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("WORDSAPI_KEY")?;

    let mut args = Args::parse();
    if args.verbose {
        args.show_all();
    }

    if args.random {
        let data = get_data(
            "https://wordsapiv1.p.rapidapi.com/words/",
            &api_key,
            HOST,
            true,
        )
        .await?;
        let mut word = data["word"].to_string();
        word.remove(0);
        word.remove(word.len() - 1);

        println!(
            "Got {}",
            if args.no_colour {
                word.clone()
            } else {
                word.purple().to_string()
            }
        );

        show_data(&data, &args, &word, &api_key).await
    } else {
        let word = args.word.as_ref().expect("No word supplied to define");
        let data = get_data(
            &format!("https://wordsapiv1.p.rapidapi.com/words/{}", word),
            &api_key,
            HOST,
            false,
        )
        .await?;

        show_data(&data, &args, word, &api_key).await
    }
}

async fn show_data(data: &Value, args: &Args, word: &str, api_key: &str) -> Result<()> {
    let defs = get_info(data, "definition")?;
    let categories = get_info(data, "partOfSpeech")?
        .iter()
        .map(|t| {
            if t == "ul" {
                "jargon".to_string()
            } else {
                t.clone()
            }
        })
        .collect::<Vec<String>>();
    let phonetic = if args.phonetic {
        Some(get_phonetics(data))
    } else {
        None
    };
    let synonyms = if args.synonyms {
        Some(get_info(data, "synonyms")?)
    } else {
        None
    };
    let antonyms = if args.antonyms {
        let data = get_data(
            &format!("https://wordsapiv1.p.rapidapi.com/words/{}/antonyms", word),
            api_key,
            HOST,
            false,
        )
        .await?;
        Some(get_antonyms(&data))
    } else {
        None
    };
    let examples = get_info(data, "examples")?;

    if args.no_colour {
        print_defs(
            &defs,
            &categories,
            &examples,
            &phonetic,
            &synonyms,
            &antonyms,
            !args.no_types,
            args.examples,
            args.max.unwrap_or(usize::MAX),
        );
    } else {
        print_defs_colour(
            &defs,
            &categories,
            &examples,
            &phonetic,
            &synonyms,
            &antonyms,
            !args.no_types,
            args.examples,
            args.max.unwrap_or(usize::MAX),
        );
    }

    Ok(())
}
