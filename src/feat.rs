use crate::api::{get_dict_result, get_jyutping_result};
use crate::formatter::{
    gen_dict_json_str, gen_dict_result_str, gen_jyutping_str, gen_translation_str,
    get_terminal_size,
};
use anyhow::Result;
use reqwest::Client;
use tokio::runtime::Runtime;

pub fn search_word_to_dict_result(
    words: Vec<String>,
    client: &Client,
    runtime: &Runtime,
    no_color: bool,
    result_t2s: bool,
) -> Result<()> {
    let terminal_size = get_terminal_size();
    let meowdict_results = get_dict_result(runtime, client, words)?;
    let result = gen_dict_result_str(meowdict_results, terminal_size, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}

pub fn search_word_to_translation_result(
    words: Vec<String>,
    client: &Client,
    runtime: &Runtime,
    no_color: bool,
    result_t2s: bool,
) -> Result<()> {
    let meowdict_results = get_dict_result(runtime, client, words)?;
    let result = gen_translation_str(meowdict_results, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}

pub fn search_word_to_jyutping_result(
    words: Vec<String>,
    client: &Client,
    runtime: &Runtime,
    no_color: bool,
    result_t2s: bool,
) -> Result<()> {
    let jyutping_results = get_jyutping_result(client, runtime, words)?;
    let result = gen_jyutping_str(jyutping_results, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}

pub fn search_word_to_json_result(
    words: Vec<String>,
    client: &Client,
    runtime: &Runtime,
    result_t2s: bool,
) -> Result<()> {
    let meowdict_results = get_dict_result(runtime, client, words)?;
    println!("{}", gen_dict_json_str(meowdict_results, result_t2s)?);

    Ok(())
}
