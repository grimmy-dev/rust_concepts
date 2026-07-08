use crate::{
    eval::EvalError,
    expr::{Expression, Operation, Statement},
    tokenizer::Token,
};

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

pub fn parse(tokens: Vec<Token>) -> Result<Statement, EvalError> {
    let mut parser = Parser { tokens, pos: 0 };
    let stmt = parser.parse_stmt()?;
    if let Some(leftover) = parser.peek() {
        return Err(EvalError::ParserError(format!(
            "unexpected trailing token: {leftover:?}"
        )));
    }
    Ok(stmt)
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: Token) -> Result<(), EvalError> {
        match self.next() {
            Some(ref t) if *t == expected => Ok(()),
            other => Err(EvalError::ParserError(format!(
                "expected {expected:?}, got {other:?}"
            ))),
        }
    }

    fn parse_expr(&mut self) -> Result<Expression, EvalError> {
        let mut left = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.next();
                    let right = self.parse_term()?;
                    left = Expression::BinaryOp {
                        op: Operation::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some(Token::Minus) => {
                    self.next();
                    let right = self.parse_term()?;
                    left = Expression::BinaryOp {
                        op: Operation::Sub,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, EvalError> {
        let mut left = self.parse_factor()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.next();
                    let right = self.parse_factor()?;
                    left = Expression::BinaryOp {
                        op: Operation::Mul,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some(Token::Slash) => {
                    self.next();
                    let right = self.parse_factor()?;
                    left = Expression::BinaryOp {
                        op: Operation::Div,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, EvalError> {
        match self.next() {
            Some(Token::Minus) => {
                let inner = self.parse_factor()?;
                Ok(Expression::UnaryMinus(Box::new(inner)))
            }
            Some(Token::Num(n)) => Ok(Expression::Num(n)),
            Some(Token::Ident(name)) => Ok(Expression::Var(name)),
            Some(Token::LParen) => {
                let inner = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(inner)
            }
            Some(other) => Err(EvalError::ParserError(format!(
                "unexpected token: {other:?}"
            ))),
            None => Err(EvalError::ParserError(
                "unexpected end of input".to_string(),
            )),
        }
    }

    fn parse_stmt(&mut self) -> Result<Statement, EvalError> {
        if let Some(Token::Let) = self.peek() {
            self.next();
            let name = match self.next() {
                Some(Token::Ident(name)) => name,
                other => {
                    return Err(EvalError::ParserError(format!(
                        "expected identifier after 'let', got {other:?}"
                    )));
                }
            };
            self.expect(Token::Equals)?;
            let val = self.parse_expr()?;
            Ok(Statement::Let(name, val))
        } else {
            let val = self.parse_expr()?;
            Ok(Statement::ExprStatement(val))
        }
    }
}
