use anyhow::{anyhow, Result};

pub mod api;
mod cli;
pub mod formatter;
mod meowdict_console;

use formatter::{opencc_convert, print_result, print_translation_result};
use meowdict_console::MeowdictConsole;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    if let Some(input) = app.values_of("INPUT") {
        let mut input = input.into_iter().map(|x| x.into()).collect::<Vec<String>>();
        let mut result_t2s = false;
        if input.is_empty() {
            return Err(anyhow!("Error: Require keyword is empty!"));
        }
        if app.occurrences_of("inputs2t") != 0 {
            for i in 0..input.len() {
                input[i] = opencc_convert(&input[i], "s2t")?;
            }
        }
        if app.occurrences_of("resultt2s") != 0 {
            result_t2s = true;
        }
        if app.occurrences_of("translation") != 0 {
            if let Err(e) = print_translation_result(&input) {
                println!("{}", e);
            }
            return Ok(());
        }
        print_result(&input, result_t2s)?;
    } else {
        let mut input_s2t = false;
        let mut result_t2s = false;
        if app.occurrences_of("inputs2tmode") != 0 {
            input_s2t = true;
        }
        if app.occurrences_of("resultt2smode") != 0 {
            result_t2s = true;
        }
        let mut console = MeowdictConsole {
            input_s2t,
            result_t2s,
        };
        console.create_console();
    }

    Ok(())
}
