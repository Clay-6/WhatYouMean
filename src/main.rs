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

    let definitions = get_info(&data, "definition");
    let examples = get_info(&data, "example");
    let categories = get_word_types(&data);
    let phonetics = if args.phonetic {
        Some(get_phonetics(&data))
    } else {
        None
    };

    if args.no_colour {
        print_defs(&definitions, &categories, &examples, &phonetics, &args);
    } else {
        print_defs_colour(&definitions, &categories, &examples, &phonetics, &args);
    }

    Ok(())
}
