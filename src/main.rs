use anyhow::{anyhow, Result};
use owo_colors::OwoColorize;
use serde_json::Value;
use std::collections::HashMap;

mod cli;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let input: Vec<&str> = app.values_of("INPUT").unwrap().collect();
    for entry in &input {
        let result = request_moedict(entry)?;
        if input.len() != 1 {
            println!("{}：", entry.yellow());
        }
        println!("{}", format_output(result));
    }

    Ok(())
}

fn request_moedict(keyword: &str) -> Result<Vec<HashMap<String, Vec<Vec<String>>>>> {
    let response =
        reqwest::blocking::get(format!("https://www.moedict.tw/a/{}.json", keyword))?.text()?;
    let result = response.replace("~", "").replace("`", "");
    if result.contains("<title>404 Not Found</title>") {
        return Err(anyhow!("Could not find keyword: {}", keyword));
    }
    let result = format_result(result)?;

    Ok(result)
}

fn format_result(result: String) -> Result<Vec<HashMap<String, Vec<Vec<String>>>>> {
    let json: HashMap<String, Value> = serde_json::from_str(&result)?;
    let dict = json
        .get("h")
        .ok_or_else(|| anyhow!("Failed to get dict!"))?
        .as_array()
        .ok_or_else(|| anyhow!("dict is not array!"))?;
    let mut result = Vec::new();
    for (dict_index, dict_val) in dict.iter().enumerate() {
        result.push(HashMap::new());
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
            if result[dict_index].get(t).is_none() {
                result[dict_index].insert(t.to_string(), vec![Vec::new()]);
                count = 0;
            } else {
                result[dict_index].get_mut(t).unwrap().push(Vec::new());
            }
            if let Some(v) = dict_item.get("f") {
                result[dict_index].get_mut(t).unwrap()[count].push(
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
                            result[dict_index].get_mut(t).unwrap()[count].push(j.to_string());
                        }
                    }
                }
            }
            count += 1;
        }
    }

    Ok(result)
}

fn format_output(moedict_result: Vec<HashMap<String, Vec<Vec<String>>>>) -> String {
    let mut result = Vec::new();
    for i in moedict_result {
        for (k, v) in i {
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
