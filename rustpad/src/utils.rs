use uuid::Uuid;
use serde_json::{json, Value};
use warp::ws::Message;
use std::error::Error;

/// Generates a new unique identifier (UUID) for a client or user.
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Serializes a given struct or value to a JSON string.
/// Returns the serialized JSON string or an error.
pub fn serialize_to_json<T: serde::Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
    serde_json::to_string(value).map_err(|e| e.into())
}

/// Deserializes a JSON string into a Rust data structure.
/// Returns the deserialized data or an error.
pub fn deserialize_from_json<T: serde::de::DeserializeOwned>(json_str: &str) -> Result<T, Box<dyn Error>> {
    serde_json::from_str(json_str).map_err(|e| e.into())
}

/// Converts a string message into a WebSocket `Message`.
/// Returns the WebSocket message or an error.
pub fn string_to_ws_message(text: &str) -> Result<Message, Box<dyn Error>> {
    Ok(Message::text(text.to_string()))
}

/// Converts a WebSocket message back into a string.
/// Returns the string message or an error.
pub fn ws_message_to_string(message: Message) -> Result<String, Box<dyn Error>> {
    if let Ok(text) = message.to_str() {
        Ok(text.to_string())
    } else {
        Err("Failed to convert WebSocket message to string".into())
    }
}

/// Builds a JSON object for broadcasting document updates.
/// Takes the content and user information and returns a serialized JSON string.
pub fn build_document_update(content: &str, user: &str) -> Result<String, Box<dyn Error>> {
    let update = json!({
        "content": content,
        "user": user
    });
    serde_json::to_string(&update).map_err(|e| e.into())
}

/// Parses the content from a received WebSocket message as a JSON value.
/// Assumes the message is properly formatted JSON.
pub fn parse_ws_message_as_json(message: &str) -> Result<Value, Box<dyn Error>> {
    serde_json::from_str(message).map_err(|e| e.into())
}
