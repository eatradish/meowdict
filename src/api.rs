use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub struct MoedictItemResult {
    pub pinyin: Option<String>,
    pub bopomofo: Option<String>,
    pub defination: Option<IndexMap<String, Vec<Vec<String>>>>,
}

#[derive(Serialize, Deserialize)]
pub struct MoedictJson {
    #[serde(flatten)]
    json: HashMap<String, Value>,
}

pub async fn request_moedict(keyword: &str) -> Result<MoedictJson> {
    let response = 
        reqwest::get(format!("https://www.moedict.tw/a/{}.json", keyword)).await?;

    match response.status().into() {
        200 => Ok(response.json::<MoedictJson>().await?),
        404 => Err(anyhow!("Could not find keyword: {}", keyword)),
        _ => Err(anyhow!("Response status code: {}", response.status())),
    }
}

fn api_get_h(json: &HashMap<String, Value>) -> Result<Vec<Value>> {
    let h = json
        .get("h")
        .ok_or_else(|| anyhow!("Failed to get dict!"))?
        .as_array()
        .ok_or_else(|| anyhow!("dict is not array!"))?
        .to_owned();

    Ok(h)
}

fn api_get_translations(json: &HashMap<String, Value>) -> Result<IndexMap<String, Vec<String>>> {
    let translation = json
        .get("translation")
        .ok_or_else(|| anyhow!("This item has no translation!"))?
        .as_object()
        .ok_or_else(|| anyhow!("translation is not Object!"))?;
    let mut translation_indexmap: IndexMap<String, Vec<String>> = IndexMap::new();
    for (lang, lang_value) in translation {
        let lang_value = lang_value
            .as_array()
            .ok_or_else(|| anyhow!("lang_value is not Array!"))?;
        let mut lang_vec = Vec::new();
        for i in lang_value {
            let i = i
                .as_str()
                .ok_or_else(|| anyhow!("lang_value item is not String!"))?;
            lang_vec.push(i.to_string());
        }
        translation_indexmap.insert(lang.to_string(), lang_vec);
    }

    Ok(translation_indexmap)
}

fn api_get_pinyin(dict_val: &Value) -> Result<String, anyhow::Error> {
    let pinyin = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("p")
        .ok_or_else(|| anyhow!("Caanot get p!"))?
        .as_str()
        .ok_or_else(|| anyhow!("p is not String!"))?
        .to_owned();

    Ok(pinyin.replace("~", "").replace("`", ""))
}

fn api_get_defination(dict_val: &Value) -> Result<IndexMap<String, Vec<Vec<String>>>> {
    let mut defination_item = IndexMap::new();
    let dicts_item = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("d")
        .ok_or_else(|| anyhow!("Cannot find d!"))?
        .as_array()
        .ok_or_else(|| anyhow!("d is not array!"))?;
    let mut count: usize = 0;
    for dict_item in dicts_item {
        let dict_item = dict_item
            .as_object()
            .ok_or_else(|| anyhow!("d item is not object!"))?;
        let t = if let Some(v) = dict_item.get("type") {
            v.as_str()
                .ok_or_else(|| anyhow!("This item is not String!"))?
                .replace("~", "")
                .replace("`", "")
        } else {
            "notype".to_string()
        };
        if defination_item.get(&t).is_none() {
            defination_item.insert(t.to_string(), vec![Vec::new()]);
            count = 0;
        } else {
            defination_item.get_mut(&t).unwrap().push(Vec::new());
        }
        if let Some(v) = dict_item.get("f") {
            defination_item.get_mut(&t).unwrap()[count].push(
                v.as_str()
                    .ok_or_else(|| anyhow!("This item is not String!"))?
                    .to_string()
                    .replace("~", "")
                    .replace("`", ""),
            );
        }
        for i in &["q", "e", "l"] {
            if let Some(v) = dict_item.get(&i.to_string()) {
                let item_list = v
                    .as_array()
                    .ok_or_else(|| anyhow!("This item is not arrays!"))?;
                for j in item_list {
                    if let Some(j) = j.as_str() {
                        defination_item.get_mut(&t).unwrap()[count]
                            .push(j.to_string().replace("~", "").replace("`", ""));
                    }
                }
            }
        }
        count += 1;
    }

    Ok(defination_item)
}

fn api_get_bopomofo(dict_val: &Value) -> Result<String> {
    let bopomofo = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("b")
        .ok_or_else(|| anyhow!("Caanot get b!"))?
        .as_str()
        .ok_or_else(|| anyhow!("b is not String!"))?
        .to_owned();

    Ok(bopomofo.replace("`", "").replace("~", ""))
}

fn api_get_english(json: &HashMap<String, Value>) -> Result<String> {
    let english = json
        .get("English")
        .ok_or_else(|| anyhow!("This item has no English!"))?
        .as_str()
        .ok_or_else(|| anyhow!("English is not String!"))?
        .to_owned();

    Ok(english)
}

impl MoedictJson {
    pub fn get_translations(&self) -> Option<IndexMap<String, Vec<String>>> {
        match api_get_translations(&self.json) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn get_moedict_item_result_vec(&self) -> Vec<MoedictItemResult> {
        let mut moedict_item_result = Vec::new();
        let dict = match api_get_h(&self.json) {
            Ok(v) => Some(v),
            Err(_) => None,
        };
        if let Some(dict) = dict {
            for dict_val in dict {
                let defination = match api_get_defination(&dict_val) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                let pinyin = match api_get_pinyin(&dict_val) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                let bopomofo = match api_get_bopomofo(&dict_val) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                moedict_item_result.push(MoedictItemResult {
                    pinyin,
                    bopomofo,
                    defination,
                });
            }
        }

        moedict_item_result
    }

    pub fn get_english(&self) -> Option<String> {
        match api_get_english(&self.json) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
