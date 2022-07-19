mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::*;

const HOST: &str = "https://wordsapiv1.p.rapidapi.com";

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

    println!("{data:?}");

    Ok(())
}
