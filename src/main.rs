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

    let mut interpreter: interpreter::Interpreter = interpreter::Interpreter::new(&program);
    match interpreter.run() {
        Ok(_) => {
            println!("Program executed successfully.");
        },
        Err(e) => {
            match e {
                interpreter::InterpreterControl::Continue => {
                    eprintln!("Error: Program continued outside of a loop.");
                },
                interpreter::InterpreterControl::Break => {
                    eprintln!("Error: Program broke outside of a loop.");
                },
                interpreter::InterpreterControl::Return(value) => {
                    eprintln!("Error: Program returned ouside of a function: {}", value);
                },
                interpreter::InterpreterControl::RuntimeError(msg) => {
                    eprintln!("Runtime error: {}", msg);
                }
            }
        }
    }
}
