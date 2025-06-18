use crate::lexer::{Token, LexError};
use crate::ast::Expr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken(Token),
    LexError(LexError),
}

/// Parses a vector of tokens into an abstract syntax tree (AST).
///
/// Returns the root `Expr` on success, or a `ParseError` if the token stream is invalid.
pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut iter = tokens.into_iter().peekable();
    parse_expr(&mut iter)
}

fn parse_expr<I>(tokens: &mut std::iter::Peekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(Token::Number(n)) => Ok(Expr::Number(n)),
        Some(Token::Boolean(b)) => Ok(Expr::Boolean(b)),
        Some(Token::String(s)) => Ok(Expr::String(s)),
        Some(Token::Symbol(s)) => Ok(Expr::Symbol(s)),
        Some(Token::LParen) => parse_list(tokens),
        Some(Token::RParen) => Err(ParseError::UnexpectedToken(Token::RParen)),
        None => Err(ParseError::UnexpectedEOF),
    }
}

fn parse_list<I>(tokens: &mut std::iter::Peekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = Token>,
{
    let mut exprs = Vec::new();

    while let Some(token) = tokens.peek() {
        if *token == Token::RParen {
            tokens.next(); // consume RParen
            return Ok(Expr::List(exprs));
        }

        let expr = parse_expr(tokens)?;
        exprs.push(expr);
    }
    
    Err(ParseError::UnexpectedEOF)
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::lexer::tokenize;
    use crate::ast::Expr;

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(7)];
        let expr = parse(tokens).unwrap();
        assert_eq!(expr, Expr::Number(7));
    }


    #[test]
    fn test_parse_symbol() {
        let tokens = vec![Token::Symbol("foo".into())];
        let expr = parse(tokens).unwrap();
        assert_eq!(expr, Expr::Symbol("foo".into()));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".into())];
        let expr = parse(tokens).unwrap();
        assert_eq!(expr, Expr::String("hello".into()));
    }

    #[test]
    fn test_parse_boolean() {
        let tokens = vec![Token::Boolean(true)];
        let expr = parse(tokens).unwrap();
        assert_eq!(expr, Expr::Boolean(true));
    }

    #[test]
    fn test_parse_simple_list() {
        let tokens = tokenize("(+ 1 2)").unwrap();
        let expr = parse(tokens).unwrap();
        assert_eq!(
            expr,
            Expr::List(vec![
                Expr::Symbol("+".into()),
                Expr::Number(1),
                Expr::Number(2),
            ])
        )
    }

    #[test]
    fn test_parse_nested_list() {
        let tokens = tokenize("(define square (lambda (x) (* x x)))").unwrap();
        let expr = parse(tokens).unwrap();
        assert_eq!(
            expr,
            Expr::List(vec![
                Expr::Symbol("define".into()),
                Expr::Symbol("square".into()),
                Expr::List(vec![
                    Expr::Symbol("lambda".into()),
                    Expr::List(vec![
                        Expr::Symbol("x".into())
                    ]),
                    Expr::List(vec![
                        Expr::Symbol("*".into()),
                        Expr::Symbol("x".into()),
                        Expr::Symbol("x".into()),
                    ])
                ])
            ])
        );
    }

    #[test]
    fn test_parse_empty_list() {
        let tokens = tokenize("()").unwrap();
        let expr = parse(tokens).unwrap();
        assert_eq!(expr, Expr::List(vec![]));
    }

    #[test]
    fn test_parse_unexpected_token() {
        let tokens = vec![Token::RParen];
        let err = parse(tokens).unwrap_err();
        assert_eq!(err, ParseError::UnexpectedToken(Token::RParen));
    }

    #[test]
    fn test_parse_unexpected_eof_in_list() {
        let tokens = tokenize("(+ 1").unwrap();
        let err = parse(tokens).unwrap_err();
        assert_eq!(err, ParseError::UnexpectedEOF);
    }

    #[test]
    fn test_parse_complex_expression() {
        let tokens = tokenize("(if #t (display \"yes\") (display \"no\"))").unwrap();
        let expr = parse(tokens).unwrap();
        assert_eq!(
            expr,
            Expr::List(vec![
                Expr::Symbol("if".into()),
                Expr::Boolean(true),
                Expr::List(vec![
                    Expr::Symbol("display".into()),
                    Expr::String("yes".into()),
                ]),
                Expr::List(vec![
                    Expr::Symbol("display".into()),
                    Expr::String("no".into()),
                ]),
            ])
        );
    }

}