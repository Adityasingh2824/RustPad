use serde::{Deserialize, Serialize};
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use warp::{Filter, Reply};

// Represents a file or folder in the file tree
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub children: Option<Vec<FileNode>>,
}

/// Manages the file tree UI and sidebar
pub struct FileManager {
    base_dir: PathBuf,
}

impl FileManager {
    /// Creates a new FileManager with the specified base directory
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: PathBuf::from(base_dir),
        }
    }

    /// Generates a file tree structure from the base directory
    pub fn generate_file_tree(&self) -> io::Result<FileNode> {
        let base_dir = self.base_dir.clone();
        let root = self.build_file_tree(base_dir)?;
        Ok(root)
    }

    /// Builds the file tree recursively
    fn build_file_tree(&self, path: PathBuf) -> io::Result<FileNode> {
        let metadata = fs::metadata(&path)?;

        let mut node = FileNode {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            is_directory: metadata.is_dir(),
            children: None,
        };

        if metadata.is_dir() {
            let entries = fs::read_dir(&path)?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| self.build_file_tree(entry.path()).ok())
                .collect::<Vec<FileNode>>();
            node.children = Some(entries);
        }

        Ok(node)
    }

    /// Lists all files and directories in the base directory as a tree structure
    pub fn list_files(&self) -> io::Result<Vec<FileNode>> {
        let file_tree = self.generate_file_tree()?;
        Ok(file_tree.children.unwrap_or_default())
    }

    /// Deletes a file or directory in the base directory
    pub fn delete_file(&self, file_path: &str) -> io::Result<()> {
        let path = self.base_dir.join(file_path);
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Renames a file or directory in the base directory
    pub fn rename_file(&self, old_path: &str, new_name: &str) -> io::Result<FileNode> {
        let old_full_path = self.base_dir.join(old_path);
        let new_full_path = old_full_path.with_file_name(new_name);
        fs::rename(&old_full_path, &new_full_path)?;

        // Return the updated node
        self.build_file_tree(new_full_path)
    }
}

/// WebSocket handler for file tree updates
pub async fn file_manager_ws_handler(ws: warp::ws::Ws, manager: FileManager) -> impl warp::Reply {
    ws.on_upgrade(move |socket| handle_file_manager_socket(socket, manager))
}

async fn handle_file_manager_socket(socket: warp::ws::WebSocket, manager: FileManager) {
    let (mut ws_tx, mut ws_rx) = socket.split();

    // Send the initial file tree structure to the connected client
    let file_tree = manager.generate_file_tree().unwrap();
    let serialized_tree = serde_json::to_string(&file_tree).unwrap();
    if ws_tx.send(warp::ws::Message::text(serialized_tree)).await.is_err() {
        return; // Handle error in sending the file tree
    }

    // Listen for file management commands (like renaming, deleting)
    while let Some(Ok(message)) = ws_rx.next().await {
        if let Ok(text) = message.to_str() {
            let cmd: serde_json::Value = serde_json::from_str(text).unwrap();

            if let Some(command) = cmd.get("command") {
                match command.as_str().unwrap() {
                    "delete" => {
                        if let Some(file_path) = cmd.get("file_path") {
                            manager.delete_file(file_path.as_str().unwrap()).unwrap();
                        }
                    }
                    "rename" => {
                        if let Some(old_path) = cmd.get("old_path") {
                            if let Some(new_name) = cmd.get("new_name") {
                                manager.rename_file(old_path.as_str().unwrap(), new_name.as_str().unwrap()).unwrap();
                            }
                        }
                    }
                    _ => {}
                }
            }

            // After executing a command, send the updated file tree
            let updated_tree = manager.generate_file_tree().unwrap();
            let updated_serialized_tree = serde_json::to_string(&updated_tree).unwrap();
            if ws_tx.send(warp::ws::Message::text(updated_serialized_tree)).await.is_err() {
                return; // Handle error in sending the updated file tree
            }
        }
    }
}

/// Route for file tree management WebSocket
pub fn file_manager_route(manager: FileManager) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path("file_manager_ws")
        .and(warp::ws())
        .and(with_manager(manager))
        .and_then(file_manager_ws_handler)
}

/// Helper function to pass the FileManager to the route
fn with_manager(manager: FileManager) -> impl Filter<Extract = (FileManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Example main function for setting up the file manager WebSocket server
#[tokio::main]
async fn main() {
    let file_manager = FileManager::new("project_files");

    // WebSocket route for file manager
    let file_manager_ws_route = file_manager_route(file_manager.clone());

    // Start the server
    println!("File Manager server running on ws://localhost:3030/file_manager_ws");
    warp::serve(file_manager_ws_route).run(([127, 0, 0, 1], 3030)).await;
}
