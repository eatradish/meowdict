use crate::api::{get_dict_result, get_jyutping_result};
use crate::formatter::{
    gen_dict_json_str, gen_dict_result_str, gen_jyutping_str, gen_translation_str,
    get_terminal_size,
};
use anyhow::Result;
use reqwest::Client;
use tokio::runtime::Runtime;

pub struct MeowdictRequest {
    pub client: Client,
    pub runtime: Runtime,
    pub no_color: bool,
}

impl MeowdictRequest {
    pub async fn search_word_to_dict_result(&self, words: &[String], result_t2s: bool) -> Result<()> {
        let terminal_size = get_terminal_size();
        let meowdict_results = get_dict_result(&self.client, words).await?;
        let result = gen_dict_result_str(meowdict_results, terminal_size, self.no_color, result_t2s);
        println!("{}", result);
    
        Ok(())
    }
    
    pub async fn search_word_to_translation_result(&self, words: &[String], result_t2s: bool) -> Result<()> {
        let meowdict_results = get_dict_result(&self.client, words).await?;
        let result = gen_translation_str(meowdict_results, self.no_color, result_t2s);
        println!("{}", result);
    
        Ok(())
    }
    
    pub async fn search_word_to_jyutping_result(&self, words: &[String], result_t2s: bool) -> Result<()> {
        let jyutping_results = get_jyutping_result(&self.client, words).await?;
        let result = gen_jyutping_str(jyutping_results, self.no_color, result_t2s);
        println!("{}", result);
    
        Ok(())
    }
    
    pub async fn search_word_to_json_result(&self, words: &[String], result_t2s: bool) -> Result<()> {
        let meowdict_results = get_dict_result(&self.client, words).await?;
        println!("{}", gen_dict_json_str(meowdict_results, result_t2s)?);
    
        Ok(())
    }
}
