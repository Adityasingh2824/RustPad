use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub user: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    pub user: String,
    pub content: String,
    pub line_number: usize,
    pub timestamp: String,
}

type ChatHistory = Arc<Mutex<Vec<ChatMessage>>>;
type Annotations = Arc<Mutex<HashMap<usize, Vec<Annotation>>>>; // Keyed by line number
type ChatClients = Arc<Mutex<Vec<warp::ws::WebSocket>>>;

/// Manages chat synchronization between collaborators
pub struct ChatSyncManager {
    chat_history: ChatHistory,
    annotations: Annotations,
    clients: ChatClients,
}

impl ChatSyncManager {
    /// Creates a new ChatSyncManager with empty chat history and annotations
    pub fn new() -> Self {
        Self {
            chat_history: Arc::new(Mutex::new(Vec::new())),
            annotations: Arc::new(Mutex::new(HashMap::new())),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers a new WebSocket client and sends the current chat history and annotations
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();

        {
            let mut clients = self.clients.lock().unwrap();
            clients.push(ws_tx.clone());
        }

        // Send current chat history and annotations to the newly connected client
        let chat_history = self.chat_history.lock().unwrap().clone();
        let annotations = self.annotations.lock().unwrap().clone();

        let initial_state = serde_json::to_string(&(chat_history, annotations)).unwrap();
        if ws_tx.send(Message::text(initial_state)).await.is_err() {
            println!("Failed to send initial state to the client");
        }

        // Listen for incoming messages from the client
        while let Some(result) = ws_rx.next().await {
            if let Ok(message) = result {
                if message.is_text() {
                    // Handle incoming chat or annotation messages
                    let parsed_message: serde_json::Value = serde_json::from_str(message.to_str().unwrap()).unwrap();

                    // Check if it's a chat message
                    if let Some(chat_msg) = parsed_message.get("chat_message") {
                        let chat_message: ChatMessage = serde_json::from_value(chat_msg.clone()).unwrap();
                        self.add_chat_message(chat_message.clone()).await;
                        self.broadcast_chat_message(chat_message).await;
                    }

                    // Check if it's an annotation
                    if let Some(annotation_msg) = parsed_message.get("annotation") {
                        let annotation: Annotation = serde_json::from_value(annotation_msg.clone()).unwrap();
                        self.add_annotation(annotation.clone()).await;
                        self.broadcast_annotation(annotation).await;
                    }
                }
            }
        }

        // Remove the WebSocket client when it disconnects
        {
            let mut clients = self.clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    }

    /// Adds a new chat message to the chat history
    async fn add_chat_message(&self, chat_message: ChatMessage) {
        let mut chat_history = self.chat_history.lock().unwrap();
        chat_history.push(chat_message);
    }

    /// Adds a new annotation to the list of annotations
    async fn add_annotation(&self, annotation: Annotation) {
        let mut annotations = self.annotations.lock().unwrap();
        annotations.entry(annotation.line_number).or_insert_with(Vec::new).push(annotation);
    }

    /// Broadcasts a chat message to all connected clients
    async fn broadcast_chat_message(&self, chat_message: ChatMessage) {
        let message = serde_json::to_string(&serde_json::json!({
            "chat_message": chat_message
        }))
        .unwrap();
        
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            if client.send(Message::text(message.clone())).await.is_err() {
                println!("Failed to send chat message to a client");
            }
        }
    }

    /// Broadcasts an annotation to all connected clients
    async fn broadcast_annotation(&self, annotation: Annotation) {
        let message = serde_json::to_string(&serde_json::json!({
            "annotation": annotation
        }))
        .unwrap();
        
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            if client.send(Message::text(message.clone())).await.is_err() {
                println!("Failed to send annotation to a client");
            }
        }
    }
}

/// WebSocket handler for the chat and annotation synchronization
pub async fn chat_sync_ws_handler(ws: warp::ws::Ws, manager: ChatSyncManager) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for the chat synchronization WebSocket
pub fn chat_sync_route(manager: ChatSyncManager) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("chat_sync_ws")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(chat_sync_ws_handler)
}

/// Helper function to pass the ChatSyncManager to the route
fn with_manager(manager: ChatSyncManager) -> impl warp::Filter<Extract = (ChatSyncManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the chat sync server
#[tokio::main]
async fn main() {
    let chat_sync_manager = ChatSyncManager::new();

    // WebSocket route for chat synchronization
    let chat_sync_ws_route = chat_sync_route(chat_sync_manager.clone());

    // Start the server
    println!("Chat and annotation sync server running on ws://localhost:3030/chat_sync_ws");
    warp::serve(chat_sync_ws_route).run(([127, 0, 0, 1], 3030)).await;
}
