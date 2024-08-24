pub mod websocket;
pub mod peer_sync;
pub mod protocol;

use websocket::WebSocketClient;
use peer_sync::PeerSync;

/// `Networking` struct acts as the central controller for managing the peer-to-peer
/// communication and WebSocket connections for collaborative editing.
pub struct Networking {
    websocket_client: WebSocketClient,
    peer_sync: PeerSync,
}

impl Networking {
    /// Creates a new `Networking` instance that initializes WebSocket and peer synchronization.
    pub fn new(server_url: &str) -> Self {
        Self {
            websocket_client: WebSocketClient::new(server_url),
            peer_sync: PeerSync::new(),
        }
    }

    /// Starts the networking service by connecting to the WebSocket server and handling
    /// incoming messages.
    pub async fn start(&mut self) {
        // Establish WebSocket connection
        if let Err(e) = self.websocket_client.connect().await {
            eprintln!("Failed to connect to WebSocket server: {}", e);
            return;
        }

        // Begin processing messages from the WebSocket connection
        self.process_incoming_messages().await;
    }

    /// Processes incoming messages from the WebSocket connection and applies them to the peer sync.
    async fn process_incoming_messages(&mut self) {
        while let Some(message) = self.websocket_client.receive_message().await {
            // Apply the received message to the peer synchronization logic
            self.peer_sync.handle_incoming_message(message).await;
        }
    }

    /// Sends a document change to all connected peers via WebSocket.
    pub async fn broadcast_change(&mut self, change: &str) {
        if let Err(e) = self.websocket_client.send_message(change).await {
            eprintln!("Failed to broadcast change: {}", e);
        }
    }

    /// Broadcasts cursor position to all connected peers (optional).
    pub async fn broadcast_cursor(&mut self, cursor_position: usize) {
        let message = format!("{{\"cursor_position\": {}}}", cursor_position);
        if let Err(e) = self.websocket_client.send_message(&message).await {
            eprintln!("Failed to broadcast cursor position: {}", e);
        }
    }
}
