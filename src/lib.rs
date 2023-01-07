use core::fmt;

use color_eyre::{eyre::Result, Report};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct WordInfo {
    definitions: Vec<Definition>,
    pronunciations: Vec<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

impl WordInfo {
    pub async fn fetch(word: &str, client: &Client, url: &str, key: &str) -> Result<Self> {
        let definitions = get_data::<Vec<Definition>>(client, url)
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
            definitions,
            pronunciations,
            synonyms,
            antonyms,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    text: Option<String>,
    #[serde(default = "Definition::no_pos")]
    part_of_speech: String,
    example_uses: Vec<Example>,
}

pub enum RelationshipType {
    Synonym,
    Antonym,
}

impl Definition {
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn part_of_speech(&self) -> String {
        self.part_of_speech.clone()
    }

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

pub async fn get_data<T: for<'a> Deserialize<'a>>(
    client: &reqwest::Client,
    url: &str,
) -> Result<T> {
    let res = client.get(url).send().await?.error_for_status()?;

    Ok(serde_json::from_str(&res.text().await?)?)
}

pub async fn get_phonetics(client: &reqwest::Client, word: &str, key: &str) -> Result<Vec<String>> {
    let url = format!("https://api.wordnik.com/v4/word.json/{word}/pronunciations?api_key={key}");
    let res = client.get(url).send().await?.error_for_status()?;

    let prons: Vec<Pronunciation> = serde_json::from_str(&res.text().await?)?;

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

pub async fn get_related(
    client: &reqwest::Client,
    word: &str,
    key: &str,
    rel_type: RelationshipType,
) -> Result<Vec<String>> {
    let url = format!(
        "https://api.wordnik.com/v4/word.json/{}/relatedWords?&relationshipTypes={}&api_key={}",
        word, rel_type, key
    );
    let res = client.get(url).send().await?.error_for_status()?;

    let val: Value = serde_json::from_str(&res.text().await?)?;
    Ok(val[0]["words"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w.to_string())
        .collect())
}

pub fn remove_tags(txt: &str) -> String {
    let re = regex::Regex::new("<[^>]*>").unwrap();

    re.replace_all(txt, "").to_string()
}
