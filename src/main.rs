mod lexer;
mod tokens;
mod parser;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::tokens::Token;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Error: Incorrect use. Use 'nextron <input.nex>' to run the program.");
        exit(101);
    }

    let mut file = match File::open(args.iter().nth(1).unwrap().as_str()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        }
    }

    let mut lex = Lexer::new(contents);
    let tokens: Vec<Token> = lex.tokenize();

    let mut parser = Parser::new(tokens);
    parser.parse();
}
