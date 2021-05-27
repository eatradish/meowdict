use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;
use indexmap::IndexMap;

pub struct MoedictItemResult {
    pub pinyin: Option<String>,
    pub bopomofo: Option<String>,
    pub translation: Option<IndexMap<String, Vec<String>>>,
    pub defination: Option<IndexMap<String, Vec<Vec<String>>>>,
}

fn format_result(result: String) -> Result<Vec<MoedictItemResult>> {
    let json: HashMap<String, Value> = serde_json::from_str(&result)?;
    let dict = json
        .get("h")
        .ok_or_else(|| anyhow!("Failed to get dict!"))?
        .as_array()
        .ok_or_else(|| anyhow!("dict is not array!"))?;
    let mut result = Vec::new();
    for dict_val in dict {
        let defination_item = match get_defination(dict_val) {
            Ok(v) => Some(v),
            Err(_) => None
        };
        let pinyin = match get_pinyin(dict_val) {
            Ok(v) => Some(v),
            Err(_) => None
        };
        let translation = match get_translations(json.clone()) {
            Ok(v) => Some(v),
            Err(_) => None
        };
        let bopomofo = match get_bopomofo(dict_val) {
            Ok(v) => Some(v),
            Err(_) => None
        };
        result.push(MoedictItemResult {
            pinyin,
            bopomofo,
            translation,
            defination: defination_item,
        })
    }

    Ok(result)
}

fn request_moedict(keyword: &str) -> Result<String> {
    let response =
        reqwest::blocking::get(format!("https://www.moedict.tw/a/{}.json", keyword))?.text()?;
    let result = response.replace("~", "").replace("`", "");
    if result.contains("<title>404 Not Found</title>") {
        return Err(anyhow!("Could not find keyword: {}", keyword));
    }

    Ok(result)
}

fn get_translations(json: HashMap<String, Value>) -> Result<IndexMap<String, Vec<String>>, anyhow::Error> {
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

fn get_pinyin(dict_val: &Value) -> Result<String, anyhow::Error> {
    let pinyin = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("p")
        .ok_or_else(|| anyhow!("Caanot get d!"))?
        .as_str()
        .ok_or_else(|| anyhow!("p is not String!"))?
        .to_owned();

    Ok(pinyin)
}

fn get_defination(dict_val: &Value) -> Result<IndexMap<String, Vec<Vec<String>>>> {
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
        } else {
            "notype"
        };
        if defination_item.get(t).is_none() {
            defination_item.insert(t.to_string(), vec![Vec::new()]);
            count = 0;
        } else {
            defination_item.get_mut(t).unwrap().push(Vec::new());
        }
        if let Some(v) = dict_item.get("f") {
            defination_item.get_mut(t).unwrap()[count].push(
                v.as_str()
                    .ok_or_else(|| anyhow!("This item is not String!"))?
                    .to_string(),
            );
        }
        for i in &["q", "e", "l"] {
            if let Some(v) = dict_item.get(&i.to_string()) {
                let item_list = v
                    .as_array()
                    .ok_or_else(|| anyhow!("This item is not arrays!"))?;
                for j in item_list {
                    if let Some(j) = j.as_str() {
                        defination_item.get_mut(t).unwrap()[count].push(j.to_string());
                    }
                }
            }
        }
        count += 1;
    }

    Ok(defination_item)
}

fn get_bopomofo(dict_val: &Value) -> Result<String> {
    let bopomofo = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("b")
        .ok_or_else(|| anyhow!("Caanot get b!"))?
        .as_str()
        .ok_or_else(|| anyhow!("b is not String!"))?
        .to_owned();

    Ok(bopomofo)
}

pub fn get_result(keyword: &str) -> Result<Vec<MoedictItemResult>> {
    let resp = request_moedict(keyword)?;
    let result = format_result(resp)?;

    Ok(result)
}
