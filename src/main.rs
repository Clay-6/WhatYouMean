mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();
    if args.verbose {
        args.show_all();
    }

    let data = get_data(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.word
    ))
    .await?;

    let definitions = get_meaning(&data, "definition");
    let examples = get_meaning(&data, "example");
    let categories = get_word_types(&data);
    let phonetics = if args.phonetic {
        Some(get_phonetics(&data))
    } else {
        None
    };
    let (synonyms, antonyms) = match (args.synonyms, args.antonyms) {
        (true, true) => {
            let tuple = get_related_words(&data);
            (Some(tuple.0), Some(tuple.1))
        }
        (true, false) => (Some(get_related_words(&data).0), None),
        (false, true) => (None, Some(get_related_words(&data).1)),
        (false, false) => (None, None),
    };

    if args.no_colour {
        print_defs(
            &definitions,
            &categories,
            &examples,
            &phonetics,
            &synonyms,
            &antonyms,
            !args.no_types,
            args.examples,
        );
    } else {
        print_defs_colour(
            &definitions,
            &categories,
            &examples,
            &phonetics,
            &synonyms,
            &antonyms,
            !args.no_types,
            args.examples,
        );
    }

    Ok(())
}
