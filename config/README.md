# config

A small Rust library that reads a simple, `.ini`-style config file and hands you
back typed values - `u16`, `bool`, `String`, `PathBuf`, even `Vec<T>` - instead of
raw strings, without pulling in `serde` or a TOML parser.

## Why hand-roll this instead of using `toml` + `serde`?

Because the point of this project was to actually learn how Rust's trait system
does type conversion under the hood - not to use the version someone else already
built. `serde` and `toml` solve this problem more completely and more safely for
real production use. This crate is deliberately narrower in scope (one level of
`[section.sub]` nesting, comma lists, no arrays-of-tables, no multiline values) in
exchange for being small enough to fully understand end to end.

If you're building something real, use `toml` + `serde`. If you want to see how
generic trait-bound conversion works from the inside, keep reading.

## File format

```ini
# comments start with a hash and are skipped
# blank lines are skipped too

[server]
port = 8080
host = localhost

[server.tls]
enabled = true
ciphers = aes256, chacha20      # trailing comments are stripped too
```

Section headers (`[server]`, `[server.tls]`) combine with the keys beneath them
into **dotted keys**: `server.port`, `server.tls.enabled`, `server.tls.ciphers`.
Nesting only goes as deep as the headers you actually write - there's no implicit
tree structure beyond that.

## Quick start

```rust
use config::RawConfig;

let cfg = RawConfig::from_file(&"example.txt".into())?;

let port: u16 = cfg.get("server.port")?;
let tls_enabled: bool = cfg.get("server.tls.enabled")?;
let ciphers: Vec<String> = cfg.get("server.tls.ciphers")?;
```

## The `FromConfigValue` trait

Every type this library can produce implements a small trait defined by this
crate - not `std::str::FromStr`:

```rust
pub trait FromConfigValue: Sized {
    fn parse(raw: &str) -> Result<Self, ConfigError>;
}
```

It's implemented for `u16`, `bool`, `String`, `PathBuf`, and - the interesting
one - `Vec<T>` for any `T` that itself implements `FromConfigValue`. Parsing a
list means splitting the raw string on commas and asking `T::parse` to handle
each piece; the `T: FromConfigValue` bound is what lets the compiler know `T`
even has a `parse` function to call at all.

Want to support a new type? Implement `FromConfigValue` for it in your own crate
and `get::<YourType>(...)` works immediately - no changes to this library needed.

## Error handling - three states, not two

The core design decision in this library: **a key being absent is not the same
kind of failure as a key being present but unparsable**, and different getters
expose that distinction differently:

| Method | Key absent | Key present, garbage value |
|---|---|---|
| `get::<T>(key)` | `Err(MissingKey)` | `Err(ParseFailed)` |
| `get_opt::<T>(key)` | `Ok(None)` | `Err(ParseFailed)` |
| `get_or::<T>(key, default)` | `Ok(default)` | `Err(ParseFailed)` |

Notice `get_opt` and `get_or` treat *absence* as a non-error, but a *garbage
value* is always an error, in every getter, without exception. Absence means
"you didn't tell me"; garbage means "you told me something wrong" - those are
different problems and callers should be able to tell them apart.

`ConfigError` also has an `Io` variant (file couldn't be read) and `BadSyntax`
(a line in the file matched neither `[section]` nor `key = value`).

## Building your own structs - `FromConfig`

```rust
use config::{RawConfig, FromConfig, ConfigError};

struct Settings {
    port: u16,
    tls_enabled: bool,
    ciphers: Vec<String>,
}

impl FromConfig for Settings {
    fn from_config(raw: &RawConfig) -> Result<Self, ConfigError> {
        Ok(Settings {
            port: raw.get("server.port")?,
            tls_enabled: raw.get("server.tls.enabled")?,
            ciphers: raw.get_or("server.tls.ciphers", Vec::new())?,
        })
    }
}
```

## Non-goals

- No `serde`, no `toml` crate - parsing is entirely hand-rolled.
- No TOML tables, no arrays-of-tables, no multiline values.
- Nesting stops at `[section.sub]` - no deeper structure than the headers you write.
- No `unwrap()` anywhere in library code; every failure path returns `Result`.