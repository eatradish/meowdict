use anyhow::Result;
pub mod api;
mod cli;
pub mod formatter;
pub mod console;

use formatter::{opencc_convert, print_result};
use crate::console::MeowdictConsole;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    if let Some(words) = app.values_of("INPUT") {
        let mut resultt2s = false;
        let mut translation_mode = false;
        let mut words = words.into_iter().map(|x| x.into()).collect::<Vec<String>>();
        if app.occurrences_of("inputs2t") != 0 {
            words = words
                .into_iter()
                .map(|x| opencc_convert(&x, "s2t").unwrap_or(x))
                .collect::<Vec<_>>();
        }
        if app.occurrences_of("resultt2s") != 0 {
            resultt2s = true;
        }
        if app.occurrences_of("translation") != 0 {
            translation_mode = true;
        }
        print_result(&words, resultt2s, translation_mode);
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
