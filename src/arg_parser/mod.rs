/*
 * Create list of args,
 * Send to method with args
 * fills out args
 *
 * */
use std::env;

struct Arg {
    flags: Vec<String>,
    filled: bool,
    count_min: u8,
    values: Option<Vec<String>>,
}

pub struct ArgParser {
    args: Option<Vec<Arg>>,
    parsed: bool,
}

impl ArgParser {
    pub fn new(a: Vec<(Vec<String>, u8)>) -> ArgParser {
        let ag = a
            .into_iter()
            .map(|x| Arg {
                flags: x.0,
                filled: false,
                count_min: x.1,
                values: None,
            })
            .collect();
        ArgParser {
            args: Some(ag),
            parsed: false,
        }
    }
    fn fill(&mut self, args: env::Args) {

    }
}
