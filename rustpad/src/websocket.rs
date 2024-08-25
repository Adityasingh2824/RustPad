use futures_util::{StreamExt, SinkExt};
use serde_json;
use tokio::sync::{broadcast, mpsc};
use warp::ws::{Message, WebSocket};
use crate::client::{Clients, Client, add_client, remove_client};
use crate::document::DocumentUpdate;
use crate::utils::{ws_message_to_string, generate_uuid};
use std::sync::{Arc, Mutex};
use crate::sessions::{verify_session, Sessions};  // Ensure the sessions module is properly linkeduse warp::reject::Reject;

/// Custom reject for invalid sessions.
#[derive(Debug)]
struct InvalidSession;
use warp::reject::Reject;

impl Reject for InvalidSession {}

/// Handles the WebSocket connection, including receiving and broadcasting messages.
pub async fn handle_websocket(
    socket: WebSocket,
    clients: Clients,
    tx: broadcast::Sender<DocumentUpdate>,
    sessions: Sessions,
) -> Result<(), warp::Rejection> {
    let client_id = generate_uuid(); // Generate a unique ID for the client

    // Split WebSocket into sender and receiver
    let (mut client_ws_tx, mut client_ws_rx) = socket.split();

    // Verify session and retrieve user information (e.g., client_id or username)
    if !verify_session(&sessions, &client_id).await {
        eprintln!("Invalid session for client: {}", client_id);
        return Err(warp::reject::custom(InvalidSession)); // Reject the connection
    }

    // Channel to send messages to the client asynchronously
    let (sender, mut receiver) = mpsc::unbounded_channel();

    // Add the client to the list of connected clients
    let client = Client::new(&client_id, "username", sender.clone()); // Use appropriate username
    add_client(clients.clone(), client_id.clone(), client);

    // Task to send messages to the WebSocket from the mpsc channel
    let send_task = tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            if client_ws_tx.send(msg).await.is_err() {
                break; // Client disconnected, break the sending task
            }
        }
    });

    // Task to receive messages from the WebSocket
    let recv_task = tokio::spawn(async move {
        while let Some(result) = client_ws_rx.next().await {
            if let Ok(message) = result {
                if let Ok(text) = ws_message_to_string(message) {
                    if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&text) {
                        // Parse the document update and broadcast it
                        if let (Some(content), Some(user)) = (
                            parsed_json.get("content").and_then(|v| v.as_str()), 
                            parsed_json.get("user").and_then(|v| v.as_str())
                        ) {
                            let update = DocumentUpdate::new(content, user);
                            if tx.send(update.clone()).is_err() {
                                break; // Broadcast to clients failed, break the task
                            }
                        }
                    }
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => (),
        _ = recv_task => (),
    }

    // Remove the client when the connection is closed
    remove_client(clients.clone(), &client_id);

    Ok(()) // Ensure this returns ()
}

/// Broadcasts a document update to all connected clients asynchronously.
pub async fn broadcast_update(clients: Clients, update: DocumentUpdate) {
    let message = serde_json::to_string(&update).unwrap();
    let clients_lock = clients.lock().unwrap();
    
    for (_client_id, client) in clients_lock.iter() {
        if let Some(sender) = &client.sender {
            if let Err(e) = sender.send(Message::text(message.clone())) {
                eprintln!("Failed to send message to client: {}", e);
            }
        }
    }
}
