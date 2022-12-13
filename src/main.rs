mod cli;
mod utils;

use clap::Parser;
use cli::Args;
use color_eyre::eyre::Result;
use owo_colors::OwoColorize;
use reqwest::Client;
use serde_json::Value;
use utils::{get_data, get_phonetics, get_related, remove_tags, Definition, RelationshipType};

const BASE_URL: &str = "http://api.wordnik.com/v4";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = Args::parse();

    if args.verbose {
        args = Args {
            phonetics: true,
            examples: true,
            antonyms: true,
            synonyms: true,
            ..args
        };
    }

    let key = if let Some(key) = args.use_key {
        key
    } else if let Ok(key) = std::env::var("WORDNIK_API_KEY") {
        key
    } else {
        include_str!("../api_key.txt").into()
    };

    let client = Client::new();

    let random_word = if args.random {
        let data = get_data::<Value>(
            &client,
            &format!("{}/words.json/randomWord?api_key={}", BASE_URL, key),
        )
        .await?;
        let word = data["word"]
            .to_string()
            .chars()
            .filter(|c| *c != '"')
            .collect::<String>();
        if args.no_colour {
            println!("Got \"{}\"", word)
        } else {
            println!("Got \"{}\"", word.purple())
        }
        word
    } else {
        "".to_string()
    };
    let word = &args.word.unwrap_or(random_word);

    let url = format!(
        "{}/word.json/{}/definitions?api_key={}",
        BASE_URL, word, key
    );

    let defs: Vec<Definition> = get_data(&client, &url).await?;

    if args.phonetics {
        let prons = get_phonetics(&client, word, &key).await?;
        if args.no_colour {
            print!("{}", prons[0]);
            for p in prons.iter().skip(1) {
                print!(", {}", p);
            }
        } else {
            print!("{}", prons[0].yellow());
            for p in prons.iter().skip(1) {
                print!(", {}", p.yellow());
            }
        }
        println!("\n");
    }

    for (i, def) in defs
        .iter()
        .filter(|d| d.text().is_some())
        .enumerate()
        .take(args.max)
    {
        let text = remove_tags(&def.text().unwrap());

        if args.no_colour {
            println!(
                "{}. {} - {}",
                format!("{}", i + 1).bold(),
                def.part_of_speech(),
                text
            );
            if args.examples {
                let example = def.top_example();
                if example.is_empty() {
                    println!("[No example]");
                } else {
                    println!("e.g. {}", def.top_example())
                }
            }
        } else {
            println!(
                "{} {} - {}",
                format!("{}.", i + 1).cyan().bold(),
                def.part_of_speech().magenta(),
                text
            );
            if args.examples {
                let example = def.top_example();
                if example.is_empty() {
                    println!("{}", "[No example]".red().italic());
                } else {
                    println!("{}", format!("e.g. {}", def.top_example()).green())
                }
            }
        }
    }

    if args.synonyms {
        if let Ok(syns) = get_related(&client, word, &key, RelationshipType::Synonym).await {
            if args.no_colour {
                print!("Synonyms: {}", syns[0]);
                for syn in syns.iter().skip(1) {
                    print!(", {}", syn);
                }
            } else {
                print!("Synonyms: {}", syns[0].yellow());
                for syn in syns.iter().skip(1) {
                    print!(", {}", syn.yellow())
                }
            }
            println!()
        } else if args.no_colour {
            println!("[No synonyms available]")
        } else {
            println!("{}", "[No synonyms available]".red().italic())
        }
    }

    if args.antonyms {
        if let Ok(ants) = get_related(&client, word, &key, RelationshipType::Antonym).await {
            if args.no_colour {
                print!("Antonyms: {}", ants[0]);
                for ant in ants.iter().skip(1) {
                    print!(", {}", ant);
                }
            } else {
                print!("Antonyms: {}", ants[0].yellow());
                for ant in ants.iter().skip(1) {
                    print!(", {}", ant.yellow())
                }
            }
            println!()
        } else if args.no_colour {
            println!("[No antonyms available]")
        } else {
            println!("{}", "[No antonyms available]".red().italic())
        }
    }

    Ok(())
}
