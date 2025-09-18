use crate::modules::app::config::theme::Theme;
use std::env;
use std::{collections::HashMap, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

pub mod theme;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigValue {
    Keybind(String),
    Int(i64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub values: HashMap<String, ConfigValue>,
    #[serde(skip)]
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            theme: Theme::default(),
        }
    }
}

impl Config {
    pub fn load(config_path: &PathBuf) -> io::Result<Self> {
        let mut config = if config_path.exists() {
            let config_str = fs::read_to_string(config_path)?;
            serde_yaml::from_str(&config_str)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        } else {
            Self::default()
        };
        // themeの読み込み
        let theme_path = Self::get_theme_path();
        config.theme = Theme::load(&theme_path).unwrap_or_else(|_| Theme::default());
        Ok(config)
    }

    pub fn save(&self) -> io::Result<()> {
        let config_str = serde_yaml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if let Some(parent) = Self::get_config_path().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(Self::get_config_path(), config_str)?;
        // themeの保存
        self.theme.save_to_path(&Self::get_theme_path())?;
        Ok(())
    }
    fn get_config_path() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
        PathBuf::from(home)
            .join(".console-ide")
            .join("config")
            .join("config.yaml")
    }
    /// $HOME/.console-ide/config/theme.yaml のパス取得
    fn get_theme_path() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
        PathBuf::from(home)
            .join(".console-ide")
            .join("config")
            .join("theme.yaml")
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: String, value: ConfigValue) {
        self.values.insert(key, value);
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        let _ = self.save();
    }
}
