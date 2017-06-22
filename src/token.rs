#[derive(Debug,PartialEq)]
pub enum Enums {
    COMMA,
    COLON,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    STRING,
    NUMBER,
    TRUE,
    FALSE,
    NULL,
}

pub struct Token {
    pub enum_type: Enums,
    pub val: String,
}