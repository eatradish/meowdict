use anyhow::{anyhow, Result};
use console::{truncate_str, Term};
use futures::future;
use indexmap::IndexMap;
use opencc_rust::*;
use owo_colors::OwoColorize;
use tokio::runtime::Builder;

use crate::api::{request_moedict, MoedictDefinition, MoedictRawResult};

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
                        match &results[index].translation {
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

fn format_defination_output(moedict_result: &MoedictRawResult) -> String {
    let mut result = Vec::new();
    if let Some(english) = moedict_result.english.to_owned() {
        result.push(
            string_split_new_line(format!("  英語：{}", english), 2)
                .fg_rgb::<125, 187, 222>()
                .to_string(),
        );
    }
    if let Some(heteronyms) = &moedict_result.h {
        for i in heteronyms {
            if let Some(pinyin) = &i.p {
                result.push(
                    format!("  拼音：{}", pinyin)
                        .fg_rgb::<236, 184, 138>()
                        .to_string(),
                );
            }
            if let Some(bopomofo) = &i.b {
                result.push(
                    format!("  注音：{}", bopomofo)
                        .fg_rgb::<208, 90, 110>()
                        .to_string(),
                );
            }
            if let Some(definition) = &i.d {
                let definition = definition_formatter(definition);
                for (k, v) in definition {
                    if k != "notype" {
                        result.push(format!("{:>3}：", k).fg_rgb::<168, 216, 165>().to_string());
                    }
                    for (index, value) in v.iter().enumerate() {
                        let result_str = string_split_new_line(
                            format!("{:>3}.{}", index + 1, value[0].to_string()),
                            2,
                        );
                        result.push(result_str.fg_rgb::<129, 199, 212>().to_string());
                        if !value[1..].is_empty() {
                            for s in &value[1..] {
                                let result_str =
                                    string_split_new_line(format!("    {}", s.to_string()), 4);
                                result.push(result_str.fg_rgb::<220, 159, 180>().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    result.join("\n")
}

fn definition_formatter(
    definitions: &Vec<MoedictDefinition>,
) -> IndexMap<String, Vec<Vec<String>>> {
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
        if let Some(f) = &i.f {
            result.get_mut(&t).unwrap()[count].push(f.to_owned());
        }
        if let Some(q) = &i.q {
            for i in q {
                result.get_mut(&t).unwrap()[count].push(i.to_owned());
            }
        }
        if let Some(e) = &i.e {
            for i in e {
                result.get_mut(&t).unwrap()[count].push(i.to_owned());
            }
        }
        if let Some(l) = &i.l {
            for i in l {
                result.get_mut(&t).unwrap()[count].push(i.to_owned());
            }
        }
        count += 1;
    }

    result
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

fn string_split_new_line(s: String, tab: usize) -> String {
    let term = Term::stdout();
    let terminal_size: usize = term.size().1.into();
    let mut result_str = String::new();
    let limit_length = if terminal_size < LINE_LENGTH {
        terminal_size
    } else {
        LINE_LENGTH
    };
    let mut ref_s = s.as_str();
    let mut i = 0;
    let tab_string = " ".repeat(tab);
    let tail = format!("\n{}", tab_string);
    loop {
        let truncate_string = truncate_str(ref_s, limit_length, &tail).to_string();
        result_str.push_str(&truncate_string);
        if s.len() == result_str.len() - i * tab - i {
            break;
        }
        ref_s = &ref_s[truncate_string.len() - tab - 1..];
        i += 1;
    }

    result_str
}
