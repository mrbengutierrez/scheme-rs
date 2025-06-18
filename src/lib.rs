use wasm_bindgen::prelude::*;
use std::rc::Rc;

use crate::env::{default_env, Env};
use crate::eval::eval;
use crate::lexer::tokenize;
use crate::parser::parse;

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod eval;
pub mod env;
pub mod builtins;

/// Persistent evaluator context across calls
#[wasm_bindgen]
pub struct EvalContext {
    env: Rc<Env>,
}

/// Expose WASM to Javascript
#[wasm_bindgen]
impl EvalContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EvalContext {
        EvalContext {
            env: default_env(),
        }
    }

    pub fn eval(&self, input: &str) -> String {
        let tokens = match tokenize(input) {
            Ok(t) => t,
            Err(e) => return format!("Lex error: {:?}", e),
        };

        let ast = match parse(tokens) {
            Ok(a) => a,
            Err(e) => return format!("Parse error: {:?}", e),
        };

        match eval(&ast, self.env.clone()) {
            Ok(val) => format!("{}", val),
            Err(e) => format!("Eval error: {:?}", e),
        }
    }
}
