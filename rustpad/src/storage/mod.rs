pub mod local_storage;
pub mod ipfs_storage;
pub mod theme;
pub mod file_storage;


use std::error::Error;

/// Represents a generic interface for document storage.
/// Allows saving, loading, and deleting documents.
pub trait Storage {
    /// Saves the given content to storage with the provided identifier (e.g., filename or hash).
    fn save(&self, identifier: &str, content: &str) -> Result<(), Box<dyn Error>>;

    /// Loads the content of the document from storage using the identifier.
    fn load(&self, identifier: &str) -> Result<String, Box<dyn Error>>;

    /// Deletes a document from storage using the identifier.
    fn delete(&self, identifier: &str) -> Result<(), Box<dyn Error>>;
}
