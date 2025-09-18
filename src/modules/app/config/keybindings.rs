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
            BindSet::new("system.focus.mainpanel", "ctrl+n"),
            BindSet::new("system.focus.subpanel", "ctrl+j"),
            BindSet::new("system.focus.leftpanel", "ctrl+b"),
            BindSet::new("system.focus.rightpanel", "ctrl+shift+b"),
            BindSet::new("system.focus.pallete", "ctrl+p"),
            BindSet::new("system.focus.pallete.command", "ctrl+shift+p"),
        ];
        Self { _path: None, mappings }
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
}

impl Drop for Keybindings {
    fn drop(&mut self) {
        if let Some(ref path) = self._path {
            let _ = self.save_to_path(path);
        }
    }
}
