#[derive(Debug)]
pub enum Error {
    INVALID
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Token {
    ObjectStart,
    ObjectEnd,
    Comma,
    Colon,
    Key(String),
    Value(Value),
    ArrayStart,
    ArrayEnd,
    None,
    Null,
    Root,
    Invalid,
    Whitespace,
    Newline,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Value {
    Boolean(bool),
    String(String),
    Number(Number),
    Null
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Number {
    pub numeral: i64,
    pub decimal: Option<u64>,
    pub exponent: Option<i8>
}
