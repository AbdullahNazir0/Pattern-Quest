use std::env;
use std::process;

use pattern_quest::{Config, run};
// use pattern::quest;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(&config) {
        println!("Problem reading file: {}", err);
        process::exit(1);
    }
}
