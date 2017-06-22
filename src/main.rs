#![recursion_limit="128"]

extern crate regex;

mod value;
mod token;

use regex::{Regex,RegexSet};
use value::{Json_Terminal,Json_Object,Json_Array, Json_Type, json};
use token::{Enums, Token};

type TokenIter<'a> = Iterator<Item=&'a Token>;

fn tokenize(json_string: &String) -> Vec<Token> {

    let re_string = Regex::new(r#"^"(\\.|[^\\"])*""#).unwrap();
    let re_num = Regex::new(r#"^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?"#).unwrap();

    let mut tokens = Vec::new();
    let mut cur_string = json_string.as_str();

    while !cur_string.is_empty() {
        cur_string = &mut cur_string.trim_left();
        println!("{}", cur_string);
        if cur_string.starts_with(",") {
            println!("Comma");
            tokens.push(Token{  enum_type: Enums::COMMA, val: ",".to_string()});
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with(":") {
            println!("Colon");
            tokens.push(Token{  enum_type: Enums::COLON, val: ":".to_string()});
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("[") {
            println!("LBracket");
            tokens.push(Token{  enum_type: Enums::LBRACKET, val: "[".to_string()});
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("]") {
             println!("RBracket");
            tokens.push(Token{  enum_type: Enums::RBRACKET, val: "]".to_string()});
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("{") {
             println!("LBRACE");
            tokens.push(Token{  enum_type: Enums::LBRACE, val: "{".to_string()});
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("}") {
             println!("RBRACE");
            tokens.push(Token{  enum_type: Enums::RBRACE, val: "}".to_string()});
            cur_string = &cur_string[1..];
        } else if cur_string.starts_with("true") {
             println!("true");
            tokens.push(Token{  enum_type: Enums::TRUE, val: "true".to_string()});
            cur_string = &cur_string[4..];
        } else if cur_string.starts_with("false") {
             println!("false");
            tokens.push(Token{  enum_type: Enums::FALSE, val: "false".to_string()});
            cur_string = &cur_string[5..];
        } else if cur_string.starts_with("null") {
             println!("null");
            tokens.push(Token{  enum_type: Enums::NULL, val: "null".to_string()});
            cur_string = &cur_string[4..];
        } else if re_string.is_match(cur_string) {
             println!("string");
            let re_match = re_string.find(cur_string).unwrap();
            tokens.push(Token{  enum_type: Enums::STRING, val: re_match.as_str().to_string()});
            cur_string = &cur_string[re_match.end()..];
        } else if re_num.is_match(cur_string) {
             println!("number");
            let re_match = re_num.find(cur_string).unwrap();
            tokens.push(Token{  enum_type: Enums::NUMBER, val: re_match.as_str().to_string()});
            cur_string = &cur_string[re_match.end()..];
        } else {
            println!("Something happened");
            break;
        }
    }

    tokens
}

fn parse_string(token: &Token) -> Box<json> {
    println!("Parsing string");
    let json_string = token.val.trim_matches('"').to_string();
    let json_value = Json_Terminal{val: json_string, json_type: Json_Type::string};
    Box::new(json_value)
    //(tokens, Box::new(Json_Terminal{val: "Error".to_string(), json_type: Json_Type::string}))
}

fn parse_number(token: &Token) -> Box<json> {
    println!("Parsing number");
    let json_string = token.val.to_string();
    let json_value = Json_Terminal{val: json_string, json_type: Json_Type::number};
    Box::new(json_value)
}

fn parse_boolean(token: &Token) -> Box<json> {
    println!("Parsing boolean");
    let json_string = token.val.to_string();
    let json_value = Json_Terminal{val: json_string, json_type: Json_Type::boolean};
    Box::new(json_value)
}

fn parse_null(token: &Token) -> Box<json> {
    println!("Parsing null");
    let json_string = token.val.to_string();
    let json_value = Json_Terminal{val: json_string, json_type: Json_Type::null};
    Box::new(json_value)
} 

fn parse_array<'a, I>(mut tokens: &mut I) -> Box<json> 
    where I: Iterator<Item=&'a Token>  {
    println!("Parsing array");
    let mut json_array = Json_Array::new();

    while true {
        let next_token = tokens.next().unwrap();
        println!("{}", next_token.val);
        if next_token.enum_type == Enums::RBRACKET {
            println!("array finished");
            break;
        } else if next_token.enum_type == Enums::COMMA {println!("Skipping comma"); }
          else if next_token.enum_type == Enums::LBRACKET {println!("Array start");}
          else {
            println!("Adding array value");
            let boxed_json_val = parse_value(&next_token, tokens);
            json_array.push(boxed_json_val);
        }
    }

    Box::new(json_array)
} 

fn parse_object<'a, I>(mut tokens: &mut I) -> Box<json> 
    where I: Iterator<Item=&'a Token> {
    println!{"Parsing object"};
    let mut json_object = Json_Object::new();
    while true {
        let next_token = tokens.next().unwrap();
        if next_token.enum_type == Enums::RBRACE {
            break;
        } else if next_token.enum_type == Enums::STRING {
            let ref entry_string = next_token.val.trim_matches('"');
            if tokens.next().unwrap().enum_type == Enums::COLON {
                json_object.insert(entry_string, parse_value(&tokens.next().unwrap(), tokens));
            }
        } else if next_token.enum_type == Enums::COMMA {}
    }

    Box::new(json_object)
}

fn parse_value<'a, I>(token: &Token, mut tokens: &mut I) -> Box<json> 
    where I: Iterator<Item=&'a Token> {
    let result = match token.enum_type {
        Enums::STRING       =>  parse_string(token),
        Enums::NUMBER       =>  parse_number(token),
        Enums::TRUE         =>  parse_boolean(token),
        Enums::FALSE        =>  parse_boolean(token),
        Enums::NULL         =>  parse_null(token),
        Enums::LBRACKET     =>  {parse_array(tokens)},
        Enums::LBRACE       =>  {parse_object(tokens)},
        _ => {println!("none!"); parse_object(tokens)}, //(tokens, Box::new(Json_Terminal{val: "Error".to_string(), json_type: Json_Type::string})),
    };

    result

}


fn parse(tokens: &Vec<Token>) -> () {
    let mut tokens = tokens.iter();
    while let Some(token) = tokens.next() {
        println!("Type: {:?}, Value: {}",token.enum_type, token.val);
    }
}

fn get_jpath(path: &str, json_val: &json) -> String {
    let path = path.split('/');
    path.fold(json_val, |next, p| next.get_path(&p).unwrap())
    .get_value()
    .to_string()

}

fn main() {

let test_string = " {
  \"firstName\": \"Test\",
  \"lastName\": \"Man\",
  \"isBoolean\": true,
  \"age\":102,
  \"favoriteDecimal\": 0.45,
  \"luckyNumbers\" : [0, 102, 37.4, 0.01, -57]}".to_string();

  let tokens = tokenize(&test_string);
  let mut tokens_iter = tokens.iter();
  
  let parsed_json = parse_value(&tokens_iter.next().unwrap(), &mut tokens_iter);
  let json_result = parsed_json.get_path("favoriteDecimal").unwrap();

  println!("{}",json_result.get_value());
  println!("{}", get_jpath("favoriteDecimal",json_result));
  
}

