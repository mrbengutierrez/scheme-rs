#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Number(i64),
    Symbol(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    UnterminatedString,
    TestError,
    InvalidToken(String),
}

/// Tokenizes a Scheme source string into a vector of `Token`s.
/// 
/// Parses the input string into tokens including parentheses, symbols,
/// numbers, booleans, string literals, and skips comments and whitespace.
/// Returns a `LexError` if any invalid token is encountered.
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        let token_result = match ch {
            '(' => parse_lparen(&mut chars),
            ')' => parse_rparen(&mut chars),
            ';' => skip_comment(&mut chars),
            ch if ch.is_whitespace() => skip_whitespace(&mut chars),
            '"' => parse_string_literal(&mut chars),
            '#' => parse_boolean(&mut chars),
            ch if ch.is_ascii_digit() => parse_number(&mut chars),
            _ => parse_symbol(&mut chars),
        };

        match token_result {
            Some(Ok(token)) => tokens.push(token),
            Some(Err(e)) => return Err(e),
            None => {} // intentionally skipped (whitespace or comment)
        }
    }

    Ok(tokens)
}




fn parse_lparen<I>(chars: &mut I) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    chars.next();
    Some(Ok(Token::LParen))
}

fn parse_rparen<I>(chars: &mut I) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    chars.next();
    Some(Ok(Token::RParen))
}

fn skip_whitespace<I>(chars: &mut I) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    chars.next();
    None
}

fn skip_comment<I>(chars: &mut I) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    while let Some(c) = chars.next() {
        if c == '\n' {
            break;
        }
    }
    None
}

fn parse_string_literal<I>(chars: &mut I) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    let mut string = String::new();
    chars.next(); // consume first quote
    while let Some(c) = chars.next() {
        if c == '"' {
            return Some(Ok(Token::String(string)));
        } else if c == '\\' {
            match chars.next() {
                Some('n') => string.push('\n'),
                Some('t') => string.push('\t'),
                Some('"') => string.push('"'),
                Some('\\') => string.push('\\'),
                Some(escaped) => return Some(Err(LexError::InvalidToken(format!("\\{}", escaped)))),
                None => return Some(Err(LexError::UnterminatedString)),
            }
        } else {
            string.push(c);
        }
    }
    Some(Err(LexError::UnterminatedString))
}

fn parse_number<I>(chars: &mut std::iter::Peekable<I>) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    let mut num_str = String::new();
    while let Some(&next) = chars.peek() {
        if next.is_ascii_digit() {
            num_str.push(next);
            chars.next();
        } else {
            break;
        }
    }

    match num_str.parse::<i64>() {
        Ok(n) => Some(Ok(Token::Number(n))),
        Err(_) => Some(Err(LexError::InvalidToken(num_str))),
    }
}

fn parse_boolean<I>(chars: &mut std::iter::Peekable<I>) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    chars.next(); // consume #
    match chars.next() {
        Some('t') => Some(Ok(Token::Boolean(true))),
        Some('f') => Some(Ok(Token::Boolean(false))),
        other => Some(Err(LexError::InvalidToken(format!("#{:?}", other)))),
    }
}

fn parse_symbol<I>(chars: &mut std::iter::Peekable<I>) -> Option<Result<Token, LexError>>
where
    I: Iterator<Item = char>,
{
    let mut sym = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '(' || c == ')' {
            break;
        }
        sym.push(c);
        chars.next();
    }
    Some(Ok(Token::Symbol(sym)))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_valid_define_input() {
        let input = "(define adder (lambda (x y) (+ x y)))";
        let expected = vec![
            Token::LParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("adder".to_string()),
            Token::LParen,
            Token::Symbol("lambda".to_string()),
            Token::LParen,
            Token::Symbol("x".to_string()),
            Token::Symbol("y".to_string()),
            Token::RParen,
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Symbol("x".to_string()),
            Token::Symbol("y".to_string()),
            Token::RParen,
            Token::RParen,
            Token::RParen,
        ];

        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

        #[test]
    fn test_tokenize_valid_input_too_many_left_parenthesises() {
        let input = "((()";
        let expected = vec![
            Token::LParen,
            Token::LParen,
            Token::LParen,
            Token::RParen,
        ];

        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_simple_symbols() {
        let input = "foo bar123 +-*";
        let expected = vec![
            Token::Symbol("foo".into()),
            Token::Symbol("bar123".into()),
            Token::Symbol("+-*".into()),
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_numbers() {
        let input = "42 0 9999";
        let expected = vec![
            Token::Number(42),
            Token::Number(0),
            Token::Number(9999),
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_booleans() {
        let input = "#t #f";
        let expected = vec![
            Token::Boolean(true),
            Token::Boolean(false),
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_string_literal() {
        let input = "\"hello\" \"he\\nllo\"";
        let expected = vec![
            Token::String("hello".into()),
            Token::String("he\nllo".into()),
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_parens_mix() {
        let input = "(foo (bar))";
        let expected = vec![
            Token::LParen,
            Token::Symbol("foo".into()),
            Token::LParen,
            Token::Symbol("bar".into()),
            Token::RParen,
            Token::RParen,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_ignores_comments() {
        let input = "(foo ; comment here\n bar)";
        let expected = vec![
            Token::LParen,
            Token::Symbol("foo".into()),
            Token::Symbol("bar".into()),
            Token::RParen,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_unterminated_string() {
        let input = "\"unterminated";
        let result = tokenize(input);
        assert_eq!(result, Err(LexError::UnterminatedString));
    }

    #[test]
    fn test_tokenize_invalid_boolean() {
        let input = "#x";
        let result = tokenize(input);
        assert_eq!(result, Err(LexError::InvalidToken("#Some('x')".into())));
    }

    #[test]
    fn test_tokenize_invalid_escape_sequence() {
        let input = "\"bad\\qescape\"";
        let result = tokenize(input);
        assert_eq!(result, Err(LexError::InvalidToken("\\q".into())));
    }


}