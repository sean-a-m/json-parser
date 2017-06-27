#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate itertools;
extern crate time;

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
use time::PreciseTime;

fn get_jpath<'a>(path: &str, json_val: &'a Json) -> Result<String, &'static str> {
    if path.len() > 0 {
        let json_value = path.split('/')
            .fold(Ok(json_val), |next, p| match next {
                Ok(r) => r.get_path(p),
                Err(_) => next,
            });

        match json_value {
            Ok(v) => Ok(v.get_value()),
            Err(e) => Err(e),
        }
    } else {
        Ok(json_val.get_value())
    }

}

fn load_file() -> Result<File, std::io::Error> {
    if let Some(arg1) = env::args().nth(1) {
        File::open(arg1)
    } else {
        File::open("test_1")
    }
}

fn parse_json(buffer: &mut String) -> Box<Json> {

    let tokens = tokenize(buffer);
    let mut tokens_iter = tokens.iter();

    let p_start = PreciseTime::now();
    let json_value = parse_value(&tokens_iter.next().unwrap(), &mut tokens_iter);
    let p_end = PreciseTime::now();
    println!("Finished parsing in: {}", p_start.to(p_end));
    json_value

}

fn main() {

    let mut f = load_file().unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let parsed_json = parse_json(&mut buffer);

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
                 get_jpath(jpath, &*parsed_json).unwrap_or("Error finding path".to_string()));
    }

}
