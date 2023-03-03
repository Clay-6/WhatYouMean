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
}

/// Different types of rationships between words
pub enum RelationshipType {
    Synonym,
    Antonym,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Syllable {
    pub text: String,
    #[serde(rename = "type")]
    pub ty: Option<String>,
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
            .map(|s| s.chars().filter(|&c| c != '"').collect::<String>())
            .collect();
        let antonyms = get_related(client, word, key, RelationshipType::Antonym)
            .await
            .unwrap_or_default()
            .iter()
            .map(|s| s.chars().filter(|&c| c != '"').collect::<String>())
            .collect();
        let syllables = get_data::<Vec<Syllable>>(
            client,
            &format!("{BASE_URL}/word.json/{word}/hyphenation?api_key={key}"),
        )
        .await
        .unwrap_or_default()
        .iter()
        .map(|s| Syllable {
            text: remove_tags(&s.text),
            ty: s.ty.clone(),
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

    /// Get the word `self` is for
    pub fn word(&self) -> &str {
        &self.word
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
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    /// Get a [`Definition`]'s
    pub fn part_of_speech(&self) -> String {
        self.part_of_speech.clone()
    }

    pub fn examples(&self) -> Vec<String> {
        self.example_uses.iter().map(|e| e.text.clone()).collect()
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
