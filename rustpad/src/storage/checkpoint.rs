use crate::storage::types::{Document, DocumentId};
use std::collections::HashMap;
use std::error::Error;

/// `Checkpoint` stores key snapshots of the document that the user can revert to.
/// This is different from history, which tracks every incremental change.
pub struct CheckpointManager {
    checkpoints: HashMap<String, Document>, // Maps checkpoint names to document snapshots
}

impl CheckpointManager {
    /// Creates a new `CheckpointManager`.
    pub fn new() -> Self {
        Self {
            checkpoints: HashMap::new(),
        }
    }

    /// Saves a checkpoint for the current document state with the specified name.
    pub fn save_checkpoint(&mut self, name: &str, document: Document) -> Result<(), Box<dyn Error>> {
        self.checkpoints.insert(name.to_string(), document);
        Ok(())
    }

    /// Loads a checkpoint by its name. Returns `None` if the checkpoint doesn't exist.
    pub fn load_checkpoint(&self, name: &str) -> Option<&Document> {
        self.checkpoints.get(name)
    }

    /// Deletes a checkpoint by its name.
    pub fn delete_checkpoint(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.checkpoints.remove(name);
        Ok(())
    }

    /// Lists all saved checkpoints by their names.
    pub fn list_checkpoints(&self) -> Vec<String> {
        self.checkpoints.keys().cloned().collect()
    }

    /// Clears all checkpoints.
    pub fn clear_checkpoints(&mut self) {
        self.checkpoints.clear();
    }
}
