use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};
use futures_util::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub user: String,
    pub message: String,
}

type ChatClients = Arc<Mutex<Vec<warp::ws::WebSocket>>>;

/// Manages the chat participants and broadcast functionality
pub struct ChatManager {
    clients: ChatClients,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers a new WebSocket client for receiving chat messages
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();
        
        {
            let mut clients = self.clients.lock().unwrap();
            clients.push(ws_tx);
        }

        // Wait for incoming chat messages from the client
        while let Some(result) = ws_rx.next().await {
            if let Ok(message) = result {
                if message.is_text() {
                    // Broadcast the received message to all clients
                    let chat_message: ChatMessage = serde_json::from_str(message.to_str().unwrap()).unwrap();
                    self.broadcast_message(chat_message).await;
                }
            }
        }

        // Remove the WebSocket client when it disconnects
        {
            let mut clients = self.clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    }

    /// Broadcasts a chat message to all connected clients
    pub async fn broadcast_message(&self, chat_message: ChatMessage) {
        let message = serde_json::to_string(&chat_message).unwrap();
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            if client.send(Message::text(message.clone())).await.is_err() {
                println!("Failed to send message to client");
            }
        }
    }
}

/// WebSocket handler for the chat WebSocket route
pub async fn chat_ws_handler(ws: warp::ws::Ws, manager: ChatManager) -> impl Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for sending chat messages via WebSocket
pub fn chat_route(manager: ChatManager) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path("chat_ws")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(chat_ws_handler)
}

/// Helper function to pass the ChatManager to the route
fn with_manager(manager: ChatManager) -> impl Filter<Extract = (ChatManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

#[tokio::main]
async fn main() {
    let chat_manager = ChatManager::new();

    // WebSocket route for the chat system
    let chat_ws_route = chat_route(chat_manager.clone());

    // Combine the routes
    println!("Chat server running at ws://localhost:3030/chat_ws");
    warp::serve(chat_ws_route).run(([127, 0, 0, 1], 3030)).await;
}
