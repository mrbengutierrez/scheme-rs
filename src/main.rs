use std::io::{self, Write};

use scheme_rs::env::default_env;
use scheme_rs::eval::eval;
use scheme_rs::lexer::tokenize;
use scheme_rs::parser::parse;

fn main() {
    let env = default_env(); // REPL uses a persistent environment
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("🦀 Welcome to the Scheme REPL (Rust Edition)");
    println!("💀 Type `exit` or `quit` when your existential dread sets in.");

    loop {
        print!("scheme-rs> ");
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
