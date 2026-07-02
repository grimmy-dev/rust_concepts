use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    MissingKey(String),
    ParseFailed { raw: String },
    BadSyntax { line_no: usize, line: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingKey(key) => write!(f, "Missing key {key}"),
            ConfigError::ParseFailed { raw } => {
                write!(f, "failed to parse value '{raw}'")
            }
            ConfigError::BadSyntax { line_no, line } => {
                write!(f, "bad syntax at line {line_no}: '{line}'")
            }
            ConfigError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}
