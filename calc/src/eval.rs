use std::{collections::HashMap, fmt};

use crate::expr::{Expression, Operation, Statement};

pub type Environment = HashMap<String, f64>;

#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
    ParserError(String),
    UndefinedError(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::DivideByZero => write!(f, "Cannot divide number by 'Zero'"),
            EvalError::ParserError(msg) => write!(f, "parse error: {msg}"),
            EvalError::UndefinedError(name) => write!(f, "undefined variable: '{name}'"),
        }
    }
}

pub fn eval(expr: &Expression, env: &Environment) -> Result<f64, EvalError> {
    match expr {
        Expression::Num(n) => Ok(*n),

        Expression::Var(name) => env
            .get(name)
            .copied()
            .ok_or_else(|| EvalError::UndefinedError(name.clone())),

        Expression::UnaryMinus(inner) => {
            let value = eval(inner, env)?;
            Ok(-value)
        }

        Expression::BinaryOp { op, left, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
            match op {
                Operation::Add => Ok(l + r),
                Operation::Sub => Ok(l - r),
                Operation::Mul => Ok(l * r),
                Operation::Div => {
                    if r == 0.0 {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
    }
}

pub fn eval_stmt(stmt: &Statement, env: &mut Environment) -> Result<f64, EvalError> {
    match stmt {
        Statement::Let(name, expr) => {
            let value = eval(expr, env)?;
            env.insert(name.clone(), value);
            Ok(value)
        }
        Statement::ExprStatement(expr) => eval(expr, env),
    }
}
