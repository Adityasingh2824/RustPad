use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use futures_util::{StreamExt, SinkExt};
use warp::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};

/// Represents a peer in the P2P network
#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub sender: mpsc::UnboundedSender<PeerMessage>,
}

/// Message format for synchronization between peers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerMessage {
    pub sender_id: String,
    pub content: String,
    pub timestamp: String,
}

/// Peer-to-peer synchronization manager
pub struct PeerSyncManager {
    peers: Arc<Mutex<HashMap<String, Peer>>>,  // Stores peers keyed by their ID
}

impl PeerSyncManager {
    /// Creates a new PeerSyncManager
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a new peer and returns a mpsc sender for communication
    pub fn register_peer(&self, peer_id: String, ws_socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = ws_socket.split();
        let (sender, mut receiver) = mpsc::unbounded_channel();

        // Add the peer to the peer map
        let peer = Peer {
            id: peer_id.clone(),
            sender,
        };
        self.peers.lock().unwrap().insert(peer_id.clone(), peer);

        // Task to handle receiving messages from the WebSocket
        let recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = ws_rx.next().await {
                if let Ok(text) = msg.to_str() {
                    let received_message: PeerMessage = serde_json::from_str(text).unwrap();
                    println!("Received message from {}: {}", received_message.sender_id, received_message.content);

                    // Apply conflict resolution or synchronization logic here
                }
            }
        });

        // Task to handle sending messages to the WebSocket
        let send_task = tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let msg_text = serde_json::to_string(&msg).unwrap();
                if ws_tx.send(Message::text(msg_text)).await.is_err() {
                    break; // Stop if we can't send the message (client disconnected)
                }
            }
        });

        tokio::select! {
            _ = recv_task => (),
            _ = send_task => (),
        }

        // Clean up the peer when the connection is closed
        self.peers.lock().unwrap().remove(&peer_id);
    }

    /// Broadcasts a message to all peers in the network
    pub fn broadcast_message(&self, sender_id: String, content: String) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let message = PeerMessage {
            sender_id: sender_id.clone(),
            content,
            timestamp,
        };

        // Broadcast the message to all peers
        let peers = self.peers.lock().unwrap();
        for (peer_id, peer) in peers.iter() {
            if *peer_id != sender_id {
                let _ = peer.sender.send(message.clone());
            }
        }
    }

    /// Handles conflict resolution for synchronized content (e.g., last-write-wins)
    pub fn resolve_conflict(&self, existing_content: &str, new_content: &str) -> String {
        // Example conflict resolution logic (last-write-wins)
        // This can be extended to use OT/CRDT algorithms for more complex conflict resolution
        if existing_content == new_content {
            existing_content.to_string()
        } else {
            new_content.to_string()  // Assume last-write-wins for simplicity
        }
    }
}

/// WebSocket handler for peer synchronization
pub async fn peer_sync_handler(ws: warp::ws::Ws, manager: PeerSyncManager, peer_id: String) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manager.register_peer(peer_id, socket))
}

/// Route for peer synchronization WebSocket
pub fn peer_sync_route(manager: PeerSyncManager) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("peer_sync_ws")
        .and(warp::ws())
        .and(warp::path::param::<String>())  // Accept peer_id as a parameter
        .and(with_manager(manager))
        .and_then(peer_sync_handler)
}

/// Helper function to pass the PeerSyncManager to the route
fn with_manager(manager: PeerSyncManager) -> impl warp::Filter<Extract = (PeerSyncManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the peer sync server
#[tokio::main]
async fn main() {
    let peer_sync_manager = PeerSyncManager::new();

    // WebSocket route for peer synchronization
    let peer_sync_ws_route = peer_sync_route(peer_sync_manager.clone());

    // Start the server
    println!("Peer-to-peer sync server running on ws://localhost:3030/peer_sync_ws/{peer_id}");
    warp::serve(peer_sync_ws_route).run(([127, 0, 0, 1], 3030)).await;
}
