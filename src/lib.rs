use wasm_bindgen::prelude::*;

use crate::env::default_env;
use crate::eval::eval;
use crate::lexer::tokenize;
use crate::parser::parse;

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod eval;
pub mod env;
pub mod builtins;

/// Expose Racket eval to JavaScript
#[wasm_bindgen]
pub fn eval_racket(input: &str) -> String {
    let env = default_env();

    let tokens = match tokenize(input) {
        Ok(t) => t,
        Err(e) => return format!("Lex error: {:?}", e),
    };

    let ast = match parse(tokens) {
        Ok(a) => a,
        Err(e) => return format!("Parse error: {:?}", e),
    };

    match eval(&ast, env) {
        Ok(val) => format!("{}", val),
        Err(e) => format!("Eval error: {:?}", e),
    }
}

