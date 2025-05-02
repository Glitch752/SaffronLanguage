use std::fs;

use clap::{command, Parser};

mod tokenizer;
mod parser;
mod interpreter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input file
    input: String
}

fn main() {
    let args: Args = Args::parse();

    // Read the input file
    let input: String = fs::read_to_string(args.input).expect("Failed to read input file.");

    let mut lex: tokenizer::Tokenizer = tokenizer::Tokenizer::new(input);

    // Split the input into tokens
    let tokens = match lex.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut parser: parser::Parser = parser::Parser::new(&tokens);
    let program = match parser.parse_program() {
        Some(program) => program,
        None => {
            eprintln!("Error: Failed to parse the program.");
            return;
        }
    };

    // Print the parsed program
    println!("\nParsed program: {:#?}", program);
}
