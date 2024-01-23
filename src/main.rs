use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), Error> {
    let file_name = "sample.json";
    let path = Path::new(file_name);
    let json  = read_to_string(path).expect("fail to read file");

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
    let mut is_string = false;
    let mut token ;
    if char.eq(&'\"') {
        let mut value = String::new();
        if is_string {
            is_string = false;
        } else {
            is_string = true;
        }
        char = char_vec.get(next_index).expect("Character was expected");
        value.push(*char);
        next_index += 1;

        while is_string && next_index < character_count && !char.eq(&'\"')  {
            let next_char = char_vec.get(next_index).expect("Character was expected");
            char = next_char;
            next_index += 1;
            if char.eq(&'\"') {
                is_string = false;
            } else {
                value.push(*next_char);
            }
        }

        token = Token::Value(Value::String(value))
    }
    else if char.eq(&'-') || char.is_numeric() {
        let mut value = String::new();
        value.push(*char);
        char = char_vec.get(next_index).expect("Character was expected");
        value.push(*char);
        next_index += 1;

        while next_index < character_count && char.is_numeric()  {
            let next_char = char_vec.get(next_index).expect("Character was expected");
            if(next_char.is_numeric()) {
                value.push(*next_char);
            }
            char = next_char;
            next_index += 1;
        }

        token = Token::Value(Value::NumberNumerical(value.parse::<i64>().unwrap()))
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

