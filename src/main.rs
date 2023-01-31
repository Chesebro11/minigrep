use std::env;
use std::fs;
use std::process;

// Collect the command line arguments into a vector and print them
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

        run(config);
}

// This function is pulled out of main to help readability of the program
fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

        println!("With text:\n{contents}");
}

//
struct Config {
    query: String,
    file_path: String,
}

// Return an isntance of a Config struct
// using clone to fix possible ownership issues, however this takes more time and memory than storing a reference
// This does make code straightforward not needing to manage lifetimes of the references
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}