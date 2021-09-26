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
use tokio::runtime::Runtime;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct MoedictHeteronym {
    #[serde(rename(deserialize = "p"))]
    pub pinyin: Option<String>,
    #[serde(rename(deserialize = "b"))]
    pub bopomofo: Option<String>,
    #[serde(rename(deserialize = "d"))]
    pub definitions: Option<Vec<MoedictDefinition>>,
}

#[derive(Deserialize, Serialize)]
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

lazy_static! {
    static ref CACHE_PATH_DIRECTORY: PathBuf =
        dirs_next::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    static ref CACHE_PATH: PathBuf = CACHE_PATH_DIRECTORY.join("jyutping.json");
}

macro_rules! push_qel {
    ($qel:expr, $result:ident, $count:ident, $t:ident) => {
        if let Some(qel) = &$qel {
            qel.into_iter()
                .for_each(|x| $result.get_mut(&$t).unwrap()[$count].push(x.to_owned()))
        }
    };
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
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
                - fs::metadata(&*CACHE_PATH)?
                    .created()?
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs()
                >= 86400))
    {
        let (response_charlist, response_wordlist) = request_wordshk(client).await?;
        create_dir_all(&*CACHE_PATH_DIRECTORY)?;
        Ok(create_jyutping_cache(
            response_charlist,
            response_wordlist,
            &*CACHE_PATH,
        )?)
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
    let (response_charlist, response_wordlist) = tokio::try_join! {
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
    }?;

    Ok((response_charlist, response_wordlist))
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

#[test]
fn test_to_meowdict_obj() {
    let test_str = r#"{"Deutsch":"ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)","English":"I","c":7,"francais":"je","h":[{"=":"6287","b":"（語音）ㄨㄛˇ","d":[{"f":"自稱。","q":["《易經．中孚卦．九二》：「我有好爵，吾與爾靡之。」","《詩經．小雅．采薇》：「昔我往矣，楊柳依依；今我來思，雨雪霏霏。」"],"type":"代"},{"f":"自稱己方。","q":["《左傳．莊公十年》：「春，齊師伐我。」","《漢書．卷五四．李廣傳》：「我軍雖煩擾，虜亦不得犯我。」"],"type":"代"},{"f":"表示親切之意的語詞。","q":["《論語．述而》：「述而不作，信而好古，竊比於我老彭。」","漢．曹操〈步出夏門行〉：「經過至我碣石，心惆悵我東海。」"],"type":"形"},{"e":["如：「大公無我」。"],"f":"私心、私意。","q":["《論語．子罕》：「毋意，毋必，毋固，毋我。」"],"type":"名"},{"f":"姓。如戰國時有我子。","type":"名"}],"p":"（語音）wǒ"},{"b":"（讀音）ㄜˇ","d":[{"f":"(一)之讀音。"}],"p":"（讀音）ě"}],"n":3,"r":"戈","t":"我","translation":{"Deutsch":["ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)"],"English":["I","me","my"],"francais":["je","moi"]}}"#;
    let json: MoedictRawResult = serde_json::from_str(test_str).unwrap();
    let result_str = serde_json::to_string(&to_meowdict_obj(json)).unwrap();
    let right_str = r#"{"name":"我","english":"I","translation":{"Deutsch":["ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)"],"English":["I","me","my"],"francais":["je","moi"]},"heteronyms":[{"pinyin":"（語音）wǒ","bopomofo":"（語音）ㄨㄛˇ","definitions":{"代":[["自稱。","《易經．中孚卦．九二》：「我有好爵，吾與爾靡之。」","《詩經．小雅．采薇》：「昔我往矣，楊柳依依；今我來思，雨雪霏霏。」"],["自稱己方。","《左傳．莊公十年》：「春，齊師伐我。」","《漢書．卷五四．李廣傳》：「我軍雖煩擾，虜亦不得犯我。」"]],"形":[["表示親切之意的語詞。","《論語．述而》：「述而不作，信而好古，竊比於我老彭。」","漢．曹操〈步出夏門行〉：「經過至我碣石，心惆悵我東海。」"]],"名":[["私心、私意。","《論語．子罕》：「毋意，毋必，毋固，毋我。」","如：「大公無我」。"],["姓。如戰國時有我子。"]]}},{"pinyin":"（讀音）ě","bopomofo":"（讀音）ㄜˇ","definitions":{"notype":[["(一)之讀音。"]]}}]}"#;

    assert_eq!(result_str, right_str);
}
