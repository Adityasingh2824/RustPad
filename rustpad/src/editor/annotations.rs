use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    pub user: String,
    pub content: String,
    pub line_number: usize,
    pub timestamp: String,
}

type Annotations = Arc<Mutex<HashMap<usize, Vec<Annotation>>>>; // Keyed by line number
type AnnotationClients = Arc<Mutex<Vec<warp::ws::WebSocket>>>;

/// Manages the inline annotations and provides real-time updates to collaborators
pub struct AnnotationManager {
    annotations: Annotations,
    clients: AnnotationClients,
}

impl AnnotationManager {
    /// Creates a new AnnotationManager with an empty annotation map
    pub fn new() -> Self {
        Self {
            annotations: Arc::new(Mutex::new(HashMap::new())),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers a new WebSocket client for receiving annotation updates
    pub async fn register_client(&self, socket: WebSocket) {
        let (mut ws_tx, mut ws_rx) = socket.split();

        {
            let mut clients = self.clients.lock().unwrap();
            clients.push(ws_tx.clone());
        }

        // Send existing annotations to the new client
        let annotations = self.annotations.lock().unwrap().clone();
        let serialized_annotations = serde_json::to_string(&annotations).unwrap();
        if ws_tx.send(Message::text(serialized_annotations)).await.is_err() {
            println!("Failed to send annotations to client");
        }

        // Listen for incoming annotation messages
        while let Some(result) = ws_rx.next().await {
            if let Ok(message) = result {
                if message.is_text() {
                    let annotation: Annotation = serde_json::from_str(message.to_str().unwrap()).unwrap();
                    self.add_annotation(annotation.clone()).await;
                    self.broadcast_annotation(annotation).await;
                }
            }
        }

        // Remove the WebSocket client when it disconnects
        {
            let mut clients = self.clients.lock().unwrap();
            clients.retain(|client| !client.is_closed());
        }
    }

    /// Adds a new annotation to the map and associates it with a line number
    pub async fn add_annotation(&self, annotation: Annotation) {
        let mut annotations = self.annotations.lock().unwrap();
        annotations.entry(annotation.line_number).or_insert_with(Vec::new).push(annotation);
    }

    /// Broadcasts a new annotation to all connected clients
    pub async fn broadcast_annotation(&self, annotation: Annotation) {
        let message = serde_json::to_string(&annotation).unwrap();
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            if client.send(Message::text(message.clone())).await.is_err() {
                println!("Failed to send annotation to client");
            }
        }
    }

    /// Retrieves annotations for a specific line number
    pub fn get_annotations_for_line(&self, line_number: usize) -> Vec<Annotation> {
        let annotations = self.annotations.lock().unwrap();
        annotations.get(&line_number).cloned().unwrap_or_default()
    }
}

/// WebSocket handler for annotations
pub async fn annotation_ws_handler(ws: warp::ws::Ws, manager: AnnotationManager) -> impl warp::Reply {
    ws.on_upgrade(move |socket| manager.register_client(socket))
}

/// Route for WebSocket annotations
pub fn annotation_route(manager: AnnotationManager) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("annotation_ws")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(annotation_ws_handler)
}

/// Helper function to pass the AnnotationManager to the route
fn with_manager(manager: AnnotationManager) -> impl warp::Filter<Extract = (AnnotationManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example of how to set up the server with WebSocket routes for annotations
#[tokio::main]
async fn main() {
    let annotation_manager = AnnotationManager::new();

    // WebSocket route for annotations
    let annotation_ws_route = annotation_route(annotation_manager.clone());

    // Start the server
    println!("Annotation server running on ws://localhost:3030/annotation_ws");
    warp::serve(annotation_ws_route).run(([127, 0, 0, 1], 3030)).await;
}
