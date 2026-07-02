# config

a library that loads a nested config file into a user's typed struct.
string values parse into real types (u16, bool, PathBuf, Vec<T>) through
a custom conversion trait. errors distinguish three states: present-and-good,
present-but-broken, and absent. concept: traits, generics, trait bounds,
fallible conversion. library crate.

`cargo new config --lib`

## required Behaviour

read a file with [section] headers and key = value lines. values are
reachable by dotted key (server.port). converting a raw string into a
typed value goes through OUR OWN trait, not std's FromStr. parsing that
can fail returns Result, never panics. three outcomes must be
distinguishable by the caller:
  - key present, parses fine        -> Ok(value)
  - key present, value is garbage   -> Err(ParseFailed)
  - key absent                      -> depends on which getter (see API)

## file format

    # comments start with hash, skipped
    # blank lines skipped

    [server]
    port = 8080
    host = localhost

    [server.tls]
    enabled = true
    ciphers = aes256, chacha20      # comma list -> Vec

dotted keys: server.port, server.tls.enabled, server.tls.ciphers

## the conversion trait (this is the core lesson)

    pub trait FromConfigValue: Sized {
        fn parse(raw: &str) -> Result<Self, ConfigError>;
    }

implement it for: u16, bool, String, PathBuf, and Vec<T>.

the Vec<T> impl is the hard one and the whole point:

    impl<T: FromConfigValue> FromConfigValue for Vec<T> { ... }

parsing a Vec means splitting on commas and parsing EACH element via
T's own parse. generics calling generics. the T: FromConfigValue bound
is what makes it work — understand why it won't compile without it.

## API

- `RawConfig::from_file(path) -> Result<RawConfig, ConfigError>`
- `get<T: FromConfigValue>(&self, key: &str) -> Result<T, ConfigError>`
      key absent -> Err(MissingKey)
- `get_opt<T: FromConfigValue>(&self, key: &str) -> Result<Option<T>, ConfigError>`
      key absent -> Ok(None)
      key present but garbage -> Err(ParseFailed)   <- absence is NOT failure
- `get_or<T: FromConfigValue>(&self, key: &str, default: T) -> Result<T, ConfigError>`
      key absent -> Ok(default); present-but-garbage still Err
- a `FromConfig` trait the user implements on their own struct:
      `fn from_config(raw: &RawConfig) -> Result<Self, ConfigError>`
- `ConfigError` enum, at minimum: MissingKey, ParseFailed, Io, BadSyntax

## constraints

- no unwrap in the library
- no serde, no toml crate — hand-roll it, or the lesson happens for you
- no nesting beyond [section] and [section.sub] headers — no TOML tables,
  no arrays-of-tables, no multiline values. that's the scope wall.
- dotted keys only go as deep as the section headers you wrote

## concept exercises

1. get<T> generic over FromConfigValue (our trait), not FromStr. one
   function parses any type that knows how to parse itself.
2. impl FromConfigValue for Vec<T> where T: FromConfigValue. parsing a
   list delegates to the element type. generics composing.
3. three-state errors: get -> MissingKey on absent; get_opt -> Ok(None)
   on absent but Err on garbage. absence and failure are different things.
4. implement FromConfig for a sample Settings struct in tests. be the
   trait's user, feel the ergonomics.
5. stretch: also impl std's TryFrom<&RawConfig> for Settings. compare to
   your own FromConfig trait. why is implementing std's conversion trait
   better for interop?

## checklist

- [ ] parses [section] headers, key=value, skips # comments and blanks
- [ ] dotted keys resolve into nested sections
- [ ] FromConfigValue implemented for u16, bool, String, PathBuf
- [ ] Vec<T> impl delegates to T's parse; I know why the bound is required
- [ ] get -> MissingKey on absent
- [ ] get_opt -> Ok(None) on absent, Err on garbage
- [ ] get_or -> default on absent, Err on garbage
- [ ] FromConfig implemented for a real struct in tests
- [ ] no unwrap, no serde, no toml crate
- [ ] out loud: why does impl<T: FromConfigValue> for Vec<T> need the bound,
      and what breaks without it?