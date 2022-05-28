use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The word to define
    pub word: String,
}
