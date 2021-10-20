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
use formatter::words_input_s2t;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<()> {
    let config = read_config()?;
    let app = cli::build_cli().get_matches();
    let no_color = config.no_color || app.is_present("no-color-output");
    let client = reqwest::Client::new();
    let meowdict_request = MeowdictRequest { client, no_color };
    let input_s2t = config.input_s2t || app.is_present("inputs2t");
    let result_t2s = config.result_t2s || app.is_present("resultt2s");
    if let Some(words) = app.values_of("INPUT") {
        let words = words_input_s2t(
            words.into_iter().map(|x| x.into()).collect::<Vec<String>>(),
            input_s2t,
        );

        meowdict_request
            .search_word_to_dict_result(&words, result_t2s)
            .await
    } else {
        match app.subcommand() {
            ("show", Some(args)) => {
                let words = words_input_s2t(words_to_vec_string(args), input_s2t);

                meowdict_request
                    .search_word_to_dict_result(&words, result_t2s)
                    .await
            }
            ("translate", Some(args)) => {
                let words = words_input_s2t(words_to_vec_string(args), input_s2t);

                meowdict_request
                    .search_word_to_translation_result(&words, result_t2s)
                    .await
            }
            ("jyutping", Some(args)) => {
                let words = words_input_s2t(words_to_vec_string(args), input_s2t);

                meowdict_request
                    .search_word_to_jyutping_result(&words, result_t2s)
                    .await
            }
            ("json", Some(args)) => {
                let words = words_input_s2t(words_to_vec_string(args), input_s2t);

                meowdict_request
                    .search_word_to_json_result(words, result_t2s)
                    .await
            }
            ("random", _) => meowdict_request.random_moedict_item(result_t2s).await,
            _ => {
                let input_s2t_mode = config.input_s2t || app.is_present("inputs2tmode");
                let result_t2s_mode = config.result_t2s || app.is_present("resultt2smode");
                let mut console = MeowdictConsole {
                    input_s2t: input_s2t_mode,
                    result_t2s: result_t2s_mode,
                    meowdict_request,
                };

                console.create_console().await
            }
        }
    }
}

fn words_to_vec_string(args: &clap::ArgMatches) -> Vec<String> {
    let words = args.values_of("INPUT").unwrap();

    words.into_iter().map(|x| x.into()).collect::<Vec<String>>()
}

fn read_config() -> Result<MeowdictConfig> {
    create_dir_all(&*CONFTG_PATH_DIRECTORY)?;

    Ok(match File::open(&*CONFIG_PATH) {
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
    })
}
