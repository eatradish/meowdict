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
    let client = reqwest::Client::new();
    if let Some(words) = app.values_of("INPUT") {
        let input_s2t = config.input_s2t || app.is_present("inputs2t");
        let result_t2s = config.result_t2s || app.is_present("resultt2s");
        let no_color = config.no_color || app.is_present("no-color-output");
        let words = words_input_s2t(
            words.into_iter().map(|x| x.into()).collect::<Vec<String>>(),
            input_s2t,
        );

        search_word_to_dict_result(&client, no_color, &words, result_t2s).await
    } else {
        match app.subcommand() {
            ("show", Some(args)) => {
                let input_s2t = config.input_s2t || args.is_present("inputs2t");
                let result_t2s = config.result_t2s || args.is_present("resultt2s");
                let no_color = config.no_color || args.is_present("no-color-output");
                let words = words_input_s2t(args.values_of_lossy("INPUT").unwrap(), input_s2t);

                search_word_to_dict_result(&client, no_color, &words, result_t2s).await
            }
            ("translate", Some(args)) => {
                let input_s2t = config.input_s2t || args.is_present("inputs2t");
                let result_t2s = config.result_t2s || args.is_present("resultt2s");
                let no_color = config.no_color || args.is_present("no-color-output");
                let words = words_input_s2t(args.values_of_lossy("INPUT").unwrap(), input_s2t);

                search_word_to_translation_result(&client, no_color, &words, result_t2s).await
            }
            ("jyutping", Some(args)) => {
                let input_s2t = config.input_s2t || args.is_present("inputs2t");
                let result_t2s = config.result_t2s || args.is_present("resultt2s");
                let no_color = config.no_color || args.is_present("no-color-output");
                let words = words_input_s2t(args.values_of_lossy("INPUT").unwrap(), input_s2t);

                search_word_to_jyutping_result(&client, no_color, &words, result_t2s).await
            }
            ("json", Some(args)) => {
                let input_s2t = config.input_s2t || args.is_present("inputs2t");
                let result_t2s = config.result_t2s || args.is_present("resultt2s");
                let words = words_input_s2t(args.values_of_lossy("INPUT").unwrap(), input_s2t);

                search_word_to_json_result(&client, words, result_t2s).await
            }
            ("random", Some(args)) => {
                let input_s2t = config.input_s2t || args.is_present("inputs2t");
                let result_t2s = config.result_t2s || args.is_present("resultt2s");
                let no_color = config.no_color || args.is_present("no-color-output");
                let words = args.values_of_lossy("INPUT");

                random_moedict_item(&client, no_color, input_s2t, result_t2s, words).await
            }
            ("terminal", Some(args)) => create_meowdict_console(config, &args, client).await,
            _ => create_meowdict_console(config, &app, client).await,
        }
    }
}

async fn create_meowdict_console(
    config: MeowdictConfig,
    app: &clap::ArgMatches<'_>,
    client: reqwest::Client,
) -> Result<()> {
    let input_s2t_mode = config.input_s2t || app.is_present("inputs2tmode");
    let result_t2s_mode = config.result_t2s || app.is_present("resultt2smode");
    let no_color = config.no_color || app.is_present("no-color-output");
    let mut console = MeowdictConsole {
        client,
        input_s2t: input_s2t_mode,
        result_t2s: result_t2s_mode,
        no_color,
    };

    console.create_console().await
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
