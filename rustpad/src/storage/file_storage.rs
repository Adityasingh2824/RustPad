use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub file_name: String,
    pub file_path: String,
    pub last_modified: String,
}

/// Manages file storage operations including saving, loading, deleting, and renaming files.
pub struct FileStorage {
    base_dir: PathBuf,
}

impl FileStorage {
    /// Creates a new FileStorage instance with the specified base directory
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: PathBuf::from(base_dir),
        }
    }

    /// Saves content to a file in the base directory.
    pub fn save_file(&self, file_name: &str, content: &str) -> io::Result<FileInfo> {
        let file_path = self.base_dir.join(file_name);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content.as_bytes())?;

        let last_modified = Self::get_last_modified(&file_path)?;

        Ok(FileInfo {
            file_name: file_name.to_string(),
            file_path: file_path.to_string_lossy().to_string(),
            last_modified,
        })
    }

    /// Loads the content of a file from the base directory.
    pub fn load_file(&self, file_name: &str) -> io::Result<String> {
        let file_path = self.base_dir.join(file_name);
        let content = fs::read_to_string(file_path)?;
        Ok(content)
    }

    /// Deletes a file from the base directory.
    pub fn delete_file(&self, file_name: &str) -> io::Result<()> {
        let file_path = self.base_dir.join(file_name);
        fs::remove_file(file_path)?;
        Ok(())
    }

    /// Renames a file in the base directory.
    pub fn rename_file(&self, old_name: &str, new_name: &str) -> io::Result<FileInfo> {
        let old_path = self.base_dir.join(old_name);
        let new_path = self.base_dir.join(new_name);
        fs::rename(&old_path, &new_path)?;

        let last_modified = Self::get_last_modified(&new_path)?;

        Ok(FileInfo {
            file_name: new_name.to_string(),
            file_path: new_path.to_string_lossy().to_string(),
            last_modified,
        })
    }

    /// Lists all files in the base directory.
    pub fn list_files(&self) -> io::Result<Vec<FileInfo>> {
        let mut files = Vec::new();

        for entry in fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let last_modified = Self::get_last_modified(&path)?;

                files.push(FileInfo {
                    file_name,
                    file_path: path.to_string_lossy().to_string(),
                    last_modified,
                });
            }
        }

        Ok(files)
    }

    /// Helper function to get the last modified time as a human-readable string.
    fn get_last_modified(path: &Path) -> io::Result<String> {
        let metadata = fs::metadata(path)?;
        let modified_time = metadata.modified()?;

        // Convert to the UNIX timestamp and then to human-readable time
        let duration_since_epoch = modified_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        // Convert to seconds since epoch
        let timestamp = duration_since_epoch.as_secs();

        Ok(timestamp.to_string())  // For simplicity, we're just returning the timestamp as a string.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_storage() {
        let temp_dir = "test_storage";
        fs::create_dir(temp_dir).unwrap();
        let storage = FileStorage::new(temp_dir);

        // Test save_file
        let file_info = storage.save_file("test.txt", "Hello, world!").unwrap();
        assert_eq!(file_info.file_name, "test.txt");

        // Test load_file
        let content = storage.load_file("test.txt").unwrap();
        assert_eq!(content, "Hello, world!");

        // Test rename_file
        let renamed_info = storage.rename_file("test.txt", "new_test.txt").unwrap();
        assert_eq!(renamed_info.file_name, "new_test.txt");

        // Test delete_file
        storage.delete_file("new_test.txt").unwrap();
        assert!(storage.load_file("new_test.txt").is_err());

        // Clean up
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
