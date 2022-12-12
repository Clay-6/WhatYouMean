use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The word to see definitions for
    pub word: Option<String>,
    /// Override API key in the `WORDNIK_API_KEY`
    /// environment variable
    #[clap(long)]
    pub use_key: Option<String>,
    /// Search for a random word
    #[clap(short, long)]
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
}
