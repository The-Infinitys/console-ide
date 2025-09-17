use serde::{Deserialize, Serialize};
use ratatui::style::Color;
use super::color::{serialize_color, deserialize_color};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub background: Color,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub foreground: Color,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub primary: Color,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub secondary: Color,
    // Add more colors as needed
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            primary: Color::LightBlue,
            secondary: Color::Gray,
        }
    }
}