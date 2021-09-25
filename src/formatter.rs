use anyhow::Result;
use console::{strip_ansi_codes, truncate_str, Term};
use opencc_rust::*;
use owo_colors::OwoColorize;

use crate::api::{MeowdictJyutPingResult, MeowdictResult};

const LINE_LENGTH: usize = 80;

pub enum OpenccConvertMode {
    S2T,
    T2S,
}

pub fn opencc_convert(input: &str, t: OpenccConvertMode) -> String {
    OpenCC::new(match t {
        OpenccConvertMode::S2T => DefaultConfig::S2TWP,
        OpenccConvertMode::T2S => DefaultConfig::TW2S,
    })
    .unwrap()
    .convert(input)
}

pub fn gen_dict_result_str(meowdict_results: Vec<MeowdictResult>) -> String {
    let mut result = Vec::new();

    for i in meowdict_results {
        result.push(
            format!("{}：", i.name)
                .fg_rgb::<178, 143, 206>()
                .to_string(),
        );
        if let Some(english) = i.english {
            result.push(
                string_split_new_line(format!("  英語：{}", english), 2)
                    .fg_rgb::<125, 187, 222>()
                    .to_string(),
            );
        }
        if let Some(heteronyms) = i.heteronyms {
            for j in heteronyms {
                if let Some(pinyin) = j.pinyin {
                    result.push(
                        format!("  拼音：{}", pinyin)
                            .fg_rgb::<236, 184, 138>()
                            .to_string(),
                    );
                }
                if let Some(bopomofo) = j.bopomofo {
                    result.push(
                        format!("  注音：{}", bopomofo)
                            .fg_rgb::<208, 90, 110>()
                            .to_string(),
                    );
                }
                if let Some(definitions) = j.definitions {
                    for (k, v) in definitions {
                        if k != "notype" {
                            result
                                .push(format!("{:>3}：", k).fg_rgb::<168, 216, 165>().to_string());
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
    }

    result.join("\n")
}

pub fn gen_translation_str(meowdict_results: Vec<MeowdictResult>) -> String {
    let mut result = Vec::new();
    for i in meowdict_results {
        result.push(
            format!("{}：", i.name)
                .fg_rgb::<178, 143, 206>()
                .to_string(),
        );
        if let Some(translation) = i.translation {
            for (k, v) in translation {
                result.push(format!("{}:", k).fg_rgb::<168, 216, 165>().to_string());
                for i in v {
                    result.push(i.fg_rgb::<220, 159, 180>().to_string());
                }
            }
        }
    }

    result.join("\n")
}

pub fn gen_jyutping_str(jyutping_result: Vec<MeowdictJyutPingResult>) -> String {
    let mut result = Vec::new();
    for i in jyutping_result {
        result.push(
            format!("{}：", i.word)
                .fg_rgb::<178, 143, 206>()
                .to_string(),
        );
        result.push(i.jyutping.join("\n").fg_rgb::<168, 216, 165>().to_string());
    }

    result.join("\n")
}

pub fn gen_dict_json_str(
    meowdict_results: Vec<MeowdictResult>,
    result_t2s: bool,
) -> Result<String> {
    let mut json = serde_json::to_string(&meowdict_results)?;
    if result_t2s {
        json = opencc_convert(&json, OpenccConvertMode::T2S);
    }
    Ok(json)
}

pub fn gen_str_no_color(str: String) -> String {
    strip_ansi_codes(&str).to_string()
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
