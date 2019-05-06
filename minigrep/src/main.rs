
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(args);
    let contents = fs::read_to_string(config.filename.clone())
                        .expect(&format!("Failted to read file: {}", config.filename.clone())[..]);

    let result = search(config.query.clone(), contents);
}

pub struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: Vec<String>) -> Config {
    Config{
        query: args[0].clone(),
        filename: args[1].clone(),
    }
}

fn excute(config: Config) -> Vec<String> {
    vec![]
}

fn search(query: String, contents: String) -> Vec<String> {
    vec![]
}