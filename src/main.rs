use anyhow::{anyhow, Result};
use console::{truncate_str, Term};
use opencc_rust::*;
use owo_colors::OwoColorize;
use rustyline::Editor;

mod api;
mod cli;

use api::{request_moedict, MoedictJson};

const LINE_LENGTH: usize = 80;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    if let Some(input) = app.values_of("INPUT") {
        let mut input = input.into_iter().map(|x| x.into()).collect::<Vec<String>>();
        if input.is_empty() {
            return Err(anyhow!("Error: Require keyword is empty!"));
        }
        if app.occurrences_of("inputs2t") != 0 {
            input = opencc_s2t(&input);
        }
        if app.occurrences_of("translation") != 0 {
            if let Err(e) = print_translation_result(&input) {
                println!("{}", e);
            }
            return Ok(());
        }
        print_result(&input)?;
    } else {
        meowdict_console();
    }

    Ok(())
}

fn opencc_s2t(input: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();
    for i in input {
        result.push(opencc.convert(i));
    }

    result
}

fn meowdict_console() {
    let mut reader = Editor::<()>::new();
    while let Ok(argument) = reader.readline("meowdict > ") {
        let argument = argument
            .trim()
            .split(' ')
            .filter(|x| x != &"")
            .map(|x| x.into())
            .collect::<Vec<String>>();
        if !argument.is_empty() {
            let args: Vec<String> = argument
                .clone()
                .into_iter()
                .filter(|x| x.starts_with('-'))
                .collect();
            let mut words: Vec<String> = argument
                .into_iter()
                .filter(|x| !x.starts_with('-'))
                .collect();
            if !args.is_empty() {
                let mut has_argument = false;
                if words.is_empty() {
                    println!("Error: Require keyword is empty!");
                    continue;
                }
                if args.contains(&"--input-s2t".to_string()) || args.contains(&"-i".to_string()) {
                    has_argument = true;
                    words = opencc_s2t(&words);
                }
                if args.contains(&"--translation".to_string()) || args.contains(&"-t".to_string()) {
                    if let Err(e) = print_translation_result(&words) {
                        println!("{}", e);
                    }
                    continue;
                }
                if !has_argument {
                    println!("Error: invaild Argument!");
                    continue;
                }
            }
            let result = print_result(&words);
            if let Err(e) = result {
                println!("{}", e);
            }
        }
    }
}

fn print_translation_result(words: &Vec<String>) -> Result<()> {
    let words_len = words.len();
    for word in words {
        if words_len != 1 {
            println!("{}：", word.fg_rgb::<178, 143, 206>());
        }
        let moedict_object = request_moedict(word)?;
        if let Some(translation) = moedict_object.get_translations() {
            for (k, v) in translation {
                println!("{}:", k.fg_rgb::<168, 216, 165>());
                for i in v {
                    println!("{}", i.fg_rgb::<220, 159, 180>());
                }
            }
        } else {
            return Err(anyhow!("Failed to get translation: {}", word));
        }
    }

    Ok(())
}

fn print_result(words: &Vec<String>) -> Result<()> {
    let words_len = words.len();
    for word in words {
        if words_len != 1 {
            println!("{}：", word.fg_rgb::<178, 143, 206>());
        }
        let moedict_object = request_moedict(word)?;
        println!("{}", format_output(moedict_object));
    }

    Ok(())
}

fn format_output(moedict_result: MoedictJson) -> String {
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
