use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoedictDefinition {
    #[serde(rename(deserialize = "type"))]
    pub word_type: Option<String>,
    pub q: Option<Vec<String>>,
    pub e: Option<Vec<String>>,
    pub f: Option<String>,
    pub l: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct MoedictHeteronym {
    pub p: Option<String>,
    pub b: Option<String>,
    pub d: Option<Vec<MoedictDefinition>>,
}

#[derive(Deserialize)]
pub struct MoedictRawResult {
    pub t: String,
    pub translation: Option<IndexMap<String, Vec<String>>>,
    pub h: Option<Vec<MoedictHeteronym>>,
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
