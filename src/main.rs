#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate itertools;
extern crate time;
extern crate crossbeam;

mod value;
mod token;
mod parser;

use std::io;
use parser::parse;
//use parser_senpai;
use value::Json;
use token::token_chan;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;

use token::Token;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc};
use std::thread;


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

fn parse_json<'a>(buffer: &'a str) -> Box<Json + Send> {


    let p_start = PreciseTime::now();

    let (tx, rx): (Sender<Token>, Receiver<Token>) = mpsc::channel();

    let json_value = crossbeam::scope(|scope| {
        scope.spawn(|| {
            token_chan(buffer, tx);
        });
        parse(rx)
    });
 
    let p_end = PreciseTime::now();
    println!("Finished parsing in: {}", p_start.to(p_end));

   json_value
}

fn main() {

    let mut f = load_file().unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    
    let parsed_json = parse_json(&buffer);

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
