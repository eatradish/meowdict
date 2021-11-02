use crate::api::{get_dict_result, get_jyutping_result, get_moedict_index, set_json_result};
use crate::formatter::{
    gen_dict_json_str, gen_dict_result_str, gen_jyutping_str, gen_translation_str,
};
use anyhow::{anyhow, Result};
use console::{strip_ansi_codes, Term};
use opencc_rust::{DefaultConfig, OpenCC};
use rand::prelude::{IteratorRandom, SliceRandom};
use reqwest::Client;

pub enum MeowdictRunCommand {
    Show,
    Translate,
    JyutPing,
    Json,
    Random,
}

enum OpenccConvertMode {
    S2T,
    T2S,
}

pub struct MeowdictResponse<'a> {
    pub command: MeowdictRunCommand,
    pub client: &'a Client,
    pub input_s2t: bool,
    pub result_t2s: bool,
    pub no_color: bool,
    pub words: Option<Vec<String>>,
}

impl MeowdictResponse<'_> {
    pub async fn match_command_to_run(&mut self) -> Result<()> {
        self.words = self.words_input_s2t();
        let result = match self.command {
            MeowdictRunCommand::Show => self.search_word_to_dict_result().await?,
            MeowdictRunCommand::Translate => self.search_word_to_translation_result().await?,
            MeowdictRunCommand::JyutPing => self.search_word_to_jyutping_result().await?,
            MeowdictRunCommand::Json => self.search_word_to_json_result().await?,
            MeowdictRunCommand::Random => self.random_moedict_item().await?,
        };
        println!("{}", self.setup_result(&result));

        Ok(())
    }

    async fn search_word_to_dict_result(&self) -> Result<String> {
        let terminal_size = get_terminal_size();
        let meowdict_results = get_dict_result(self.client, &self.words.as_ref().unwrap()).await?;
        let result = gen_dict_result_str(meowdict_results, terminal_size);

        Ok(result)
    }

    async fn search_word_to_translation_result(&self) -> Result<String> {
        let meowdict_results = get_dict_result(self.client, &self.words.as_ref().unwrap()).await?;
        let result = gen_translation_str(meowdict_results);

        Ok(result)
    }

    async fn search_word_to_jyutping_result(&self) -> Result<String> {
        let jyutping_results =
            get_jyutping_result(self.client, &self.words.as_ref().unwrap()).await?;
        let result = gen_jyutping_str(jyutping_results);

        Ok(result)
    }

    async fn search_word_to_json_result(&self) -> Result<String> {
        let json_obj = set_json_result(self.client, &self.words.as_ref().unwrap()).await;
        let result = gen_dict_json_str(json_obj)?;

        Ok(result)
    }

    async fn random_moedict_item(&self) -> Result<String> {
        let moedict_index = get_moedict_index(self.client).await?;
        let rng = &mut rand::thread_rng();
        let terminal_size = get_terminal_size();
        let rand_words = match &self.words {
            Some(words) => {
                let mut result = Vec::new();
                for word in words {
                    result.push(
                        moedict_index
                            .iter()
                            .filter(|x| x.contains(word))
                            .choose(rng)
                            .ok_or_else(|| anyhow!("Cannot choose one!"))?
                            .to_owned(),
                    )
                }

                result
            }
            None => {
                vec![moedict_index
                    .choose(rng)
                    .ok_or_else(|| anyhow!("Cannot choose one!"))?
                    .to_owned()]
            }
        };
        let moedict_results = get_dict_result(self.client, &rand_words).await?;
        let result = gen_dict_result_str(moedict_results, terminal_size);

        Ok(result)
    }

    fn setup_result(&self, result: &str) -> String {
        let result = if self.no_color {
            strip_ansi_codes(result).to_string()
        } else {
            result.to_string()
        };

        if self.result_t2s {
            opencc_convert(&result, OpenccConvertMode::T2S)
        } else {
            result
        }
    }

    fn words_input_s2t(&self) -> Option<Vec<String>> {
        if let Some(words) = &self.words {
            if self.input_s2t {
                Some(
                    words
                        .iter()
                        .map(|x| opencc_convert(x, OpenccConvertMode::S2T))
                        .collect::<Vec<_>>(),
                )
            } else {
                Some(words.to_vec())
            }
        } else {
            None
        }
    }
}

fn get_terminal_size() -> usize {
    Term::stdout().size().1.into()
}

fn opencc_convert(input: &str, t: OpenccConvertMode) -> String {
    OpenCC::new(match t {
        OpenccConvertMode::S2T => DefaultConfig::S2TWP,
        OpenccConvertMode::T2S => DefaultConfig::TW2S,
    })
    .unwrap()
    .convert(input)
}

#[test]
fn test_opencc() {
    let s = "老师";
    let t = "老師";

    assert_eq!(opencc_convert(s, OpenccConvertMode::S2T), t);
    assert_eq!(opencc_convert(t, OpenccConvertMode::T2S), s);
}
