use serde::{Deserialize, Serialize};
use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use crate::storage::file_storage::FileStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChange {
    pub file_name: String,
    pub content: String,
    pub user: String,
    pub timestamp: String,
}

type SyncClients = Arc<Mutex<Vec<warp::ws::WebSocket>>>;

/// Manages file synchronization between the server and clients
pub struct SyncManager {
    clients: SyncClients,
    file_storage: Arc<FileStorage>,
}

impl SyncManager {
    /// Creates a new SyncManager with a list of connected clients and file storage
    pub fn new(file_storage: Arc<FileStorage>) -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
            file_storage,
        }
    }

    /// Registers a new WebSocket client for file synchronization
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();

        {
            let mut clients = self.clients.lock().unwrap();
            clients.push(ws_tx.clone());
        }

        // Listen for incoming file changes from the client
        while let Some(result) = ws_rx.next().await {
            if let Ok(message) = result {
                if message.is_text() {
                    let file_change: FileChange = serde_json::from_str(message.to_str().unwrap()).unwrap();
                    self.apply_file_change(file_change.clone()).await;
                    self.broadcast_file_change(file_change).await;
                }
            }
        }

        // Remove the WebSocket client when it disconnects
        {
            let mut clients = self.clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    }

    /// Applies a file change to the server's file storage
    pub async fn apply_file_change(&self, file_change: FileChange) {
        // Save the file change to the file system using FileStorage
        let result = self.file_storage.save_file(&file_change.file_name, &file_change.content);
        
        if let Err(e) = result {
            eprintln!("Failed to save file: {}", e);
        }
    }

    /// Broadcasts a file change to all connected clients
    pub async fn broadcast_file_change(&self, file_change: FileChange) {
        let message = serde_json::to_string(&file_change).unwrap();
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            if client.send(Message::text(message.clone())).await.is_err() {
                eprintln!("Failed to send file change to client");
            }
        }
    }
}

/// WebSocket handler for file synchronization
pub async fn sync_ws_handler(ws: warp::ws::Ws, manager: SyncManager) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for file synchronization WebSocket
pub fn sync_route(manager: SyncManager) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("sync_ws")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(sync_ws_handler)
}

/// Helper function to pass the SyncManager to the route
fn with_manager(manager: SyncManager) -> impl warp::Filter<Extract = (SyncManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the file sync server
#[tokio::main]
async fn main() {
    let file_storage = Arc::new(FileStorage::new("project_files"));
    let sync_manager = SyncManager::new(file_storage.clone());

    // WebSocket route for file synchronization
    let sync_ws_route = sync_route(sync_manager.clone());

    // Start the server
    println!("File sync server running on ws://localhost:3030/sync_ws");
    warp::serve(sync_ws_route).run(([127, 0, 0, 1], 3030)).await;
}
