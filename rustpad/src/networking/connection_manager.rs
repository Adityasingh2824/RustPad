use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use async_tungstenite::tokio::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::task::JoinHandle;
use std::net::SocketAddr;

/// Represents a peer's connection, holding a WebSocket sender.
pub struct PeerConnection {
    pub sender: UnboundedSender<Message>,
}

/// `ConnectionManager` manages the WebSocket connections between peers.
pub struct ConnectionManager {
    peers: Arc<Mutex<HashMap<SocketAddr, PeerConnection>>>, // Manages peer connections
}

impl ConnectionManager {
    /// Creates a new `ConnectionManager`.
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a new peer to the connection manager and spawns a task to handle its connection.
    pub async fn add_peer(&self, stream: TcpStream, peer_addr: SocketAddr) -> JoinHandle<()> {
        let (tx, mut rx) = unbounded_channel();

        // Add peer to the peer map
        self.peers.lock().unwrap().insert(peer_addr, PeerConnection { sender: tx });

        let peers = self.peers.clone();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
            let (mut write, mut read) = ws_stream.split();

            // Task to send messages to the peer
            let send_task = tokio::spawn(async move {
                while let Some(message) = rx.recv().await {
                    if write.send(message).await.is_err() {
                        break; // If sending fails, break out of the loop
                    }
                }
            });

            // Task to receive messages from the peer
            let recv_task = tokio::spawn(async move {
                while let Some(Ok(message)) = read.next().await {
                    if let Message::Text(text) = message {
                        ConnectionManager::broadcast_message(&peers, &peer_addr, text).await;
                    }
                }

                // When the peer disconnects, remove them from the peer map
                peers.lock().unwrap().remove(&peer_addr);
            });

            // Wait for both tasks to complete
            tokio::select! {
                _ = send_task => {},
                _ = recv_task => {},
            }
        })
    }

    /// Broadcasts a message to all connected peers except the sender.
    async fn broadcast_message(peers: &Arc<Mutex<HashMap<SocketAddr, PeerConnection>>>, sender_addr: &SocketAddr, message: String) {
        let peers = peers.lock().unwrap();
        for (peer_addr, peer) in peers.iter() {
            if peer_addr != sender_addr {
                let _ = peer.sender.send(Message::Text(message.clone()));
            }
        }
    }

    /// Sends a message to a specific peer.
    pub async fn send_to_peer(&self, peer_addr: &SocketAddr, message: String) {
        let peers = self.peers.lock().unwrap();
        if let Some(peer) = peers.get(peer_addr) {
            let _ = peer.sender.send(Message::Text(message));
        }
    }

    /// Broadcasts a message to all connected peers.
    pub async fn broadcast_to_all(&self, message: String) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.values() {
            let _ = peer.sender.send(Message::Text(message.clone()));
        }
    }
}
