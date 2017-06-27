use regex::{Regex};

#[derive(Debug,PartialEq)]
pub enum Enums {
    COMMA,
    COLON,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    TERMINAL,
}

pub struct Token {
    pub enum_type: Enums,
    pub val: String,
}

pub fn tokenize(json_string: &str) -> Vec<Token> {

    let re_string = Regex::new(r#"(^"(\\.|[^\\"])*")"#).unwrap();
    let re_number = Regex::new(r#"(^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)"#).unwrap();

    let mut tokens = Vec::new();
    //remove whitespace from the end to avoid parsing
    let mut cur_string = json_string.trim_right();

    while !cur_string.is_empty() {
        cur_string = &mut cur_string.trim_left();
        //println!("{}", cur_string);
        if cur_string.starts_with(",") {
            tokens.push(Token {
                enum_type: Enums::COMMA,
                val: ",".to_string(),
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with(":") {
            tokens.push(Token {
                enum_type: Enums::COLON,
                val: ":".to_string(),
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("[") {
            tokens.push(Token {
                enum_type: Enums::LBRACKET,
                val: "[".to_string(),
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("]") {
            tokens.push(Token {
                enum_type: Enums::RBRACKET,
                val: "]".to_string(),
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("{") {
            tokens.push(Token {
                enum_type: Enums::LBRACE,
                val: "{".to_string(),
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("}") {
            tokens.push(Token {
                enum_type: Enums::RBRACE,
                val: "}".to_string(),
            });
            cur_string = &cur_string[1..];
        }   else if cur_string.starts_with("true") {
                tokens.push(Token {
                    enum_type: Enums::TERMINAL,
                    val: "true".to_string(),
                });
            cur_string = &cur_string[4..];
        } else if cur_string.starts_with("false") {
                tokens.push(Token {
                    enum_type: Enums::TERMINAL,
                    val: "false".to_string(),
                });
            cur_string = &cur_string[5..];
        } else if cur_string.starts_with("null") {
                tokens.push(Token {
                    enum_type: Enums::TERMINAL,
                    val: "null".to_string(),
                });
            cur_string = &cur_string[5..];
        } else if cur_string.starts_with("\"") {
                let re_match = re_string.find(cur_string).unwrap();
                tokens.push(Token {
                    enum_type: Enums::TERMINAL,
                    val: re_match.as_str().to_string(),
                });
            cur_string = &cur_string[re_match.end()..];
        } else {
                let re_match = re_number.find(cur_string).unwrap();
                tokens.push(Token {
                    enum_type: Enums::TERMINAL,
                    val: re_match.as_str().to_string(),
                });
            cur_string = &cur_string[re_match.end()..];
        }
    }

    println!("Finished tokens");

    tokens
}

