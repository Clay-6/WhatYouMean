mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::*;

const HOST: &str = "wordsapiv1.p.rapidapi.com";

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("WORDSAPI_KEY")?;

    let mut args = Args::parse();
    if args.verbose {
        args.show_all();
    }

    let data = get_data(
        &format!("https://wordsapiv1.p.rapidapi.com/words/{}", args.word),
        &api_key,
        HOST,
    )
    .await?;

    let defs = get_info(&data, "definition");
    let categories = get_info(&data, "partOfSpeech");
    let phonetic = if args.phonetic {
        Some(get_phonetics(&data))
    } else {
        None
    };
    let synonyms = if args.synonyms {
        Some(get_info(&data, "synonyms"))
    } else {
        None
    };
    let antonyms = if args.antonyms {
        let data = get_data(
            &format!(
                "https://wordsapiv1.p.rapidapi.com/words/{}/antonyms",
                args.word
            ),
            &api_key,
            HOST,
        )
        .await?;
        Some(get_antonyms(&data))
    } else {
        None
    };
    let examples = get_info(&data, "examples");

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
        );
    }

    Ok(())
}
