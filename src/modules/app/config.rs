use std::{collections::HashMap, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

pub mod theme;
pub mod color;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigValue {
    Keybind(String),
    Int(i64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub values: HashMap<String, ConfigValue>,
}

impl Config {
    pub fn load(path: &PathBuf) -> io::Result<Self> {
        if path.exists() {
            let config_str = fs::read_to_string(path)?;
            let config: Config = serde_yaml::from_str(&config_str)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &PathBuf) -> io::Result<()> {
        let config_str = serde_yaml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, config_str)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: String, value: ConfigValue) {
        self.values.insert(key, value);
    }
}
