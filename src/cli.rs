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
    /// Display the word's synonyms
    #[clap(short, long)]
    pub synonyms: bool,
    /// Display the word's antonyms
    #[clap(short, long)]
    pub antonyms: bool,
    /// Display all available info about the word
    #[clap(short, long)]
    pub verbose: bool,
    /// The maximum amount of definitions to display.
    /// Defaults to showing all available ones
    #[clap(short, long)]
    pub max: Option<usize>,
}

impl Args {
    pub fn show_all(&mut self) {
        self.examples = true;
        self.phonetic = true;
        self.synonyms = true;
        self.antonyms = true;
    }
}
