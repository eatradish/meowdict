use anyhow::{anyhow, Result};
use owo_colors::OwoColorize;
use serde_json::Value;
use std::collections::HashMap;

mod cli;

type MoedictResult = HashMap<String, HashMap<String, Vec<Vec<String>>>>;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let input: Vec<&str> = app.values_of("INPUT").unwrap().collect();
    for entry in &input {
        let result = request_moedict(entry)?;
        if input.len() != 1 {
            println!("{}：", entry.fg_rgb::<178, 143, 206>());
        }
        println!("{}", format_output(result));
    }

    Ok(())
}

fn request_moedict(keyword: &str) -> Result<MoedictResult> {
    let response =
        reqwest::blocking::get(format!("https://www.moedict.tw/a/{}.json", keyword))?.text()?;
    let result = response.replace("~", "").replace("`", "");
    if result.contains("<title>404 Not Found</title>") {
        return Err(anyhow!("Could not find keyword: {}", keyword));
    }
    let result = format_result(result)?;

    Ok(result)
}

fn format_result(result: String) -> Result<MoedictResult> {
    let json: HashMap<String, Value> = serde_json::from_str(&result)?;
    let dict = json
        .get("h")
        .ok_or_else(|| anyhow!("Failed to get dict!"))?
        .as_array()
        .ok_or_else(|| anyhow!("dict is not array!"))?;
    let mut result = HashMap::new();
    for dict_val in dict {
        let dicts_item = dict_val
            .as_object()
            .ok_or_else(|| anyhow!("dict item is not object!"))?
            .get("d")
            .ok_or_else(|| anyhow!("Cannot find d!"))?
            .as_array()
            .ok_or_else(|| anyhow!("d is not array!"))?;
        let pinyin = dict_val
            .as_object()
            .ok_or_else(|| anyhow!("dict item is not object!"))?
            .get("p")
            .ok_or_else(|| anyhow!("Caanot get d!"))?
            .as_str()
            .ok_or_else(|| anyhow!("p is not String!"))?;
        if result.get(pinyin).is_none() {
            result.insert(pinyin.to_string(), HashMap::new());
        }
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
            if result.get(pinyin).unwrap().get(t).is_none() {
                result
                    .get_mut(&pinyin.to_string())
                    .unwrap()
                    .insert(t.to_string(), vec![Vec::new()]);
                count = 0;
            } else {
                result
                    .get_mut(pinyin)
                    .unwrap()
                    .get_mut(t)
                    .unwrap()
                    .push(Vec::new());
            }
            if let Some(v) = dict_item.get("f") {
                result.get_mut(pinyin).unwrap().get_mut(t).unwrap()[count].push(
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
                            result.get_mut(pinyin).unwrap().get_mut(t).unwrap()[count]
                                .push(j.to_string());
                        }
                    }
                }
            }
            count += 1;
        }
    }

    Ok(result)
}

fn format_output(moedict_result: MoedictResult) -> String {
    let mut result = Vec::new();
    for (pinyin, moedict_result_val) in moedict_result {
        result.push(
            format!("讀音：{}", pinyin)
                .fg_rgb::<236, 184, 138>()
                .to_string(),
        );
        for (k, v) in moedict_result_val {
            if k != "notype" {
                result.push(format!("{}：", k.fg_rgb::<168, 216, 165>()));
            }
            for (index, value) in v.iter().enumerate() {
                result.push(
                    format!("{}.{}", index + 1, value[0].clone())
                        .fg_rgb::<129, 199, 212>()
                        .to_string(),
                );
                if !value[1..].is_empty() {
                    result.push(value[1..].join("\n").fg_rgb::<220, 159, 180>().to_string())
                }
            }
        }
    }

    result.join("\n")
}
