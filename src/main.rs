use std::env;
use std::fs;

// Collect the command line arguments into a vector and print them
fn main() {
    let args: Vec<String> = env::args().collect();

    
    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}

// Creating variables to hold the query argument and file path argument
// Programs name takes up the first value [0] which is why we start the variables at [1]
// moved query and filepath logic to seperate function
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}