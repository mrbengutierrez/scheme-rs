use std::io::{self, Write};

use racket_rs::env::default_env;
use racket_rs::eval::eval;
use racket_rs::lexer::tokenize;
use racket_rs::parser::parse;

fn main() {
    let env = default_env(); // REPL uses a persistent environment
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("🦀 Welcome to the Racket REPL (Rust Edition)");
    println!("💀 Type `exit` or `quit` when your existential dread sets in.");

    loop {
        print!("racket-rs> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        match tokenize(trimmed) {
            Ok(tokens) => match parse(tokens) {
                Ok(ast) => match eval(&ast, env.clone()) {
                    Ok(result) => println!("{}", result),
                    Err(e) => eprintln!("Eval error: {:?}", e),
                },
                Err(e) => eprintln!("Parse error: {:?}", e),
            },
            Err(e) => eprintln!("Lex error: {:?}", e),
        }
    }

    println!("👋 Goodbye and thanks for all the fish!");
}
