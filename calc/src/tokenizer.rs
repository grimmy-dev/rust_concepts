use crate::eval::EvalError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Ident(String),
    Let,
    Equals,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, EvalError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num_str = String::new();
                let mut seen_dot = false;
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        chars.next();
                    } else if c == '.' && !seen_dot {
                        seen_dot = true;
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n: f64 = num_str
                    .parse()
                    .map_err(|_| EvalError::ParserError(format!("Invalid number : {num_str}")))?;
                tokens.push(Token::Num(n));
            }
            c if c == '_' || c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if ident == "let" {
                    tokens.push(Token::Let);
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            other => {
                return Err(EvalError::ParserError(format!(
                    "unexcepted character :{other:?}"
                )));
            }
        }
    }
    Ok(tokens)
}
