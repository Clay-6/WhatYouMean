mod cli;

use clap::Parser;
use cli::Args;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let key = &args.use_key.unwrap_or(std::env::var("WORDNIK_API_KEY")?);

    Ok(())
}
