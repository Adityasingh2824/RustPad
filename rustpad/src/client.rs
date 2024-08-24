use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::ws::Message;
use tokio::sync::mpsc;

/// Type alias for the shared state containing the list of connected clients.
/// The `Clients` is an `Arc<Mutex<HashMap<String, Client>>>` to allow safe shared access.
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

/// Represents a connected client.
/// Each client has an ID (usually a UUID) and a WebSocket sender.
#[derive(Debug)]
pub struct Client {
    pub id: String,
    pub sender: Option<mpsc::UnboundedSender<Message>>, // Unbounded sender for WebSocket messages
}

impl Client {
    /// Creates a new client with the given ID and WebSocket sender.
    pub fn new(id: &str, sender: mpsc::UnboundedSender<Message>) -> Self {
        Client {
            id: id.to_string(),
            sender: Some(sender),
        }
    }

    /// Disconnects the client by setting its sender to `None`.
    pub fn disconnect(&mut self) {
        self.sender = None;
    }
}

/// Adds a client to the list of connected clients.
pub fn add_client(clients: Clients, id: String, client: Client) {
    clients.lock().unwrap().insert(id, client);
}

/// Removes a client from the list of connected clients by its ID.
pub fn remove_client(clients: Clients, id: &str) {
    clients.lock().unwrap().remove(id);
}

/// Broadcasts a message to all connected clients.
pub fn broadcast_message(clients: Clients, message: &str) {
    let clients_guard = clients.lock().unwrap();
    
    // Send the message to each connected client
    for (_id, client) in clients_guard.iter() {
        if let Some(sender) = &client.sender {
            if let Err(e) = sender.send(Message::text(message.clone())) {
                eprintln!("Failed to send message to client: {}", e);
            }
        }
    }
}

/// Returns the number of connected clients.
pub fn get_client_count(clients: Clients) -> usize {
    clients.lock().unwrap().len()
}

/// Lists all connected clients' IDs.
pub fn list_clients(clients: Clients) -> Vec<String> {
    clients.lock().unwrap().keys().cloned().collect()
}
