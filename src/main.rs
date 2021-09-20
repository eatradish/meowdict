pub mod api;
mod cli;
pub mod console;
mod feat;
pub mod formatter;

use crate::console::MeowdictConsole;
use crate::feat::*;
use anyhow::Result;
use formatter::{opencc_convert, OpenccConvertMode};
use tokio::runtime::Builder;

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let input_s2t = app.is_present("inputs2tmode");
    let result_t2s = app.is_present("resultt2smode");
    let translation_mode = app.is_present("translation");
    let jyutping_mode = app.is_present("jyutping");
    let no_color_output = app.is_present("no-color-output");
    let json_mode = app.is_present("json");
    let client = reqwest::Client::new();
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(10)
        .build()
        .unwrap();
    if let Some(words) = app.values_of("INPUT") {
        let mut words = words.into_iter().map(|x| x.into()).collect::<Vec<String>>();
        if input_s2t {
            words = words
                .into_iter()
                .map(|x| opencc_convert(&x, OpenccConvertMode::S2T))
                .collect::<Vec<_>>();
        }
        if translation_mode {
            search_word_to_translation_result(
                words,
                &client,
                &runtime,
                no_color_output,
                result_t2s,
            )?;
        } else if jyutping_mode {
            search_word_to_jyutping_result(words, &client, &runtime, no_color_output, result_t2s)?;
        } else if json_mode {
            search_word_to_json_result(words, &client, &runtime)?;
        } else {
            search_word_to_dict_result(words, &client, &runtime, no_color_output, result_t2s)?;
        }
    } else {
        let mut console = MeowdictConsole {
            input_s2t,
            result_t2s,
            client,
            runtime,
            no_color_output,
        };
        console.create_console();
    }

    Ok(())
}
