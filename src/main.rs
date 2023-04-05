
use std::env;
use std::process;
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



#[cfg(test)]
mod test {
    use mini_grep::{search, search_case_insensitive};
    #[test]
    fn one_result(){
        let query = "application"; // for searching in application word
        let contents = "Rust application test";
        assert_eq!(vec!["Rust application test"] , search(query , contents))
    }
    #[test]
    fn case_insensitive(){
        let query = "ruSt"; // for searching in application word
        let contents = "Rust application test";
        assert_eq!(vec!["Rust application test"] , search_case_insensitive(query , contents))
    }
}