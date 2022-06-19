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

    format_info(info)
}

pub fn get_phonetics(data: &Value) -> Vec<String> {
    let mut phonetics = Vec::new();
    let array = data[0]["phonetics"].as_array().unwrap();

    for phonetic in array {
        let val = &phonetic["text"];

        if !val.is_null() {
            let mut formatted = val.to_string();
            formatted.remove(0);
            formatted.remove(formatted.len() - 1);
            phonetics.push(formatted);
        }
    }

    phonetics
}

fn format_info(defs: Vec<String>) -> Vec<String> {
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

    format_info(types)
}

pub fn print_defs(
    definitions: Vec<String>,
    categories: Vec<String>,
    examples: Vec<String>,
    phonetics: Option<Vec<String>>,
    args: &crate::cli::Args,
) {
    if let Some(ref phonetic) = phonetics {
        if phonetic.is_empty() {
            println!("[No phonetics available]");
        } else {
            print!("{}", phonetic[0]);
            for p in phonetic.iter().skip(1) {
                print!(", {}", p);
            }
        }
        println!("\n");
    }

    for (i, def) in definitions.iter().enumerate() {
        if args.no_types {
            println!("{}. {}", i + 1, def);
        } else {
            println!("{}. {} - {}", i + 1, categories[i], def);
        }

        if args.examples {
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
    phonetics: Option<Vec<String>>,
    args: &crate::cli::Args,
) {
    if let Some(ref phonetic) = phonetics {
        if phonetic.is_empty() {
            println!("{}", "[No phonetics available]".red().italic())
        } else {
            print!("{}", phonetic[0].bright_yellow());
            for p in phonetic.iter().skip(1) {
                print!(", {}", p.bright_yellow())
            }
        }
        println!("\n");
    }

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

        if args.examples {
            if examples[i] == "ul" || examples[i] == "null" {
                println!("{}", "[No example]".red().italic());
            } else {
                println!("{}", format!("e.g: {}", examples[i]).green().italic());
            }
        }
    }
}
