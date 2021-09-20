use crate::api::{get_dict_result, get_jyutping_result};
use crate::formatter::{
    gen_dict_json_str, gen_dict_result_str, gen_jyutping_str, gen_str_no_color,
    gen_translation_str, opencc_convert, OpenccConvertMode,
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
    let meowdict_results = get_dict_result(runtime, client, words)?;
    let result_with_color = gen_dict_result_str(meowdict_results);
    let result = if !no_color {
        result_with_color
    } else {
        gen_str_no_color(result_with_color)
    };
    let result = if !result_t2s {
        result
    } else {
        opencc_convert(result.as_str(), OpenccConvertMode::T2S)
    };
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
    let translation_result_with_color = gen_translation_str(meowdict_results);
    let result = if !no_color {
        translation_result_with_color
    } else {
        gen_str_no_color(translation_result_with_color)
    };
    let result = if !result_t2s {
        result
    } else {
        opencc_convert(result.as_str(), OpenccConvertMode::T2S)
    };
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
    let jyutping_results_str_with_color = gen_jyutping_str(jyutping_results);

    let result = if !no_color {
        jyutping_results_str_with_color
    } else {
        gen_str_no_color(jyutping_results_str_with_color)
    };
    let result = if !result_t2s {
        result
    } else {
        opencc_convert(result.as_str(), OpenccConvertMode::T2S)
    };
    println!("{}", result);

    Ok(())
}

pub fn search_word_to_json_result(
    words: Vec<String>,
    client: &Client,
    runtime: &Runtime,
    result_t2s: bool
) -> Result<()> {
    let meowdict_results = get_dict_result(runtime, client, words)?;
    println!("{}", gen_dict_json_str(meowdict_results, result_t2s)?);

    Ok(())
}
