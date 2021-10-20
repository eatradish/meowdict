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
    Random,
    InputS2TMode(bool),
    ResultT2SMode(bool),
}

enum RunStatusMode {
    InputS2T,
    ResultT2S,
}

const USAGE: &str = r#"Usage:
[WORDS]
.show [WORDS]
.rand(random)
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
    ($run_status:ident, $values:ident, $mode:expr, $meowdict_command:expr) => {
        if $run_status.is_some() {
            return Err(anyhow!("Cannot perform multiple queries!"));
        }
        $run_status = Some($meowdict_command(match $values[0].as_str() {
            "on" => true,
            "off" => false,
            _ => {
                return Err(anyhow!(
                    "Usage: .set_{} true or .set_{} false",
                    $mode,
                    $mode
                ));
            }
        }));
    };
}

fn match_run_sattus_mode(t: RunStatusMode) -> &'static str {
    match t {
        RunStatusMode::InputS2T => "input_s2t",
        RunStatusMode::ResultT2S => "result_t2s",
    }
}

impl MeowdictConsole {
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
        let run_status = get_run_status(
            args,
            &mut command_input_s2t,
            &mut command_result_t2s,
            &values,
        )?;
        match run_status {
            Some(MeowdictCommand::Show) | None => {
                let words = words_input_s2t(values, self.input_s2t || command_input_s2t);
                
                self.meowdict_request
                    .search_word_to_dict_result(&words, self.result_t2s || command_result_t2s)
                    .await?
            }
            Some(MeowdictCommand::JyutPing) => {
                let words = words_input_s2t(values, self.input_s2t || command_input_s2t);

                self.meowdict_request
                    .search_word_to_jyutping_result(&words, self.result_t2s || command_result_t2s)
                    .await?
            }
            Some(MeowdictCommand::Translate) => {
                let words = words_input_s2t(values, self.input_s2t || command_input_s2t);

                self.meowdict_request
                    .search_word_to_translation_result(
                        &words,
                        self.result_t2s || command_result_t2s,
                    )
                    .await?
            }
            Some(MeowdictCommand::Random) => {
                let words = if !values.is_empty() {
                    Some(words_input_s2t(values, self.input_s2t || command_input_s2t))
                } else {
                    None
                };
                
                self.meowdict_request
                    .random_moedict_item(self.result_t2s || command_result_t2s, words)
                    .await?
            }
            Some(MeowdictCommand::InputS2TMode(enable)) => {
                self.set_console_mode(&OpenccConvertMode::S2T, enable);
            }
            Some(MeowdictCommand::ResultT2SMode(enable)) => {
                self.set_console_mode(&OpenccConvertMode::T2S, enable);
            }
            Some(MeowdictCommand::Help) => {
                println!("{}", USAGE);
            }
        }

        Ok(())
    }
}

fn get_run_status(
    args: Vec<&str>,
    command_input_s2t: &mut bool,
    command_result_t2s: &mut bool,
    values: &[String],
) -> Result<Option<MeowdictCommand>> {
    let mut run_status: Option<MeowdictCommand> = None;
    for arg in args {
        match arg {
            ".show" | "" => {
                set_run_status!(run_status, MeowdictCommand::Show);
            }
            ".jyutping" | ".jyut" => {
                set_run_status!(run_status, MeowdictCommand::JyutPing);
            }
            ".translate" | ".trans" => {
                set_run_status!(run_status, MeowdictCommand::Translate);
            }
            ".input_s2t" => {
                *command_input_s2t = true;
            }
            ".result_t2s" => {
                *command_result_t2s = true;
            }
            ".set_input_s2t_mode" => {
                set_run_status_mode!(
                    run_status,
                    values,
                    match_run_sattus_mode(RunStatusMode::InputS2T),
                    MeowdictCommand::InputS2TMode
                );
            }
            ".set_result_t2s_mode" => {
                set_run_status_mode!(
                    run_status,
                    values,
                    match_run_sattus_mode(RunStatusMode::ResultT2S),
                    MeowdictCommand::ResultT2SMode
                );
            }
            ".help" => {
                set_run_status!(run_status, MeowdictCommand::Help);
            }
            ".random" | ".rand" => {
                set_run_status!(run_status, MeowdictCommand::Random);
            }
            _ => return Err(anyhow!("Invaild argument: {}!", arg)),
        }
    }

    Ok(run_status)
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

#[test]
fn test_argument_splitter() {
    let argument = ".jyut 我";
    let (command, values) = argument_spliter(argument.split_whitespace().collect::<Vec<_>>());

    assert_eq!(vec![".jyut"], command);
    assert_eq!(vec!["我"], values);
}
