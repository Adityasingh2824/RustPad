pub mod editor;
pub mod syntax_highlighting;
pub mod version_control;
pub mod events;
pub mod state;
pub mod diff_engine;
pub mod extensions;


use crate::editor::state::EditorState;
use crate::editor::events::{EventHandler, InputEvent};
use crate::editor::version_control::VersionControl;
use crate::editor::syntax_highlighting::SyntaxHighlighter;
use crate::networking::peer_sync::PeerSync;
use crate::ui::renderer::Renderer;

/// The `Editor` struct encapsulates the entire editor, managing the text state, events, version control,
/// syntax highlighting, and peer-to-peer synchronization for collaborative editing.
pub struct Editor {
    state: EditorState,
    event_handler: EventHandler,
    version_control: VersionControl,
    syntax_highlighter: SyntaxHighlighter,
    peer_sync: PeerSync,
    renderer: Renderer,
}

impl Editor {
    /// Initializes a new `Editor` instance with all the components needed for editing.
    pub fn new() -> Self {
        Self {
            state: EditorState::new(),
            event_handler: EventHandler::new(),
            version_control: VersionControl::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
            peer_sync: PeerSync::new(),
            renderer: Renderer::new(),
        }
    }

    /// Main loop to run the editor, processing events, applying syntax highlighting,
    /// synchronizing with peers, and rendering the updated state.
    pub fn run(&mut self) {
        loop {
            // Poll for user input or synchronization events
            let events = self.event_handler.poll_events();

            // Handle each event (e.g., text input, cursor movement, undo/redo)
            for event in events {
                self.handle_event(event);
            }

            // Apply syntax highlighting to the current state
            self.syntax_highlighter.highlight(&mut self.state);

            // Sync the editor state with peers in real-time
            self.peer_sync.sync(&self.state);

            // Render the updated state to the UI
            self.renderer.render(&self.state);
        }
    }

    /// Handles different types of input events by calling appropriate methods.
    fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::InsertText(text) => {
                self.state.insert_text(&text);
                self.version_control.track_change(&self.state);
                self.peer_sync.broadcast_change(&self.state);
            }
            InputEvent::DeleteText(start, end) => {
                self.state.delete_text(start, end);
                self.version_control.track_change(&self.state);
                self.peer_sync.broadcast_change(&self.state);
            }
            InputEvent::MoveCursor(cursor_move) => {
                self.state.move_cursor(cursor_move);
                // Optionally sync cursor position with peers
                self.peer_sync.broadcast_cursor(&self.state);
            }
            InputEvent::Undo => {
                if let Some(previous_state) = self.version_control.undo(&self.state) {
                    self.state = previous_state;
                    self.peer_sync.broadcast_change(&self.state);
                }
            }
            InputEvent::Redo => {
                if let Some(next_state) = self.version_control.redo(&self.state) {
                    self.state = next_state;
                    self.peer_sync.broadcast_change(&self.state);
                }
            }
        }
    }
}

