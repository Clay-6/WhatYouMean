use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    pub word: String,
    /// Override API key in the `WORDNIK_API_KEY`
    /// environment variable
    #[clap(long)]
    pub use_key: Option<String>,
}
