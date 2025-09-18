use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keybindings {
    pub mappings: HashMap<String, Vec<String>>,
}

impl Default for Keybindings {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert("system.quit".to_string(), vec!["q".to_string()]);
        mappings.insert("system.focus.to_main_panel".to_string(), vec!["m".to_string()]);
        mappings.insert("system.focus.to_left_panel".to_string(), vec!["l".to_string()]);
        mappings.insert("system.focus.to_sub_panel".to_string(), vec!["s".to_string()]);
        mappings.insert("system.focus.to_right_panel".to_string(), vec!["r".to_string()]);
        mappings.insert("system.focus.to_palette".to_string(), vec!["p".to_string()]);
        // Add more default keybindings as needed
        Self { mappings }
    }
}
