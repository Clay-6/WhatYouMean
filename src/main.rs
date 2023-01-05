mod cli;
mod utils;

use clap::Parser;
use cli::Args;
use color_eyre::eyre::Result;
use dotenvy_macro::dotenv;
use owo_colors::{OwoColorize, Stream::Stdout};
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

    if args.no_colour {
        owo_colors::set_override(false);
    }

    let key = if let Some(key) = args.use_key {
        key
    } else if let Ok(key) = std::env::var("WORDNIK_API_KEY") {
        key
    } else {
        return Err(color_eyre::eyre::eyre!("Couldn't find API key"));
    };

    let client = Client::new();

    let random_word = if args.random {
        let data = get_data::<Value>(
            &client,
            &format!("{}/words.json/randomWord?api_key={}", BASE_URL, &key),
        )
        .await?;
        let word = data["word"]
            .to_string()
            .chars()
            .filter(|c| *c != '"')
            .collect::<String>();

        println!("Got \"{}\"", word.if_supports_color(Stdout, |t| t.purple()));
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
        if let Ok(prons) = get_phonetics(&client, word, &key).await {
            print!("{}", prons[0].if_supports_color(Stdout, |t| t.yellow()));
            for p in prons.iter().skip(1) {
                print!(", {}", p.if_supports_color(Stdout, |t| t.yellow()));
            }
            println!("\n");
        } else {
            println!(
                "{}\n",
                "[No phonetics available]"
                    .if_supports_color(Stdout, |t| t.red())
                    .italic()
            )
        }
    }

    for (i, def) in defs
        .iter()
        .filter(|d| d.text().is_some())
        .enumerate()
        .take(args.max)
    {
        let text = remove_tags(&def.text().unwrap());
        println!(
            "{} {} - {}",
            format!("{}.", i + 1)
                .if_supports_color(Stdout, |t| t.cyan())
                .bold(),
            def.part_of_speech()
                .if_supports_color(Stdout, |t| t.magenta()),
            text
        );
        if args.examples {
            let example = def.top_example();
            if example.is_empty() {
                println!(
                    "{}",
                    "[No example]"
                        .if_supports_color(Stdout, |t| t.red())
                        .italic()
                );
            } else {
                println!(
                    "{}",
                    format!("e.g. {}", def.top_example()).if_supports_color(Stdout, |t| t.green())
                )
            }
        }
    }

    if args.synonyms {
        if let Ok(syns) = get_related(&client, word, &key, RelationshipType::Synonym).await {
            print!(
                "Synonyms: {}",
                syns[0].if_supports_color(Stdout, |t| t.yellow())
            );
            for syn in syns.iter().skip(1) {
                print!(", {}", syn.if_supports_color(Stdout, |t| t.yellow()))
            }

            println!()
        } else {
            println!(
                "{}",
                "[No synonyms available]"
                    .if_supports_color(Stdout, |t| t.red())
                    .italic()
            )
        }
    }

    if args.antonyms {
        if let Ok(ants) = get_related(&client, word, &key, RelationshipType::Antonym).await {
            print!(
                "Antonyms: {}",
                ants[0].if_supports_color(Stdout, |t| t.yellow())
            );
            for ant in ants.iter().skip(1) {
                print!(", {}", ant.if_supports_color(Stdout, |t| t.yellow()))
            }

            println!()
        } else {
            println!(
                "{}",
                "[No antonyms available]"
                    .if_supports_color(Stdout, |t| t.red())
                    .italic()
            )
        }
    }

    Ok(())
}
