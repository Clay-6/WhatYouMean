use anyhow::Result;
use colored::Colorize as _;
use serde_json::Value;

pub async fn get_data(url: &str) -> Result<Value> {
    let response = reqwest::get(url).await?.error_for_status()?.text().await?;

    let data = serde_json::from_str::<Value>(&response)?;

    Ok(data)
}

pub fn get_info(data: &Value, key: &str) -> Vec<String> {
    let meanings = &data[0]["meanings"];
    let meanings = meanings.as_array().unwrap();

    let mut info = Vec::new();
    for meaning in meanings {
        info.push(meaning["definitions"][0][key].to_string())
    }

    info
}

pub fn format_info(defs: Vec<String>) -> Vec<String> {
    let mut defs = defs;

    for def in defs.iter_mut() {
        def.remove(0); // Leading "
        *def = def.replace("\\\"", "\""); // Useless escapes
        def.remove(def.len() - 1); // Trailing "
    }

    defs
}

pub fn get_word_types(data: &Value) -> Vec<String> {
    let meanings = &data[0]["meanings"];
    let meanings = meanings.as_array().unwrap();

    let mut types = Vec::new();
    for meaning in meanings {
        types.push(meaning["partOfSpeech"].to_string())
    }

    types
}

pub fn print_defs(
    definitions: Vec<String>,
    categories: Vec<String>,
    examples: Vec<String>,
    args: &crate::cli::Args,
) {
    for (i, def) in definitions.iter().enumerate() {
        if args.no_types {
            println!("{}. {}", i + 1, def);
        } else {
            println!("{}. {} - {}", i + 1, categories[i], def);
        }

        if args.show_examples {
            if examples[i] == "ul" || examples[i] == "null" {
                println!("[No example]");
            } else {
                println!("e.g: {}", examples[i]);
            }
        }
    }
}

pub fn print_defs_colour(
    definitions: Vec<String>,
    categories: Vec<String>,
    examples: Vec<String>,
    args: &crate::cli::Args,
) {
    for (i, def) in definitions.iter().enumerate() {
        if args.no_types {
            println!("{} {}", format!("{}.", i + 1).cyan().bold(), def);
        } else {
            println!(
                "{} {} - {}",
                format!("{}.", i + 1).cyan().bold(),
                categories[i].bright_purple(),
                def
            );
        }

        if args.show_examples {
            if examples[i] == "ul" || examples[i] == "null" {
                println!("{}", "[No example]".red().italic());
            } else {
                println!("{}", format!("e.g: {}", examples[i]).green().italic());
            }
        }
    }
}
