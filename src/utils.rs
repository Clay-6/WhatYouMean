#![allow(clippy::too_many_arguments)]

use anyhow::{bail, Result};
use colored::Colorize as _;
use serde_json::Value;

pub async fn get_data(
    client: &reqwest::Client,
    url: &str,
    api_key: &str,
    host: &str,
    random: bool,
) -> Result<Value> {
    let res = client
        .get(url)
        .header("X-RapidAPI-Key", api_key)
        .header("X-RapidAPI-Host", host)
        .query(&[("random", random.to_string())])
        .send()
        .await?
        .error_for_status()?;

    Ok(serde_json::from_str(&res.text().await?)?)
}

pub fn get_info(data: &Value, key: &str) -> Result<Vec<String>> {
    let meanings = &data["results"];
    let meanings = match meanings.as_array() {
        Some(m) => m,
        None => bail!(format!("No {key} info found for that word")),
    };

    let mut info = Vec::new();
    for meaning in meanings {
        info.push(meaning[key].to_string());
    }

    Ok(format_info(info))
}

pub fn get_phonetics(data: &Value) -> String {
    let val = &data["pronunciation"]["all"];

    val.to_string().replace('"', "/")
}

pub fn get_antonyms(data: &Value) -> Vec<String> {
    let arr = data["antonyms"]
        .as_array()
        .expect("No `antonyms` JSON field found");

    arr.iter()
        .map(|a| a.to_string())
        .filter(|a| a != "null" && a != "ul")
        .collect()
}

pub fn print_defs(
    definitions: &[String],
    categories: &[String],
    examples: &[String],
    phonetic: &Option<String>,
    synonyms: &Option<Vec<String>>,
    antonyms: &Option<Vec<String>>,
    show_types: bool,
    show_examples: bool,
    max: usize,
) {
    if let Some(p) = phonetic {
        if p == "ul" || p == "null" {
            println!("[No phonetic available]\n");
        } else {
            println!("{p}\n")
        }
    }

    for (i, def) in definitions.iter().take(max).enumerate() {
        if !show_types {
            println!("{}. {}", i + 1, def);
        } else {
            println!("{}. {} - {}", i + 1, categories[i], def);
        }

        if show_examples {
            if examples[i] == "ul" || examples[i] == "null" {
                println!("[No example]");
            } else {
                println!("e.g: {}", examples[i]);
            }
        }
    }

    println!();

    if let Some(list) = synonyms {
        if list.is_empty() {
            print!("[No synonyms available]")
        } else {
            print!("Synonyms: {}", list[0]);
            for synonym in list.iter().skip(1) {
                if synonym != "ul" && synonym != "null" {
                    print!(", {synonym}");
                }
            }
        }
        println!();
    }
    if let Some(list) = antonyms {
        if list.is_empty() {
            print!("[No antonyms available]");
        } else {
            print!("Antonyms: {}", list[0]);
            for antonym in list.iter().skip(1) {
                if antonym != "ul" && antonym != "null" {
                    print!(", {antonym}");
                }
            }
        }
        println!()
    }
}

pub fn print_defs_colour(
    definitions: &[String],
    categories: &[String],
    examples: &[String],
    phonetic: &Option<String>,
    synonyms: &Option<Vec<String>>,
    antonyms: &Option<Vec<String>>,
    show_types: bool,
    show_examples: bool,
    max: usize,
) {
    if let Some(p) = phonetic {
        if p == "ul" || p == "null" {
            println!("{}\n", "[No phonetic available]".red().italic())
        } else {
            println!("{}\n", p.bright_yellow())
        }
    }

    for (i, def) in definitions.iter().take(max).enumerate() {
        if !show_types {
            println!("{} {}", format!("{}.", i + 1).cyan().bold(), def);
        } else {
            println!(
                "{} {} - {}",
                format!("{}.", i + 1).cyan().bold(),
                categories[i].bright_purple(),
                def
            );
        }

        if show_examples {
            if examples[i] == "ul" || examples[i] == "null" {
                println!("{}", "[No example]".red().italic());
            } else {
                println!("{}", format!("e.g: {}", examples[i]).green().italic());
            }
        }
    }

    println!();

    if let Some(list) = synonyms {
        if list.is_empty() {
            print!("{}", "[No synonyms available]".red().italic());
        } else {
            print!("{}", format!("Synonyms: {}", list[0]).cyan());
            for synonym in list.iter().skip(1) {
                if synonym != "ul" && synonym != "null" {
                    print!("{}", format!(", {synonym}").cyan());
                }
            }
        }
        println!();
    }
    if let Some(list) = antonyms {
        if list.is_empty() {
            print!("{}", "[No antonyms available]".red().italic());
        } else {
            print!("{}", format!("Antonyms: {}", list[0]).magenta());
            for antonym in list.iter().skip(1) {
                if antonym != "ul" && antonym != "null" {
                    print!("{}", format!(", {antonym}").magenta());
                }
            }
        }
        println!()
    }
}

fn format_info(mut defs: Vec<String>) -> Vec<String> {
    for def in &mut defs {
        def.remove(0); // Leading "
        *def = def.replace(r#"\""#, r#"""#); // Useless escapes
        def.remove(def.len() - 1); // Trailing "
    }

    defs
}
