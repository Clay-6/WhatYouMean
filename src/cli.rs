use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    pub word: Option<String>,
    /// Override API key in the `WORDNIK_API_KEY`
    /// environment variable
    #[clap(long)]
    pub use_key: Option<String>,
    /// Search for a random word
    #[clap(short, long)]
    pub random: bool,
    #[clap(long, default_value_t = 10)]
    pub max: usize,
}
