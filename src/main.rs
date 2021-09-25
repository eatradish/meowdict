pub mod api;
mod cli;
pub mod console;
mod feat;
pub mod formatter;

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::console::MeowdictConsole;
use crate::feat::*;
use anyhow::Result;
use formatter::{opencc_convert, OpenccConvertMode};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

lazy_static! {
    static ref CONFTG_PATH_DIRECTORY: PathBuf =
        dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("./config"));
    static ref CONFIG_PATH: PathBuf = CONFTG_PATH_DIRECTORY.join("meowdict.toml");
}

#[derive(Deserialize, Serialize)]
pub struct MeowdictConfig {
    input_s2t: bool,
    result_t2s: bool,
    no_color: bool,
}

fn main() -> Result<()> {
    create_dir_all(&*CONFTG_PATH_DIRECTORY)?;
    let config = match File::open(&*CONFIG_PATH) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            toml::from_slice(&buffer)?
        }
        Err(_) => {
            let default_meowdict_config = MeowdictConfig {
                input_s2t: false,
                result_t2s: false,
                no_color: false,
            };
            let mut f = File::create(&*CONFIG_PATH)?;
            f.write_all(&toml::to_vec(&default_meowdict_config)?)?;

            default_meowdict_config
        }
    };
    let app = cli::build_cli().get_matches();
    let translation_mode = app.is_present("translation");
    let jyutping_mode = app.is_present("jyutping");
    let no_color_output = config.no_color || app.is_present("no-color-output");
    let json_mode = app.is_present("json");
    let client = reqwest::Client::new();
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(10)
        .build()
        .unwrap();
    if let Some(words) = app.values_of("INPUT") {
        let input_s2t = config.input_s2t || app.is_present("inputs2t");
        let result_t2s = config.result_t2s || app.is_present("resultt2s");
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
            search_word_to_json_result(words, &client, &runtime, result_t2s)?;
        } else {
            search_word_to_dict_result(words, &client, &runtime, no_color_output, result_t2s)?;
        }
    } else {
        let input_s2t_mode = config.input_s2t || app.is_present("inputs2tmode");
        let result_t2s_mode = config.result_t2s || app.is_present("resultt2smode");
        let mut console = MeowdictConsole {
            input_s2t: input_s2t_mode,
            result_t2s: result_t2s_mode,
            client,
            runtime,
            no_color_output,
        };
        console.create_console();
    }

    Ok(())
}
