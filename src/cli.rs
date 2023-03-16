use clap::Parser;

use crate::utils::SourceDict;

#[derive(Debug, Clone, Parser)]
#[command(name = "WhatYouMean")]
#[command(author = "Clay66 <clay@clay66.dev>")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The word to see definitions for
    pub word: Option<String>,
    /// Override API key in the `WORDNIK_API_KEY`
    /// environment variable
    #[clap(long)]
    pub use_key: Option<String>,
    /// Search for a random word
    #[clap(long)]
    pub random: bool,
    /// The maximum amount of definitions to
    /// display
    #[clap(long, default_value_t = 10)]
    pub max: usize,
    /// Disable coloured output
    #[clap(long)]
    pub no_colour: bool,
    /// Display the phonetics for a word
    #[clap(short, long)]
    pub phonetics: bool,
    /// Show example usage for a word
    #[clap(short, long)]
    pub examples: bool,
    /// Show a word's synonyms
    #[clap(short, long)]
    pub synonyms: bool,
    /// Show a word's antonyms
    #[clap(short, long)]
    pub antonyms: bool,
    /// Display all available info about a word
    #[clap(short, long)]
    pub verbose: bool,
    /// Output all data for a word in JSON format
    #[clap(long)]
    pub json: bool,
    /// Fetch info for Wordnik's Word of the Day
    #[clap(long)]
    pub wotd: bool,
    /// Return the syllables for a word
    #[clap(long)]
    pub syllables: bool,
    /// Show the source for each definition
    #[clap(short = 'S', long)]
    pub sources: bool,
    /// Show results only from the specified source dictionary
    ///
    /// One of `ahd-5`, `century`, `gcide`, `wiktionary`, `webster`, or `wordnet`
    #[clap(short, long)]
    pub from: Option<SourceDict>,
}
