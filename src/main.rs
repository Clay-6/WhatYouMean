mod cli;
mod lib;

use clap::Parser as _;
use cli::Args;
use lib::{get_data, get_definitions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data = get_data(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.word
    ))
    .await?;

    let mut defs = get_definitions(&data);

    for def in defs.iter_mut() {
        *def = def.replace('\\', "");
    }

    for def in defs {
        println!("{}", def)
    }

    Ok(())
}
