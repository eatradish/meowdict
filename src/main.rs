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
use clap::ArgMatches;
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

pub struct MeowdictRunStatus {
    pub run_command: MeowdictRunCommand,
    pub input_s2t: bool,
    pub result_t2s: bool,
    pub no_color: bool,
    pub words: Option<Vec<String>>,
}

enum MeowdictMode {
    Normal,
    Terminal,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = read_config()?;
    let app = cli::build_cli().get_matches();
    let client = reqwest::Client::new();
    let mut input_s2t = config.input_s2t || app.is_present("inputs2t");
    let mut result_t2s = config.result_t2s || app.is_present("resultt2s");
    let mut no_color = config.no_color || app.is_present("no-color-output");
    let mode = match_meowdict_mode(&app);
    match mode {
        MeowdictMode::Normal => {
            if app.values_of("INPUT").is_some() {
                MeowdictResponse {
                    command: MeowdictRunCommand::Show,
                    client: &client,
                    input_s2t,
                    result_t2s,
                    no_color,
                    words: app.values_of_lossy("INPUT"),
                }
                .match_command_to_run()
                .await
            } else {
                let (subcommand, args) = app.subcommand();
                let command = match subcommand {
                    "show" => MeowdictRunCommand::Show,
                    "translate" => MeowdictRunCommand::Translate,
                    "jyutping" => MeowdictRunCommand::JyutPing,
                    "random" => MeowdictRunCommand::Random,
                    _ => panic!(),
                };
                let mut words: Option<Vec<String>> = None;
                if let Some(args) = args {
                    words = args.values_of_lossy("INPUT");
                    input_s2t = input_s2t || args.is_present("inputs2t");
                    result_t2s = result_t2s || args.is_present("resultt2s");
                    no_color = no_color || args.is_present("no-color-output");
                }
                MeowdictResponse {
                    command,
                    client: &client,
                    input_s2t,
                    result_t2s,
                    no_color,
                    words,
                }
                .match_command_to_run()
                .await
            }
        }
        MeowdictMode::Terminal => {
            let input_s2t_mode = config.input_s2t || app.is_present("inputs2tmode");
            let result_t2s_mode = config.result_t2s || app.is_present("resultt2smode");
            let no_color = config.no_color || app.is_present("no-color-output");
            let mut console = MeowdictConsole {
                client: &client,
                input_s2t: input_s2t_mode,
                result_t2s: result_t2s_mode,
                no_color,
            };
            console.create_console().await
        }
    }
}

fn match_meowdict_mode(app: &ArgMatches) -> MeowdictMode {
    if app.values_of("INPUT").is_some() {
        return MeowdictMode::Normal;
    }

    match app.subcommand().0 {
        "terminal" | "" => MeowdictMode::Terminal,
        _ => MeowdictMode::Normal,
    }
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
