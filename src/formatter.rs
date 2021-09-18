use console::{truncate_str, Term};
use futures::future;
use indexmap::IndexMap;
use opencc_rust::*;
use owo_colors::OwoColorize;
use reqwest::Client;
use tokio::runtime::{Builder, Runtime};

use crate::api::{request_moedict, get_wordshk, MoedictDefinition, MoedictRawResult};

const LINE_LENGTH: usize = 80;

pub enum OpenccConvertMode {
    S2T,
    T2S,
}

macro_rules! push_qel {
    ($qel:expr, $result:ident, $count:ident, $t:ident) => {
        if let Some(qel) = &$qel {
            qel.into_iter()
                .for_each(|x| $result.get_mut(&$t).unwrap()[$count].push(x))
        }
    };
}

pub fn opencc_convert(input: &str, t: OpenccConvertMode) -> String {
    OpenCC::new(match t {
        OpenccConvertMode::S2T => DefaultConfig::S2TWP,
        OpenccConvertMode::T2S => DefaultConfig::TW2S,
    })
    .unwrap()
    .convert(input)
}

pub fn print_result(
    words: &[String],
    result_t2s: bool,
    translation_mode: bool,
    jyutping_mode: bool,
) {
    let client = reqwest::Client::new();
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(10)
        .build()
        .unwrap();
    if !jyutping_mode {
        if !translation_mode {
            print_dict_result(&runtime, words, &client, result_t2s)
        } else {
            print_translation_result(&runtime, words, &client, result_t2s)
        }
    } else {
        print_jyutping_result(&runtime, words, &client, result_t2s)
    }
}

fn print_dict_result(runtime: &Runtime, words: &[String], client: &Client, result_t2s: bool) {
    runtime.block_on(async move {
        let mut tesk = Vec::new();
        for word in words {
            tesk.push(request_moedict(word, client));
        }
        let results = future::try_join_all(tesk).await;
        match results {
            Ok(results) => {
                for (index, word) in words.iter().enumerate() {
                    println!(
                        "{}",
                        format!(
                            "{}：",
                            if result_t2s {
                                opencc_convert(word, OpenccConvertMode::T2S)
                            } else {
                                word.to_string()
                            }
                        )
                        .fg_rgb::<178, 143, 206>()
                    );
                    let result = format_dict_output(&results[index]);
                    if result_t2s {
                        println!("{}", opencc_convert(&result, OpenccConvertMode::T2S));
                    } else {
                        println!("{}", result);
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    })
}

fn print_translation_result(runtime: &Runtime, words: &[String], client: &Client, result_t2s: bool) {
    runtime.block_on(async move {
        let mut tesk = Vec::new();
        for word in words {
            tesk.push(request_moedict(word, client));
        }
        let results = future::try_join_all(tesk).await;
        match results {
            Ok(results) => {
                for (index, word) in words.iter().enumerate() {
                    println!(
                        "{}",
                        format!(
                            "{}：",
                            if result_t2s {
                                opencc_convert(word, OpenccConvertMode::T2S)
                            } else {
                                word.to_string()
                            }
                        )
                        .fg_rgb::<178, 143, 206>()
                    );
                    let result = match &results[index].translation {
                        Some(translation) => format_translation_output(translation),
                        None => continue,
                    };
                    if result_t2s {
                        println!("{}", opencc_convert(&result, OpenccConvertMode::T2S));
                    } else {
                        println!("{}", result);
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    })
}

fn print_jyutping_result(runtime: &Runtime, words: &[String], client: &Client, result_t2s: bool) {
    runtime.block_on(async move {
        let jyutping_map = get_wordshk(client).await;
        match jyutping_map {
            Ok(jyutping_map) => {
                for word in words {
                    if jyutping_map.get(word).is_some() {
                        println!(
                            "{}",
                            format!(
                                "{}：",
                                if result_t2s {
                                    opencc_convert(word, OpenccConvertMode::T2S)
                                } else {
                                    word.to_string()
                                }
                            )
                            .fg_rgb::<178, 143, 206>()
                        );
                        println!(
                            "{}",
                            jyutping_map
                                .get(word)
                                .unwrap()
                                .join("\n")
                                .fg_rgb::<168, 216, 165>()
                        );
                    } else {
                        println!("Could find keywords: {}", word);
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    })
}


fn format_dict_output(moedict_result: &MoedictRawResult) -> String {
    let mut result = Vec::new();
    if let Some(english) = moedict_result.english.to_owned() {
        result.push(
            string_split_new_line(format!("  英語：{}", english), 2)
                .fg_rgb::<125, 187, 222>()
                .to_string(),
        );
    }
    if let Some(heteronyms) = &moedict_result.heteronyms {
        for i in heteronyms {
            if let Some(pinyin) = &i.pinyin {
                result.push(
                    format!("  拼音：{}", pinyin)
                        .fg_rgb::<236, 184, 138>()
                        .to_string(),
                );
            }
            if let Some(bopomofo) = &i.bopomofo {
                result.push(
                    format!("  注音：{}", bopomofo)
                        .fg_rgb::<208, 90, 110>()
                        .to_string(),
                );
            }
            if let Some(definition) = &i.definitions {
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

fn definition_formatter(definitions: &[MoedictDefinition]) -> IndexMap<&str, Vec<Vec<&str>>> {
    let mut result = IndexMap::new();
    let mut count: usize = 0;
    for i in definitions {
        let t = if let Some(ref word_type) = i.word_type {
            word_type
        } else {
            "notype"
        };
        if result.get(&t).is_none() {
            result.insert(t, vec![Vec::new()]);
            count = 0;
        } else {
            result.get_mut(&t).unwrap().push(Vec::new());
        }
        if let Some(f) = &i.def {
            result.get_mut(&t).unwrap()[count].push(f.as_str());
        }
        push_qel!(i.quote, result, count, t);
        push_qel!(i.example, result, count, t);
        push_qel!(i.link, result, count, t);
        count += 1;
    }

    result
}

fn format_translation_output(translation: &IndexMap<String, Vec<String>>) -> String {
    let mut result = Vec::new();
    for (k, v) in translation {
        result.push(format!("{}:", k).fg_rgb::<168, 216, 165>().to_string());
        for i in v {
            result.push(i.fg_rgb::<220, 159, 180>().to_string());
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
