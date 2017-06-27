use regex::Regex;
use time::PreciseTime;

#[derive(Debug,PartialEq)]
pub enum Enums {
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    TERMINAL,
}

pub struct Token<'a> {
    pub enum_type: Enums,
    pub val: &'a str,
}

pub fn tokenize(json_string: &str) -> Vec<Token> {

    let re_string = Regex::new(r#"(^"(\\.|[^\\"])*")"#).unwrap();
    let re_number = Regex::new(r#"(^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)"#).unwrap();

    let mut tokens = Vec::new();
    //remove whitespace from the end to avoid parsing
    let mut cur_string = json_string.trim_right();

    let t_start = PreciseTime::now();

    while !cur_string.is_empty() {
        cur_string = &mut cur_string.trim_left();
        //println!("{}", cur_string);
        if cur_string.starts_with(",") {
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with(":") {
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("[") {
            tokens.push(Token {
                enum_type: Enums::LBRACKET,
                val: &cur_string[0..0],
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("]") {
            tokens.push(Token {
                enum_type: Enums::RBRACKET,
                val: &cur_string[0..0],
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("{") {
            tokens.push(Token {
                enum_type: Enums::LBRACE,
                val: &cur_string[0..0],
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("}") {
            tokens.push(Token {
                enum_type: Enums::RBRACE,
                val: &cur_string[0..0],
            });
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("true") {
            tokens.push(Token {
                enum_type: Enums::TERMINAL,
                val: &cur_string[0..3],
            });
            cur_string = &cur_string[4..];
        } else if cur_string.starts_with("false") {
            tokens.push(Token {
                enum_type: Enums::TERMINAL,
                val: &cur_string[0..4],
            });
            cur_string = &cur_string[5..];
        } else if cur_string.starts_with("null") {
            tokens.push(Token {
                enum_type: Enums::TERMINAL,
                val: &cur_string[0..3],
            });
            cur_string = &cur_string[4..];
        } else if cur_string.starts_with("\"") {
            let re_match = re_string.find(cur_string).unwrap();
            //println!("{}",&cur_string[0..(re_match.end())]);
            tokens.push(Token {
                enum_type: Enums::TERMINAL,
                //val: re_match.as_str().to_string(),
                //FIXME: this may be 1 byte less, and not 1 char less
                val: &cur_string[0..(re_match.end() - 1)],
            });
            cur_string = &cur_string[re_match.end()..];
        } else {
            let re_match = re_number.find(cur_string).unwrap();
            tokens.push(Token {
                enum_type: Enums::TERMINAL,
                //FIXME: this may be 1 byte less, and not 1 char less
                val: &cur_string[0..re_match.end()],
            });
            cur_string = &cur_string[re_match.end()..];
        }
    }

    let t_end = PreciseTime::now();
    println!("Finished tokens in: {}", t_start.to(t_end));

    tokens
}
