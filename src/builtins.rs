use crate::env::{Value, EvalError};

/// Adds all numeric arguments. Returns the sum.
pub fn builtin_add(args: Vec<Value>) -> Result<Value, EvalError> {
    let sum = args.into_iter().map(|v| match v {
        Value::Number(n) => Ok(n),
        _ => Err(EvalError::TypeError("Expected number".into())),
    }).collect::<Result<Vec<_>, _>>()?.into_iter().sum();

    Ok(Value::Number(sum))
}

/// Subtracts all subsequent numbers from the first. Requires at least one argument.
pub fn builtin_sub(args: Vec<Value>) -> Result<Value, EvalError> {
    let mut nums = args
        .into_iter()
        .map(|v| match v {
            Value::Number(n) => Ok(n),
            _ => Err(EvalError::TypeError("Expected number".into())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    if nums.is_empty() {
        return Err(EvalError::Other("Expected at least one argument".into()));
    }

    let first = nums.remove(0);
    let result = nums.into_iter().fold(first, |acc, x| acc - x);

    Ok(Value::Number(result))
}

/// Multiplies all numeric arguments. Returns the product.
pub fn builtin_mul(args: Vec<Value>) -> Result<Value, EvalError> {
    let product = args.into_iter().map(|v| match v {
        Value::Number(n) => Ok(n),
        _ => Err(EvalError::TypeError("Expected number".into())),
    }).collect::<Result<Vec<_>, _>>()?.into_iter().product();

    Ok(Value::Number(product))
}

/// Divides the first number by each subsequent number. Returns an error on division by zero or if no arguments are provided.
pub fn builtin_div(args: Vec<Value>) -> Result<Value, EvalError> {
    let mut nums = args.into_iter().map(|v| match v {
        Value::Number(n) => Ok(n),
        _ => Err(EvalError::TypeError("Expected number".into())),
    }).collect::<Result<Vec<_>, _>>()?;

    if nums.is_empty() {
        return Err(EvalError::Other("Expected at least one argument".into()));
    }

    let first = nums.remove(0);
    let result = nums.into_iter().try_fold(first, |acc, x| {
        if x == 0 {
            Err(EvalError::Other("Division by zero".into()))
        } else {
            Ok(acc / x)
        }
    })?;

    Ok(Value::Number(result))
}

/// Returns true if all arguments are equal.
pub fn builtin_eq(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() < 2 {
        return Ok(Value::Boolean(true)); // Trivially equal
    }

    let first = &args[0];
    Ok(Value::Boolean(args.iter().all(|x| x == first)))
}

/// Returns true if arguments are in strictly increasing order.
pub fn builtin_lt(args: Vec<Value>) -> Result<Value, EvalError> {
    let nums = extract_numbers(args)?;
    Ok(Value::Boolean(nums.windows(2).all(|w| w[0] < w[1])))
}

/// Returns true if arguments are in strictly decreasing order.
pub fn builtin_gt(args: Vec<Value>) -> Result<Value, EvalError> {
    let nums = extract_numbers(args)?;
    Ok(Value::Boolean(nums.windows(2).all(|w| w[0] > w[1])))
}

/// Extracts and validates numeric arguments. Used internally.
fn extract_numbers(args: Vec<Value>) -> Result<Vec<i64>, EvalError> {
    args.into_iter().map(|v| match v {
        Value::Number(n) => Ok(n),
        _ => Err(EvalError::TypeError("Expected number".into())),
    }).collect()
}

/// Returns false if any argument is false, otherwise true. All arguments must be booleans.
pub fn builtin_and(args: Vec<Value>) -> Result<Value, EvalError> {
    for arg in args {
        match arg {
            Value::Boolean(false) => return Ok(Value::Boolean(false)),
            Value::Boolean(true) => continue,
            _ => return Err(EvalError::TypeError("Expected boolean".into())),
        }
    }
    Ok(Value::Boolean(true))
}

/// Returns true if any argument is true, otherwise false. All arguments must be booleans.
pub fn builtin_or(args: Vec<Value>) -> Result<Value, EvalError> {
    for arg in args {
        match arg {
            Value::Boolean(true) => return Ok(Value::Boolean(true)),
            Value::Boolean(false) => continue,
            _ => return Err(EvalError::TypeError("Expected boolean".into())),
        }
    }
    Ok(Value::Boolean(false))
}

/// Returns the boolean negation of the single argument. Argument must be a boolean.
pub fn builtin_not(args: Vec<Value>) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityMismatch);
    }
    match args[0] {
        Value::Boolean(b) => Ok(Value::Boolean(!b)),
        _ => Err(EvalError::TypeError("Expected boolean".into())),
    }
}

/// Constructs a list from the given arguments.
pub fn builtin_list(args: Vec<Value>) -> Result<Value, EvalError> {
    Ok(Value::List(args))
}

/// Returns the first element of a non-empty list.
pub fn builtin_car(args: Vec<Value>) -> Result<Value, EvalError> {
    match &args[..] {
        [Value::List(list)] if !list.is_empty() => Ok(list[0].clone()),
        _ => Err(EvalError::TypeError("Expected non-empty list".into())),
    }
}

/// Returns the rest of a non-empty list, excluding the first element.
pub fn builtin_cdr(args: Vec<Value>) -> Result<Value, EvalError> {
    match &args[..] {
        [Value::List(list)] if !list.is_empty() => {
            Ok(Value::List(list[1..].to_vec()))
        }
        _ => Err(EvalError::TypeError("Expected non-empty list".into())),
    }
}

/// Prepends a value to a list and returns the new list.
pub fn builtin_cons(args: Vec<Value>) -> Result<Value, EvalError> {
    match &args[..] {
        [item, Value::List(rest)] => {
            let mut new_list = vec![item.clone()];
            new_list.extend_from_slice(rest);
            Ok(Value::List(new_list))
        }
        _ => Err(EvalError::TypeError("Expected value and list".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::Value;

    #[test]
    fn test_builtin_add_normal_case() {
        let args = vec![Value::Number(1), Value::Number(2), Value::Number(3)];
        let result = builtin_add(args).unwrap();
        assert_eq!(result, Value::Number(6));
    }

    #[test]
    fn test_builtin_add_empty_args() {
        let args = vec![];
        let result = builtin_add(args).unwrap();
        assert_eq!(result, Value::Number(0)); // Sum of no numbers is 0
    }

    #[test]
    fn test_builtin_add_type_error() {
        let args = vec![Value::Number(1), Value::String("bad".into())];
        let result = builtin_add(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_sub_normal_case() {
        let args = vec![Value::Number(10), Value::Number(3), Value::Number(2)];
        let result = builtin_sub(args).unwrap();
        assert_eq!(result, Value::Number(5)); // 10 - 3 - 2
    }

    #[test]
    fn test_builtin_sub_single_arg() {
        let args = vec![Value::Number(5)];
        let result = builtin_sub(args).unwrap();
        assert_eq!(result, Value::Number(5)); // single arg, nothing to subtract
    }

    #[test]
    fn test_builtin_sub_type_error() {
        let args = vec![Value::Number(1), Value::Boolean(true)];
        let result = builtin_sub(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_sub_empty_args_should_error() {
        let args = vec![];
        let result = builtin_sub(args);
        assert!(result.is_err()); // panics or returns an error due to remove(0)
    }

        #[test]
    fn test_builtin_mul_normal_case() {
        let args = vec![Value::Number(2), Value::Number(3), Value::Number(4)];
        let result = builtin_mul(args).unwrap();
        assert_eq!(result, Value::Number(24));
    }

    #[test]
    fn test_builtin_mul_empty_args() {
        let args = vec![];
        let result = builtin_mul(args).unwrap();
        assert_eq!(result, Value::Number(1)); // product of no numbers is 1
    }

    #[test]
    fn test_builtin_mul_type_error() {
        let args = vec![Value::Number(2), Value::Boolean(true)];
        let result = builtin_mul(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_div_normal_case() {
        let args = vec![Value::Number(20), Value::Number(2), Value::Number(2)];
        let result = builtin_div(args).unwrap();
        assert_eq!(result, Value::Number(5)); // 20 / 2 / 2
    }

    #[test]
    fn test_builtin_div_divide_by_zero() {
        let args = vec![Value::Number(10), Value::Number(0)];
        let result = builtin_div(args);
        assert!(matches!(result, Err(EvalError::Other(_))));
    }

    #[test]
    fn test_builtin_div_empty_args_should_error() {
        let args = vec![];
        let result = builtin_div(args);
        assert!(matches!(result, Err(EvalError::Other(_))));
    }

    #[test]
    fn test_builtin_eq_true() {
        let args = vec![Value::Number(5), Value::Number(5), Value::Number(5)];
        let result = builtin_eq(args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_eq_false() {
        let args = vec![Value::Number(5), Value::Number(6)];
        let result = builtin_eq(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_lt_true() {
        let args = vec![Value::Number(1), Value::Number(2), Value::Number(3)];
        let result = builtin_lt(args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_lt_false() {
        let args = vec![Value::Number(1), Value::Number(3), Value::Number(2)];
        let result = builtin_lt(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_gt_true() {
        let args = vec![Value::Number(5), Value::Number(3), Value::Number(1)];
        let result = builtin_gt(args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_gt_false() {
        let args = vec![Value::Number(5), Value::Number(6)];
        let result = builtin_gt(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_and_true() {
        let args = vec![Value::Boolean(true), Value::Boolean(true)];
        let result = builtin_and(args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_and_false() {
        let args = vec![Value::Boolean(true), Value::Boolean(false)];
        let result = builtin_and(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_and_type_error() {
        let args = vec![Value::Boolean(true), Value::Number(1)];
        let result = builtin_and(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_or_true() {
        let args = vec![Value::Boolean(false), Value::Boolean(true)];
        let result = builtin_or(args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_or_false() {
        let args = vec![Value::Boolean(false), Value::Boolean(false)];
        let result = builtin_or(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_or_type_error() {
        let args = vec![Value::Boolean(false), Value::Number(42)];
        let result = builtin_or(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_not_true() {
        let args = vec![Value::Boolean(true)];
        let result = builtin_not(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_builtin_not_false() {
        let args = vec![Value::Boolean(false)];
        let result = builtin_not(args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_builtin_not_wrong_arity() {
        let args = vec![];
        let result = builtin_not(args);
        assert!(matches!(result, Err(EvalError::ArityMismatch)));
    }

    #[test]
    fn test_builtin_list_basic() {
        let args = vec![Value::Number(1), Value::Boolean(true)];
        let result = builtin_list(args.clone()).unwrap();
        assert_eq!(result, Value::List(args));
    }

    #[test]
    fn test_builtin_car_success() {
        let args = vec![Value::List(vec![Value::Number(42), Value::Boolean(false)])];
        let result = builtin_car(args).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_builtin_car_empty_list_error() {
        let args = vec![Value::List(vec![])];
        let result = builtin_car(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_cdr_success() {
        let args = vec![Value::List(vec![Value::Number(42), Value::Boolean(false)])];
        let result = builtin_cdr(args).unwrap();
        assert_eq!(result, Value::List(vec![Value::Boolean(false)]));
    }

    #[test]
    fn test_builtin_cdr_empty_list_error() {
        let args = vec![Value::List(vec![])];
        let result = builtin_cdr(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }

    #[test]
    fn test_builtin_cons_success() {
        let args = vec![
            Value::Number(1),
            Value::List(vec![Value::Number(2), Value::Number(3)]),
        ];
        let result = builtin_cons(args).unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(3)])
        );
    }

    #[test]
    fn test_builtin_cons_type_error() {
        let args = vec![Value::Number(1), Value::Number(2)];
        let result = builtin_cons(args);
        assert!(matches!(result, Err(EvalError::TypeError(_))));
    }
}
