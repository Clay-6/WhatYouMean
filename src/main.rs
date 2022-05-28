mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::{format_defs, get_data, get_definitions};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let data = get_data(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.word
    ))
    .await?;

    let defs = format_defs(get_definitions(&data));

    for (idx, def) in defs.iter().enumerate() {
        println!("{}: {}", idx + 1, def)
    }

    Ok(())
}
