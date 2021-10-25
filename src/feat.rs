use crate::api::{get_dict_result, get_jyutping_result, get_moedict_index, set_json_result};
use crate::formatter::{
    gen_dict_json_str, gen_dict_result_str, gen_jyutping_str, gen_translation_str,
    get_terminal_size, words_input_s2t,
};
use anyhow::anyhow;
use anyhow::Result;
use rand::prelude::{IteratorRandom, SliceRandom};
use reqwest::Client;

pub enum MeowdictRunCommand {
    Show,
    Translate,
    JyutPing,
    Json,
    Random,
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
    pub async fn match_command_to_run(&self) -> Result<()> {
        match self.command {
            MeowdictRunCommand::Show => self.search_word_to_dict_result().await,
            MeowdictRunCommand::Translate => self.search_word_to_translation_result().await,
            MeowdictRunCommand::JyutPing => self.search_word_to_jyutping_result().await,
            MeowdictRunCommand::Json => self.search_word_to_json_result().await,
            MeowdictRunCommand::Random => self.random_moedict_item().await,
        }
    }

    pub async fn search_word_to_dict_result(&self) -> Result<()> {
        let terminal_size = get_terminal_size();
        let meowdict_results = get_dict_result(
            self.client,
            &words_input_s2t(self.words.as_ref().unwrap(), self.input_s2t),
        )
        .await?;
        let result = gen_dict_result_str(
            meowdict_results,
            terminal_size,
            self.no_color,
            self.result_t2s,
        );
        println!("{}", result);

        Ok(())
    }

    pub async fn search_word_to_translation_result(&self) -> Result<()> {
        let meowdict_results = get_dict_result(
            self.client,
            &words_input_s2t(self.words.as_ref().unwrap(), self.input_s2t),
        )
        .await?;
        let result = gen_translation_str(meowdict_results, self.no_color, self.result_t2s);
        println!("{}", result);

        Ok(())
    }

    pub async fn search_word_to_jyutping_result(&self) -> Result<()> {
        let jyutping_results = get_jyutping_result(
            self.client,
            &words_input_s2t(self.words.as_ref().unwrap(), self.input_s2t),
        )
        .await?;
        let result = gen_jyutping_str(jyutping_results, self.no_color, self.result_t2s);
        println!("{}", result);

        Ok(())
    }

    pub async fn search_word_to_json_result(&self) -> Result<()> {
        let json_obj = set_json_result(
            self.client,
            &words_input_s2t(self.words.as_ref().unwrap(), self.input_s2t),
        )
        .await;
        println!("{}", gen_dict_json_str(json_obj, self.result_t2s)?);

        Ok(())
    }

    pub async fn random_moedict_item(&self) -> Result<()> {
        let moedict_index = get_moedict_index(self.client).await?;
        let rng = &mut rand::thread_rng();
        let terminal_size = get_terminal_size();
        let rand_words = match &self.words {
            Some(words) => {
                let words = words_input_s2t(words, self.input_s2t);
                let mut result = Vec::new();
                for word in words {
                    result.push(
                        moedict_index
                            .iter()
                            .filter(|x| x.contains(&word))
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
        let result = gen_dict_result_str(
            moedict_results,
            terminal_size,
            self.no_color,
            self.result_t2s,
        );
        println!("{}", result);

        Ok(())
    }
}
