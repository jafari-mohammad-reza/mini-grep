use std::env;
use std::fs;
use std::process;
use std::error::Error;
use mini_grep::{Config, run};

fn main() {
    let args :Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}" , err);
        process::exit(1);
    });
    match run(config){
        Ok(..) => println!("\n"),
        Err(err) => {
            println!("Application Error: {}" , err);
            process::exit(1);
        }
    };
}


