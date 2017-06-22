use regex::Regex;

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

pub fn tokenize(json_string: &str) -> Vec<Token> {

    let re_string = Regex::new(r#"^"(\\.|[^\\"])*""#).unwrap();
    let re_num = Regex::new(r#"^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?"#).unwrap();

    let mut tokens = Vec::new();
    let mut cur_string = json_string;

    while !cur_string.is_empty() {
        cur_string = &mut cur_string.trim_left();
        println!("{}", cur_string);
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
        } else if cur_string.starts_with("true") {
            tokens.push(Token {
                enum_type: Enums::TRUE,
                val: "true".to_string(),
            });
            cur_string = &cur_string[4..];
        } else if cur_string.starts_with("false") {
            tokens.push(Token {
                enum_type: Enums::FALSE,
                val: "false".to_string(),
            });
            cur_string = &cur_string[5..];
        } else if cur_string.starts_with("null") {
            tokens.push(Token {
                enum_type: Enums::NULL,
                val: "null".to_string(),
            });
            cur_string = &cur_string[4..];
        } else if re_string.is_match(cur_string) {
            let re_match = re_string.find(cur_string).unwrap();
            tokens.push(Token {
                enum_type: Enums::STRING,
                val: re_match.as_str().trim_matches('"').to_string(),
            });
            cur_string = &cur_string[re_match.end()..];
        } else if re_num.is_match(cur_string) {
            let re_match = re_num.find(cur_string).unwrap();
            tokens.push(Token {
                enum_type: Enums::NUMBER,
                val: re_match.as_str().to_string(),
            });
            cur_string = &cur_string[re_match.end()..];
        } else {
            println!("Something happened");
            break;
        }
    }

    tokens
}
