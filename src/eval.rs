use std::rc::Rc;

use crate::env::{Env, EvalError, Value, Lambda};
use crate::ast::Expr;

/// Evaluates a Racket expression in the given environment.
///
/// Supports literals (number, boolean, string), variable lookup, and
/// special forms: `define`, `lambda`, `begin`, `if`, and `let`.
/// Function calls are evaluated as applications of built-in or user-defined
/// functions (lambdas).
///
/// # Arguments
/// * `expr` - The Racket expression to evaluate.
/// * `env` - The current lexical environment.
///
/// # Returns
/// * `Ok(Value)` if evaluation succeeds.
/// * `Err(EvalError)` on undefined symbols, type errors, arity mismatches, or invalid calls.
pub fn eval(expr: &Expr, env: Rc<Env>) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Symbol(s) => env.get(s).ok_or_else(|| EvalError::UndefinedSymbol(s.clone())),
        Expr::List(list) => {
            if list.is_empty() {
                return Ok(Value::List(vec![]));
            }

            match &list[0] {
                Expr::Symbol(s) if s == "define" => eval_define(&list, env),
                Expr::Symbol(s) if s == "lambda" => eval_lambda(&list, env),
                Expr::Symbol(s) if s == "begin" => eval_begin(&list, env),
                Expr::Symbol(s) if s == "if" => eval_if(&list, env),
                Expr::Symbol(s) if s == "let" => eval_let(&list, env),
                _ => eval_application(&list, env),
            }
        }
    }
}

fn eval_define(list: &[Expr], env: Rc<Env>) -> Result<Value, EvalError> {
    let name = match &list[1] {
        Expr::Symbol(sym) => sym.clone(),
        _ => return Err(EvalError::TypeError("Expected symbol after define".into())),
    };
    let value = eval(&list[2], env.clone())?;
    env.define(name, value.clone());
    Ok(value)
}

fn eval_lambda(list: &[Expr], env: Rc<Env>) -> Result<Value, EvalError> {
    let params = match &list[1] {
        Expr::List(p) => p.iter().map(|x| match x {
            Expr::Symbol(s) => Ok(s.clone()),
            _ => Err(EvalError::TypeError("Expected symbol in parameter list".into())),
        }).collect::<Result<Vec<_>, _>>()?,
        _ => return Err(EvalError::TypeError("Expected list of params".into())),
    };
    let body = list[2].clone();
    Ok(Value::Lambda(Lambda { params, body, env }))
}

fn eval_begin(list: &[Expr], env: Rc<Env>) -> Result<Value, EvalError> {
    let mut result = Value::Boolean(false);
    for expr in &list[1..] {
        result = eval(expr, env.clone())?;
    }
    Ok(result)
}

fn eval_if(list: &[Expr], env: Rc<Env>) -> Result<Value, EvalError> {
    if list.len() != 4 {
        return Err(EvalError::ArityMismatch);
    }
    let cond = eval(&list[1], env.clone())?;
    match cond {
        Value::Boolean(true) => eval(&list[2], env),
        Value::Boolean(false) => eval(&list[3], env),
        _ => Err(EvalError::TypeError("Expected boolean in if condition".into())),
    }
}

fn eval_let(list: &[Expr], env: Rc<Env>) -> Result<Value, EvalError> {
    if list.len() != 3 {
        return Err(EvalError::ArityMismatch);
    }

    let bindings = match &list[1] {
        Expr::List(pairs) => pairs,
        _ => return Err(EvalError::TypeError("Expected list of bindings in let".into())),
    };

    let new_env = Env::extend(env.clone());
    for pair in bindings {
        match pair {
            Expr::List(pair_vec) if pair_vec.len() == 2 => {
                let name = match &pair_vec[0] {
                    Expr::Symbol(s) => s.clone(),
                    _ => return Err(EvalError::TypeError("Expected symbol in let binding".into())),
                };
                let value = eval(&pair_vec[1], env.clone())?;
                new_env.define(name, value);
            }
            _ => return Err(EvalError::TypeError("Invalid binding in let".into())),
        }
    }

    eval(&list[2], new_env)
}

fn eval_application(list: &[Expr], env: Rc<Env>) -> Result<Value, EvalError> {
    let func_val = eval(&list[0], env.clone())?;
    let arg_vals = list[1..].iter()
        .map(|arg| eval(arg, env.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    match func_val {
        Value::Function(f) => f(arg_vals),
        Value::Lambda(l) => {
            if l.params.len() != arg_vals.len() {
                return Err(EvalError::ArityMismatch);
            }
            let new_env = Env::extend(l.env);
            for (k, v) in l.params.iter().zip(arg_vals.into_iter()) {
                new_env.define(k.clone(), v);
            }
            eval(&l.body, new_env)
        }
        _ => Err(EvalError::NotCallable),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;
    use crate::env::default_env;
    
    fn eval_expr(source: &str) -> Result<Value, EvalError> {
        let tokens = crate::lexer::tokenize(source).unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let env = default_env();  // Use built-ins
        eval(&ast, env)
    }

    #[test]
    fn test_eval_number() {
        let result = eval_expr("42").unwrap();
        assert_eq!(result, Value::Number(42));
    }

     #[test]
    fn test_eval_boolean() {
        let result = eval_expr("#t").unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_eval_string() {
        let result = eval_expr("\"hello\"").unwrap();
        assert_eq!(result, Value::String("hello".into()));
    }

    #[test]
    fn test_define_variable() {
        let tokens = tokenize("(define x 10)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = Env::new();
        let result = eval(&ast, env.clone()).unwrap();
        assert_eq!(result, Value::Number(10));
        assert_eq!(env.get("x"), Some(Value::Number(10)));
    }

    #[test]
    fn test_simple_lambda() {
        let tokens = crate::lexer::tokenize("((lambda (x) x) 5)").unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let result = eval(&ast, Env::new()).unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_non_callable_error() {
        let tokens = crate::lexer::tokenize("(42 1)").unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let result = eval(&ast, Env::new());
        assert!(matches!(result, Err(EvalError::NotCallable)));
    }

    #[test]
    fn test_arity_mismatch() {
        let tokens = crate::lexer::tokenize("((lambda (x y) x) 1)").unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let result = eval(&ast, Env::new());
        assert!(matches!(result, Err(EvalError::ArityMismatch)));
    }

    #[test]
    fn test_undefined_symbol() {
        let tokens = crate::lexer::tokenize("y").unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let result = eval(&ast, Env::new());
        assert!(matches!(result, Err(EvalError::UndefinedSymbol(_))));
    }

    #[test]
    fn test_if_true_branch() {
        let result = eval_expr("(if #t 1 2)").unwrap();
        assert_eq!(result, Value::Number(1));
    }

    #[test]
    fn test_if_false_branch() {
        let result = eval_expr("(if #f 1 2)").unwrap();
        assert_eq!(result, Value::Number(2));
    }

    #[test]
    fn test_if_nested_conditions() {
        let result = eval_expr("(if #f 1 (if #t 2 3))").unwrap();
        assert_eq!(result, Value::Number(2));
    }

    #[test]
    fn test_if_non_boolean_condition_should_error() {
        let result = eval_expr("(if 5 1 2)");
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_if_with_side_effects() {
        let tokens = tokenize("
            (begin
                (define x 0)
                (if #t (define x 42) (define x 99))
                x
            )
        ").unwrap();
        let ast = parse(tokens).unwrap();
        let env = Env::new();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_if_arity_too_small() {
        let result = eval_expr("(if #t 1)");
        assert!(matches!(result, Err(EvalError::ArityMismatch)));
    }

    #[test]
    fn test_if_arity_too_large() {
        let result = eval_expr("(if #t 1 2 3)");
        assert!(matches!(result, Err(EvalError::ArityMismatch)));
    }

    #[test]
    fn test_begin_returns_last_value() {
        let result = eval_expr("(begin 1 2 3)").unwrap();
        assert_eq!(result, Value::Number(3));
    }

    #[test]
    fn test_begin_side_effect_define() {
        let tokens = tokenize("(begin (define x 5) x)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = Env::new();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_let_binds_variables() {
        let result = eval_expr("(let ((x 2) (y 3)) (+ x y))").unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_let_inner_shadowing() {
        let result = eval_expr("(let ((x 1)) (let ((x 2)) x))").unwrap();
        assert_eq!(result, Value::Number(2));
    }

    #[test]
    fn test_let_scope_is_local() {
        let tokens = tokenize("(begin (let ((x 1)) x) x)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = Env::new();
        let result = eval(&ast, env);
        assert!(matches!(result, Err(EvalError::UndefinedSymbol(sym)) if sym == "x"));
    }

    #[test]
    fn test_let_type_error_if_not_pair() {
        let result = eval_expr("(let (x 1) x)");
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_let_type_error_on_bad_binding() {
        let result = eval_expr("(let ((1 2)) 3)");
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }



    // Test built-ins:

    #[test]
    fn test_builtin_add_evaluates_correctly() {
        let tokens = crate::lexer::tokenize("(+ 1 2 3)").unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let env = crate::env::default_env();  // Use the populated env
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(6));
    }

    #[test]
    fn test_builtin_sub_evaluates_correctly() {
        let tokens = crate::lexer::tokenize("(- 10 3)").unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let env = crate::env::default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(7));
    }

    #[test]
    fn test_builtin_add_type_error_evaluates() {
        let tokens = tokenize("(+ 1 \"oops\")").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

        #[test]
    fn test_builtin_mul_evaluates_correctly() {
        let tokens = tokenize("(* 2 3 4)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(24));
    }

    #[test]
    fn test_builtin_div_evaluates_correctly() {
        let tokens = tokenize("(/ 20 2 2)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn test_builtin_div_by_zero_error() {
        let tokens = tokenize("(/ 5 0)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env);
        assert!(matches!(result, Err(EvalError::Other(_))));
    }

    #[test]
    fn test_builtin_eq_true() {
        let tokens = tokenize("(= 5 5 5)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_eq_false() {
        let tokens = tokenize("(= 1 2)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_lt_true() {
        let tokens = tokenize("(< 1 2 3)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_lt_false() {
        let tokens = tokenize("(< 1 3 2)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_gt_true() {
        let tokens = tokenize("(> 5 4 2)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_gt_false() {
        let tokens = tokenize("(> 5 6)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_and_true() {
        let tokens = tokenize("(and #t #t)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_and_false() {
        let tokens = tokenize("(and #t #f)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_or_true() {
        let tokens = tokenize("(or #f #t)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_or_false() {
        let tokens = tokenize("(or #f #f)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_not_true() {
        let tokens = tokenize("(not #t)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_not_false() {
        let tokens = tokenize("(not #f)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_list() {
        let tokens = tokenize("(list 1 2 3)").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
        ]));
    }

    #[test]
    fn test_builtin_car() {
        let tokens = tokenize("(car (list 10 20))").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::Number(10));
    }

    #[test]
    fn test_builtin_cdr() {
        let tokens = tokenize("(cdr (list 10 20 30))").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(result, Value::List(vec![Value::Number(20), Value::Number(30)]));
    }

    #[test]
    fn test_builtin_cons() {
        let tokens = tokenize("(cons 5 (list 6 7))").unwrap();
        let ast = parse(tokens).unwrap();
        let env = default_env();
        let result = eval(&ast, env).unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(5), Value::Number(6), Value::Number(7)])
        );
    }

}