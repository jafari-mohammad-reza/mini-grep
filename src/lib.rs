mod search;

use std::error::Error;
use std::{env, fs};
use crate::search::search::{search, search_case_insensitive};

pub fn run(config:Config) -> Result<() , Box<dyn Error>>{
    let content:String = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
         search_case_insensitive(&config.query , &content)
    } else{
        search(&config.query , &content)
    };
    for result in results{
        println!("{}" , result)
    }
    Ok(())
}



pub struct Config {
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
impl Config {
    pub fn new(args:&[String]) ->  Result<Config , &str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query : String = args[1].clone();
        let filename : String = args[2].clone();
        let case_sensitive = env::args().any(|arg| arg == "--insensitive");
        Ok(Config {query,filename, case_sensitive })
    }
}
