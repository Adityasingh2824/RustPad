use warp::ws::{Message, WebSocket};
use warp::{Filter};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid; // For generating unique client IDs

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DocumentUpdate {
    content: String,
    user: String,
}

type Clients = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    // Shared state: document and list of connected clients
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Create a broadcast channel for real-time collaboration
    let (tx, _rx) = broadcast::channel::<DocumentUpdate>(100);

    // Serve static files (HTML, CSS, JS)
    let static_files = warp::fs::dir("static");

    // WebSocket route for real-time collaboration
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_broadcast(tx.clone()))
        .map(|ws: warp::ws::Ws, clients, tx| {
            ws.on_upgrade(move |socket| handle_socket(socket, clients, tx))
        });

    // Combine routes: static files and WebSocket
    let routes = static_files.or(ws_route);

    // Start the server
    println!("Server running on http://localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

// Handler for WebSocket connections
async fn handle_socket(socket: WebSocket, clients: Clients, tx: broadcast::Sender<DocumentUpdate>) {
    let client_id = Uuid::new_v4().to_string(); // Generate unique client ID
    let (client_ws_tx, mut client_ws_rx) = socket.split();

    // Channel to send messages to the client
    let (sender, mut receiver) = mpsc::unbounded_channel();
    
    // Add the client to the list
    clients.lock().unwrap().insert(client_id.clone(), sender);

    // Wrap the WebSocket sender in an Arc<Mutex> for safe sharing between tasks
    let client_ws_tx = Arc::new(tokio::sync::Mutex::new(client_ws_tx));

    // Task to receive messages from the broadcast channel and send to WebSocket
    let send_task = {
        let client_ws_tx = client_ws_tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(update) = rx.recv().await {
                let message = serde_json::to_string(&update).unwrap();
                if client_ws_tx.lock().await.send(Message::text(message)).await.is_err() {
                    break; // Client disconnected
                }
            }
        })
    };

    // Task to receive messages from the WebSocket
    let recv_task = tokio::spawn(async move {
        while let Some(result) = client_ws_rx.next().await {
            if let Ok(message) = result {
                if let Ok(text) = message.to_str() {
                    let update: DocumentUpdate = serde_json::from_str(text).unwrap();
                    println!("Received update from {}: {}", update.user, update.content);
                    
                    // Broadcast the update to other clients
                    let _ = tx.send(update.clone());
                }
            }
        }
    });

    // Task to forward messages from the internal channel to the WebSocket
    let forward_task = {
        let client_ws_tx = client_ws_tx.clone();
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                if client_ws_tx.lock().await.send(msg).await.is_err() {
                    break; // Client disconnected
                }
            }
        })
    };

    // Wait for either send_task, recv_task, or forward_task to complete
    tokio::select! {
        _ = send_task => (),
        _ = recv_task => (),
        _ = forward_task => (),
    }

    // Remove the client from the list when the connection is closed
    clients.lock().unwrap().remove(&client_id);
}

// Utility functions to pass the state around
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_broadcast(tx: broadcast::Sender<DocumentUpdate>) -> impl Filter<Extract = (broadcast::Sender<DocumentUpdate>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tx.clone())
}
