use anyhow::{anyhow, Result};
use rustyline::Editor;

use crate::formatter::{opencc_convert, print_result};

pub struct MeowdictConsole {
    pub input_s2t: bool,
    pub result_t2s: bool,
}

impl MeowdictConsole {
    pub fn create_console(&mut self) {
        let mut reader = Editor::<()>::new();
        while let Ok(argument) = reader.readline("meowdict > ") {
            let argument = argument
                .trim()
                .split(' ')
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            if !argument.is_empty() {
                let (args, words) = argument_spliter(argument);
                if let Err(e) = self.args_runner(args, words) {
                    println!("{}", e);
                }
            }
        }
    }

    fn set_console_mode(&mut self, t: &str) -> Result<()> {
        match t {
            "unset-input-s2t" => {
                println!("Unsetting input mode...");
                self.input_s2t = false;
            }
            "unset-result-t2s" => {
                println!("Unsetting result mode...");
                self.result_t2s = false;
            }
            "input-s2t" => {
                println!("Setting input mode as s2t...");
                self.input_s2t = true;
            }
            "result-t2s" => {
                println!("Setting result mode as t2s...");
                self.result_t2s = true
            }
            _ => return Err(anyhow!("Unsupport this mode!")),
        };

        Ok(())
    }

    fn args_runner(&mut self, args: Vec<&str>, words: Vec<&str>) -> Result<()> {
        let mut words_mut: Vec<String> = words.into_iter().map(|x| x.into()).collect();
        let mut command_result_t2s = false;
        let mut command_input_s2t = false;
        let mut translation_mode = false;
        for i in args {
            match i {
                "--input-s2t" => command_input_s2t = true,
                "-i" => command_input_s2t = true,
                "--result-t2s" => command_result_t2s = true,
                "-r" => command_result_t2s = true,
                "--translation" => translation_mode = true,
                "-t" => translation_mode = true,
                "--set-mode-input-s2t" => self.set_console_mode("input-s2t")?,
                "--set-mode-result-t2s" => self.set_console_mode("result-t2s")?,
                "--unset-mode-input-s2t" => self.set_console_mode("unset-input-s2t")?,
                "--unset-mode-result-t2s" => self.set_console_mode("unset-result-t2s")?,
                "--unset-mode-all" => {
                    self.set_console_mode("unset-input-s2t")?;
                    self.set_console_mode("unset-result-t2s")?;
                }
                _ => println!("Invaild argument: {}", i),
            };
        }
        if self.input_s2t || command_input_s2t {
            words_mut = words_mut
                .into_iter()
                .map(|x| opencc_convert(&x, "s2t").unwrap_or(x))
                .collect::<Vec<_>>();
        }
        print_result(
            &words_mut,
            self.result_t2s || command_result_t2s,
            translation_mode,
        );

        Ok(())
    }
}

fn argument_spliter(argument: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let args: Vec<&str> = argument
        .clone()
        .into_iter()
        .filter(|x| x.starts_with('-'))
        .collect();
    let words: Vec<&str> = argument
        .into_iter()
        .filter(|x| !x.starts_with('-'))
        .collect();

    (args, words)
}