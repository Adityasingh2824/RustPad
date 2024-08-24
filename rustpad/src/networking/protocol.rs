use crate::editor::diff_engine::DiffOperation;
use serde::{Serialize, Deserialize};

/// `SyncMessage` represents a message that contains a series of diff operations
/// to apply changes to the document for synchronization between peers.
#[derive(Serialize, Deserialize, Debug)]
pub struct SyncMessage {
    pub operations: Vec<DiffOperation>,
}

impl SyncMessage {
    /// Creates a new `SyncMessage` from a list of diff operations.
    pub fn new(operations: Vec<DiffOperation>) -> Self {
        SyncMessage { operations }
    }

    /// Create a `SyncMessage` by computing the difference between the previous
    /// and current state of the editor. This assumes a diff method is available.
    pub fn new_from_state(prev_state: &str, current_state: &str) -> Self {
        let operations = crate::editor::diff_engine::DiffEngine::diff(prev_state, current_state);
        SyncMessage { operations }
    }
}

/// `CursorMessage` represents a message that communicates a user's cursor position.
#[derive(Serialize, Deserialize, Debug)]
pub struct CursorMessage {
    pub cursor_position: usize,
}

impl CursorMessage {
    /// Creates a new `CursorMessage` with the specified cursor position.
    pub fn new(cursor_position: usize) -> Self {
        CursorMessage { cursor_position }
    }
}

/// `ProtocolMessage` represents all possible messages that can be sent between peers.
/// It can encapsulate different types of messages, like sync messages and cursor updates.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ProtocolMessage {
    Sync(SyncMessage),
    Cursor(CursorMessage),
}

impl ProtocolMessage {
    /// Serializes the protocol message to a JSON string, ready to be sent over the WebSocket.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes a JSON string into a `ProtocolMessage`.
    pub fn from_json(json: &str) -> Result<ProtocolMessage, serde_json::Error> {
        serde_json::from_str(json)
    }
}
