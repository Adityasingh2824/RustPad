use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use futures_util::{StreamExt, SinkExt};
use warp::ws::{Message, WebSocket};
use tokio::sync::broadcast;
use chrono::Utc;

/// Represents a collaborative edit from a user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edit {
    pub user: String,
    pub content: String,
    pub cursor_position: usize,
    pub timestamp: String,
}

/// Manages collaborative editing and broadcasting updates to users
pub struct CollaborationManager {
    document: Arc<Mutex<String>>,                 // Shared document content
    edits: Arc<Mutex<Vec<Edit>>>,                 // Log of edits
    broadcaster: broadcast::Sender<Edit>,         // Broadcast channel for updates
}

impl CollaborationManager {
    /// Creates a new CollaborationManager with an empty document and edit log
    pub fn new() -> Self {
        let (broadcaster, _) = broadcast::channel(100); // Create a broadcast channel with capacity
        Self {
            document: Arc::new(Mutex::new(String::new())),
            edits: Arc::new(Mutex::new(Vec::new())),
            broadcaster,
        }
    }

    /// Registers a new WebSocket client for collaborative editing
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();
        let mut rx = self.broadcaster.subscribe();

        // Task to send document updates to the client
        let send_task = tokio::spawn(async move {
            while let Ok(edit) = rx.recv().await {
                let msg = serde_json::to_string(&edit).unwrap();
                if ws_tx.send(Message::text(msg)).await.is_err() {
                    break; // Client disconnected
                }
            }
        });

        // Task to receive edits from the client
        let recv_task = tokio::spawn(async move {
            while let Some(result) = ws_rx.next().await {
                if let Ok(msg) = result {
                    if msg.is_text() {
                        let edit: Edit = serde_json::from_str(msg.to_str().unwrap()).unwrap();
                        self.apply_edit(edit.clone()).await;
                        let _ = self.broadcaster.send(edit);  // Broadcast the edit to all clients
                    }
                }
            }
        });

        tokio::select! {
            _ = send_task => (),
            _ = recv_task => (),
        }
    }

    /// Applies an edit to the shared document
    pub async fn apply_edit(&self, edit: Edit) {
        let mut document = self.document.lock().unwrap();
        let mut edits = self.edits.lock().unwrap();

        // Add the edit to the log
        edits.push(edit.clone());

        // Merge the edit into the document (simple append for now, can be more complex)
        *document = edit.content.clone();

        println!("Document updated by {}: {}", edit.user, document);
    }

    /// Retrieves the current document content
    pub fn get_document(&self) -> String {
        let document = self.document.lock().unwrap();
        document.clone()
    }
}

/// WebSocket handler for collaborative editing
pub async fn collaboration_ws_handler(ws: warp::ws::Ws, manager: Arc<CollaborationManager>) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for WebSocket collaborative editing
pub fn collaboration_route(manager: Arc<CollaborationManager>) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("collaborate")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(collaboration_ws_handler)
}

/// Helper function to pass the CollaborationManager to the route
fn with_manager(manager: Arc<CollaborationManager>) -> impl warp::Filter<Extract = (Arc<CollaborationManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the collaboration server
#[tokio::main]
async fn main() {
    let manager = Arc::new(CollaborationManager::new());

    // WebSocket route for collaborative editing
    let collaborate_route = collaboration_route(manager.clone());

    // Start the server
    println!("Collaboration server running on ws://localhost:3030/collaborate");
    warp::serve(collaborate_route).run(([127, 0, 0, 1], 3030)).await;
}
