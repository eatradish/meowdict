use anyhow::{anyhow, Result};
use rustyline::Editor;

use crate::feat::*;
use crate::formatter::{display_meowdict_version, words_input_s2t, OpenccConvertMode};

pub struct MeowdictConsole {
    pub input_s2t: bool,
    pub result_t2s: bool,
    pub meowdict_request: MeowdictRequest,
}

enum MeowdictCommand {
    Show,
    JyutPing,
    Translate,
    Help,
    InputS2TMode(bool),
    ResultT2SMode(bool),
}

impl MeowdictConsole {
    pub async fn create_console(&mut self) -> Result<()> {
        display_meowdict_version(self.meowdict_request.no_color);
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
        let mut run_status: Option<MeowdictCommand> = None;
        for arg in args {
            match arg {
                ".show" | "" => {
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot set multi arguments!"));
                    }
                    run_status = Some(MeowdictCommand::Show);
                }
                ".jyutping" | ".jyut" => {
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot set multi arguments!"));
                    }
                    run_status = Some(MeowdictCommand::JyutPing);
                }
                ".translate" | ".trans" => {
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot set multi arguments!"));
                    }
                    run_status = Some(MeowdictCommand::Translate);
                }
                ".input_s2t" => {
                    command_input_s2t = true;
                }
                ".result_t2s" => {
                    command_result_t2s = true;
                }
                ".set_input_s2t_mode" => {
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot run multi arguments!"));
                    }
                    let enable = if values[0] == "true" {
                        true
                    } else if values[0] == "false" {
                        false
                    } else {
                        return Err(anyhow!("Usage: .input_s2t_mode true or .input_s2t_mode false"))
                    };
                    run_status = Some(MeowdictCommand::InputS2TMode(enable));
                }
                ".set_result_t2s_mode" => {
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot run multi arguments!"));
                    }
                    let enable = if values[0] == "true" {
                        true
                    } else if values[0] == "false" {
                        false
                    } else {
                        return Err(anyhow!("Usage: .input_s2t_mode true or .input_s2t_mode false"))
                    };
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot run multi arguments!"));
                    }
                    run_status = Some(MeowdictCommand::ResultT2SMode(enable));
                }
                ".help" => {
                    if run_status.is_some() {
                        return Err(anyhow!("Cannot run multi arguments!"));
                    }
                    run_status = Some(MeowdictCommand::Help);
                }
                _ => {
                    run_status = Some(MeowdictCommand::Show);
                }
            }
        }
        match run_status {
            Some(MeowdictCommand::Show) | None => {
                let words = words_input_s2t(values, self.input_s2t || command_input_s2t);
                self.meowdict_request
                    .search_word_to_dict_result(words, self.result_t2s || command_result_t2s)
                    .await?
            }
            Some(MeowdictCommand::JyutPing) => {
                let words = words_input_s2t(values, self.input_s2t || command_input_s2t);
                self.meowdict_request
                    .search_word_to_jyutping_result(words, self.result_t2s || command_result_t2s)
                    .await?
            }
            Some(MeowdictCommand::Translate) => {
                let words = words_input_s2t(values, self.input_s2t || command_input_s2t);
                self.meowdict_request
                    .search_word_to_translation_result(words, self.result_t2s || command_result_t2s)
                    .await?
            }
            Some(MeowdictCommand::InputS2TMode(enable)) => {
                self.set_console_mode(&OpenccConvertMode::S2T, enable);
            }
            Some(MeowdictCommand::ResultT2SMode(enable)) => {
                self.set_console_mode(&OpenccConvertMode::T2S, enable);
            }
            Some(MeowdictCommand::Help) => {
                println!(r#"Usage:
.show [WORDS]
.jyut(jyutping) [WORDS]
.trans(translate) [WORDS]
.show .input_s2t [WORDS]
.show .result_t2s [WORDS]
.set_input_s2t_mode [true|false]
.set_result_t2s_mode [true|false]
"#)
            }
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
