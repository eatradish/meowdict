pub mod api;
mod cli;
pub mod console;
pub mod formatter;

use crate::console::MeowdictConsole;
use formatter::{opencc_convert, print_result, OpenccConvertMode};

fn main() {
    let app = cli::build_cli().get_matches();
    let input_s2t = app.is_present("inputs2tmode");
    let result_t2s = app.is_present("resultt2smode");
    let translation_mode = app.is_present("translation");
    if let Some(words) = app.values_of("INPUT") {
        let mut words = words.into_iter().map(|x| x.into()).collect::<Vec<String>>();
        if input_s2t {
            words = words
                .into_iter()
                .map(|x| opencc_convert(&x, OpenccConvertMode::S2T).unwrap_or(x))
                .collect::<Vec<_>>();
        }
        print_result(&words, result_t2s, translation_mode);
    } else {
        let mut console = MeowdictConsole {
            input_s2t,
            result_t2s,
        };
        console.create_console();
    }
}
