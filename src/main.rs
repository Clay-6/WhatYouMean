mod cli;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use utils::{format_info, get_data, get_info};

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

    for (i, def) in definitions.iter().enumerate() {
        println!("{}: {}", i + 1, def);
        if args.show_examples {
            if examples[i] == "ul" || examples[i] == "null" {
                println!("[No example]");
            } else {
                println!("e.g: {}", examples[i]);
            }
        }
    }

    Ok(())
}
