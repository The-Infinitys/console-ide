use console_ide_feature::extension::Extension;
use console_ide_feature::resolver::{ExtensionId, ResolutionError, Resolver};
use instance_pipe::{Client as Pipe, Event as PipeEvent};
use serde::{Deserialize, Serialize};
use std::process::Child;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

// Define a message protocol for host-extension communication
#[derive(Debug, Serialize, Deserialize)]
pub enum HostMessage {
    SpawnWidget { widget_type: String, data: String },
    // Add other host-to-extension commands here
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtensionMessage {
    WidgetSpawned { widget_id: String },
    // Add other extension-to-host events here
}

#[derive(Default)]
pub struct ExtensionManager {
    pub resolved_extensions: HashMap<ExtensionId, Extension>,
    active_pipes: HashMap<ExtensionId, Pipe>,
    process: Option<Child>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        ExtensionManager {
            resolved_extensions: HashMap::new(),
            active_pipes: HashMap::new(),
            process: None,
        }
    }

    pub fn load_and_resolve_extensions(&mut self) -> Result<(), Vec<ResolutionError>> {
        let extensions_dir = get_extensions_dir();
        if !extensions_dir.exists() {
            return Ok(()); // No extensions directory, nothing to load
        }

        let mut available_extensions = HashMap::new();
        let mut resolver = Resolver::new();

        for entry in fs::read_dir(&extensions_dir)
            .map_err(|e| vec![ResolutionError::NotFound(ExtensionId::from(e.to_string()))])?
        {
            let entry = entry
                .map_err(|e| vec![ResolutionError::NotFound(ExtensionId::from(e.to_string()))])?;
            let path = entry.path();

            if path.is_file() {
                // Assume each file in the extensions directory is an executable extension
                match self.get_extension_info(&path) {
                    Ok(extension) => {
                        let ext_id = ExtensionId::from(extension.id.clone());
                        resolver.add_extension(extension.clone());
                        available_extensions.insert(ext_id, extension);
                    }
                    Err(e) => {
                        eprintln!("Failed to get info for extension {:?}: {}", path, e);
                    }
                }
            }
        }

        let root_extension_ids: Vec<ExtensionId> = available_extensions.keys().cloned().collect();
        match resolver.resolve(&root_extension_ids) {
            Ok(resolved) => {
                self.resolved_extensions = resolved
                    .into_iter()
                    .map(|(id, res_ext)| (id, res_ext.extension))
                    .collect();
                Ok(())
            }
            Err(errors) => Err(errors),
        }
    }

    fn get_extension_info(&self, path: &PathBuf) -> Result<Extension, Box<dyn std::error::Error>> {
        let output = Command::new(path)
            .arg("info")
            .arg("relation")
            .stdout(Stdio::piped())
            .spawn()? // Use spawn to avoid deadlock with large output
            .wait_with_output()?;

        if output.status.success() {
            let json_output = String::from_utf8(output.stdout)?; // Use from_utf8 for stdout
            let extension: Extension = serde_json::from_str(&json_output)?;
            Ok(extension)
        } else {
            Err(format!("Extension info command failed: {:?}", output).into())
        }
    }

    pub fn activate_extension(
        &mut self,
        extension_id: &ExtensionId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let extension = self
            .resolved_extensions
            .get(extension_id)
            .ok_or_else(|| format!("Extension {} not found or not resolved.", extension_id))?;

        let extension_path = get_extensions_dir().join(&extension.id); // Assuming binary name is extension ID
        if !extension_path.exists() {
            return Err(format!("Extension binary not found at {:?}", extension_path).into());
        }

        let pipe_name = format!("console-ide-pipe-{}", extension_id);
        let child = Command::new(&extension_path)
            .arg("exec")
            .arg("background-process")
            .env("INSTANCE_PIPE_NAME", &pipe_name) // Pass pipe name via environment variable
            .spawn()?;

        let pipe = Pipe::start(&pipe_name)?; // Host creates the pipe
        self.active_pipes.insert(extension_id.clone(), pipe);
        self.process = Some(child);
        // Example: Send a message to the extension
        // self.send_message(extension_id, HostMessage::SpawnWidget { widget_type: "terminal".to_string(), data: "".to_string() })?;

        Ok(())
    }

    pub fn send_message(
        &mut self,
        extension_id: &ExtensionId,
        message: HostMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(pipe) = self.active_pipes.get_mut(extension_id) {
            let serialized_message = serde_json::to_string(&message)?;
            pipe.send(&serialized_message)?; // Send serialized message
            Ok(())
        } else {
            Err(format!("Pipe for extension {} not active.", extension_id).into())
        }
    }

    pub fn receive_message(
        &mut self,
        extension_id: &ExtensionId,
    ) -> Result<Option<ExtensionMessage>, Box<dyn std::error::Error>> {
        if let Some(pipe) = self.active_pipes.get_mut(extension_id) {
            match pipe.poll_event() {
                Ok(Some(PipeEvent::MessageReceived(message))) => Ok(Some(message)),
                Ok(Some(_)) => Ok(None), // Ignore other events for now
                Ok(None) => Ok(None),    // No message received within timeout
                Err(e) => Err(e.into()),
            }
        } else {
            Err(format!("Pipe for extension {} not active.", extension_id).into())
        }
    }
}

fn get_extensions_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
    PathBuf::from(home).join(".console-ide").join("extensions")
}
