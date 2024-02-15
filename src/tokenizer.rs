use regex::Regex;
use crate::{Token, Value};
use crate::models::Number;
use crate::validator::JsonValidator;

pub fn get_next_token(
    char_vec: &Vec<char>,
    current_index: usize,
    character_count: usize
) -> (Token, usize) {
    let mut token_to_return = Token::Whitespace;
    let mut next_index_to_return = current_index;
    while (Token::Whitespace.eq(&token_to_return) || Token::Newline.eq(&token_to_return)) && character_count > next_index_to_return {
        let (token, next_index) = get_next_token_core(char_vec, next_index_to_return, character_count);
        token_to_return = token;
        next_index_to_return = next_index;
    }
    return (token_to_return, next_index_to_return);
}

pub fn get_next_token_core(
    char_vec: &Vec<char>,
    current_index: usize,
    character_count: usize
) -> (Token, usize) {

    let mut next_index: usize = current_index + 1;
    let char = char_vec.get(current_index).expect("Character was expected");
    let token;
    let escape_characters = vec!['"', '\\', '/', 'b', 'f', 'n', 'r', 't', 'u'];


    if char.eq(&'\"') {
        let mut value = String::new();
        // TODO: single double quotes support in strings, escaping
        let mut is_escaped = false;
        while let Some(char) = char_vec.get(next_index) {

            if (!is_escaped && char.eq(&'\"')) || next_index >= character_count {
                next_index += 1;
                break;
            }

            let is_escape = char.eq(&'\\');
            if is_escaped && is_escape {
                value.push(*char);
                next_index += 1;
                is_escaped = false;
                continue;
            }
            if is_escape && !escape_characters.contains(char_vec.get(next_index + 1).unwrap()) {
                panic!("Invalid Escape Character");
            }
            if is_escaped && char.eq(&'u') {
            //     TODO: validate hex digit
                let hex_digits = get_next_chars(&char_vec, next_index + 1, 3);
                let hex_regex = Regex::new("^[0-9A-Fa-f]{4}$").unwrap();
                if !hex_regex.is_match(&hex_digits) {
                    panic!("Invalid Hex Provided");
                }
                next_index = next_index + 5;
                value.push(*char);
                value.push_str(&hex_digits);
                is_escaped = false;
                continue;
            }

            value.push(*char);
            next_index += 1;
            is_escaped = is_escape;
        }

        let next_token = get_next_token(&char_vec, next_index, character_count);
        if Token::Colon.eq(&next_token.0) {
            token = Token::Key(value);
        } else if Token::Comma.eq(&next_token.0) || Token::ArrayEnd.eq(&next_token.0) || Token::ObjectEnd.eq(&next_token.0) {
            token = Token::Value(Value::String(value));
        } else {
            dbg!(next_token.0);
            panic!("Invalid Next Token");
        }
    }
    else if char.eq(&'-') || char.is_numeric() {
        let mut value = char.to_string();
        while let Some(char) = char_vec.get(next_index) {
            if (char.ge(&'0') && char.le(&'9')) || char.eq(&'.') || char.eq(&'e') || char.eq(&'E') || char.eq(&'-') || char.eq(&'+') {
                value.push(*char);
            } else {
                break;
            }
            next_index += 1;
        }
        let regex = Regex::new("^(?<integer>(-)?[0-9]+)((\\.)(?<decimal>[0-9]+))?(([eE])(?<exponent>(([-|+]*)[0-9]+)))?$").unwrap();

        dbg!(&value);
        if let Some(captures) = regex.captures(&value) {
            let numeral_match = captures.name("integer");
            let decimal_match = captures.name("decimal");
            let exponent_match = captures.name("exponent");

            let numeral: i64;
            let decimal: Option<u64>;
            let exponent: Option<i8>;

            if numeral_match.is_none() {
                panic!("Invalid Number");
            } else {
                numeral = numeral_match.unwrap().as_str().parse::<i64>().unwrap();
            }

            if decimal_match.is_none() {
                decimal = None;
            } else {
                let decimal_option = decimal_match.unwrap().as_str().parse::<u64>();
                if decimal_option.is_err() {
                    panic!("Invalid Decimal part of Number");
                }
                decimal = Some(decimal_option.unwrap());
            }

            if exponent_match.is_none() {
                exponent = None;
            } else {
                let exponent_option = exponent_match.unwrap().as_str().parse::<i8>();
                if exponent_option.is_err() {
                    panic!("Invalid Exponent part of Number")
                }
                exponent = Some(exponent_option.unwrap());
            }

            if exponent.is_none() && decimal.is_none() && char.eq(&'0') && numeral.ne(&0) {
                panic!("Number cannot have leading zero");
            }

            token = Token::Value(Value::Number(Number {
                numeral,
                decimal,
                exponent
            }));
        } else {
            panic!("Invalid Number")
        }
        return (token, next_index);
    }
    else if char.eq(&'t') {
        let value = get_next_chars(char_vec, current_index, 3);
        if value.eq("true") {
            token = Token::Value(Value::Boolean(true));
            next_index += 3;
        } else {
            panic!("Invalid JSON")
        }
    }
    else if char.eq(&'f') {
        let value = get_next_chars(char_vec, current_index, 4);
        if value.eq("false") {
            token = Token::Value(Value::Boolean(false));
            next_index += 4;
        } else {
            panic!("Invalid JSON")
        }
    }
    else if char.eq(&'n') {
        let value = get_next_chars(char_vec, current_index, 3);
        if value.eq("null") {
            token = Token::Null;
            next_index += 3;
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
            ' ' => Token::Whitespace,
            '\n' => Token::Newline,
            _ => Token::Invalid
        };
    }

    (token, next_index)
}


fn get_next_chars(char_vec: &Vec<char>, mut current_index: usize, count: usize) -> String {
    let mut value = String::new();
    for _ in 0..=count {
        value.push(*char_vec.get(current_index).unwrap());
        current_index += 1;
    }
    value
}

pub fn get_tokens(json: &str) -> Vec<Token> {
    let char_vec: Vec<char> = json.chars().collect();
    let character_count = char_vec.len();

    let mut token_vec: Vec<Token> = Vec::new();
    let mut json_validator = JsonValidator::new();

    let mut current_index: usize = 0;
    while current_index < character_count {
        let (next_token, new_index) = get_next_token(&char_vec, current_index, character_count);
        current_index = new_index;
        if next_token == Token::Invalid {
            panic!("Invalid JSON");
        }
        if next_token != Token::Whitespace && next_token != Token::Newline {
            let validate_token = match next_token {
                Token::Value(Value::String(_)) => Token::Value(Value::String("".to_string())),
                Token::Value(Value::Boolean(_)) => Token::Value(Value::Boolean(false)),
                Token::Value(Value::Number(_)) => Token::Value(Value::Number(Number{exponent: None, decimal: None, numeral: 1})),
                Token::Key(_) => Token::Key("".to_string()),
                _ => next_token.clone()
            };

            dbg!(&next_token);
            if !json_validator.validate(&validate_token) {
                panic!("Invalid JSON")
            }
            token_vec.push(next_token);
        }
    }

    if !json_validator.is_done_processing() {
        panic!("Invalid JSON")
    }
    token_vec
}