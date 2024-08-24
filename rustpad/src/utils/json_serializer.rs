use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::error::Error;

/// Serializes a given struct or value to a JSON string.
/// It returns the JSON string on success or an error on failure.
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(value)?;
    Ok(json_string)
}

/// Deserializes a JSON string into a specified Rust data type.
/// It returns the deserialized struct or an error on failure.
pub fn from_json_string<T: for<'de> Deserialize<'de>>(json_str: &str) -> Result<T, Box<dyn Error>> {
    let value: T = serde_json::from_str(json_str)?;
    Ok(value)
}

/// Parses a JSON string and returns a `serde_json::Value` for general manipulation.
/// This is useful for dynamically interacting with unstructured JSON data.
pub fn parse_json(json_str: &str) -> Result<Value, Box<dyn Error>> {
    let parsed_json: Value = serde_json::from_str(json_str)?;
    Ok(parsed_json)
}

/// Serializes a given struct to a JSON string and writes it to a file.
pub fn save_json_to_file<T: Serialize>(file_path: &str, value: &T) -> Result<(), Box<dyn Error>> {
    let json_string = to_json_string(value)?;
    std::fs::write(file_path, json_string)?;
    Ok(())
}

/// Reads a JSON file and deserializes its content into a specified Rust data type.
pub fn load_json_from_file<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<T, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let value: T = from_json_string(&file_content)?;
    Ok(value)
}
