use std::{collections::HashMap, fs, path::PathBuf};

use crate::{error::ConfigError, value::FromConfigValue};

#[derive(Debug)]
pub struct RawConfig {
    store: HashMap<String, String>,
}

impl RawConfig {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn parse_content(&mut self, path: &PathBuf) -> Result<(), ConfigError> {
        let content = Self::read_file(path)?;
        let mut current_section = String::new();

        for (i, raw_line) in content.lines().enumerate() {
            // check content before c_comment
            let line = match raw_line.split_once('#') {
                Some((before_comment, _)) => before_comment,
                None => raw_line,
            };

            // trim empty spaces
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].trim().to_string();
            } else if let Some((key, val)) = line.rsplit_once('=') {
                let key = key.trim();
                let val = val.trim().to_string();
                // creates the dotted pattern of key
                let complete_key = if current_section.is_empty() {
                    key.to_string()
                } else {
                    format!("{current_section}.{key}")
                };
                self.store.insert(complete_key, val);
            } else {
                return Err(ConfigError::BadSyntax {
                    line_no: i + 1,
                    line: raw_line.to_string(),
                });
            }
        }
        Ok(())
    }

    fn read_file(path: &PathBuf) -> Result<String, ConfigError> {
        Ok(fs::read_to_string(path)?)
    }

    pub fn get<T: FromConfigValue>(&self, key: &str) -> Result<T, ConfigError> {
        let raw = self
            .store
            .get(key)
            .ok_or_else(|| ConfigError::MissingKey(key.to_string()))?;

        T::parse(raw).map_err(|e| match e {
            ConfigError::ParseFailed { raw } => ConfigError::ParseFailed { raw },
            other => other,
        })
    }

    pub fn get_opt<T: FromConfigValue>(&self, key: &str) -> Result<Option<T>, ConfigError> {
        self.store
            .get(key)
            .map(|raw| {
                T::parse(raw).map_err(|e| match e {
                    ConfigError::ParseFailed { raw } => ConfigError::ParseFailed { raw },
                    other => other,
                })
            })
            .transpose()
    }

    pub fn get_or<T: FromConfigValue>(&self, key: &str, default: T) -> Result<T, ConfigError> {
        Ok(self.get_opt::<T>(key)?.unwrap_or(default))
    }

    pub fn from_file(path: &PathBuf) -> Result<RawConfig, ConfigError> {
        let mut config = RawConfig::new();
        config.parse_content(path)?;
        Ok(config)
    }
}
