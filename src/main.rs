mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let data = get_data(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.word
    ))
    .await?;

    let definitions = format_info(get_info(&data, "definition"));
    let examples = format_info(get_info(&data, "example"));
    let categories = format_info(get_word_types(&data));
    let phonetic = if args.phonetic {
        Some(get_phonetics(&data))
    } else {
        None
    };

    if !args.no_colour {
        print_defs_colour(definitions, categories, examples, phonetic, &args)
    } else {
        print_defs(definitions, categories, examples, phonetic, &args);
    }

    Ok(())
}
