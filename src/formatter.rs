use anyhow::{anyhow, Result};
use console::{truncate_str, Term};
use futures::future;
use indexmap::IndexMap;
use opencc_rust::*;
use owo_colors::OwoColorize;
use tokio::runtime::Builder;

use crate::api::{request_moedict, MoedictJson};

const LINE_LENGTH: usize = 80;

pub fn opencc_convert(input: &str, t: &str) -> Result<String> {
    match t {
        "s2t" => Ok(OpenCC::new(DefaultConfig::S2TWP).unwrap().convert(input)),
        "t2s" => Ok(OpenCC::new(DefaultConfig::TW2S).unwrap().convert(input)),
        _ => Err(anyhow!("Unsupport this convert!")),
    }
}

pub fn print_result(words: &[String], result_t2s: bool, translation_mode: bool) {
    let client = reqwest::Client::new();
    let runtime = Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .worker_threads(10)
        .build()
        .unwrap();
    runtime.block_on(async move {
        let mut tesk = Vec::new();
        for word in words {
            tesk.push(request_moedict(word, &client));
        }
        let results = future::try_join_all(tesk).await;
        match results {
            Ok(results) => {
                for (index, word) in words.iter().enumerate() {
                    println!("{}：", word.fg_rgb::<178, 143, 206>());
                    let result = if !translation_mode {
                        format_defination_output(&results[index])
                    } else {
                        match &results[index].get_translations() {
                            Some(translation) => format_translation_output(translation),
                            None => continue,
                        }
                    };
                    if result_t2s {
                        if let Ok(result) = opencc_convert(&result, "t2s") {
                            println!("{}", result);
                        }
                    } else {
                        println!("{}", result);
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    })
}

fn format_defination_output(moedict_result: &MoedictJson) -> String {
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
                    let s = format!("{}.{}", index + 1, value[0].to_string());
                    result.push(s.fg_rgb::<129, 199, 212>().to_string());
                    if !value[1..].is_empty() {
                        for s in &value[1..] {
                            result.push(s.fg_rgb::<220, 159, 180>().to_string());
                        }
                    }
                }
            }
        }
    }

    let result = result
        .into_iter()
        .map(|x| string_split_new_line(x))
        .collect::<Vec<_>>();

    result.join("\n")
}

fn format_translation_output(translation: &IndexMap<String, Vec<String>>) -> String {
    let mut result = Vec::new();
    for (k, v) in translation {
        result.push(format!("{}:", k.fg_rgb::<168, 216, 165>()));
        for i in v {
            result.push(format!("{}", i.fg_rgb::<220, 159, 180>()));
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
