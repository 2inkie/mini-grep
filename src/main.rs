use std::{env, fs, process};
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments - usage: mini_grep <query> <filename>");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}' in file '{}'", config.query, config.filename);

    let filepath = "./files/".to_string();
    let file = filepath + &config.filename;

    let contents = fs::read_to_string(file)?;

    search_in_file(&config.query, &contents);
    Ok(())
}

fn search_in_file(query: &str, file_contents: &str) {
    for line in file_contents.lines() {
        if line.contains(query) {
            println!("Found: {}", line);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}