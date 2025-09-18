use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keybindings {
    #[serde(skip_serializing, skip_deserializing)]
    _path: Option<PathBuf>,
    pub mappings: Vec<BindSet>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct BindSet {
    pub id: String,
    pub keys: String,
}

impl BindSet {
    pub fn new(id: &str, keys: &str) -> Self {
        Self {
            id: id.to_string(),
            keys: keys.to_string(),
        }
    }
}
impl Default for Keybindings {
    fn default() -> Self {
        let mappings = vec![
            BindSet::new("system.quit", "ctrl+q"),
            BindSet::new("system.focus.mainpanel", "ctrl+e"),
            BindSet::new("system.focus.subpanel", "ctrl+j"),
            BindSet::new("system.focus.subpanel.terminal", "ctrl+`"),
            BindSet::new("system.focus.leftpanel", "ctrl+b"),
            BindSet::new("system.focus.rightpanel", "ctrl+shift+b"),
            BindSet::new("system.focus.pallete", "ctrl+p"),
            BindSet::new("system.focus.pallete.command", "ctrl+shift+p"),
            BindSet::new("focus.mainpanel.new-editor", "ctrl+n"),
        ];
        Self {
            _path: None,
            mappings,
        }
    }
}

impl Keybindings {
    pub fn load(path: &PathBuf) -> io::Result<Self> {
        if path.exists() {
            let keybindings_str = fs::read_to_string(path)?;
            let mut keybindings: Self = serde_yaml::from_str(&keybindings_str)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            keybindings._path = Some(path.clone());
            Ok(keybindings)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save_to_path(&self, path: &PathBuf) -> io::Result<()> {
        let keybindings_str = serde_yaml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, keybindings_str)?;
        Ok(())
    }
    fn parse_keybind(key: &KeyEvent) -> Result<String, ()> {
        use crossterm::event;
        let code = match key.code {
            event::KeyCode::Char(c) => c.to_string(),
            event::KeyCode::Up => "up".to_string(),
            event::KeyCode::Down => "down".to_string(),
            event::KeyCode::Left => "left".to_string(),
            event::KeyCode::Right => "right".to_string(),
            event::KeyCode::Delete => "delete".to_string(),
            event::KeyCode::Backspace => "backspace".to_string(),
            event::KeyCode::Tab => "tab".to_string(),
            event::KeyCode::BackTab => "backtab".to_string(),
            _ => return Err(()),
        };
        let mut modifiers = Vec::with_capacity(3);
        if key.modifiers.contains(event::KeyModifiers::ALT) {
            modifiers.push("alt");
        }
        if key.modifiers.contains(event::KeyModifiers::CONTROL) {
            modifiers.push("ctrl");
        }
        if key.modifiers.contains(event::KeyModifiers::SHIFT) {
            modifiers.push("shift");
        }
        let modifiers = modifiers.join("+");
        Ok(format!("{}+{}", modifiers, code))
    }
    pub fn detect_events(&self, key: &KeyEvent) -> Vec<String> {
        let mut result = Vec::new();
        for bindset in &self.mappings {
            if let Ok(bind) = &Self::parse_keybind(&key) {
                if bind == &bindset.keys {
                    result.push(bindset.id.clone());
                }
            }
        }
        result
    }
}

impl Drop for Keybindings {
    fn drop(&mut self) {
        if let Some(ref path) = self._path {
            let _ = self.save_to_path(path);
        }
    }
}
