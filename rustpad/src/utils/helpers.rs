use chrono::{Utc, DateTime};
use uuid::Uuid;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use sha2::{Sha256, Digest};

/// Generates a universally unique identifier (UUID).
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Returns the current UTC timestamp as an RFC3339 string.
pub fn current_utc_timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/// Hashes a string using SHA-256 and returns the resulting hex-encoded hash.
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Writes the given content to a file at the specified path.
pub fn write_to_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Reads the content of a file into a string.
pub fn read_file_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

/// Converts a byte slice to a hex-encoded string.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

/// Ensures that a directory exists, creating it if it does not.
pub fn ensure_directory_exists(path: &str) -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
