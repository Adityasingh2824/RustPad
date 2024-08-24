use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures_util::{StreamExt, SinkExt};
use warp::ws::{Message, WebSocket};

/// Represents a collaborator's cursor position
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cursor {
    pub user: String,        // The user's name or identifier
    pub position: usize,     // The cursor's position (character index) in the document
    pub color: String,       // The color of the cursor to distinguish users
}

/// Manages tracking and displaying of user cursors in the collaborative editor
pub struct CursorManager {
    cursors: Arc<Mutex<HashMap<String, Cursor>>>,  // Map of user ID to cursor positions
}

impl CursorManager {
    /// Creates a new CursorManager with an empty cursor map
    pub fn new() -> Self {
        Self {
            cursors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a new cursor for a user
    pub fn register_cursor(&self, user: String, initial_position: usize, color: String) {
        let mut cursors = self.cursors.lock().unwrap();
        cursors.insert(
            user.clone(),
            Cursor {
                user,
                position: initial_position,
                color,
            },
        );
    }

    /// Updates the cursor position of a user
    pub fn update_cursor(&self, user: String, new_position: usize) {
        let mut cursors = self.cursors.lock().unwrap();
        if let Some(cursor) = cursors.get_mut(&user) {
            cursor.position = new_position;
        }
    }

    /// Removes a cursor when a user disconnects
    pub fn remove_cursor(&self, user: &str) {
        let mut cursors = self.cursors.lock().unwrap();
        cursors.remove(user);
    }

    /// Retrieves the current cursor positions for all users
    pub fn get_cursors(&self) -> Vec<Cursor> {
        let cursors = self.cursors.lock().unwrap();
        cursors.values().cloned().collect()
    }

    /// Broadcasts cursor positions to all clients
    pub async fn broadcast_cursors(&self, socket: WebSocket) {
        let cursors = self.get_cursors();
        let serialized_cursors = serde_json::to_string(&cursors).unwrap();
        let (mut ws_tx, _) = socket.split();
        let _ = ws_tx.send(Message::text(serialized_cursors)).await;
    }
}

/// WebSocket handler for cursor synchronization
pub async fn cursor_ws_handler(ws: warp::ws::Ws, manager: Arc<CursorManager>) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manage_cursors(socket, manager))
}

async fn manage_cursors(mut socket: WebSocket, manager: Arc<CursorManager>) {
    while let Some(result) = socket.next().await {
        if let Ok(message) = result {
            if let Ok(text) = message.to_str() {
                let cursor: Cursor = serde_json::from_str(text).unwrap();
                manager.update_cursor(cursor.user.clone(), cursor.position);
                
                // Broadcast updated cursor positions to all clients
                manager.broadcast_cursors(socket.clone()).await;
            }
        }
    }
}

/// Route for WebSocket cursor updates
pub fn cursor_route(manager: Arc<CursorManager>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("cursors")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(cursor_ws_handler)
}

/// Helper function to pass the CursorManager to the route
fn with_manager(manager: Arc<CursorManager>) -> impl warp::Filter<Extract = (Arc<CursorManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the cursor sync server
#[tokio::main]
async fn main() {
    let manager = Arc::new(CursorManager::new());

    // WebSocket route for cursor synchronization
    let cursors_route = cursor_route(manager.clone());

    // Start the server
    println!("Cursor synchronization server running on ws://localhost:3030/cursors");
    warp::serve(cursors_route).run(([127, 0, 0, 1], 3030)).await;
}
