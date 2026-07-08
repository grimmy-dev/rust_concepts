#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Num(f64),
    Var(String),
    BinaryOp {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryMinus(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expression),
    ExprStatement(Expression),
}
