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

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(&ch) = chars.peek() {
        println!("{}", ch);
        match ch {
            '(' => { // Left Parenthesis
                tokens.push(Token::LParen);
                chars.next();
            }

            ')' => { // Right Parenthesis
                tokens.push(Token::RParen);
                chars.next();
            }
            
            ';' => { // Comment, skip to end of lin
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    }
                }
            }

            ch if ch.is_whitespace() => { // Whitespace, skip
                chars.next(); 
            }

            '"' => { // String literal
                chars.next(); // consume the opening quote
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' { // closing quote
                        break;
                    } else if c == '\\' { // special characters
                        match chars.next() {
                            Some('n') => string.push('\n'),
                            Some('t') => string.push('\t'),
                            Some('"') => string.push('"'),
                            Some('\\') => string.push('\\'),
                            Some(escaped) => {
                                return Err(LexError::InvalidToken(format!("\\{}", escaped)))
                            }
                            None => return Err(LexError::UnterminatedString),
                        }
                    } else {
                        string.push(c);
                    }
                }
                tokens.push(Token::String(string));
            }

            '#' => { // Boolean: #t or #f
                chars.next(); // consume #
                match chars.next() {
                    Some('t') => tokens.push(Token::Boolean(true)),
                    Some('f') => tokens.push(Token::Boolean(false)),
                    other => {
                        return Err(LexError::InvalidToken(format!(
                            "#{:?}",
                            other.unwrap_or('\0')
                        )))
                    }
                }
            }

            // number
            ch if ch.is_ascii_digit() || ch == '_' => {
                // Number (we assume integers for now)
                let mut num_str = String::new();
                num_str.push(ch);
                chars.next();

                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        num_str.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match num_str.parse::<i64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => return Err(LexError::InvalidToken(num_str)),
                }
            }

            _ => { // Symbol
                let mut sym = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '(' || c == ')' {
                        break;
                    }
                    sym.push(c);
                    chars.next();
                }
                tokens.push(Token::Symbol(sym));
            }
        }
    }

    println!("tokens: {:?}", tokens);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "this (is) a test";
        let result = tokenize(input);
        assert!(matches!(result, Err(LexError::TestError)));
    }
}