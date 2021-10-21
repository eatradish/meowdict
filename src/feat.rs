use crate::api::{get_dict_result, get_jyutping_result, get_moedict_index, set_json_result};
use crate::formatter::{gen_dict_json_str, gen_dict_result_str, gen_jyutping_str, gen_translation_str, get_terminal_size, words_input_s2t};
use anyhow::anyhow;
use anyhow::Result;
use rand::prelude::{IteratorRandom, SliceRandom};
use reqwest::Client;


pub async fn search_word_to_dict_result(
    client: &Client,
    no_color: bool,
    words: &[String],
    result_t2s: bool,
) -> Result<()> {
    let terminal_size = get_terminal_size();
    let meowdict_results = get_dict_result(client, words).await?;
    let result =
        gen_dict_result_str(meowdict_results, terminal_size, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}

pub async fn search_word_to_translation_result(
    client: &Client,
    no_color: bool,
    words: &[String],
    result_t2s: bool,
) -> Result<()> {
    let meowdict_results = get_dict_result(client, words).await?;
    let result = gen_translation_str(meowdict_results, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}

pub async fn search_word_to_jyutping_result(
    client: &Client,
    no_color: bool,
    words: &[String],
    result_t2s: bool,
) -> Result<()> {
    let jyutping_results = get_jyutping_result(client, words).await?;
    let result = gen_jyutping_str(jyutping_results, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}

pub async fn search_word_to_json_result(
    client: &Client,
    words: Vec<String>,
    result_t2s: bool,
) -> Result<()> {
    let json_obj = set_json_result(client, words).await;
    println!("{}", gen_dict_json_str(json_obj, result_t2s)?);

    Ok(())
}

pub async fn random_moedict_item(
    client: &Client,
    no_color: bool,
    input_s2t: bool,
    result_t2s: bool,
    words: Option<Vec<String>>,
) -> Result<()> {
    let moedict_index = get_moedict_index(client).await?;
    let rng = &mut rand::thread_rng();
    let terminal_size = get_terminal_size();
    let rand_words = match words {
        Some(words) => {
            let words = words_input_s2t(words, input_s2t);
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
    let moedict_results = get_dict_result(client, &rand_words).await?;
    let result = gen_dict_result_str(moedict_results, terminal_size, no_color, result_t2s);
    println!("{}", result);

    Ok(())
}
