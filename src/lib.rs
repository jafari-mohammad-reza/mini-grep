mod search;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::error::Error;
use std::{env, fs};
use std::io::Write;
use crate::search::search::{search, search_case_insensitive};

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.filename)?;
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Blue)).set_bold(true);
    let results = if config.case_sensitive {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in results {
        let colored_line = line.replacen(&config.query, &format!(
            "{}{}{}",
            "\x1B[33m", // ANSI escape code to set the color to orange
            &config.query,
            "\x1B[0m" // ANSI escape code to reset the color
        ), 1);
        stdout.set_color(&color_spec)?;
        writeln!(stdout, "{}", colored_line)?;
        stdout.reset()?;
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
