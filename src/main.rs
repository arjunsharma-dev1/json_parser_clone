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

    let tokens = tokenizer::get_tokens(&json);
    println!("{:#?}", tokens);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::Path;
    use crate::tokenizer;
    #[test]
    #[should_panic(expected = "Invalid Next Token")]
    fn test1() {
        let json = r#""A JSON payload should be an object or array, not a string.""#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Next Token")]
    fn test2() {
        let json = r#"["Unclosed array""#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test3() {
        let json = r#"{unquoted_key: "keys must be quoted"}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test4() {
        let json = r#"["extra comma",]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test5() {
        let json = r#"["double extra comma",,]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test6() {
        let json = r#"[   , "<-- missing value"]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test7() {
        let json = r#"["Comma after the close"],"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test8() {
        let json = r#"["Extra close"]]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test9() {
        let json = r#"{"Extra comma": true,}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Next Token")]
    fn test10() {
        let json = r#"{"Extra value after close": true} "misplaced quoted value""#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test11() {
        let json = r#"{"Illegal expression": 1 + 2}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test12() {
        let json = r#"{"Illegal invocation": alert()}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Number cannot have leading zero")]
    fn test13() {
        let json = r#"{"Numbers cannot have leading zeroes": 013}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test14() {
        let json = r#"{"Numbers cannot be hex": 0x14}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Escape Character")]
    fn test15() {
        let json = r#"["Illegal backslash escape: \x15"]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test16() {
        let json = r#"[\naked]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Escape Character")]
    fn test17() {
        let json = r#"["Illegal backslash escape: \017"]"#;
        tokenizer::get_tokens(&json);
    }


    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test18() {
        let json = r#"[[[[[[[[[[[[[[[[[[[["Too deep"]]]]]]]]]]]]]]]]]]]]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Next Token")]
    fn test19() {
        let json = r#"{"Missing colon" null}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test20() {
        let json = r#"{"Double colon":: null}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test21() {
        let json = r#"{"Comma instead of colon", null}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test22() {
        let json = r#"["Colon instead of comma": false]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test23() {
        let json = r#"["Bad value", truth]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test24() {
        let json = r#"['single quote']"#;
        tokenizer::get_tokens(&json);
    }

    // #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test25() {
        let json = r#"["	tab	character	in	string	"]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Escape Character")]
    fn test26() {
        let json = r#"["tab\   character\   in\  string\  "]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Line break was not expected")]
    fn test27() {
        let json = r#"["line
break"]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Escape Character")]
    fn test28() {
        let json = r#"["line\
break"]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Number")]
    fn test29() {
        let json = r#"[0e]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Number")]
    fn test30() {
        let json = r#"[0e+]"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid Exponent part of Number")]
    fn test31() {
        let json = r#"[0e+-1]"#;
        tokenizer::get_tokens(&json);
    }


    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test32() {
        let json = r#"{"Comma instead if closing brace": true,"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test33() {
        let json = r#"["mismatch"}"#;
        tokenizer::get_tokens(&json);
    }

    #[test]
    fn test34() {
        let json = read_to_string("pass1.json").unwrap();
        tokenizer::get_tokens(&json);
    }

    #[test]
    fn test35() {
        let json = read_to_string("pass2.json").unwrap();
        tokenizer::get_tokens(&json);
    }

    #[test]
    fn test36() {
        let json = read_to_string("pass3.json").unwrap();
        tokenizer::get_tokens(&json);
    }
}