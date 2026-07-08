mod eval;
mod expr;
mod parser;
mod tokenizer;

use std::collections::HashMap;
use std::io::{self, Write};

use eval::Environment;

use crate::expr::Statement;

fn main() {
    let mut env: Environment = HashMap::new();

    println!("calc — recursive expression evaluator. `let x = ...` to assign, Ctrl-D to quit.");

    loop {
        print!("calc> ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut line)
            .expect("failed to read from stdin");

        if bytes_read == 0 {
            println!();
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        run_line(line, &mut env);
    }
}

fn run_line(line: &str, env: &mut Environment) {
    let tokens = match tokenizer::tokenize(line) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("error: {e}");
            return;
        }
    };

    let stmt = match parser::parse(tokens) {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("error: {e}");
            return;
        }
    };

    let is_let = matches!(stmt, Statement::Let(_, _));
    let name = if let Statement::Let(ref name, _) = stmt {
        Some(name.clone())
    } else {
        None
    };

    match eval::eval_stmt(&stmt, env) {
        Ok(value) => {
            if is_let {
                println!("{} = {}", name.unwrap(), value);
            } else {
                println!("{value}");
            }
        }
        Err(e) => println!("error: {e}"),
    }
}
