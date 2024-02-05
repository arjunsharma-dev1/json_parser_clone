use crate::{Token, tokenizer, Value};
use crate::validator::JsonValidator;

pub fn get_next_token(
    char_vec: &Vec<char>,
    current_index: usize,
    character_count: usize
) -> (Token, usize) {

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

        let next_token = get_next_token(&char_vec, next_index, character_count);
        if Token::Colon.eq(&next_token.0) {
            token = Token::Key(value);
        } else {
            token = Token::Value(Value::String(value));
        }
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
        token = Token::Value(Value::NumberFloating(value.parse::<i64>().unwrap()))
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
    else if char.eq(&'n') {
        let value = get_next_chars(char_vec, current_index, 3);
        if value.eq("null") {
            token = Token::Null
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


fn get_next_chars(char_vec: &Vec<char>, mut current_index: usize, count: usize) -> String {
    let mut value = String::new();
    for num in 0..=count {
        value.push(*char_vec.get(current_index).unwrap());
        current_index += 1;
    }
    value
}

pub fn get_tokens(char_vec: Vec<char>, character_count: usize) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();
    let mut json_validator = JsonValidator::new();

    let mut current_index: usize = 0;
    while current_index < character_count {
        let (token, new_index) = get_next_token(&char_vec, current_index, character_count);
        current_index = new_index;
        if token != Token::None {
            let validate_token = match token {
                Token::Value(Value::String(_)) => Token::Value(Value::String("".to_string())),
                Token::Value(Value::Boolean(_)) => Token::Value(Value::Boolean(false)),
                Token::Value(Value::NumberFloating(_)) => Token::Value(Value::NumberFloating(0)),
                Token::Key(_) => Token::Value(Value::String("".to_string())),
                _ => token.clone()
            };

            dbg!(&token);
            if !json_validator.validate(&validate_token) {
                panic!("Invalid Json")
            }
            token_vec.push(token);
        }
    }

    token_vec
}