#[derive(Clone)]
pub struct EditorState {
    text: String,            // The content of the document
    cursor_position: usize,   // The current cursor position (character index)
    selection_start: Option<usize>, // Optional start of text selection
    selection_end: Option<usize>,   // Optional end of text selection
}

impl EditorState {
    /// Creates a new instance of `EditorState` with an empty document.
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
        }
    }

    /// Returns the entire document text.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Inserts text at the current cursor position, updating the cursor position accordingly.
    pub fn insert_text(&mut self, text: &str) {
        self.text.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();  // Move the cursor forward by the length of the inserted text
    }

    /// Deletes text between the given start and end positions. Updates the cursor position.
    pub fn delete_text(&mut self, start: usize, end: usize) {
        if start < end && end <= self.text.len() {
            self.text.replace_range(start..end, "");  // Remove text between start and end
            self.cursor_position = start;  // Set the cursor to the start of the deleted range
        }
    }

    /// Moves the cursor based on input command or direct position.
    pub fn move_cursor(&mut self, position: usize) {
        self.cursor_position = position.min(self.text.len());
    }

    /// Selects text between the start and end positions.
    pub fn set_selection(&mut self, start: usize, end: usize) {
        self.selection_start = Some(start.min(self.text.len()));
        self.selection_end = Some(end.min(self.text.len()));
    }

    /// Clears the current text selection.
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    /// Returns the current cursor position.
    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Returns the current selection range as a tuple (start, end), or None if no selection.
    pub fn get_selection_range(&self) -> Option<(usize, usize)> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            Some((start, end))
        } else {
            None
        }
    }

    /// Replaces the entire document text with new content.
    pub fn replace_text(&mut self, new_text: String) {
        self.text = new_text;
        self.cursor_position = self.text.len();  // Set the cursor at the end of the new text
        self.clear_selection();  // Clear selection since the document has changed
    }

    /// Applies a synchronization update by replacing a section of the text.
    /// This is used for real-time collaboration to update the editor's state with incoming changes.
    pub fn apply_sync(&mut self, start: usize, end: usize, new_text: &str) {
        self.text.replace_range(start..end, new_text);
        self.cursor_position = start + new_text.len();  // Adjust the cursor after the synced change
    }
}

