use crate::editor::state::EditorState;

/// `InputHandler` handles user input and updates the `EditorState`.
pub struct InputHandler;

impl InputHandler {
    /// Creates a new `InputHandler` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Processes keyboard input events and updates the editor state accordingly.
    /// Supports inserting text, moving the cursor, deleting text, and handling special keys.
    pub fn handle_input(&self, input_event: InputEvent, state: &mut EditorState) {
        match input_event {
            InputEvent::CharacterInput(character) => {
                state.insert_text(&character);
            }
            InputEvent::Backspace => {
                state.delete_character_before_cursor();
            }
            InputEvent::Delete => {
                state.delete_character_at_cursor();
            }
            InputEvent::CursorLeft => {
                state.move_cursor_left();
            }
            InputEvent::CursorRight => {
                state.move_cursor_right();
            }
            InputEvent::CursorUp => {
                state.move_cursor_up();
            }
            InputEvent::CursorDown => {
                state.move_cursor_down();
            }
            InputEvent::Enter => {
                state.insert_newline();
            }
            InputEvent::Tab => {
                state.insert_text("\t");
            }
            InputEvent::Copy => {
                state.copy_selected_text();
            }
            InputEvent::Cut => {
                state.cut_selected_text();
            }
            InputEvent::Paste(pasted_text) => {
                state.insert_text(&pasted_text);
            }
        }
    }
}

/// Enum representing various types of input events that the editor can handle.
pub enum InputEvent {
    /// A single character input by the user (e.g., typing 'a', 'b', etc.).
    CharacterInput(String),

    /// Backspace key pressed (delete the character before the cursor).
    Backspace,

    /// Delete key pressed (delete the character at the cursor).
    Delete,

    /// Move the cursor left.
    CursorLeft,

    /// Move the cursor right.
    CursorRight,

    /// Move the cursor up.
    CursorUp,

    /// Move the cursor down.
    CursorDown,

    /// Enter key pressed (insert a new line).
    Enter,

    /// Tab key pressed (insert a tab character).
    Tab,

    /// Copy the selected text.
    Copy,

    /// Cut the selected text.
    Cut,

    /// Paste text from the clipboard.
    Paste(String),
}
