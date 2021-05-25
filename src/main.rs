use anyhow::Result;
use owo_colors::OwoColorize;

mod cli;
mod api;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let input: Vec<&str> = app.values_of("INPUT").unwrap().collect();
    for entry in &input {
        let result = api::get_result(entry)?;
        if input.len() != 1 {
            println!("{}：", entry.fg_rgb::<178, 143, 206>());
        }
        println!("{}", format_output(result));
    }

    Ok(())
}

fn format_output(moedict_result: Vec<api::MoedictItemResult>) -> String {
    let mut result = Vec::new();
    for i in moedict_result {
        result.push(
            format!("讀音：{}", i.pinyin)
                .fg_rgb::<236, 184, 138>()
                .to_string(),
        );
        for (k, v) in i.defination {
            if k != "notype" {
                result.push(format!("{}：", k.fg_rgb::<168, 216, 165>()));
            }
            for (index, value) in v.iter().enumerate() {
                result.push(
                    format!("{}.{}", index + 1, value[0].clone())
                        .fg_rgb::<129, 199, 212>()
                        .to_string(),
                );
                if !value[1..].is_empty() {
                    result.push(value[1..].join("\n").fg_rgb::<220, 159, 180>().to_string())
                }
            }
        }
    }

    result.join("\n")
}
