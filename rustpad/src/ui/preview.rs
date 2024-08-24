use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};
use futures_util::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
struct PreviewUpdate {
    html: String,
    css: String,
    js: String,
}

type PreviewClients = Arc<Mutex<Vec<warp::ws::WebSocket>>>;

/// Manages the live preview updates and WebSocket connections
pub struct PreviewManager {
    clients: PreviewClients,
}

impl PreviewManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers a new WebSocket client for receiving preview updates
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();
        
        {
            let mut clients = self.clients.lock().unwrap();
            clients.push(ws_tx);
        }

        // Wait for incoming messages (this can be commands for the preview, e.g., reload)
        while let Some(result) = ws_rx.next().await {
            if let Ok(message) = result {
                if message.is_text() {
                    // Handle incoming WebSocket messages (if needed)
                    println!("Received message: {}", message.to_str().unwrap());
                }
            }
        }

        // Remove the WebSocket client when it disconnects
        {
            let mut clients = self.clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    }

    /// Broadcasts the updated HTML, CSS, and JS to all connected clients
    pub async fn broadcast_update(&self, update: PreviewUpdate) {
        let message = serde_json::to_string(&update).unwrap();
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            if client.send(Message::text(message.clone())).await.is_err() {
                // If sending the message fails, the client has probably disconnected
                println!("Failed to send message to client");
            }
        }
    }
}

/// WebSocket handler for the preview WebSocket route
pub async fn preview_ws_handler(ws: warp::ws::Ws, manager: PreviewManager) -> impl Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for sending updates to the preview pane
pub fn send_preview_update_route(manager: PreviewManager) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path("update_preview")
        .and(warp::body::json())
        .and(with_manager(manager))
        .map(|update: PreviewUpdate, manager: PreviewManager| {
            tokio::spawn(async move {
                manager.broadcast_update(update).await;
            });
            warp::reply::json(&"Preview updated")
        })
}

/// Helper function to pass the PreviewManager to the route
fn with_manager(manager: PreviewManager) -> impl Filter<Extract = (PreviewManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example of how to create the server with WebSocket and preview update routes
#[tokio::main]
async fn main() {
    let preview_manager = PreviewManager::new();

    // WebSocket route for live preview
    let preview_ws_route = warp::path("preview_ws")
        .and(warp::ws())
        .and(with_manager(preview_manager.clone()))
        .and_then(preview_ws_handler);

    // Route for sending updates to the preview pane
    let update_route = send_preview_update_route(preview_manager.clone());

    // Serve both routes
    let routes = preview_ws_route.or(update_route);

    println!("Server running on ws://localhost:3030/preview_ws and http://localhost:3030/update_preview");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
