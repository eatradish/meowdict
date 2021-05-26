use anyhow::Result;
use console::{measure_text_width, truncate_str, Term};
use owo_colors::OwoColorize;
use rustyline::Editor;

mod api;
mod cli;

const LINE_LENGTH: usize = 80;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    if let Some(input) = app.values_of("INPUT") {
        let input = input.collect::<Vec<&str>>();
        print_result(input)?;
    } else {
        let mut reader = Editor::<()>::new();
        while let Ok(words) = reader.readline("meowdict > ") {
            let words = words
                .trim()
                .split(" ")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            if !words.is_empty() {
                let result = print_result(words);
                if let Err(e) = result {
                    println!("{}", e);
                }
            }
        }
    }

    Ok(())
}

fn print_result(words: Vec<&str>) -> Result<()> {
    for word in &words {
        let result = api::get_result(&word)?;
        if words.len() != 1 {
            println!("{}：", word.fg_rgb::<178, 143, 206>());
        }
        println!("{}", format_output(result));
    }

    Ok(())
}

fn format_output(moedict_result: Vec<api::MoedictItemResult>) -> String {
    let mut result = Vec::new();
    for i in moedict_result {
        result.push(
            format!("拼音：{}", i.pinyin)
                .fg_rgb::<236, 184, 138>()
                .to_string(),
        );
        result.push(
            format!("注音：{}", i.bopomofo)
                .fg_rgb::<208, 90, 110>()
                .to_string(),
        );
        for (k, v) in i.defination {
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
        loop {
            let truncate_string = truncate_str(ref_s, LINE_LENGTH, "\n").to_string();
            result_str.push_str(&truncate_string);
            if measure_text_width(&truncate_string) < LINE_LENGTH {
                break;
            }
            ref_s = &ref_s[truncate_string.len() - 1..];
        }
    }

    result_str
}
