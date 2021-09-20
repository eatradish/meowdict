use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    time::SystemTime,
};

use anyhow::{anyhow, Error, Result};
use futures::future;
use indexmap::IndexMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

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

#[derive(Serialize)]
pub struct MeowdictResult {
    pub name: String,
    pub english: Option<String>,
    pub translation: Option<IndexMap<String, Vec<String>>>,
    pub heteronyms: Option<Vec<MeowdictHeteronym>>,
}

#[derive(Serialize)]
pub struct MeowdictHeteronym {
    pub pinyin: Option<String>,
    pub bopomofo: Option<String>,
    pub definitions: Option<IndexMap<String, Vec<Vec<String>>>>,
}

#[derive(Serialize)]
pub struct MeowdictJyutPingResult {
    pub word: String,
    pub jyutping: Vec<String>,
}

macro_rules! push_qel {
    ($qel:expr, $result:ident, $count:ident, $t:ident) => {
        if let Some(qel) = &$qel {
            qel.into_iter()
                .for_each(|x| $result.get_mut(&$t).unwrap()[$count].push(x.to_owned()))
        }
    };
}

async fn request_moedict(keyword: &str, client: &Client) -> Result<MoedictRawResult> {
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

async fn get_wordshk(client: &Client) -> Result<HashMap<String, Vec<String>>> {
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

pub fn to_meowdict_obj(moedict_obj: MoedictRawResult) -> MeowdictResult {
    let name = moedict_obj.title;
    let english = moedict_obj.english;
    let translation = moedict_obj.translation;
    let meowdict_heteronyms = if let Some(heteronyms) = moedict_obj.heteronyms {
        let mut result = Vec::new();
        for item in heteronyms {
            let word_type = item
                .definitions
                .map(|definition| definition_formatter(&definition));
            result.push(MeowdictHeteronym {
                pinyin: item.pinyin,
                bopomofo: item.bopomofo,
                definitions: word_type,
            });
        }

        Some(result)
    } else {
        None
    };

    MeowdictResult {
        name,
        english,
        translation,
        heteronyms: meowdict_heteronyms,
    }
}

fn definition_formatter(definitions: &[MoedictDefinition]) -> IndexMap<String, Vec<Vec<String>>> {
    let mut result = IndexMap::new();
    let mut count: usize = 0;
    for i in definitions {
        let t = if let Some(word_type) = i.word_type.to_owned() {
            word_type
        } else {
            "notype".to_string()
        };
        if result.get(&t).is_none() {
            result.insert(t.to_owned(), vec![Vec::new()]);
            count = 0;
        } else {
            result.get_mut(&t).unwrap().push(Vec::new());
        }
        if let Some(f) = &i.def {
            result.get_mut(&t).unwrap()[count].push(f.to_owned());
        }
        push_qel!(i.quote, result, count, t);
        push_qel!(i.example, result, count, t);
        push_qel!(i.link, result, count, t);
        count += 1;
    }

    result
}

pub fn get_dict_result(
    runtime: &Runtime,
    client: &Client,
    words: Vec<String>,
) -> Result<Vec<MeowdictResult>> {
    runtime.block_on(async {
        let mut result = Vec::new();
        let mut tesk = Vec::new();
        for word in &words {
            tesk.push(request_moedict(word, client));
        }
        let response_results = future::try_join_all(tesk).await?;
        for i in response_results {
            result.push(to_meowdict_obj(i));
        }

        Ok(result)
    })
}

pub fn get_jyutping_result(
    client: &Client,
    runtime: &Runtime,
    words: Vec<String>,
) -> Result<Vec<MeowdictJyutPingResult>> {
    runtime.block_on(async {
        let mut result = Vec::new();
        let jyutping_map = get_wordshk(client).await?;
        for word in &words {
            result.push(MeowdictJyutPingResult {
                word: word.to_owned(),
                jyutping: jyutping_map
                    .get(word)
                    .ok_or_else(|| anyhow!("Cannot find jyutping: {}", word))?
                    .to_owned(),
            });
        }

        Ok(result)
    })
}
