use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

mod cli;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let input: Vec<&str> = app.values_of("INPUT").unwrap().collect();
    if input.len() == 1 {
        let result = request_moedict(input[0])?;
        println!("{}", format_output(result));
    } else {
        for entry in app.values_of("INPUT").unwrap() {
            let result = request_moedict(entry)?;
            println!("{}:\n{}", entry, format_output(result));
        }
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
            let t;
            if let Some(v) = dict_item.get("type") {
                t = v
                    .as_str()
                    .ok_or_else(|| anyhow!("This item is not String!"))?;
            } else {
                t = "notype";
            }
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
                result.push(k);
            }
            for (i, value) in v.iter().enumerate() {
                result.push(format!("{}.{}", i + 1, value.join("\n")));
            }
        }
    }
    result.join("\n")
}
