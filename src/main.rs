mod cli;

use clap::Parser;
use cli::Args;
use color_eyre::eyre::Result;
use owo_colors::{OwoColorize, Stream::Stdout};
use reqwest::Client;
use whatyoumean::{
    get_definitions, get_phonetics, get_random_word, get_related, remove_tags, RelationshipType,
    WordInfo,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

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

    let key = args.use_key.unwrap_or(std::env::var("WORDNIK_API_KEY")?);

    let random_word = if args.random {
        get_random_word(&client, &key).await?
    } else {
        "".to_string()
    };
    let word = &args.word.unwrap_or(random_word);

    if args.json {
        let info = WordInfo::fetch(word, &client, &key).await?;
        println!("{}", serde_json::to_string_pretty(&info)?)
    } else {
        let defs = get_definitions(&client, word, &key).await?;

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
                        format!("e.g. {}", def.top_example())
                            .if_supports_color(Stdout, |t| t.green())
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
    }
    Ok(())
}
