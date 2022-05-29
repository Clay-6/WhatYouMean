use std::{fmt, str::FromStr};

use anyhow::anyhow;
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
    #[clap(short, long = "lang", default_value_t = Language::GB)]
    pub lang_code: Language,
}

#[derive(Debug)]
pub enum Language {
    GB,
    US,
}

impl FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gb" | "uk" => Ok(Self::GB),
            "us" | "usa" => Ok(Self::US),
            _ => Err(anyhow!("Invalid language code")),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::GB => write!(f, "GB"),
            Language::US => write!(f, "US"),
        }
    }
}
