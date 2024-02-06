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
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Value {
    Boolean(bool),
    String(String),
    NumberFloating(i64),
    NumberNumerical(i64),
    Null
}

