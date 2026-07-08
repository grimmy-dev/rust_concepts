# calc

A recursive-descent expression evaluator with a REPL, written to learn
enums, recursive data (`Box`), and exhaustive pattern matching in Rust.

## Build & run

```bash
cargo run
```

You'll get a `calc>` prompt. Type expressions, hit enter, see results.
Ctrl-D exits.

## Grammar

```
stmt   := "let" IDENT "=" expr | expr
expr   := term (("+" | "-") term)*
term   := factor (("*" | "/") factor)*
factor := "-" factor | NUMBER | IDENT | "(" expr ")"
```

Precedence falls out of the call order, not a table:
`parse_expr` calls `parse_term` calls `parse_factor`. Each level only
ever sees *whole* results from the level below it, so `*`/`/` naturally
bind tighter than `+`/`-`, and unary minus — handled directly inside
`parse_factor` — binds tightest of all.

## Examples

```
calc> 1 + 2 * 3
7
calc> (1 + 2) * 3
9
calc> -(2 + 3) * 2
-10
calc> let x = 5
x = 5
calc> x + 1
6
calc> x / 0
error: division by zero
calc> y
error: undefined variable 'y'
calc> (1 + 2
error: parse error: unexpected end of input
```

## Project layout

| File | Stage | Responsibility |
|---|---|---|
| `src/expr.rs` | — | `Op`, `Expr`, `Stmt` — the AST. `Expr` is recursive via `Box`, since an enum can't contain itself by value (unknown size). |
| `src/token.rs` | 1 | `tokenize`: `&str -> Vec<Token>`, one pass over chars. |
| `src/parser.rs` | 2 | `parse`: `Vec<Token> -> Stmt`, recursive descent (`parse_expr` → `parse_term` → `parse_factor`). |
| `src/eval.rs` | 3 | `eval`: `&Expr -> Result<f64, EvalError>`. One recursive match, no `_` catch-all — every `Expr` variant handled explicitly, on purpose. |
| `src/main.rs` | 4 | The REPL loop: read a line, tokenize, parse, evaluate, print, repeat. Bad input costs a line, never crashes the session. |

## Errors

`EvalError` has three variants, all real, none a panic:

- `DivByZero` — `x / 0`
- `UndefinedVar(String)` — referencing a variable that was never `let`-bound
- `ParseError(String)` — bad syntax: unexpected token, unclosed paren, etc.

## Design notes

- **Why `Box<Expr>`**: an enum's size is fixed at compile time to fit its
  largest variant. If `BinaryOp` held `Expr` directly instead of
  `Box<Expr>`, the compiler would need `Expr`'s size to compute `Expr`'s
  size — unsolvable. `Box` is a fixed-size pointer; the actual recursion
  happens on the heap at runtime instead of in the type's layout.
- **Why no `_` in `eval`**: adding a new `Expr` variant (e.g. `Call` for
  functions, or a `Power` op inside `Op`) breaks compilation at every
  match site that touches it, forcing you to handle the new case
  everywhere instead of silently falling through a catch-all.
- **`let` is a `Stmt`, not an `Expr`**: assignment is an action, not a
  value, so it's kept out of the enum that represents values.

## Stretch exercises

- [ ] Add `Op::Mod` (`%`). Watch the compiler point at every match arm
  that needs updating.
- [ ] Add functions: `sqrt(x)`, `abs(x)`. Needs a new `Expr::Call`
  variant — parsing an identifier followed by `(` becomes a function
  call instead of just a variable lookup.
