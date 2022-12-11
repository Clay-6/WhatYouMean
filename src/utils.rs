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
    let mut text = txt.to_string();
    let re = regex::Regex::new("<[^>]*>").unwrap();

    while let Some(mat) = re.find(&text.clone()) {
        for _ in mat.range() {
            /*
            every removal shifts whole string back by one, so match is always at
            index of mat.start
            e.g., if removing numbers >=3:

            1, 2, 3, 4, 5
            1, 2, 4, 5
            1, 2, 5
            1, 2
            */
            text.remove(mat.start());
        }
    }

    text
}
