use std::env::current_exe;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let file_name = "sample.json";
    let path = Path::new(file_name);
    let json  = read_to_string(path).expect("fail to read file");

    // let json = r#"{"first": "sample1", "second": "sample2", "third": -100}"#;

    let character_vec: Vec<char> = json.chars().collect();
    let character_count = character_vec.len();

    let tokens = get_tokens(character_vec, character_count);
    println!("{:#?}", tokens);

    return Ok(());
}

fn get_tokens(char_vec: Vec<char>, character_count: usize) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();

    let mut current_index: usize = 0;
    while current_index < character_count {
        let (token, new_index) = get_next_token(&char_vec, current_index, character_count);
        current_index = new_index;
        if token != Token::None {
            token_vec.push(token);
        }
    }

    token_vec
}

fn get_next_token(char_vec: &Vec<char>, current_index: usize, character_count: usize) -> (Token, usize) {
    let mut next_index: usize = current_index + 1;
    let mut char = char_vec.get(current_index).expect("Character was expected");
    let mut token ;
    if char.eq(&'\"') {
        let mut value = String::new();
        // TODO: single double quotes support in strings, escaping
        while let Some(char) = char_vec.get(next_index) {
            if char.eq(&'\"') || next_index >= character_count {
                next_index += 1;
                break;
            }
            value.push(*char);
            next_index += 1;
        }

        token = Token::Value(Value::String(value))
    }
    else if char.eq(&'-') || char.is_numeric() {
        let mut value = char.to_string();
        while let Some(char) = char_vec.get(next_index) {
            if (char.ge(&'0') && char.lt(&'9')) || char.eq(&'.') || char.eq(&'e') || char.eq(&'-') {
                value.push(*char);
            } else if char.eq(&',') || char.eq(&'}') {
                break;
            } else if char.eq(&'\n') || char.eq(&'\r') || char.eq(&'\t') {
                next_index += 1;
                continue;
            }

            next_index += 1;
        }
        dbg!(&value);
        token = Token::Value(Value::NumberFloating(value.parse::<f64>().unwrap()))
    }
    else if char.eq(&'t') {
        let value = get_next_chars(char_vec, current_index, 3);
        if value.eq("true") {
            token = Token::Value(Value::Boolean(true))
        } else {
            panic!("Invalid JSON")
        }
    }
    else if char.eq(&'f') {
        let value = get_next_chars(char_vec, current_index, 4);
        if value.eq("false") {
            token = Token::Value(Value::Boolean(false))
        } else {
            panic!("Invalid JSON")
        }
    }
    else {
        token = match char {
            '{' => Token::ObjectStart,
            '}' => Token::ObjectEnd,
            '[' => Token::ArrayStart,
            ']' => Token::ArrayEnd,
            ',' => Token::Comma,
            ':' => Token::Colon,
            _ => Token::None
        };
    }

    (token, next_index)
}

pub fn get_next_chars(char_vec: &Vec<char>, mut current_index: usize, count: usize) -> String {
    let mut value = String::new();
    for num in 0..=count {
        value.push(*char_vec.get(current_index).unwrap());
        current_index += 1;
    }
    value
}

#[derive(Debug)]
enum Error {
    INVALID
}
#[derive(Debug, PartialEq)]
enum Token {
    ObjectStart,
    ObjectEnd,
    Comma,
    Colon,
    Key(String),
    Value(Value),
    ArrayStart,
    ArrayEnd,
    None
}

#[derive(Debug, PartialEq)]
enum Value {
    Boolean(bool),
    String(String),
    NumberFloating(f64),
    NumberNumerical(i64),
    Null
}

