use anyhow::Result;
use clap::crate_version;
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

pub fn gen_dict_result_str(meowdict_results: Vec<MeowdictResult>, terminal_size: usize) -> String {
    let mut result = Vec::new();

    for i in meowdict_results {
        result.push(
            format!("{}：", i.name)
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

pub fn get_terminal_size() -> usize {
    Term::stdout().size().1.into()
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

pub fn display_meowdict_version(no_color: bool) {
    let s = format!("Welcome to meowdict {}!", crate_version!())
        .fg_rgb::<177, 180, 121>()
        .to_string();
    let result = if no_color { gen_str_no_color(s) } else { s };
    println!("{}", result);
}

#[test]
fn test_opencc() {
    let s = "老师";
    let t = "老師";

    assert_eq!(opencc_convert(s, OpenccConvertMode::S2T), t);
    assert_eq!(opencc_convert(t, OpenccConvertMode::T2S), s);
}

#[test]
fn test_result_str() {
    let test_str = r#"{"name":"空穴來風","english":"lit. wind from an empty cave (idiom)","translation":{"English":["lit. wind from an empty cave (idiom)","fig. unfounded (story)","baseless (claim)"],"francais":["(expr. idiom.) les fissures laissent passer le vent","les faiblesses donnent prise à la médisance","prêter le flanc à la critique"]},"heteronyms":[{"pinyin":"kōng xuè lái fēng","bopomofo":"ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ","definitions":{"notype":[["有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的傳聞，不足以採信。」"]]}}]}"#;
    let test_obj: MeowdictResult = serde_json::from_str(test_str).unwrap();
    const LESS_80: usize = 79;
    const MORE_80: usize = 81;
    let result_with_less_80 =
        gen_str_no_color(gen_dict_result_str(vec![test_obj.clone()], LESS_80));
    let right_result_with_less_80 = r#"空穴來風：
  英語：lit. wind from an empty cave (idiom)
  拼音：kōng xuè lái fēng
  注音：ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ
  1.有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空
  穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的
  傳聞，不足以採信。」"#;
    let result_with_more_80 = gen_str_no_color(gen_dict_result_str(vec![test_obj], MORE_80));
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
    let test_str = r#"{"name":"空穴來風","english":"lit. wind from an empty cave (idiom)","translation":{"English":["lit. wind from an empty cave (idiom)","fig. unfounded (story)","baseless (claim)"],"francais":["(expr. idiom.) les fissures laissent passer le vent","les faiblesses donnent prise à la médisance","prêter le flanc à la critique"]},"heteronyms":[{"pinyin":"kōng xuè lái fēng","bopomofo":"ㄎㄨㄥ　ㄒㄩㄝˋ　ㄌㄞˊ　ㄈㄥ","definitions":{"notype":[["有空穴，就有風吹來。語出《文選．宋玉．風賦》：「臣聞於師：『枳句來巢，空穴來風，其所託者然，則風氣殊焉。』」後比喻流言乘隙而入。如：「那些空穴來風的傳聞，不足以採信。」"]]}}]}"#;
    let test_obj: MeowdictResult = serde_json::from_str(test_str).unwrap();
    let result_str = gen_str_no_color(gen_translation_str(vec![test_obj]));
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
    let test_obj = MeowdictJyutPingResult {
        word: "我".to_string(),
        jyutping: vec!["ngo5".to_string()],
    };
    let result_str = gen_str_no_color(gen_jyutping_str(vec![test_obj]));
    let right_str = r#"我：
ngo5"#;

    assert_eq!(result_str, right_str);
}

#[test]
fn test_json_output() {
    let test_str = r#"{"name":"我","english":"I","translation":{"Deutsch":["ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)"],"English":["I","me","my"],"francais":["je","moi"]},"heteronyms":[{"pinyin":"（語音）wǒ","bopomofo":"（語音）ㄨㄛˇ","definitions":{"代":[["自稱。","《易經．中孚卦．九二》：「我有好爵，吾與爾靡之。」","《詩經．小雅．采薇》：「昔我往矣，楊柳依依；今我來思，雨雪霏霏。」"],["自稱己方。","《左傳．莊公十年》：「春，齊師伐我。」","《漢書．卷五四．李廣傳》：「我軍雖煩擾，虜亦不得犯我。」"]],"形":[["表示親切之意的語詞。","《論語．述而》：「述而不作，信而好古，竊比於我老彭。」","漢．曹操〈步出夏門行〉：「經過至我碣石，心惆悵我東海。」"]],"名":[["私心、私意。","《論語．子罕》：「毋意，毋必，毋固，毋我。」","如：「大公無我」。"],["姓。如戰國時有我子。"]]}},{"pinyin":"（讀音）ě","bopomofo":"（讀音）ㄜˇ","definitions":{"notype":[["(一)之讀音。"]]}}]}"#;
    let test_obj: MeowdictResult = serde_json::from_str(test_str).unwrap();
    let result_str_with_no_t2s = gen_dict_json_str(vec![test_obj.clone()], false).unwrap();
    let right_result_with_no_t2s = r#"[{"name":"我","english":"I","translation":{"Deutsch":["ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)"],"English":["I","me","my"],"francais":["je","moi"]},"heteronyms":[{"pinyin":"（語音）wǒ","bopomofo":"（語音）ㄨㄛˇ","definitions":{"代":[["自稱。","《易經．中孚卦．九二》：「我有好爵，吾與爾靡之。」","《詩經．小雅．采薇》：「昔我往矣，楊柳依依；今我來思，雨雪霏霏。」"],["自稱己方。","《左傳．莊公十年》：「春，齊師伐我。」","《漢書．卷五四．李廣傳》：「我軍雖煩擾，虜亦不得犯我。」"]],"形":[["表示親切之意的語詞。","《論語．述而》：「述而不作，信而好古，竊比於我老彭。」","漢．曹操〈步出夏門行〉：「經過至我碣石，心惆悵我東海。」"]],"名":[["私心、私意。","《論語．子罕》：「毋意，毋必，毋固，毋我。」","如：「大公無我」。"],["姓。如戰國時有我子。"]]}},{"pinyin":"（讀音）ě","bopomofo":"（讀音）ㄜˇ","definitions":{"notype":[["(一)之讀音。"]]}}]}]"#;
    let result_str_with_t2s = gen_dict_json_str(vec![test_obj], true).unwrap();
    let right_result_with_t2s = r#"[{"name":"我","english":"I","translation":{"Deutsch":["ich (mir, mich) <Personalpronomen 1. Pers.&gt (Pron)"],"English":["I","me","my"],"francais":["je","moi"]},"heteronyms":[{"pinyin":"（语音）wǒ","bopomofo":"（语音）ㄨㄛˇ","definitions":{"代":[["自称。","《易经．中孚卦．九二》：「我有好爵，吾与尔靡之。」","《诗经．小雅．采薇》：「昔我往矣，杨柳依依；今我来思，雨雪霏霏。」"],["自称己方。","《左传．庄公十年》：「春，齐师伐我。」","《汉书．卷五四．李广传》：「我军虽烦扰，虏亦不得犯我。」"]],"形":[["表示亲切之意的语词。","《论语．述而》：「述而不作，信而好古，窃比于我老彭。」","汉．曹操〈步出夏门行〉：「经过至我碣石，心惆怅我东海。」"]],"名":[["私心、私意。","《论语．子罕》：「毋意，毋必，毋固，毋我。」","如：「大公无我」。"],["姓。如战国时有我子。"]]}},{"pinyin":"（读音）ě","bopomofo":"（读音）ㄜˇ","definitions":{"notype":[["(一)之读音。"]]}}]}]"#;

    assert_eq!(result_str_with_no_t2s, right_result_with_no_t2s);
    assert_eq!(result_str_with_t2s, right_result_with_t2s);
}
