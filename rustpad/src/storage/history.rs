use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileVersion {
    pub version_id: usize,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub description: String, // Optional description or commit message for the version
}

pub struct HistoryManager {
    base_dir: PathBuf,
    max_versions: usize, // Maximum number of versions to retain
    versions: VecDeque<FileVersion>, // Keeps versions in a queue with a maximum length
}

impl HistoryManager {
    /// Creates a new HistoryManager for tracking file versions
    pub fn new(base_dir: &str, max_versions: usize) -> Self {
        Self {
            base_dir: PathBuf::from(base_dir),
            max_versions,
            versions: VecDeque::new(),
        }
    }

    /// Adds a new version to the version history, saving the file and tracking its content
    pub fn add_version(&mut self, file_name: &str, content: &str, description: &str) -> io::Result<()> {
        let version_id = self.versions.len() + 1; // Increment version ID
        let timestamp = Utc::now();

        // Create a new FileVersion instance
        let version = FileVersion {
            version_id,
            content: content.to_string(),
            timestamp,
            description: description.to_string(),
        };

        // Save the version to disk
        self.save_version(file_name, &version)?;

        // Add the version to the queue
        self.versions.push_back(version);

        // Trim the queue to maintain the max_versions limit
        if self.versions.len() > self.max_versions {
            self.versions.pop_front(); // Remove the oldest version
        }

        Ok(())
    }

    /// Retrieves a specific version by its ID
    pub fn get_version(&self, version_id: usize) -> Option<FileVersion> {
        self.versions.iter().find(|&v| v.version_id == version_id).cloned()
    }

    /// Reverts the file to a specific version by overwriting the current file with the version's content
    pub fn revert_to_version(&self, file_name: &str, version_id: usize) -> io::Result<()> {
        if let Some(version) = self.get_version(version_id) {
            let file_path = self.base_dir.join(file_name);
            let mut file = fs::File::create(file_path)?;
            file.write_all(version.content.as_bytes())?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Version not found"))
        }
    }

    /// Saves the content of a version to a file on disk
    fn save_version(&self, file_name: &str, version: &FileVersion) -> io::Result<()> {
        let version_file_name = format!("{}_v{}.txt", file_name, version.version_id);
        let version_path = self.base_dir.join(version_file_name);
        let mut file = fs::File::create(version_path)?;
        file.write_all(version.content.as_bytes())?;
        Ok(())
    }

    /// Loads version history from disk (if required)
    pub fn load_history(&mut self, file_name: &str) -> io::Result<()> {
        // This can be implemented as needed to load previously saved history
        // This could involve reading saved version files from the base directory
        // For now, we assume the history is kept in memory during runtime
        Ok(())
    }

    /// Lists all versions in the history for a specific file
    pub fn list_versions(&self) -> Vec<FileVersion> {
        self.versions.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_history_manager() {
        let temp_dir = "test_history";
        fs::create_dir(temp_dir).unwrap();
        let mut history_manager = HistoryManager::new(temp_dir, 5);

        // Test adding versions
        history_manager.add_version("test.txt", "Version 1 content", "Initial version").unwrap();
        history_manager.add_version("test.txt", "Version 2 content", "Second version").unwrap();

        // Test get_version
        let version_1 = history_manager.get_version(1).unwrap();
        assert_eq!(version_1.content, "Version 1 content");

        let version_2 = history_manager.get_version(2).unwrap();
        assert_eq!(version_2.content, "Version 2 content");

        // Test reverting to a version
        history_manager.revert_to_version("test.txt", 1).unwrap();
        let content = fs::read_to_string(temp_dir.to_string() + "/test.txt").unwrap();
        assert_eq!(content, "Version 1 content");

        // Test trimming versions
        history_manager.add_version("test.txt", "Version 3 content", "Third version").unwrap();
        history_manager.add_version("test.txt", "Version 4 content", "Fourth version").unwrap();
        history_manager.add_version("test.txt", "Version 5 content", "Fifth version").unwrap();
        history_manager.add_version("test.txt", "Version 6 content", "Sixth version").unwrap();

        assert_eq!(history_manager.versions.len(), 5); // Max versions is 5
        assert!(history_manager.get_version(1).is_none()); // Oldest version should be removed

        // Clean up
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
