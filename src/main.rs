use crate::{
    evaluator::evaluator::evaluate,
    lexer::tokeniser::tokenise,
    parsers::main_parser::parse_programme,
};
mod evaluator;
mod expressions;
mod lexer;
mod parsers;
mod regex;
mod tokens;

use std::env;
use std::fs::File;
use std::io::{ self, Read };

fn main() {
    // Get the filename from command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if filename is provided as argument
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    // Open the file
    let file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: Unable to open file '{}'", args[1]);
            return;
        }
    };

    // Process the file content
    match process_file(file) {
        Ok(content) => {
            match tokenise(&content) {
                Ok(tokens) => {
                    match parse_programme(tokens) {
                        Ok(program) => {
                            match evaluate(program) {
                                Ok(result) => println!("{}", result),
                                Err(e) => println!("Error evaluating program: {}", e),
                            }
                        }
                        Err(e) => println!("Error parsing program: {}", e),
                    }
                }
                Err(e) => println!("Error tokenising content: {}", e),
            }
        }
        Err(e) => println!("Error processing file: {}", e),
    }
}

fn process_file(mut file: File) -> io::Result<String> {
    // Read the entire file content into a string
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
