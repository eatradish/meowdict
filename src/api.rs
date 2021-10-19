use std::{
    collections::HashMap,
    fs::{self, create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::{anyhow, Error, Result};
use futures::future;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct MoedictHeteronym {
    #[serde(rename(deserialize = "p"))]
    pub pinyin: Option<String>,
    #[serde(rename(deserialize = "b"))]
    pub bopomofo: Option<String>,
    #[serde(rename(deserialize = "d"))]
    pub definitions: Option<Vec<MoedictDefinition>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MoedictRawResult {
    #[serde(rename(deserialize = "t"))]
    pub title: String,
    pub translation: Option<IndexMap<String, Vec<String>>>,
    #[serde(rename(deserialize = "h"))]
    pub heteronyms: Option<Vec<MoedictHeteronym>>,
    #[serde(rename(deserialize = "English"))]
    pub english: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MeowdictJyutPingResult {
    pub word: String,
    pub jyutping: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MeowdictJsonResult {
    #[serde(flatten)]
    pub moedict_raw_result: MoedictRawResult,
    pub jyutping: Option<Vec<String>>,
}

lazy_static! {
    static ref CACHE_PATH_DIRECTORY: PathBuf =
        dirs_next::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    static ref CACHE_PATH: PathBuf = CACHE_PATH_DIRECTORY.join("jyutping.json");
}

type JyutPingCharList = HashMap<String, HashMap<String, usize>>;
type JyutPingWordList = HashMap<String, Vec<String>>;

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
    if !CACHE_PATH.exists()
        || (CACHE_PATH.exists()
            && (SystemTime::now()
                .duration_since(fs::metadata(&*CACHE_PATH)?.modified()?)?
                .as_secs()
                >= 24 * 60 * 60))
    {
        let (response_charlist, response_wordlist) = request_wordshk(client).await?;
        create_dir_all(&*CACHE_PATH_DIRECTORY)?;

        create_jyutping_cache(response_charlist, response_wordlist, &*CACHE_PATH)
    } else {
        Ok(serde_json::from_reader(&File::open(&*CACHE_PATH)?)?)
    }
}

fn create_jyutping_cache(
    response_charlist: JyutPingCharList,
    response_wordlist: JyutPingWordList,
    cache_path: &Path,
) -> Result<HashMap<String, Vec<String>>> {
    let charlist: HashMap<String, Vec<String>> = response_charlist
        .into_iter()
        .map(|(word, jyutping_map)| (word, jyutping_map.keys().map(|x| x.to_string()).collect()))
        .collect();
    let mut f = fs::File::create(cache_path)?;
    let json: HashMap<String, Vec<String>> = charlist
        .into_iter()
        .chain(response_wordlist.into_iter())
        .collect();
    f.write_all(serde_json::to_string(&json)?.as_bytes())?;

    Ok(json)
}

async fn request_wordshk(client: &Client) -> Result<(JyutPingCharList, JyutPingWordList)> {
    tokio::try_join! {
        async {
            Ok::<_, Error>(client
                .get("https://words.hk/faiman/analysis/charlist.json")
                .send()
                .await?
                .json::<JyutPingCharList>()
                .await?)
        },
        async {
            Ok(client
                .get("https://words.hk/faiman/analysis/wordslist.json")
                .send()
                .await?
                .json::<JyutPingWordList>()
                .await?)
        },
    }
}

pub async fn get_dict_result(client: &Client, words: &[String]) -> Result<Vec<MoedictRawResult>> {
    let mut tesk = Vec::new();
    for word in words {
        tesk.push(request_moedict(word, client));
    }

    Ok(future::try_join_all(tesk).await?)
}

pub async fn get_jyutping_result(
    client: &Client,
    words: &[String],
) -> Result<Vec<MeowdictJyutPingResult>> {
    let mut result = Vec::new();
    let jyutping_map = get_wordshk(client).await?;
    for word in words {
        result.push(MeowdictJyutPingResult {
            word: word.to_owned(),
            jyutping: jyutping_map
                .get(word)
                .ok_or_else(|| anyhow!("Cannot find jyutping: {}", word))?
                .to_owned(),
        });
    }

    Ok(result)
}

pub async fn set_json_result(client: &Client, words: &[String]) -> Result<Vec<MeowdictJsonResult>> {
    let mut result = Vec::new();
    let moedict_raw_results = get_dict_result(client, words).await?;
    let jyutping = match get_jyutping_result(client, words).await {
        Ok(jyutping) => Some(jyutping),
        Err(_) => None,
    };

    for (index, moedict_raw_result) in moedict_raw_results.iter().enumerate() {
        let jyutping = if let Some(ref jyutping) = jyutping {
            Some(jyutping[index].jyutping.clone())
        } else {
            None
        };
        result.push(MeowdictJsonResult {
            moedict_raw_result: moedict_raw_result.to_owned(),
            jyutping,
        });
    }

    Ok(result)
}

#[test]
fn test_cache_jyutping_result() {
    use tempfile::NamedTempFile;
    let mut response_charlist: JyutPingCharList = HashMap::new();
    let mut charlist_value = HashMap::new();
    charlist_value.insert("ngo5".to_string(), 0usize);
    response_charlist.insert("我".to_string(), charlist_value);
    let mut response_wordlist: JyutPingWordList = HashMap::new();
    response_wordlist.insert("我哋".to_string(), vec!["ngo5 dei6".to_string()]);
    let file = NamedTempFile::new().unwrap();
    let json = create_jyutping_cache(response_charlist, response_wordlist, file.path()).unwrap();

    assert_eq!(json["我"], vec!["ngo5".to_string()]);
    assert_eq!(json["我哋"], vec!["ngo5 dei6".to_string()]);
}

#[tokio::test]
async fn test_moedict_api_result() {
    let client = reqwest::Client::new();
    let keyword = "我";
    let result = request_moedict(keyword, &client).await.unwrap();
    let result_str = serde_json::to_string(&result).unwrap();
    let right_result = r#"{"title":"我","translation":{"Deutsch":["ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)"],"English":["I","me","my"],"francais":["je","moi"]},"heteronyms":[{"pinyin":"（語音）wǒ","bopomofo":"（語音）ㄨㄛˇ","definitions":[{"word_type":"代","quote":["《易經．中孚卦．九二》：「我有好爵，吾與爾靡之。」","《詩經．小雅．采薇》：「昔我往矣，楊柳依依；今我來思，雨雪霏霏。」"],"example":null,"def":"自稱。","link":null},{"word_type":"代","quote":["《左傳．莊公十年》：「春，齊師伐我。」","《漢書．卷五四．李廣傳》：「我軍雖煩擾，虜亦不得犯我。」"],"example":null,"def":"自稱己方。","link":null},{"word_type":"形","quote":["《論語．述而》：「述而不作，信而好古，竊比於我老彭。」","漢．曹操〈步出夏門行〉：「經過至我碣石，心惆悵我東海。」"],"example":null,"def":"表示親切之意的語詞。","link":null},{"word_type":"名","quote":["《論語．子罕》：「毋意，毋必，毋固，毋我。」"],"example":["如：「大公無我」。"],"def":"私心、私意。","link":null},{"word_type":"名","quote":null,"example":null,"def":"姓。如戰國時有我子。","link":null}]},{"pinyin":"（讀音）ě","bopomofo":"（讀音）ㄜˇ","definitions":[{"word_type":null,"quote":null,"example":null,"def":"(一)之讀音。","link":null}]}],"english":"I"}"#;

    assert_eq!(result_str, right_result);
}

#[tokio::test]
async fn test_wordshk_api_result() {
    let client = reqwest::Client::new();
    let word_1 = "我";
    let word_2 = "我哋";
    let (wordshk_charlist, wordshk_wordlist) = request_wordshk(&client).await.unwrap();
    let mut result_1 = HashMap::new();
    result_1.insert("ngo5".to_string(), 41usize);
    let result_2 = vec!["ngo5 dei6".to_string()];

    assert_eq!(wordshk_charlist[word_1], result_1);
    assert_eq!(wordshk_wordlist[word_2], result_2);
}
