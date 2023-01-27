use std::env;
use std::fs;

// Cikkect the command line arguments into a vector and print them
fn main() {
    let args: Vec<String> = env::args().collect();

    // Creating variables to hold the query argument and file path argument
    // Programs name takes up the first value [0] which is why we start the variables at [1]
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}