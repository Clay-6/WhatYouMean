use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The word to define
    pub word: String,
    /// Whether or not to show example usages
    /// alongside the definitions
    #[clap(short = 'e', long)]
    pub show_examples: bool,
}
