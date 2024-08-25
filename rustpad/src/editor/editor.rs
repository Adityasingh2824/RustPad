use crate::editor::state::EditorState;
use crate::editor::events::{InputEvent, CursorMove};
use crate::editor::version_control::VersionControl;
use crate::networking::peer_sync::PeerSync;

/// `Editor` is the core structure that manages text input, cursor position,
/// document state, and interactions with other modules like version control and peer sync.
pub struct Editor {
    pub state: EditorState,
    pub version_control: VersionControl,
    pub peer_sync: PeerSync,
}

impl Editor {
    /// Creates a new instance of the editor with a fresh state.
    pub fn new() -> Self {
        Self {
            state: EditorState::new(),
            version_control: VersionControl::new(),
            peer_sync: PeerSync::new(),
        }
    }

    /// Handles text insertion into the document. Updates the document state,
    /// cursor position, and synchronization with peers.
    pub fn insert_text(&mut self, text: &str) {
        // Update the document state by inserting the text
        self.state.insert_text(text);

        // Track this change in version control
        self.version_control.track_change(&self.state);

        // Sync the change with peers
        self.peer_sync.broadcast_change(&self.state);
    }

    /// Handles text deletion from the document.
    pub fn delete_text(&mut self, start: usize, end: usize) {
        // Update the document state by deleting the text
        self.state.delete_text(start, end);

        // Track this change in version control
        self.version_control.track_change(&self.state);

        // Sync the change with peers
        self.peer_sync.broadcast_change(&self.state);
    }

    /// Moves the cursor based on user input and updates the editor state.
    pub fn move_cursor(&mut self, position: usize) {
        self.state.move_cursor(position);

        // Optionally broadcast cursor movement to peers (for collaborative cursor tracking)
        self.peer_sync.broadcast_cursor(&self.state);
    }

    /// Handles input events like character typing, backspace, or delete.
    pub fn handle_input_event(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::InsertText(text) => {
                self.insert_text(&text);
            }
            InputEvent::DeleteText(start, end) => {
                self.delete_text(start, end);
            }
            InputEvent::MoveCursor(cursor_move) => {
                self.move_cursor(cursor_move as usize);
            }
            InputEvent::Undo => {
                self.undo();
            }
            InputEvent::Redo => {
                self.redo();
            }
        }
    }

    /// Undo the last change by retrieving a previous state from version control.
    pub fn undo(&mut self) {
        if let Some(previous_state) = self.version_control.undo(&self.state) {
            self.state = previous_state;

            // Sync the reverted state with peers
            self.peer_sync.broadcast_change(&self.state);
        }
    }

    /// Redo the last undone change by retrieving the next state from version control.
    pub fn redo(&mut self) {
        if let Some(next_state) = self.version_control.redo(&self.state) {
            self.state = next_state;

            // Sync the redone state with peers
            self.peer_sync.broadcast_change(&self.state);
        }
    }

    /// Gets the current state of the editor, useful for rendering and synchronization.
    pub fn get_state(&self) -> &EditorState {
        &self.state
    }
}
