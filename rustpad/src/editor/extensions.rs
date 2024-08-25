use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

/// Trait that defines the basic functionality of an extension
pub trait Extension: Send + Sync {
    /// Returns a unique identifier for the extension
    fn id(&self) -> String;

    /// Returns a short description of the extension
    fn description(&self) -> String;

    /// Initialization logic for the extension (optional)
    fn initialize(&self) {
        println!("Initializing extension: {}", self.description());
    }
}

/// Represents a custom extension/plugin added by the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomExtension {
    pub id: String,
    pub description: String,
}

impl Extension for CustomExtension {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

/// Store for managing installed extensions, using `Arc<Mutex<_>>` for thread-safe shared access
pub type ExtensionStore = Arc<Mutex<HashMap<String, Arc<dyn Extension>>>>;

/// Initializes the extension store with built-in and user-defined extensions
pub fn initialize_extensions() -> ExtensionStore {
    let mut extensions: HashMap<String, Arc<dyn Extension>> = HashMap::new();

    // Example of a built-in extension
    let autocomplete_extension: Arc<dyn Extension> = Arc::new(CustomExtension {
        id: "autocomplete".to_string(),
        description: "Provides autocompletion for common programming languages.".to_string(),
    });

    // Insert the built-in extension into the store
    extensions.insert(autocomplete_extension.id(), autocomplete_extension);

    // Return the store wrapped in `Arc<Mutex<>>`
    Arc::new(Mutex::new(extensions))
}

/// Adds a custom extension to the editor
pub fn add_extension(extension_store: ExtensionStore, extension: Arc<dyn Extension>) -> Result<(), String> {
    let mut store = extension_store.lock().unwrap();

    if store.contains_key(&extension.id()) {
        Err(format!("Extension with ID '{}' already exists.", extension.id()))
    } else {
        store.insert(extension.id(), extension);
        Ok(())
    }
}

/// Removes an extension from the editor by its ID
pub fn remove_extension(extension_store: ExtensionStore, extension_id: &str) -> Result<(), String> {
    let mut store = extension_store.lock().unwrap();

    if store.remove(extension_id).is_some() {
        Ok(())
    } else {
        Err(format!("Extension with ID '{}' not found.", extension_id))
    }
}

/// Lists all installed extensions by their IDs
pub fn list_extensions(extension_store: ExtensionStore) -> Vec<String> {
    let store = extension_store.lock().unwrap();
    store.keys().cloned().collect()
}

/// Retrieves a specific extension by its ID
pub fn get_extension(extension_store: ExtensionStore, extension_id: &str) -> Option<Arc<dyn Extension>> {
    let store = extension_store.lock().unwrap();
    store.get(extension_id).cloned()
}

/// Initializes all installed extensions
pub fn initialize_all_extensions(extension_store: ExtensionStore) {
    let store = extension_store.lock().unwrap();

    for extension in store.values() {
        extension.initialize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_management() {
        let extension_store = initialize_extensions();

        // Test listing built-in extensions
        let extensions = list_extensions(extension_store.clone());
        assert!(extensions.contains(&"autocomplete".to_string()));

        // Add a new extension
        let custom_ext = Arc::new(CustomExtension {
            id: "syntax_highlight".to_string(),
            description: "Syntax highlighting for various languages.".to_string(),
        });
        assert!(add_extension(extension_store.clone(), custom_ext.clone()).is_ok());

        // Ensure the new extension was added
        let extensions = list_extensions(extension_store.clone());
        assert!(extensions.contains(&"syntax_highlight".to_string()));

        // Initialize all extensions
        initialize_all_extensions(extension_store.clone());

        // Remove the extension
        assert!(remove_extension(extension_store.clone(), "syntax_highlight").is_ok());

        // Ensure the extension was removed
        let extensions = list_extensions(extension_store);
        assert!(!extensions.contains(&"syntax_highlight".to_string()));
    }
}
