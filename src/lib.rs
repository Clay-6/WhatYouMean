use core::fmt;

use color_eyre::{eyre::Result, Report};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_URL: &str = "https://api.wordnik.com/v4";

/// Stores all available info for a word
#[derive(Debug, Serialize)]
pub struct WordInfo {
    word: String,
    definitions: Vec<Definition>,
    pronunciations: Vec<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

/// Info relating to a word's definition
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    text: Option<String>,
    #[serde(default = "Definition::no_pos")]
    part_of_speech: String,
    example_uses: Vec<Example>,
}

/// Different types of rationships between words
pub enum RelationshipType {
    Synonym,
    Antonym,
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
        .filter(|c| *c != '"')
        .collect::<String>())
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
        let definitions = get_definitions(client, word, key)
            .await?
            .iter()
            .map(|d| Definition {
                text: d.text.as_ref().map(|text| remove_tags(text)),
                part_of_speech: d.part_of_speech.clone(),
                example_uses: d.example_uses.clone(),
            })
            .collect();
        let pronunciations = get_phonetics(client, word, key).await.unwrap_or_default();
        let synonyms = get_related(client, word, key, RelationshipType::Synonym)
            .await
            .unwrap_or_default()
            .iter()
            .map(|s| s.chars().filter(|c| *c != '"').collect::<String>())
            .collect();
        let antonyms = get_related(client, word, key, RelationshipType::Antonym)
            .await
            .unwrap_or_default()
            .iter()
            .map(|s| s.chars().filter(|c| *c != '"').collect::<String>())
            .collect();

        Ok(Self {
            word: word.to_owned(),
            definitions,
            pronunciations,
            synonyms,
            antonyms,
        })
    }
}

impl Definition {
    /// Get the text of a [`Definition`] if it exists
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    /// Get a [`Definition`]'s
    pub fn part_of_speech(&self) -> String {
        self.part_of_speech.clone()
    }

    /// Return a word's top example
    pub fn top_example(&self) -> String {
        if self.example_uses.is_empty() {
            "".into()
        } else {
            self.example_uses
                .iter()
                .map(|e| e.text.clone())
                .collect::<Vec<_>>()[0]
                .clone()
        }
    }

    fn no_pos() -> String {
        "[None]".into()
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

    Ok(serde_json::from_str(&res.text().await?)?)
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
