use difference::{Changeset, Difference};

/// Represents a single difference between two versions of a document.
#[derive(Debug, Clone)]
pub enum DiffOperation {
    Insert(String),
    Delete(String),
    Equal(String),
}

/// Computes the diff between two versions of a document.
/// Returns a vector of `DiffOperation` representing the changes.
pub fn compute_diff(old_version: &str, new_version: &str) -> Vec<DiffOperation> {
    let changeset = Changeset::new(old_version, new_version, "\n");
    let mut operations = Vec::new();

    for diff in changeset.diffs {
        match diff {
            Difference::Add(addition) => operations.push(DiffOperation::Insert(addition)),
            Difference::Rem(deletion) => operations.push(DiffOperation::Delete(deletion)),
            Difference::Same(equality) => operations.push(DiffOperation::Equal(equality)),
        }
    }

    operations
}

/// Applies a series of diff operations to the original document.
/// Returns the updated document after applying all operations.
pub fn apply_diff(original: &str, diff_operations: &[DiffOperation]) -> String {
    let mut result = String::new();

    for operation in diff_operations {
        match operation {
            DiffOperation::Insert(insert) => result.push_str(insert),
            DiffOperation::Delete(_) => {
                // Deletions are skipped, as they represent removal of text
            }
            DiffOperation::Equal(equal) => result.push_str(equal),
        }
    }

    result
}

/// Reverts a series of diff operations to go back to the original document.
/// Returns the reverted document.
pub fn revert_diff(modified: &str, diff_operations: &[DiffOperation]) -> String {
    let mut result = String::new();
    
    for operation in diff_operations {
        match operation {
            DiffOperation::Insert(_) => {
                // Insertions are skipped, as they represent additions in the modified document
            }
            DiffOperation::Delete(deleted_text) => result.push_str(deleted_text),
            DiffOperation::Equal(equal) => result.push_str(equal),
        }
    }

    result
}
