use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::io;

type ExtensionCallback = Box<dyn Fn(&str) -> io::Result<()> + Send + Sync>;

/// Represents an extension that can be added to the editor.
#[derive(Debug, Clone)]
pub struct Extension {
    pub name: String,
    pub description: String,
    pub on_load: Option<ExtensionCallback>,  // Optional function to call when extension is loaded
    pub on_save: Option<ExtensionCallback>,  // Optional function to call when the file is saved
    pub on_change: Option<ExtensionCallback>, // Optional function to call when the content changes
}

/// Manages editor extensions.
pub struct ExtensionManager {
    extensions: HashMap<String, Arc<Mutex<Extension>>>,  // Map from extension name to the extension itself
}

impl ExtensionManager {
    /// Creates a new ExtensionManager.
    pub fn new() -> Self {
        ExtensionManager {
            extensions: HashMap::new(),
        }
    }

    /// Registers a new extension by its name.
    pub fn register_extension(&mut self, extension: Extension) {
        self.extensions.insert(extension.name.clone(), Arc::new(Mutex::new(extension)));
    }

    /// Loads an extension by invoking its `on_load` callback.
    pub fn load_extension(&self, name: &str) -> io::Result<()> {
        if let Some(extension) = self.extensions.get(name) {
            if let Some(on_load) = &extension.lock().unwrap().on_load {
                on_load(name)
            } else {
                Ok(())  // No load behavior, return Ok.
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Extension not found"))
        }
    }

    /// Calls the `on_save` callback for all extensions when a file is saved.
    pub fn on_file_save(&self, file_path: &str) -> io::Result<()> {
        for extension in self.extensions.values() {
            if let Some(on_save) = &extension.lock().unwrap().on_save {
                on_save(file_path)?;
            }
        }
        Ok(())
    }

    /// Calls the `on_change` callback for all extensions when content changes.
    pub fn on_content_change(&self, new_content: &str) -> io::Result<()> {
        for extension in self.extensions.values() {
            if let Some(on_change) = &extension.lock().unwrap().on_change {
                on_change(new_content)?;
            }
        }
        Ok(())
    }

    /// Returns a list of all registered extensions.
    pub fn list_extensions(&self) -> Vec<String> {
        self.extensions.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_register_and_load_extension() {
        let mut manager = ExtensionManager::new();

        let load_callback: ExtensionCallback = Box::new(|name: &str| {
            println!("Extension {} loaded!", name);
            Ok(())
        });

        let extension = Extension {
            name: "TestExtension".to_string(),
            description: "A test extension".to_string(),
            on_load: Some(load_callback),
            on_save: None,
            on_change: None,
        };

        manager.register_extension(extension);
        let result = manager.load_extension("TestExtension");
        assert!(result.is_ok());
    }

    #[test]
    fn test_on_file_save_callback() {
        let mut manager = ExtensionManager::new();

        let save_callback: ExtensionCallback = Box::new(|file_path: &str| {
            println!("File {} saved!", file_path);
            Ok(())
        });

        let extension = Extension {
            name: "SaveExtension".to_string(),
            description: "An extension that hooks into file save".to_string(),
            on_load: None,
            on_save: Some(save_callback),
            on_change: None,
        };

        manager.register_extension(extension);
        let result = manager.on_file_save("example.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_on_content_change_callback() {
        let mut manager = ExtensionManager::new();

        let change_callback: ExtensionCallback = Box::new(|content: &str| {
            println!("Content changed to: {}", content);
            Ok(())
        });

        let extension = Extension {
            name: "ChangeExtension".to_string(),
            description: "An extension that hooks into content change".to_string(),
            on_load: None,
            on_save: None,
            on_change: Some(change_callback),
        };

        manager.register_extension(extension);
        let result = manager.on_content_change("New content");
        assert!(result.is_ok());
    }
}
