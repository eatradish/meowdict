use anyhow::{anyhow, Result};
use clap::crate_version;
use lazy_static::lazy_static;
use reqwest::Client;
use rustyline::Editor;

use crate::feat::*;
use crate::formatter::OpenccConvertMode;

pub struct MeowdictConsole<'a> {
    pub client: &'a Client,
    pub input_s2t: bool,
    pub result_t2s: bool,
    pub no_color: bool,
}

lazy_static! {
    static ref WELCOME_INFO: String = format!(
        r#"Welcome to meowdict {}!
Please enter .help for more information"#,
        crate_version!()
    );
}

const USAGE: &str = r#"Usage:
[WORDS]
.show [WORDS]
.rand(random)
.rand(random) [WORDS]
.jyut(jyutping) [WORDS]
.trans(translate) [WORDS]
.show .input_s2t [WORDS]
.show .result_t2s [WORDS]
.set_input_s2t_mode [on|off]
.set_result_t2s_mode [on|off]
"#;

macro_rules! set_run_status {
    ($run_status:ident, $meowdict_command:expr) => {
        if $run_status.is_some() {
            return Err(anyhow!("Cannot perform multiple queries!"));
        }
        $run_status = Some($meowdict_command);
    };
}

macro_rules! set_run_status_mode {
    ($mode:expr, $values:ident, $run_status:ident, $self:ident) => {
        if $run_status.is_some() {
            return Err(anyhow!("Cannot perform multiple queries!"));
        }
        let is_on = match $values[0].as_str() {
            "on" => true,
            "off" => false,
            _ => return Err(anyhow!("unsupport mode!")),
        };
        $self.set_console_mode(&$mode, is_on);
    };
}

impl MeowdictConsole<'_> {
    pub async fn create_console(&mut self) -> Result<()> {
        display_meowdict_version();
        let mut reader = Editor::<()>::new();
        while let Ok(argument) = reader.readline("meowdict > ") {
            let argument = argument
                .trim()
                .split(' ')
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            if !argument.is_empty() {
                let (args, words) = argument_spliter(argument);
                if let Err(e) = self.args_runner(args, words).await {
                    println!("{}", e);
                }
            }
        }

        Ok(())
    }

    fn set_console_mode(&mut self, t: &OpenccConvertMode, enable: bool) {
        match t {
            OpenccConvertMode::S2T => {
                println!(
                    "{} input mode ...",
                    if enable { "Setting" } else { "Unsetting" }
                );
                self.input_s2t = enable;
            }
            OpenccConvertMode::T2S => {
                println!(
                    "{} result mode...",
                    if enable { "Setting" } else { "Unsetting" }
                );
                self.result_t2s = enable;
            }
        };
    }

    async fn args_runner(&mut self, args: Vec<&str>, values: Vec<&str>) -> Result<()> {
        let values: Vec<String> = values.into_iter().map(|x| x.into()).collect();
        let mut command_result_t2s = false;
        let mut command_input_s2t = false;
        let mut run_status: Option<MeowdictRunCommand> = None;
        if args.is_empty() && !values.is_empty() {
            set_run_status!(run_status, MeowdictRunCommand::Show);
        }
        for arg in args {
            match arg {
                ".show" => {
                    set_run_status!(run_status, MeowdictRunCommand::Show);
                }
                ".translate" | ".trans" => {
                    set_run_status!(run_status, MeowdictRunCommand::Translate);
                }
                ".jyutping" | ".jyut" => {
                    set_run_status!(run_status, MeowdictRunCommand::JyutPing);
                }
                ".input_s2t" => {
                    command_input_s2t = true;
                }
                ".result_t2s" => {
                    command_result_t2s = true;
                }
                ".set_input_s2t_mode" => {
                    set_run_status_mode!(&OpenccConvertMode::S2T, values, run_status, self);
                }
                ".set_result_t2s_mode" => {
                    set_run_status_mode!(&OpenccConvertMode::T2S, values, run_status, self);
                }
                ".random" | ".rand" => {
                    set_run_status!(run_status, MeowdictRunCommand::Random);
                }
                ".help" => {
                    println!("{}", USAGE);
                }
                _ => {
                    return Err(anyhow!("Invaild argument: {}!", arg));
                }
            }
        }
        if run_status.is_none() && !values.is_empty() {
            run_status = Some(MeowdictRunCommand::Show);
        }
        let input_s2t = command_input_s2t || self.input_s2t;
        let result_t2s = command_result_t2s || self.result_t2s;
        let no_color = self.no_color;
        let words = if !values.is_empty() {
            Some(values)
        } else {
            None
        };
        if let Some(run_status) = run_status {
            MeowdictResponse {
                command: run_status,
                client: self.client,
                input_s2t,
                result_t2s,
                no_color,
                words
            }
            .match_command_to_run()
            .await?;
        }

        Ok(())
    }
}

fn argument_spliter(argument: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut values = Vec::new();
    let mut command = Vec::new();
    for i in argument {
        if i.starts_with('.') {
            command.push(i);
        } else {
            values.push(i);
        }
    }

    (command, values)
}

fn display_meowdict_version() {
    println!("{}", WELCOME_INFO.as_str());
}

#[test]
fn test_argument_splitter() {
    let argument = ".jyut 我";
    let (command, values) = argument_spliter(argument.split_whitespace().collect::<Vec<_>>());

    assert_eq!(vec![".jyut"], command);
    assert_eq!(vec!["我"], values);
}
