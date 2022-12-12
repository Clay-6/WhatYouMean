use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    text: Option<String>,
    #[serde(default = "Definition::no_pos")]
    part_of_speech: String,
    example_uses: Vec<Example>,
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

pub fn remove_tags(txt: &str) -> String {
    let re = regex::Regex::new("<[^>]*>").unwrap();

    re.replace_all(txt, "").to_string()
}
