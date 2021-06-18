use anyhow::Result;
pub mod api;
mod cli;
pub mod formatter;
mod meowdict_console;

use formatter::{opencc_convert, print_result, print_translation_result};
use meowdict_console::MeowdictConsole;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    if let Some(words) = app.values_of("INPUT") {
        let mut resultt2s = false;
        let mut words = words.into_iter().map(|x| x.into()).collect::<Vec<String>>();
        if app.occurrences_of("inputs2t") != 0 {
            words = words.into_iter().map(|x| {
                if let Ok(v) = opencc_convert(&x, "s2t") {
                    v
                } else {
                    x
                }
            }).collect::<Vec<_>>();
        }
        if app.occurrences_of("resultt2s") != 0 {
            resultt2s = true;
        }
        if app.occurrences_of("translation") != 0 {
            print_translation_result(&words);
            return Ok(());
        }
        print_result(&words, resultt2s);
    } else {
        let mut input_s2t_mode = false;
        let mut result_t2s_mode = false;
        if app.occurrences_of("inputs2tmode") != 0 {
            input_s2t_mode = true;
        }
        if app.occurrences_of("resultt2smode") != 0 {
            result_t2s_mode = true;
        }
        let mut console = MeowdictConsole {
            input_s2t: input_s2t_mode,
            result_t2s: result_t2s_mode,
        };
        console.create_console();
    }

    Ok(())
}
