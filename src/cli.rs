use clap::Parser;
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The word to define
    pub word: String,
    /// Show example usage alongside the definitions
    #[clap(short, long)]
    pub examples: bool,
    /// Remove the word types from the definitions
    #[clap(long)]
    pub no_types: bool,
    /// Disable coloured output
    #[clap(long)]
    pub no_colour: bool,
    /// Display the word's phonetic form for its pronounciation
    #[clap(short, long)]
    pub phonetic: bool,
}
