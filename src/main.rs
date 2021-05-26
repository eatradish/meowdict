use anyhow::{anyhow, Result};
use owo_colors::OwoColorize;
use terminal_size::{terminal_size, Width};

mod api;
mod cli;

const LINE_LENGTH: usize = 80;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let input: Vec<&str> = app.values_of("INPUT").unwrap().collect();
    for entry in &input {
        let result = api::get_result(entry)?;
        if input.len() != 1 {
            println!("{}：", entry.fg_rgb::<178, 143, 206>());
        }
        println!("{}", format_output(result)?);
    }

    Ok(())
}

fn format_output(moedict_result: Vec<api::MoedictItemResult>) -> Result<String> {
    let mut result = Vec::new();
    let size = terminal_size();
    if let Some((Width(w), _)) = size {
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
                    let mut s = format!("{}.{}", index + 1, value[0].clone())
                        .fg_rgb::<129, 199, 212>()
                        .to_string();
                    if w > 80 {
                        s = string_split_new_line(s);
                    }
                    result.push(s);
                    if !value[1..].is_empty() {
                        let mut s = value[1..].join("\n").fg_rgb::<220, 159, 180>().to_string();
                        if w > 80 {
                            s = string_split_new_line(s);
                        }
                        result.push(s);
                    }
                }
            }
        }
    } else {
        return Err(anyhow!("Unable to get terminal size!"));
    }

    Ok(result.join("\n"))
}

fn string_split_new_line(s: String) -> String {
    let mut remaining = s
        .split("")
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>();
    let mut result = String::new();
    while remaining.len() > LINE_LENGTH {
        result.push_str(&format!("{}\n", remaining[..LINE_LENGTH].join("")));
        remaining = remaining[LINE_LENGTH..].to_vec();
        if remaining.len() < LINE_LENGTH {
            result.push_str(&remaining.join(""));
        }
    }
    if result.is_empty() {
        result = s
    }

    result
}
