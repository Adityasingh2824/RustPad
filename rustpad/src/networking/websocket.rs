use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RealTimeMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}

type WebSocketClients = Arc<Mutex<Vec<warp::ws::WebSocket>>>;

pub struct WebSocketManager {
    clients: WebSocketClients,
    broadcaster: broadcast::Sender<RealTimeMessage>,
}

impl WebSocketManager {
    /// Creates a new WebSocketManager with an empty client list and a broadcast channel
    pub fn new() -> Self {
        let (broadcaster, _) = broadcast::channel(100); // Creates a broadcast channel with 100 capacity
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
            broadcaster,
        }
    }

    /// Registers a new WebSocket client and starts listening for messages
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();

        {
            let mut clients = self.clients.lock().unwrap();
            clients.push(ws_tx.clone());
        }

        let mut rx = self.broadcaster.subscribe();

        // Task to forward messages from broadcast channel to this client
        let send_task = tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                let msg_text = serde_json::to_string(&message).unwrap();
                if ws_tx.send(Message::text(msg_text)).await.is_err() {
                    break; // Client disconnected
                }
            }
        });

        // Task to receive messages from this WebSocket client
        let recv_task = tokio::spawn(async move {
            while let Some(result) = ws_rx.next().await {
                if let Ok(msg) = result {
                    if msg.is_text() {
                        let msg_text = msg.to_str().unwrap();
                        let received_message: RealTimeMessage = serde_json::from_str(msg_text).unwrap();
                        // Broadcast the received message to all clients
                        let _ = self.broadcaster.send(received_message);
                    }
                }
            }
        });

        // Wait for either the send or receive task to complete
        tokio::select! {
            _ = send_task => (),
            _ = recv_task => (),
        }

        // Remove the client when the connection is closed
        {
            let mut clients = self.clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    }
}

/// WebSocket handler for real-time communication
pub async fn websocket_handler(ws: warp::ws::Ws, manager: WebSocketManager) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for WebSocket real-time communication
pub fn websocket_route(manager: WebSocketManager) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(websocket_handler)
}

/// Helper function to pass the WebSocketManager to the route
fn with_manager(manager: WebSocketManager) -> impl warp::Filter<Extract = (WebSocketManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the WebSocket server
#[tokio::main]
async fn main() {
    let ws_manager = WebSocketManager::new();

    // WebSocket route for real-time communication
    let ws_route = websocket_route(ws_manager.clone());

    // Start the WebSocket server
    println!("WebSocket server running on ws://localhost:3030/ws");
    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}
