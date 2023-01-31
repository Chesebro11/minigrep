// Move over LIB logic to this file
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

// Defining just enough of the search function so our test will compile
// In order to pass the test i need to:
// Iterate through each line of the contents.
// Check whether the line contains our query string
// If it does, add it to the list of values we're returning.
// If it doesn't, do nothing.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // line method handles line by line iteration of strings
    for line in contents.lines() {
        // if line contains query do something
        if line.contains(query) {
            // store the lines that match so we can return them
            results.push(line);
        }
    }

    results
}

// Creating a failing test for the search function I wish I had
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
 safe, fast, productive.
Pick three.";

            assert_eq!(vec![" safe, fast, productive."], search(query, contents));
    }
}