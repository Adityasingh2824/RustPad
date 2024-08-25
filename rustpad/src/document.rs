use serde::{Deserialize, Serialize};

/// Represents an update to the document. This struct is shared between
/// the server and clients to communicate document changes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentUpdate {
    pub content: String,
    pub user: String,
    pub timestamp: String,  // Adding a timestamp to track when the update occurred
}

impl DocumentUpdate {
    /// Creates a new `DocumentUpdate` with the given content, user, and timestamp.
    pub fn new(content: &str, user: &str) -> Self {
        DocumentUpdate {
            content: content.to_string(),
            user: user.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        }
    }
}

/// Represents the overall document that multiple clients are collaborating on.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub content: String,
    pub history: Vec<DocumentUpdate>, // History of updates for undo/redo functionality
}

impl Document {
    /// Creates a new empty document.
    pub fn new() -> Self {
        Document {
            content: String::new(),
            history: Vec::new(),
        }
    }

    /// Applies a new update to the document, modifying its content.
    pub fn apply_update(&mut self, update: DocumentUpdate) {
        self.history.push(update.clone());
        self.content = update.content;
    }

    /// Retrieves the current document content.
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Retrieves the history of updates made to the document.
    pub fn get_history(&self) -> &Vec<DocumentUpdate> {
        &self.history
    }

    /// Rolls back the document to the previous state by removing the last update.
    pub fn undo_last_update(&mut self) -> Option<&DocumentUpdate> {
        if self.history.len() > 1 {
            self.history.pop(); // Remove the latest update
            self.content = self.history.last().unwrap().content.clone();
            Some(self.history.last().unwrap())
        } else {
            None // No more history to undo
        }
    }

    /// Redo functionality to apply the next state after an undo.
    pub fn redo_update(&mut self, update: DocumentUpdate) {
        self.apply_update(update);
    }
}
