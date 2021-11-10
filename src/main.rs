pub mod api;
mod cli;
pub mod console;
mod feat;
pub mod formatter;

use std::{
    fs::create_dir_all,
    io::{Read, Write},
    path::PathBuf,
};

use crate::console::MeowdictConsole;
use crate::feat::*;
use anyhow::Result;
use clap::ArgMatches;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

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

impl Default for MeowdictConfig {
    fn default() -> Self {
        MeowdictConfig {
            input_s2t: false,
            result_t2s: false,
            no_color: false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = read_config()?;
    let app = cli::build_cli().get_matches();
    let client = reqwest::Client::new();
    let mut input_s2t = config.input_s2t || app.is_present("inputs2t");
    let mut result_t2s = config.result_t2s || app.is_present("resultt2s");
    let mut no_color = config.no_color || app.is_present("no-color-output");
    let (subcommand, args) = app.subcommand();
    let mut is_all = false;
    if !is_meowdict_terminal(&app) {
        if app.values_of("INPUT").is_some() {
            MeowdictResponse {
                command: MeowdictRunCommand::Show,
                client: &client,
                input_s2t,
                result_t2s,
                no_color,
                words: app.values_of_lossy("INPUT"),
                is_all,
            }
            .match_command_to_run()
            .await
        } else {
            let command = match subcommand {
                "show" => MeowdictRunCommand::Show,
                "translate" => MeowdictRunCommand::Translate,
                "jyutping" => MeowdictRunCommand::JyutPing,
                "random" => MeowdictRunCommand::Random,
                "reverse" => MeowdictRunCommand::Reverse,
                _ => unreachable!(),
            };
            let mut words: Option<Vec<String>> = None;
            if let Some(args) = args {
                words = args.values_of_lossy("INPUT");
                input_s2t = input_s2t || args.is_present("inputs2t");
                result_t2s = result_t2s || args.is_present("resultt2s");
                no_color = no_color || args.is_present("no-color-output");
                is_all = args.is_present("all");
            }

            MeowdictResponse {
                command,
                client: &client,
                input_s2t,
                result_t2s,
                no_color,
                words,
                is_all
            }
            .match_command_to_run()
            .await
        }
    } else {
        let mut input_s2t_mode = config.input_s2t || app.is_present("inputs2tmode");
        let mut result_t2s_mode = config.result_t2s || app.is_present("resultt2smode");
        let mut no_color = config.no_color || app.is_present("no-color-output");
        if subcommand == "terminal" {
            if let Some(args) = args {
                input_s2t_mode = input_s2t_mode || args.is_present("inputs2tmode");
                result_t2s_mode = result_t2s_mode || args.is_present("resultt2smode");
                no_color = no_color || args.is_present("no-color-output");
            }
        }
        let mut console = MeowdictConsole {
            client: &client,
            input_s2t: input_s2t_mode,
            result_t2s: result_t2s_mode,
            no_color,
        };

        console.create_console().await
    }
}

fn is_meowdict_terminal(app: &ArgMatches) -> bool {
    app.values_of("INPUT").is_none() && app.subcommand_name().is_none()
        || app.subcommand_name() == Some("terminal")
}

fn read_config() -> Result<MeowdictConfig> {
    create_dir_all(&*CONFTG_PATH_DIRECTORY)?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&*CONFIG_PATH)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(match toml::from_slice(&buffer) {
        Ok(config) => config,
        Err(_) => {
            let default = MeowdictConfig::default();
            file.write_all(&toml::to_vec(&default)?)?;

            default
        }
    })
}
