use std::fs;

use clap::{command, Parser};

mod tokenizer;

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
    let tokens: Vec<tokenizer::Token> = match lex.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Print the tokens
    for token in tokens.clone() {
        println!("{:?}", token);
    }

    // Reverse format the tokens
    for token in tokens {
        print!("{} ", token.reverse_format());
    }

    println!("Done.");
}
