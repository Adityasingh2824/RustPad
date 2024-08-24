/// Represents the type of change detected between document states.
#[derive(Debug, PartialEq, Clone)]
pub enum DiffOperation {
    Insert(usize, String),  // Insert text at position (pos, "text")
    Delete(usize, usize),   // Delete text from start to end (start, end)
    Replace(usize, usize, String), // Replace text from start to end with new text (start, end, "new_text")
}

/// The `DiffEngine` struct calculates differences between two versions of a document.
/// These differences can be used for synchronization, version control, and collaborative editing.
pub struct DiffEngine;

impl DiffEngine {
    /// Compares two versions of a document and returns a list of diff operations.
    ///
    /// # Arguments
    /// * `old_text` - The original text before changes.
    /// * `new_text` - The updated text after changes.
    ///
    /// # Returns
    /// * A `Vec` of `DiffOperation` representing the changes between `old_text` and `new_text`.
    pub fn diff(old_text: &str, new_text: &str) -> Vec<DiffOperation> {
        let mut operations = Vec::new();
        
        let common_prefix = DiffEngine::find_common_prefix(old_text, new_text);
        let common_suffix = DiffEngine::find_common_suffix(old_text, new_text, common_prefix);

        let old_middle = &old_text[common_prefix..old_text.len() - common_suffix];
        let new_middle = &new_text[common_prefix..new_text.len() - common_suffix];

        if old_middle.is_empty() && !new_middle.is_empty() {
            // Insertion detected
            operations.push(DiffOperation::Insert(common_prefix, new_middle.to_string()));
        } else if !old_middle.is_empty() && new_middle.is_empty() {
            // Deletion detected
            operations.push(DiffOperation::Delete(common_prefix, common_prefix + old_middle.len()));
        } else if !old_middle.is_empty() && !new_middle.is_empty() && old_middle != new_middle {
            // Replacement detected
            operations.push(DiffOperation::Replace(common_prefix, common_prefix + old_middle.len(), new_middle.to_string()));
        }

        operations
    }

    /// Finds the length of the common prefix between two strings.
    fn find_common_prefix(old_text: &str, new_text: &str) -> usize {
        let min_len = old_text.len().min(new_text.len());
        for i in 0..min_len {
            if old_text.as_bytes()[i] != new_text.as_bytes()[i] {
                return i;
            }
        }
        min_len
    }

    /// Finds the length of the common suffix between two strings, considering the common prefix.
    fn find_common_suffix(old_text: &str, new_text: &str, common_prefix: usize) -> usize {
        let old_len = old_text.len();
        let new_len = new_text.len();
        let min_len = old_len.min(new_len) - common_prefix;
        
        for i in 0..min_len {
            if old_text.as_bytes()[old_len - 1 - i] != new_text.as_bytes()[new_len - 1 - i] {
                return i;
            }
        }
        min_len
    }
}

