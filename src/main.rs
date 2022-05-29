mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::{format_info, get_data, get_definitions, get_examples};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let data = get_data(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.word
    ))
    .await?;

    let defs = format_info(get_definitions(&data));

    if args.show_examples {
        let examples = format_info(get_examples(&data));

        for (idx, def) in defs.iter().enumerate() {
            println!("{}: {}", idx + 1, def);

            let example = &examples[idx];
            if example == "null" || example == "ul" {
                println!("[No example]");
            } else {
                println!("e.g: {}", example);
            }
        }
    } else {
        for (idx, def) in defs.iter().enumerate() {
            println!("{}: {}", idx + 1, def)
        }
    }
    Ok(())
}
