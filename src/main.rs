mod arg_parser;
use crate::arg_parser::ArgParser;
use std::env;

fn main() {
    let args = env::args();
    let parser = ArgParser::new(vec![
        (vec![String::from("-f"), String::from("--from")], 1)
    ]);

    println!("Hello, world!");
}
