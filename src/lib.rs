#[wasm_bindgen]
pub struct EvalContext {
    env: Rc<Env>,
}

#[wasm_bindgen]
impl EvalContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EvalContext {
        EvalContext {
            env: default_env(),
        }
    }

    pub fn eval_line(&self, input: &str) -> String {
        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            return "👋 Goodbye and thanks for all the fish!".into();
        }

        let tokens = match tokenize(trimmed) {
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
