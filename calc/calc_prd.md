# calc

a recursive expression evaluator with a REPL. parses and evaluates math
expressions with +, -, *, /, parentheses, unary minus, and variables
(let x = 5). the AST is an enum; evaluation is one recursive match.
concept: enums, recursive data, exhaustive pattern matching. binary crate.

`cargo new calc`

## required Behaviour

read a line, tokenize it, parse into an Expr tree, evaluate to a number.
loop (REPL). the expression grammar supports:
  - numbers (floats):        3, 3.5, .5
  - binary ops:              + - * /  with correct precedence
  - parentheses:             (1 + 2) * 3
  - unary minus:             -5, -(2 + 3)
  - variables:               x, foo   (looked up in an environment)
  - assignment:              let x = 2 + 3   (stores x, prints nothing or the value)

precedence: * / bind tighter than + -. parens override. unary minus
binds tightest.

## the core types (this is the lesson)

    enum Token { Num(f64), Plus, Minus, Star, Slash, LParen, RParen, Ident(String), Let, Equals }

    enum Expr {
        Num(f64),
        Var(String),
        BinaryOp { op: Op, left: Box<Expr>, right: Box<Expr> },
        UnaryMinus(Box<Expr>),
    }

    enum Op { Add, Sub, Mul, Div }

note the Box<Expr> — a tree needs indirection or it'd be infinitely sized.
that's the ownership lesson from arena resurfacing: an enum can't contain
itself by value.

## the three stages

1. tokenizer: &str -> Vec<Token>. a match over characters.
2. parser: Vec<Token> -> Expr. recursive descent — parse_expr calls
   parse_term calls parse_factor. precedence falls out of the call order.
3. evaluator: &Expr -> Result<f64, EvalError>. ONE recursive match over
   the Expr enum. this is the money function and it should be tiny.

## exhaustiveness is the point

eval is a match with one arm per Expr variant. don't use a catch-all `_`.
when you add a variant later (say, a Power op), the compiler must force
you to handle it everywhere. a `_` arm defeats that — it's the enum
equivalent of unwrap. banned.

## errors (real Result, three+ variants)

    enum EvalError {
        DivByZero,
        UndefinedVar(String),
        ParseError(String),      // unexpected token, unclosed paren, etc.
    }

division by zero is a runtime error, not a panic. an undefined variable
is an error, not a default of 0. an unclosed paren is a parse error with
a message.

## constraints

- no `_` catch-all in the eval match — handle every variant explicitly
- no unwrap in the library paths (REPL top-level may expect once on stdin)
- no parser-generator crates (nom, pest, lalrpop) — hand-roll it, that's
  the lesson. clap is fine for the binary shell if you want it.
- floats only (f64), don't do integer/float type juggling — out of scope

## concept exercises

1. eval as a single recursive match, no catch-all. adding a variant
   breaks compilation everywhere until handled. FEEL that — add Op::Mod
   at the end and watch the errors guide you.
2. Box<Expr> for the recursive tree. understand why it won't compile
   without the Box (infinite size).
3. precedence via recursive descent: parse_expr (+/-) -> parse_term (*//)
   -> parse_factor (numbers, parens, unary). precedence is just call order.
4. variables + environment: a HashMap<String, f64>. `let x = ...` inserts,
   a bare `x` looks up (UndefinedVar if missing).
5. stretch: add functions — sqrt(x), abs(x). now Expr has a Call variant
   and the exhaustive match makes you handle it. this is where enums shine.

## checklist

- [x] tokenizer: &str -> Vec<Token> via char match
- [x] parser: recursive descent, correct precedence, parens, unary minus
- [x] Expr is a recursive enum with Box; I know why Box is required
- [x] eval is ONE recursive match, no `_` catch-all
- [x] div-by-zero, undefined-var, parse-error are distinct EvalError variants
- [x] variables work: let x = 5, then x + 1 == 6
- [x] REPL loops, prints results, errors don't crash the loop
- [x] out loud: how does recursive-descent call order create precedence?
