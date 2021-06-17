use anyhow::{anyhow, Result};
use console::{truncate_str, Term};
use futures::future::join_all;
use opencc_rust::*;
use owo_colors::OwoColorize;

use crate::api::{request_moedict, MoedictJson};

const LINE_LENGTH: usize = 80;

pub fn opencc_convert(input: &String, t: &str) -> Result<String> {
    if !&["s2t", "t2s"].contains(&t) {
        return Err(anyhow!("Unsupport this convert!"));
    }
    let opencc = match t {
        "s2t" => OpenCC::new(DefaultConfig::S2TWP).unwrap(),
        "t2s" => OpenCC::new(DefaultConfig::TW2S).unwrap(),
        _ => unreachable!(),
    };
    let result = opencc.convert(input);

    Ok(result)
}

pub async fn print_result(words: &Vec<String>, result_t2s: bool) -> Result<()> {
    let words_len = words.len();
    let mut tesk = Vec::new();
    for word in words {
        tesk.push(request_moedict(word));
    }
    let result = join_all(tesk).await;

    for (index, word) in words.iter().enumerate() {
        if words_len != 1 {
            println!("{}：", word.fg_rgb::<178, 143, 206>());
        }
        if let Ok(v) = &result[index] {
            let result = format_output(v);
            if result_t2s {
                println!("{}", opencc_convert(&result, "t2s")?);
            } else {
                println!("{}", result);
            }
        }
    }

    Ok(())
}

pub async fn print_translation_result(words: &Vec<String>) -> Result<()> {
    let words_len = words.len();
    let mut tesk = Vec::new();
    for word in words {
        tesk.push(request_moedict(word));
    }
    let result = join_all(tesk).await;
    for (index, word) in words.iter().enumerate() {
        if words_len != 1 {
            println!("{}：", word.fg_rgb::<178, 143, 206>());
        }
        if let Ok(moedict_obj) = &result[index] {
            if let Some(translation) = moedict_obj.get_translations() {
                for (k, v) in translation {
                    println!("{}:", k.fg_rgb::<168, 216, 165>());
                    for i in v {
                        println!("{}", i.fg_rgb::<220, 159, 180>());
                    }
                }
            }
        } else {
            return Err(anyhow!("Failed to get translation: {}", word));
        }
    }

    Ok(())
}

fn format_output(moedict_result: &MoedictJson) -> String {
    let mut result = Vec::new();
    if let Some(english) = moedict_result.get_english() {
        result.push(
            format!("英語：{}", english)
                .fg_rgb::<125, 187, 222>()
                .to_string(),
        );
    }
    let definations = moedict_result.get_moedict_item_result_vec();
    for i in definations {
        if let Some(pinyin) = i.pinyin {
            result.push(
                format!("拼音：{}", pinyin)
                    .fg_rgb::<236, 184, 138>()
                    .to_string(),
            );
        }
        if let Some(bopomofo) = i.bopomofo {
            result.push(
                format!("注音：{}", bopomofo)
                    .fg_rgb::<208, 90, 110>()
                    .to_string(),
            );
        }
        if let Some(defination) = i.defination {
            for (k, v) in defination {
                if k != "notype" {
                    result.push(format!("{}：", k.fg_rgb::<168, 216, 165>()));
                }
                for (index, value) in v.iter().enumerate() {
                    let result_str =
                        string_split_new_line(format!("{}.{}", index + 1, value[0].to_string()));
                    result.push(result_str.fg_rgb::<129, 199, 212>().to_string());
                    if !value[1..].is_empty() {
                        for s in &value[1..] {
                            let result_str = string_split_new_line(s.to_string());
                            result.push(result_str.fg_rgb::<220, 159, 180>().to_string());
                        }
                    }
                }
            }
        }
    }

    result.join("\n")
}

fn string_split_new_line(s: String) -> String {
    let term = Term::stdout();
    let terminal_size: usize = term.size().1.into();
    let mut result_str = String::new();
    if terminal_size < LINE_LENGTH {
        return s;
    } else {
        let mut ref_s = s.as_str();
        let mut i = 0;
        loop {
            let truncate_string = truncate_str(ref_s, LINE_LENGTH, "\n").to_string();
            result_str.push_str(&truncate_string);
            if s.len() == result_str.len() - i {
                break;
            }
            ref_s = &ref_s[truncate_string.len() - 1..];
            i += 1;
        }
    }

    result_str
}
