mod cli;
mod utils;

use clap::Parser as _;
use cli::Args;
use utils::{format_defs, get_data, get_definitions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data = get_data(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.word
    ))
    .await?;

    let defs = format_defs(get_definitions(&data));

    for def in defs {
        println!("{}", def)
    }

    Ok(())
}
