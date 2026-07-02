use config::{RawConfig, error::ConfigError};
use std::path::PathBuf;

fn example_path() -> PathBuf {
    PathBuf::from("./example.txt")
}

#[test]
fn get_present_key_parses_correctly() {
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let port: u16 = cfg.get("server.port").unwrap();
    assert_eq!(port, 8080);
}

#[test]
fn get_missing_key_is_err() {
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let result: Result<u16, ConfigError> = cfg.get("server.nonexistent");
    assert!(matches!(result, Err(ConfigError::MissingKey(_))));
}

#[test]
fn get_opt_missing_key_is_ok_none() {
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let result: Result<Option<u16>, ConfigError> = cfg.get_opt("server.nonexistent");
    assert!(matches!(result, Ok(None)));
}

#[test]
fn get_opt_present_garbage_is_err() {
    // requires a key in example.txt with a value that fails u16 parsing,
    // e.g. server.host = localhost, tested against u16
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let result: Result<Option<u16>, ConfigError> = cfg.get_opt("server.host");
    assert!(matches!(result, Err(ConfigError::ParseFailed { .. })));
}

#[test]
fn get_or_missing_key_returns_default() {
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let val: u16 = cfg.get_or("server.nonexistent", 9999).unwrap();
    assert_eq!(val, 9999);
}

#[test]
fn get_or_present_garbage_is_still_err() {
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let result: Result<u16, ConfigError> = cfg.get_or("server.host", 9999);
    assert!(result.is_err());
}

#[test]
fn vec_field_parses_from_comma_list() {
    let cfg = RawConfig::from_file(&example_path()).unwrap();
    let ciphers: Vec<String> = cfg.get("server.tls.ciphers").unwrap();
    assert_eq!(ciphers, vec!["aes256".to_string(), "chacha20".to_string()]);
}