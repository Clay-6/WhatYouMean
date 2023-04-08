use core::fmt;

use color_eyre::{eyre::Result, Report};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};

const BASE_URL: &str = "https://api.wordnik.com/v4";

/// Stores all available info for a word
#[derive(Debug, Serialize)]
pub struct WordInfo {
    word: String,
    definitions: Vec<Definition>,
    pronunciations: Vec<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
    syllables: Vec<Syllable>,
}

/// Info relating to a word's definition
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    text: Option<String>,
    #[serde(default = "Definition::no_pos")]
    part_of_speech: String,
    example_uses: Vec<Example>,
    source_dictionary: SourceDict,
    attribution_url: String,
}

/// Different types of rationships between words
pub enum RelationshipType {
    Synonym,
    Antonym,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Syllable {
    text: String,
    #[serde(rename = "type")]
    ty: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum SourceDict {
    #[strum(serialize = "ahd-5", to_string = "AHD-5")]
    #[serde(rename(deserialize = "ahd-5"))]
    Ahd5,
    #[strum(ascii_case_insensitive)]
    Century,
    #[strum(ascii_case_insensitive, to_string = "GCIDE")]
    Gcide,
    #[strum(ascii_case_insensitive)]
    Wiktionary,
    #[strum(ascii_case_insensitive)]
    Webster,
    #[strum(ascii_case_insensitive)]
    Wordnet,
}

/// Get a [`Vec`] of all a word's available [`Definition`]s
pub async fn get_definitions(client: &Client, word: &str, key: &str) -> Result<Vec<Definition>> {
    get_data::<Vec<Definition>>(
        client,
        &format!("{BASE_URL}/word.json/{word}/definitions?api_key={key}"),
    )
    .await
}

/// Get a random word
pub async fn get_random_word(client: &Client, key: &str) -> Result<String> {
    let data = get_data::<Value>(
        client,
        &format!("{BASE_URL}/words.json/randomWord?api_key={key}"),
    )
    .await?;
    Ok(data["word"]
        .to_string()
        .chars()
        .filter(|&c| c != '"')
        .collect())
}

pub async fn get_wotd(client: &Client, key: &str) -> Result<String> {
    let data = get_data::<Value>(
        client,
        &format!("{BASE_URL}/words.json/wordOfTheDay?api_key={key}"),
    )
    .await?;
    Ok(data["word"]
        .to_string()
        .chars()
        .filter(|&c| c != '"')
        .collect())
}

/// Get a [`Vec`] of a word's IPA phonetic representations
pub async fn get_phonetics(client: &reqwest::Client, word: &str, key: &str) -> Result<Vec<String>> {
    let url = format!("{BASE_URL}/word.json/{word}/pronunciations?api_key={key}");
    let prons = get_data::<Vec<Pronunciation>>(client, &url).await?;

    let ipa_prons = prons
        .iter()
        .filter(|p| p.raw_type == "IPA")
        .map(|p| p.raw.clone())
        .collect::<Vec<_>>();

    if ipa_prons.is_empty() {
        Err(Report::msg("No IPA phonetics"))
    } else {
        Ok(ipa_prons)
    }
}

/// Get words related to a `word` in different ways
pub async fn get_related(
    client: &Client,
    word: &str,
    key: &str,
    rel_type: RelationshipType,
) -> Result<Vec<String>> {
    let url = format!(
        "{BASE_URL}/word.json/{word}/relatedWords?&relationshipTypes={rel_type}&api_key={key}",
    );
    let val = get_data::<Value>(client, &url).await?;
    Ok(val[0]["words"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w.to_string())
        .collect())
}

/// Remove HTML tags from text
pub fn remove_tags(txt: &str) -> String {
    let re = regex::Regex::new("<[^>]*>").unwrap();

    re.replace_all(txt, "").to_string()
}

impl WordInfo {
    /// Constructs a [`WordInfo`] by fetching data from Wordnik's API
    pub async fn fetch(word: &str, client: &Client, key: &str) -> Result<Self> {
        let syl_url = format!("{BASE_URL}/word.json/{word}/hyphenation?api_key={key}");
        let (definitions, pronunciations, synonyms, antonyms, syllables) = tokio::join!(
            get_definitions(client, word, key),
            get_phonetics(client, word, key),
            get_related(client, word, key, RelationshipType::Synonym),
            get_related(client, word, key, RelationshipType::Antonym),
            get_data::<Vec<Syllable>>(client, &syl_url,)
        );
        let definitions = definitions?
            .into_iter()
            .map(|d| Definition {
                text: d.text.as_ref().map(|text| remove_tags(text)),
                example_uses: d
                    .example_uses
                    .iter()
                    .map(|e| Example {
                        text: remove_tags(&e.text),
                    })
                    .collect(),
                ..d
            })
            .collect();
        let pronunciations = pronunciations.unwrap_or_default();
        let synonyms = synonyms
            .unwrap_or_default()
            .iter()
            .map(|s| s.chars().filter(|&c| c != '"').collect::<String>())
            .collect();
        let antonyms = antonyms
            .unwrap_or_default()
            .iter()
            .map(|s| s.chars().filter(|&c| c != '"').collect::<String>())
            .collect();
        let syllables = syllables
            .unwrap_or_default()
            .into_iter()
            .map(|s| Syllable {
                text: remove_tags(&s.text),
                ..s
            })
            .collect();

        Ok(Self {
            word: word.to_owned(),
            definitions,
            pronunciations,
            synonyms,
            antonyms,
            syllables,
        })
    }

    /// Get the word's pronunciations
    pub fn pronunciations(&self) -> &[String] {
        &self.pronunciations
    }

    /// Get the word's definitions as [`Definition`] structs
    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    /// Get the word's synonyms
    pub fn synonyms(&self) -> &[String] {
        &self.synonyms
    }

    /// Get the word's antonyms
    pub fn antonyms(&self) -> &[String] {
        &self.antonyms
    }

    pub fn syllables(&self) -> &[Syllable] {
        &self.syllables
    }
}

impl Definition {
    /// Get the text of a [`Definition`] if it exists
    pub fn text(&self) -> Option<&str> {
        self.text.as_ref().map(|t| t.as_ref())
    }

    /// Get a [`Definition`]'s
    pub fn part_of_speech(&self) -> &str {
        self.part_of_speech.as_ref()
    }

    /// Return a word's top example
    pub fn top_example(&self) -> Option<&str> {
        self.example_uses.first().map(|e| e.text.as_ref())
    }

    pub fn source(&self) -> SourceDict {
        self.source_dictionary
    }

    pub fn attrib_url(&self) -> &str {
        &self.attribution_url
    }

    fn no_pos() -> String {
        "[None]".to_owned()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pronunciation {
    raw: String,
    raw_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Example {
    text: String,
}

async fn get_data<T: for<'a> Deserialize<'a>>(client: &Client, url: &str) -> Result<T> {
    let res = client
        .get(url)
        .send()
        .await?
        .error_for_status()
        .map_err(|e| e.without_url())?;

    Ok(res.json().await?)
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RelationshipType::Synonym => "synonym",
                RelationshipType::Antonym => "antonym",
            },
        )
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.text,
            if let Some(t) = &self.ty {
                format!("({t})")
            } else {
                String::new()
            }
        )
    }
}
