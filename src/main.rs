mod validator;
mod tokenizer;
mod models;

use std::fs::read_to_string;
use std::path::Path;
use crate::models::{Token, Value, Error};

fn main() -> Result<(), Error> {
    let file_name = "sample.json";
    let path = Path::new(file_name);
    let json  = read_to_string(path).expect("fail to read file");
    // let json = r#"{"first": "sample1", "second": "sample2", "third": -100}"#;

    let character_vec: Vec<char> = json.chars().collect();
    let character_count = character_vec.len();

    let tokens = tokenizer::get_tokens(character_vec, character_count);
    println!("{:#?}", tokens);

    return Ok(());
}