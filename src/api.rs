use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

pub struct MoedictItemResult {
    pub pinyin: String,
    pub bopomofo: String,
    pub translation: HashMap<String, Vec<String>>,
    pub defination: HashMap<String, Vec<Vec<String>>>,
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
        let defination_item = get_defination(dict_val)?;
        let pinyin = get_pinyin(dict_val)?;
        let translation = get_translations(json.clone())?;
        let bopomofo = get_bopomofo(dict_val)?;
        result.push(MoedictItemResult {
            pinyin: pinyin.to_string(),
            bopomofo: bopomofo.to_string(),
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

fn get_translations(
    json: HashMap<String, Value>,
) -> Result<HashMap<String, Vec<String>>, anyhow::Error> {
    let translation = json
        .get("translation")
        .ok_or_else(|| anyhow!("This item has no translation!"))?
        .as_object()
        .ok_or_else(|| anyhow!("translation is not Object!"))?;
    let mut translation_hashmap: HashMap<String, Vec<String>> = HashMap::new();
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
        translation_hashmap.insert(lang.to_string(), lang_vec);
    }
    Ok(translation_hashmap)
}

fn get_pinyin(dict_val: &Value) -> Result<&str, anyhow::Error> {
    let pinyin = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("p")
        .ok_or_else(|| anyhow!("Caanot get d!"))?
        .as_str()
        .ok_or_else(|| anyhow!("p is not String!"))?;

    Ok(pinyin)
}

fn get_defination(dict_val: &Value) -> Result<HashMap<String, Vec<Vec<String>>>> {
    let mut defination_item = HashMap::new();
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
            let s = string_split_new_line(
                v.as_str()
                    .ok_or_else(|| anyhow!("This item is not String!"))?
                    .to_string(),
            );
            defination_item.get_mut(t).unwrap()[count].push(s);
        }
        for i in &["q", "e", "l"] {
            if let Some(v) = dict_item.get(&i.to_string()) {
                let item_list = v
                    .as_array()
                    .ok_or_else(|| anyhow!("This item is not arrays!"))?;
                for j in item_list {
                    if let Some(j) = j.as_str() {
                        let s = string_split_new_line(j.to_string());
                        defination_item.get_mut(t).unwrap()[count].push(s);
                    }
                }
            }
        }
        count += 1;
    }

    Ok(defination_item)
}

fn string_split_new_line(s: String) -> String {
    let mut remaining = s
        .split("")
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>();
    let mut result = String::new();
    while remaining.len() > 32 {
        result.push_str(&format!("{}\n", remaining[..32].join("")));
        remaining = remaining[32..].to_vec();
        if remaining.len() < 32 {
            result.push_str(&remaining.join(""));
        }
    }
    if result.is_empty() {
        result = s
    }

    result
}

fn get_bopomofo(dict_val: &Value) -> Result<&str> {
    let bopomofo = dict_val
        .as_object()
        .ok_or_else(|| anyhow!("dict item is not object!"))?
        .get("b")
        .ok_or_else(|| anyhow!("Caanot get b!"))?
        .as_str()
        .ok_or_else(|| anyhow!("b is not String!"))?;

    Ok(bopomofo)
}

pub fn get_result(keyword: &str) -> Result<Vec<MoedictItemResult>> {
    let resp = request_moedict(keyword)?;
    let result = format_result(resp)?;

    Ok(result)
}
