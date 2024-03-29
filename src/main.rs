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
    let mut input_s2t = config.input_s2t || app.contains_id("inputs2t");
    let mut result_t2s = config.result_t2s || app.contains_id("resultt2s");
    let mut no_color = config.no_color || app.contains_id("no-color-output");
    let subcmd = app.subcommand();
    let mut is_all = false;
    if !is_meowdict_terminal(&app) {
        if let Some(words) = app.get_many::<String>("INPUT") {
            let words = words.into_iter().map(|x| x.into()).collect::<Vec<String>>();
            MeowdictResponse {
                command: MeowdictRunCommand::Show,
                client: &client,
                input_s2t,
                result_t2s,
                no_color,
                words: Some(words),
                is_all,
            }
            .match_command_to_run()
            .await
        } else {
            let command = match subcmd.unwrap().0 {
                "show" => MeowdictRunCommand::Show,
                "translate" => MeowdictRunCommand::Translate,
                "jyutping" => MeowdictRunCommand::JyutPing,
                "random" => MeowdictRunCommand::Random,
                _ => unreachable!(),
            };
            let mut words = None;
            if let Some((_, args)) = subcmd {
                words = args
                    .get_many::<String>("INPUT")
                    .map(|x| x.cloned().collect());
                input_s2t = input_s2t || args.contains_id("inputs2t");
                result_t2s = result_t2s || args.contains_id("resultt2s");
                no_color = no_color || args.contains_id("no-color-output");
                is_all = args.contains_id("all");
            }

            MeowdictResponse {
                command,
                client: &client,
                input_s2t,
                result_t2s,
                no_color,
                words,
                is_all,
            }
            .match_command_to_run()
            .await
        }
    } else {
        let mut input_s2t_mode = config.input_s2t || app.contains_id("inputs2tmode");
        let mut result_t2s_mode = config.result_t2s || app.contains_id("resultt2smode");
        let mut no_color = config.no_color || app.contains_id("no-color-output");
        if let Some((cmd, args)) = subcmd {
            if cmd == "terminal" {
                input_s2t_mode = input_s2t_mode || args.contains_id("inputs2tmode");
                result_t2s_mode = result_t2s_mode || args.contains_id("resultt2smode");
                no_color = no_color || args.contains_id("no-color-output");
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
    app.get_many::<String>("INPUT").is_none() && app.subcommand_name().is_none()
        || app.subcommand_name() == Some("terminal")
}

fn read_config() -> Result<MeowdictConfig> {
    create_dir_all(&*CONFTG_PATH_DIRECTORY)?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&*CONFIG_PATH)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(match toml::from_str(&buffer) {
        Ok(config) => config,
        Err(_) => {
            let default = MeowdictConfig::default();
            file.write_all(&toml::to_string(&default)?.as_bytes())?;

            default
        }
    })
}
