use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    time::SystemTime,
};

use anyhow::{anyhow, Error, Result};
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

pub async fn get_wordshk(client: &Client) -> Result<HashMap<String, Vec<String>>> {
    let cache_path = dirs_next::cache_dir()
        .ok_or_else(|| anyhow!("Cannot find cache dir!"))?
        .join("jyutping.json");
    if !cache_path.exists()
        || (cache_path.exists()
            && (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
                - fs::metadata(&cache_path)?
                    .created()?
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs()
                >= 86400))
    {
        let (response_charlist, response_wordlist) = tokio::try_join! {
            async {
                Ok::<_, Error>(client
                    .get("https://words.hk/faiman/analysis/charlist.json")
                    .send()
                    .await?
                    .json::<HashMap<String, HashMap<String, usize>>>()
                    .await?)
            },
            async {
                Ok(client
                    .get("https://words.hk/faiman/analysis/wordslist.json")
                    .send()
                    .await?
                    .json::<HashMap<String, Vec<String>>>()
                    .await?)
            },
        }?;

        let charlist: HashMap<String, Vec<String>> = response_charlist
            .into_iter()
            .map(|(word, jyutping_map)| {
                (word, jyutping_map.keys().map(|x| x.to_string()).collect())
            })
            .collect();
        let mut f = fs::File::create(&cache_path)?;
        let json: HashMap<String, Vec<String>> = charlist
            .into_iter()
            .chain(response_wordlist.into_iter())
            .collect();
        f.write_all(serde_json::to_string(&json)?.as_bytes())?;
    }

    let f = File::open(&cache_path)?;

    Ok(serde_json::from_reader(&f)?)
}
