extern crate regex;
extern crate itertools;

mod value;
mod token;
mod parser;

use std::io;
use parser::parse_value;
use value::Json;
use token::tokenize;

fn get_jpath(path: &str, json_val: &Json) -> Result<String, &'static str> {
    //path.fold(json_val, |next, p| next.get_path(p).unwrap())
    let json_value = path.split('/')
    .fold(Ok(json_val), |next, p| match next {
        Ok(r) => r.get_path(p), 
        Err(_) => next
        });

    match json_value {
        Ok(v) => Ok(v.get_value().to_string()),
        Err(e) => Err(e)
    }

}

fn main() {

let test_string = " {
  \"firstName\": \"Test\",
  \"lastName\": \"Man\",
  \"isBoolean\": true,
  \"age\":102,
  \"favoriteDecimal\": 0.45,
  \"pets\": {\"dog\": true, \"catArray\": [\"firstString\", \"second string\"]},
  \"luckyNumbers\" : [0, 102, 37.4, 0.01, -57]}";

  let tokens = tokenize(&test_string);
  let mut tokens_iter = tokens.iter();
  
  let parsed_json = parse_value(&tokens_iter.next().unwrap(), &mut tokens_iter);

  println!("\n");
  loop {
    
    println!("Input JSON path:");

    let mut jpath = String::new();

    io::stdin().read_line(&mut jpath)
    .expect("Something went wrong");

    //removes the new line character and returns an &str
    let jpath = jpath.lines().next().unwrap(); 

    println!("JSON value is: {}", get_jpath(jpath, &*parsed_json).unwrap_or("Error finding path".to_string()));
  }
  
}

