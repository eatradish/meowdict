use anyhow::Result;
use console::truncate_str;
use indexmap::IndexMap;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::api::{MeowdictJsonResult, MeowdictJyutPingResult, MoedictDefinition, MoedictRawResult};

const LINE_LENGTH: usize = 80;

macro_rules! push_qel {
    ($qel:expr, $result:ident, $count:ident, $t:ident) => {
        if let Some(qel) = &$qel {
            qel.into_iter().for_each(|x| $result[$t][$count].push(x))
        }
    };
}

#[derive(Deserialize, Serialize)]
pub struct MeowdictResult {
    #[serde(flatten)]
    pub meowdict_raw_result: Vec<MoedictRawResult>,
    pub jyutping: Option<Vec<String>>,
}

pub enum OpenccConvertMode {
    S2T,
    T2S,
}

pub fn gen_dict_result_str(moedict_result: Vec<MoedictRawResult>, terminal_size: usize) -> String {
    let mut result = Vec::new();

    for i in moedict_result {
        result.push(
            format!("{}：", i.title)
                .fg_rgb::<178, 143, 206>()
                .to_string(),
        );
        if let Some(english) = i.english {
            result.push(
                string_split_new_line(format!("  英語：{}", english), 2, terminal_size)
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
                    let definitions = definition_formatter(&definitions);
                    for (k, v) in definitions {
                        if k != "notype" {
                            result
                                .push(format!("{:>3}：", k).fg_rgb::<168, 216, 165>().to_string());
                        }
                        for (index, value) in v.iter().enumerate() {
                            let result_str = string_split_new_line(
                                format!("{:>3}.{}", index + 1, value[0].to_string()),
                                2,
                                terminal_size,
                            );
                            result.push(result_str.fg_rgb::<129, 199, 212>().to_string());
                            if !value[1..].is_empty() {
                                for s in &value[1..] {
                                    let result_str = string_split_new_line(
                                        format!("    {}", s.to_string()),
                                        4,
                                        terminal_size,
                                    );
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

fn definition_formatter(definitions: &[MoedictDefinition]) -> IndexMap<&str, Vec<Vec<&str>>> {
    let mut result = IndexMap::new();
    let mut count: usize = 0;
    for i in definitions {
        let t = if let Some(ref word_type) = i.word_type {
            word_type
        } else {
            "notype"
        };
        if result.get(t).is_none() {
            result.insert(t, vec![Vec::new()]);
            count = 0;
        } else {
            result[t].push(Vec::new());
        }
        if let Some(f) = &i.def {
            result[t][count].push(f.as_str());
        }
        push_qel!(i.quote, result, count, t);
        push_qel!(i.example, result, count, t);
        push_qel!(i.link, result, count, t);
        count += 1;
    }

    result
}

pub fn gen_translation_str(meowdict_results: Vec<MoedictRawResult>) -> String {
    let mut result = Vec::new();
    for i in meowdict_results {
        result.push(
            format!("{}：", i.title)
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

pub fn gen_dict_json_str(moedict_results: Vec<MeowdictJsonResult>) -> Result<String> {
    Ok(serde_json::to_string(&moedict_results)?)
}

fn string_split_new_line(s: String, tab: usize, terminal_size: usize) -> String {
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

#[test]
fn test_result_str() {
    use console::strip_ansi_codes;
    let test_str = r#"{"t":"空穴來風","translation":{"English":["lit. wind from an empty cave (idiom)","fig. unfounded (story)","baseless (claim)"],"francais":["(expr. idiom.) les fissures laissent passer le vent","les faiblesses donnent prise à la médisance","prêter le flanc à la critique"]},"h":[{"p":"kōng xuè lái fēng","b":"ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ","d":[{"type":null,"q":null,"e":null,"f":"有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的傳聞，不足以採信。」","l":null}]}],"English":"lit. wind from an empty cave (idiom)"}"#;
    let test_obj: MoedictRawResult = serde_json::from_str(test_str).unwrap();
    const LESS_80: usize = 79;
    const MORE_80: usize = 81;
    let result_with_less_80 =
        strip_ansi_codes(&gen_dict_result_str(vec![test_obj.clone()], LESS_80)).to_string();
    let right_result_with_less_80 = r#"空穴來風：
  英語：lit. wind from an empty cave (idiom)
  拼音：kōng xuè lái fēng
  注音：ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ
  1.有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空
  穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的
  傳聞，不足以採信。」"#;
    let result_with_more_80 =
        strip_ansi_codes(&gen_dict_result_str(vec![test_obj], MORE_80)).to_string();
    let right_result_with_more_80 = r#"空穴來風：
  英語：lit. wind from an empty cave (idiom)
  拼音：kōng xuè lái fēng
  注音：ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ
  1.有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空穴
  來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的傳聞
  ，不足以採信。」"#;

    assert_eq!(result_with_less_80, right_result_with_less_80);
    assert_eq!(result_with_more_80, right_result_with_more_80);
}

#[test]
fn test_transtation_str() {
    use console::strip_ansi_codes;
    let test_str = r#"{"t":"空穴來風","translation":{"English":["lit. wind from an empty cave (idiom)","fig. unfounded (story)","baseless (claim)"],"francais":["(expr. idiom.) les fissures laissent passer le vent","les faiblesses donnent prise à la médisance","prêter le flanc à la critique"]},"h":[{"p":"kōng xuè lái fēng","b":"ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ","d":[{"type":null,"q":null,"e":null,"f":"有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的傳聞，不足以採信。」","l":null}]}],"English":"lit. wind from an empty cave (idiom)"}"#;
    let test_obj: MoedictRawResult = serde_json::from_str(test_str).unwrap();
    let result_str = strip_ansi_codes(&gen_translation_str(vec![test_obj])).to_string();
    let right_str = r#"空穴來風：
English:
lit. wind from an empty cave (idiom)
fig. unfounded (story)
baseless (claim)
francais:
(expr. idiom.) les fissures laissent passer le vent
les faiblesses donnent prise à la médisance
prêter le flanc à la critique"#;

    assert_eq!(result_str, right_str);
}

#[test]
fn test_jyutping_str() {
    use console::strip_ansi_codes;
    let test_obj = MeowdictJyutPingResult {
        word: "我".to_string(),
        jyutping: vec!["ngo5".to_string()],
    };
    let result_str = strip_ansi_codes(&gen_jyutping_str(vec![test_obj])).to_string();
    let right_str = r#"我：
ngo5"#;

    assert_eq!(result_str, right_str);
}
