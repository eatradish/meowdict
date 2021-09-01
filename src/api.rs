use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoedictDefinition {
    #[serde(rename(deserialize = "type"))]
    pub word_type: Option<String>,
    #[serde(rename(deserialize = "q"))]
    pub quote: Option<Vec<String>>,
    #[serde(rename(deserialize = "e"))]
    pub example: Option<Vec<String>>,
    #[serde(rename(deserialize = "f"))]
    pub def: Option<String>,
    #[serde(rename(deserialize = "l"))]
    pub link: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct MoedictHeteronym {
    #[serde(rename(deserialize = "p"))]
    pub pinyin: Option<String>,
    #[serde(rename(deserialize = "b"))]
    pub bopomofo: Option<String>,
    #[serde(rename(deserialize = "d"))]
    pub definitions: Option<Vec<MoedictDefinition>>,
}

#[derive(Deserialize)]
pub struct MoedictRawResult {
    #[serde(rename(deserialize = "t"))]
    pub title: String,
    pub translation: Option<IndexMap<String, Vec<String>>>,
    #[serde(rename(deserialize = "h"))]
    pub heteronyms: Option<Vec<MoedictHeteronym>>,
    #[serde(rename(deserialize = "English"))]
    pub english: Option<String>,
}

pub async fn request_moedict(keyword: &str, client: &Client) -> Result<MoedictRawResult> {
    let response = client
        .get(format!("https://www.moedict.tw/a/{}.json", keyword))
        .send()
        .await?;

    match response.status().into() {
        200 => {
            let obj: MoedictRawResult = serde_json::from_str(
                response
                    .text()
                    .await?
                    .replace("`", "")
                    .replace("~", "")
                    .as_str(),
            )?;

            Ok(obj)
        }
        404 => Err(anyhow!("Could not find keyword: {}", keyword)),
        _ => Err(anyhow!("Response status code: {}", response.status())),
    }
}
