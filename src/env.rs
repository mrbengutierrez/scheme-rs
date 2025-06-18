use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::Expr;
use crate::builtins::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    parent: Option<Rc<Env>>,
    vars: RefCell<HashMap<String,Value>>,
}

impl Env {
    /// Creates a new, empty global environment with no parent.
    pub fn new() -> Rc<Self> {
        Rc::new(Env {
            parent: None,
            vars: RefCell::new(HashMap::new()),
        })
    }

    /// Creates a new environment that extends a parent environment.
    pub fn extend(parent: Rc<Env>) -> Rc<Self> {
        Rc::new(Env {
            parent: Some(parent),
            vars: RefCell::new(HashMap::new()),
        })
    }

    /// Defines a new variable or updates an existing one in the current environment.
    pub fn define(&self, key: String, value: Value) {
        self.vars.borrow_mut().insert(key, value);
    }

    /// Looks up a variable by name, searching parent environments if needed.
    pub fn get(&self, key: &str) -> Option<Value> {
        self.vars.borrow().get(key).cloned().or_else(|| {
            self.parent.as_ref()?.get(key)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(String),
    Symbol(String),
    Function(fn(Vec<Value>) -> Result<Value, EvalError>), // built-in functions
    Lambda(Lambda), // user-defined functions
    List(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Expr,
    pub env: Rc<Env>
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    TypeError(String),
    ArityMismatch,
    NotCallable,
    Other(String),
}


/// Returns the default global environment with all built-in functions registered.
pub fn default_env() -> Rc<Env> {
    let env = Env::new();

    env.define("+".into(), Value::Function(builtin_add));
    env.define("-".into(), Value::Function(builtin_sub));
    env.define("*".into(), Value::Function(builtin_mul));
    env.define("/".into(), Value::Function(builtin_div));

    env.define("=".into(), Value::Function(builtin_eq));
    env.define("<".into(), Value::Function(builtin_lt));
    env.define(">".into(), Value::Function(builtin_gt));

    env.define("and".into(), Value::Function(builtin_and));
    env.define("or".into(), Value::Function(builtin_or));
    env.define("not".into(), Value::Function(builtin_not));

    env.define("list".into(), Value::Function(builtin_list));
    env.define("car".into(), Value::Function(builtin_car));
    env.define("cdr".into(), Value::Function(builtin_cdr));
    env.define("cons".into(), Value::Function(builtin_cons));

    env
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get() {
        let env = Env::new();
        env.define("x".to_string(), Value::Number(42));
        assert_eq!(env.get("x"), Some(Value::Number(42)));
    }

    #[test]
    fn test_get_from_parent_env() {
        let parent = Env::new();
        parent.define("x".to_string(), Value::Number(1));

        let child = Env::extend(parent);
        assert_eq!(child.get("x"), Some(Value::Number(1)));
    }

    #[test]
    fn test_shadowing_variable_in_child() {
        let parent = Env::new();
        parent.define("x".to_string(), Value::Number(1));

        let child = Env::extend(parent.clone());
        child.define("x".to_string(), Value::Number(99));

        // Ensure parent isn't overwritten
        assert_eq!(child.get("x"), Some(Value::Number(99)));
        assert_eq!(parent.get("x"), Some(Value::Number(1)));
    }

    #[test]
    fn test_undefined_variable_returns_none() {
        let env = Env::new();
        assert_eq!(env.get("y"), None);
    }
}