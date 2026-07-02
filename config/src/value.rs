use std::path::PathBuf;

use crate::error::ConfigError;

pub trait FromConfigValue: Sized {
    fn parse(raw: &str) -> Result<Self, ConfigError>;
}

impl FromConfigValue for bool {
    fn parse(raw: &str) -> Result<Self, ConfigError> {
        match raw.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            other => Err(ConfigError::ParseFailed {
                raw: other.to_string(),
            }),
        }
    }
}

impl FromConfigValue for u16 {
    fn parse(raw: &str) -> Result<Self, ConfigError> {
        raw.trim()
            .parse::<u16>()
            .map_err(|_| ConfigError::ParseFailed {
                raw: raw.to_string(),
            })
    }
}

impl FromConfigValue for String {
    fn parse(raw: &str) -> Result<Self, ConfigError> {
        Ok(raw.trim().to_string())
    }
}

impl FromConfigValue for PathBuf {
    fn parse(raw: &str) -> Result<Self, ConfigError> {
        Ok(PathBuf::from(raw.trim()))
    }
}

impl<T: FromConfigValue> FromConfigValue for Vec<T> {
    fn parse(raw: &str) -> Result<Self, ConfigError> {
        if raw.trim().is_empty() {
            return Ok(Vec::new());
        }
        raw.split(',').map(|s| T::parse(s.trim())).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_parses() {
        assert!(matches!(bool::parse("true"), Ok(true)));
        assert!(matches!(bool::parse("false"), Ok(false)));
        assert!(bool::parse("claude").is_err());
    }

    #[test]
    fn test_vec_parsing_u16() {
        let nums = Vec::<u16>::parse("10, 20, 30");
        assert!(matches!(nums, Ok(v) if v == vec![10u16, 20, 30]));
    }

    #[test]
    fn test_vec_parsing_string() {
        let strs = Vec::<String>::parse("apple, orange");
        assert!(matches!(strs, Ok(ref v) if v == &vec!["apple".to_string(), "orange".to_string()]));
    }

    #[test]
    fn test_vec_rejects_bad_element() {
        let nums = Vec::<u16>::parse("10, banana, 30");
        assert!(nums.is_err());
    }
}
