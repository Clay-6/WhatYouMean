use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Definition {
    text: Option<String>,
    #[serde(rename = "partOfSpeech", default = "Definition::no_pos")]
    part_of_speech: String,
}

impl Definition {
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn part_of_speech(&self) -> String {
        self.part_of_speech.clone()
    }

    fn no_pos() -> String {
        "[None]".into()
    }
}

pub async fn get_data<T: for<'a> Deserialize<'a>>(
    client: &reqwest::Client,
    url: &str,
) -> Result<T> {
    let res = client.get(url).send().await?.error_for_status()?;

    Ok(serde_json::from_str(&res.text().await?)?)
}

pub fn remove_tags(txt: &str) -> String {
    let re = regex::Regex::new("<[^>]*>").unwrap();

    re.replace_all(txt, "").to_string()
}
