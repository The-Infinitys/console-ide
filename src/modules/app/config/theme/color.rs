use ratatui::style::Color;
use serde::{Deserialize, Deserializer, Serializer};

// Helper function to serialize Color to a string (HEX or named)
pub fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let color_str = match color {
        Color::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
        Color::Black => "Black".to_string(),
        Color::Red => "Red".to_string(),
        Color::Green => "Green".to_string(),
        Color::Yellow => "Yellow".to_string(),
        Color::Blue => "Blue".to_string(),
        Color::Magenta => "Magenta".to_string(),
        Color::Cyan => "Cyan".to_string(),
        Color::White => "White".to_string(),
        Color::Gray => "Gray".to_string(),
        Color::DarkGray => "DarkGray".to_string(),
        Color::LightRed => "LightRed".to_string(),
        Color::LightGreen => "LightGreen".to_string(),
        Color::LightYellow => "LightYellow".to_string(),
        Color::LightBlue => "LightBlue".to_string(),
        Color::LightMagenta => "LightMagenta".to_string(),
        Color::LightCyan => "LightCyan".to_string(),
        Color::Reset => "Transparent".to_string(), // Map Reset to Transparent
        _ => "Black".to_string(),                  // Default to black for unsupported colors
    };
    serializer.serialize_str(&color_str)
}

// Helper function to deserialize Color from a string (HEX or named)
pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.starts_with("#") && s.len() == 7 {
        let r = u8::from_str_radix(&s[1..3], 16).map_err(serde::de::Error::custom)?;
        let g = u8::from_str_radix(&s[3..5], 16).map_err(serde::de::Error::custom)?;
        let b = u8::from_str_radix(&s[5..7], 16).map_err(serde::de::Error::custom)?;
        Ok(Color::Rgb(r, g, b))
    } else {
        match s.as_str() {
            "Black" => Ok(Color::Black),
            "Red" => Ok(Color::Red),
            "Green" => Ok(Color::Green),
            "Yellow" => Ok(Color::Yellow),
            "Blue" => Ok(Color::Blue),
            "Magenta" => Ok(Color::Magenta),
            "Cyan" => Ok(Color::Cyan),
            "White" => Ok(Color::White),
            "Gray" => Ok(Color::Gray),
            "DarkGray" => Ok(Color::DarkGray),
            "LightRed" => Ok(Color::LightRed),
            "LightGreen" => Ok(Color::LightGreen),
            "LightYellow" => Ok(Color::LightYellow),
            "LightBlue" => Ok(Color::LightBlue),
            "LightMagenta" => Ok(Color::LightMagenta),
            "LightCyan" => Ok(Color::LightCyan),
            "Transparent" => Ok(Color::Reset), // Map Transparent to Reset
            _ => Err(serde::de::Error::custom(format!("Unknown color: {}", s))),
        }
    }
}
