use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

mod cli;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let result = response_moedict(app.value_of("INPUT").unwrap())?;
    for (k, v) in result {
        println!("{}:", k);
        for (i, value) in v.iter().enumerate() {
            println!("{}.{}", i + 1, value.join("\n"));
        }
    }

    Ok(())
}

fn response_moedict(keyword: &str) -> Result<HashMap<String, Vec<Vec<String>>>> {
    let response =
        reqwest::blocking::get(format!("https://www.moedict.tw/a/{}.json", keyword))?.text()?;
    let result = response.replace("~", "").replace("`", "");
    if result.contains("<title>404 Not Found</title>") {
        return Err(anyhow!("Could not find keyword: {}", keyword));
    }
    let json: HashMap<String, Value> = serde_json::from_str(&result)?;
    let dict = json
        .get("h")
        .ok_or_else(|| anyhow!("Failed to get dict!"))?
        .as_array()
        .ok_or_else(|| anyhow!("dict is not array!"))?;
    let mut result: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for i in dict {
        let dicts_item = i
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
            let t;
            if let Some(v) = dict_item.get("type") {
                t = v
                    .as_str()
                    .ok_or_else(|| anyhow!("This item is not String!"))?;
            } else {
                t = keyword;
            }
            if result.get(t).is_none() {
                result.insert(t.to_string(), vec![Vec::new()]);
                count = 0;
            } else {
                result.get_mut(t).unwrap().push(Vec::new());
            }
            if let Some(v) = dict_item.get("f") {
                result.get_mut(t).unwrap()[count].push(
                    v.as_str()
                        .ok_or_else(|| anyhow!("This item is not String!"))?
                        .to_string(),
                );
            } else {
                for i in vec!["q", "e", "l"] {
                    if let Some(v) = dict_item.get(i) {
                        let item_list = v
                            .as_array()
                            .ok_or_else(|| anyhow!("This item is not arrays!"))?;
                        for j in item_list {
                            if let Some(j) = j.as_str() {
                                result.get_mut(t).unwrap()[count].push(j.to_string());
                            }
                        }
                    }
                }
                count += 1;
            }
        }
    }
    Ok(result)
}
