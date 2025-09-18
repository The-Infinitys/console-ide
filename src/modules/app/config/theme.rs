use color::{deserialize_color, serialize_color};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
mod color;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    #[serde(skip_serializing, skip_deserializing)]
    _path: Option<PathBuf>,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub background: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub foreground: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub primary: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub secondary: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub accent1: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub accent2: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub accent3: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub accent4: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub accent5: Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub accent6: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            _path: None,
            background: Color::Black,
            foreground: Color::White,
            primary: Color::LightBlue,
            secondary: Color::Gray,
            accent1: Color::Red,
            accent2: Color::Yellow,
            accent3: Color::Green,
            accent4: Color::Cyan,
            accent5: Color::Blue,
            accent6: Color::Magenta,
        }
    }
}
impl Theme {
    pub fn load(path: &PathBuf) -> io::Result<Self> {
        if path.exists() {
            let theme_str = fs::read_to_string(path)?;
            let mut theme: Self = serde_yaml::from_str(&theme_str)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            theme._path = Some(path.clone());
            Ok(theme)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save_to_path(&self, path: &PathBuf) -> io::Result<()> {
        let theme_str = serde_yaml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, theme_str)?;
        Ok(())
    }
}

impl TryFrom<&PathBuf> for Theme {
    type Error = io::Error;
    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Self::load(value)
    }
}
impl Drop for Theme {
    fn drop(&mut self) {
        if let Some(ref path) = self._path {
            let _ = self.save_to_path(path);
        }
    }
}
