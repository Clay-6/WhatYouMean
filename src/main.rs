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
        "https://api.dictionaryapi.dev/api/v2/entries/en_{}/{}",
        args.lang_code, args.word
    ))
    .await?;

    let definitions = format_info(get_info(&data, "definition"));
    let examples = format_info(get_info(&data, "example"));
    let categories = format_info(get_word_types(&data));

    if !args.no_colour {
        print_defs_colour(definitions, categories, examples, &args)
    } else {
        print_defs(definitions, categories, examples, &args);
    }

    Ok(())
}
