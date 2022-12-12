use core::fmt;

use color_eyre::eyre::Result;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

    Ok(prons
        .iter()
        .filter(|p| p.raw_type == "IPA")
        .map(|p| p.raw.clone())
        .collect())
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
