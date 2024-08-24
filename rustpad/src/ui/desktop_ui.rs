use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Window};
use crate::editor::state::EditorState;
use crate::editor::syntax_highlighting::SyntaxHighlighter;
use crate::networking::peer_sync::PeerSync;
use crate::networking::websocket::WebSocketClient;

pub struct DesktopUI {
    state: EditorState,
    syntax_highlighter: SyntaxHighlighter,
    peer_sync: PeerSync,
    websocket_client: Option<WebSocketClient>,
}

impl DesktopUI {
    /// Creates a new `DesktopUI` instance with the necessary components.
    pub fn new() -> Self {
        Self {
            state: EditorState::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
            peer_sync: PeerSync::new(),
            websocket_client: Some(WebSocketClient::new("ws://localhost:8080")),
        }
    }

    /// Runs the main event loop for the desktop application, handling input, synchronization, and rendering.
    pub fn run(&mut self, window: Window) {
        let cloned_window = window.clone();

        // Listen for changes in the editor state
        cloned_window.listen("text_input", move |event| {
            if let Some(text) = event.payload() {
                self.handle_input(text);
                self.render(&window);
            }
        });

        // Continuously listen for WebSocket messages and apply them to the editor state.
        self.listen_for_websocket_messages(&window);
    }

    /// Handles text input from the user and updates the editor state accordingly.
    fn handle_input(&mut self, input: &str) {
        self.state.insert_text(input);
        self.syntax_highlighter.highlight(&mut self.state);

        if let Some(ref mut ws_client) = self.websocket_client {
            let _ = self.peer_sync.broadcast_change(&self.state, ws_client);
        }
    }

    /// Renders the updated editor state to the desktop UI.
    fn render(&self, window: &Window) {
        // Get the updated text with syntax highlighting
        let highlighted_text = self.state.get_highlighted_lines().join("\n");

        // Send the highlighted text to the frontend (in Tauri, this is the webview)
        window.emit("render_text", highlighted_text).expect("Failed to render text");
    }

    /// Continuously listens for incoming WebSocket messages and applies them to the editor state.
    fn listen_for_websocket_messages(&mut self, window: &Window) {
        if let Some(ref mut ws_client) = self.websocket_client {
            let cloned_window = window.clone();
            // Spawn a task to listen for messages
            tauri::async_runtime::spawn(async move {
                while let Some(message) = ws_client.receive_message().await {
                    cloned_window.emit("ws_message_received", message).expect("Failed to emit WebSocket message");
                }
            });
        }
    }
}
