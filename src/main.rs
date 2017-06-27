#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate itertools;

mod value;
mod token;
mod parser;

use std::io;
use parser::parse_value;
use value::Json;
use token::tokenize;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{Error, ErrorKind};

fn get_jpath<'a>(path: &str, json_val: &'a Json) -> Result<&'a str, &'static str> {
    //path.fold(json_val, |next, p| next.get_path(p).unwrap())
    let json_value = path.split('/')
        .fold(Ok(json_val), |next, p| match next {
            Ok(r) => r.get_path(p),
            Err(_) => next,
        });

    match json_value {
        Ok(v) => {
            println!("Size of value: {:?}", std::mem::size_of_val(&json_value));
            Ok(v.get_value())},
        Err(e) => Err(e),
    }

}

fn load_file() -> Result<File, std::io::Error> {    
    if let Some(arg1) = env::args().nth(1) {
        println!("The first argument is {}", arg1);
        File::open(arg1)
    } else {
        File::open("test_1")
    }
}

fn parse_json(buffer: &mut String) -> Box<Json> {
    let tokens = tokenize(buffer);
    let mut tokens_iter = tokens.iter();
    parse_value(&tokens_iter.next().unwrap(), &mut tokens_iter)
}

fn main() {

    let mut f = load_file().unwrap();

  let mut buffer = String::new();
  f.read_to_string(&mut buffer);

    //let tokens = tokenize2(&buffer);
    //let mut tokens_iter = tokens.iter();
    //parse_value(&tokens_iter.next().unwrap(), &mut tokens_iter);
    
    let parsed_json = parse_json(&mut buffer); 
    println!("Finished parsing");

    println!("\n");
    loop {
        println!("Input JSON path:");

        let mut jpath = String::new();

        io::stdin()
            .read_line(&mut jpath)
            .expect("Something went wrong");

        //removes the new line character and returns an &str
        let jpath = jpath.lines().next().unwrap();

        println!("JSON value is: {}",
                 get_jpath(jpath, &*parsed_json).unwrap_or("Error finding path"));
    }

}
