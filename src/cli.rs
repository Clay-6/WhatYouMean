use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The word to define
    pub word: String,
}
