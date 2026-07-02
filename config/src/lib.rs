pub mod error;
pub mod raw_config;
pub mod value;

pub use error::ConfigError;
pub use raw_config::RawConfig;
pub use value::FromConfigValue;

/// Implemented by user-defined structs that want to be built directly
/// from a [`RawConfig`], pulling out and typing each field themselves.
pub trait FromConfig: Sized {
    fn from_config(raw: &RawConfig) -> Result<Self, ConfigError>;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn parse_config() {
        let path = PathBuf::from("./example.txt");
        match RawConfig::from_file(&path) {
            Ok(config) => {
                println!("{config:?}")
            }
            Err(e) => {
                eprintln!("Failed to parse the file -> {e}")
            }
        }
    }
}
