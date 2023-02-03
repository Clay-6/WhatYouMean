mod cli;

use clap::Parser;
use cli::Args;
use color_eyre::eyre::{eyre, Result};
use owo_colors::{
    OwoColorize,
    Stream::{Stderr, Stdout},
};
use reqwest::Client;
use whatyoumean::{get_random_word, get_wotd, remove_tags, WordInfo};

#[tokio::main]
async fn main() {
    if let Err(e) = color_eyre::install() {
        eprintln!("Error: {e}");
        std::process::exit(1)
    }

    std::process::exit(match dym().await {
        Ok(_) => 0,
        Err(e) => {
            eprintln!(
                "{}: {}",
                "Error"
                    .if_supports_color(Stderr, owo_colors::OwoColorize::red)
                    .bold(),
                e
            );
            1
        }
    })
}

/// Run the actual application
async fn dym() -> Result<()> {
    let mut args = Args::parse();
    let client = Client::new();

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
    } else {
        std::env::var("WORDNIK_API_KEY").map_err(|e| eyre!("`WORDNIK_API_KEY` {}", e))?
    };

    let word = if let Some(wrd) = args.word {
        wrd
    } else if args.random {
        get_random_word(&client, &key).await?
    } else if args.wotd {
        get_wotd(&client, &key).await?
    } else {
        return Err(eyre!(
            "No word supplied. `--random` can be used to search for a random word"
        ));
    };

    let info = WordInfo::fetch(&word, &client, &key).await?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&info)?);
    } else {
        if args.random {
            println!(
                "Got '{}'",
                word.if_supports_color(Stdout, owo_colors::OwoColorize::purple)
            );
        } else if args.wotd {
            println!(
                "Word of the Day is '{}'",
                word.if_supports_color(Stdout, owo_colors::OwoColorize::purple)
            )
        }

        if args.phonetics {
            let prons = info.pronunciations();
            if !prons.is_empty() {
                for p in prons.iter().skip(1) {
                    print!(
                        ", {}",
                        p.if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    );
                }
                println!("\n");
            } else {
                println!(
                    "{}\n",
                    "[No phonetics available]"
                        .if_supports_color(Stdout, owo_colors::OwoColorize::red)
                        .italic()
                );
            }
        }

        for (i, def) in info
            .definitions()
            .iter()
            .filter(|d| d.text().is_some())
            .enumerate()
            .take(args.max)
        {
            let text = remove_tags(&def.text().unwrap());
            println!(
                "{} {} - {}",
                format!("{}.", i + 1)
                    .if_supports_color(Stdout, owo_colors::OwoColorize::cyan)
                    .bold(),
                def.part_of_speech()
                    .if_supports_color(Stdout, owo_colors::OwoColorize::magenta),
                text
            );
            if args.examples {
                let example = def.top_example();
                if example.is_empty() {
                    println!(
                        "{}",
                        "[No example]"
                            .if_supports_color(Stdout, owo_colors::OwoColorize::red)
                            .italic()
                    );
                } else {
                    println!(
                        "{}",
                        format!("e.g. {}", def.top_example())
                            .if_supports_color(Stdout, owo_colors::OwoColorize::green)
                    );
                }
            }
        }

        if args.synonyms {
            let syns = info.synonyms();
            if !syns.is_empty() {
                print!(
                    "Synonyms: {}",
                    syns[0].if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                );
                for syn in syns.iter().skip(1) {
                    print!(
                        ", {}",
                        syn.if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    );
                }

                println!();
            } else {
                println!(
                    "{}",
                    "[No synonyms available]"
                        .if_supports_color(Stdout, owo_colors::OwoColorize::red)
                        .italic()
                );
            }
        }

        if args.antonyms {
            let ants = info.antonyms();
            if !ants.is_empty() {
                print!(
                    "Antonyms: {}",
                    ants[0].if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                );
                for ant in ants.iter().skip(1) {
                    print!(
                        ", {}",
                        ant.if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    );
                }

                println!();
            } else {
                println!(
                    "{}",
                    "[No antonyms available]"
                        .if_supports_color(Stdout, owo_colors::OwoColorize::red)
                        .italic()
                );
            }
        }
    }
    Ok(())
}
