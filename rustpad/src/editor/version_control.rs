use crate::editor::state::EditorState;
use std::collections::VecDeque;

/// `VersionControl` is responsible for managing the undo/redo stack and tracking
/// changes to the document's state. It allows users to revert to previous states
/// and redo changes after undo operations.
pub struct VersionControl {
    undo_stack: VecDeque<EditorState>,  // Stack to hold states for undo
    redo_stack: VecDeque<EditorState>,  // Stack to hold states for redo
    max_history: usize,                 // Maximum number of states to store
}

impl VersionControl {
    /// Creates a new `VersionControl` instance with a specified history limit.
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history: 100,  // Default max history states
        }
    }

    /// Tracks changes by storing the current state of the editor in the undo stack.
    /// Clears the redo stack since new changes invalidate the redo history.
    pub fn track_change(&mut self, state: &EditorState) {
        if self.undo_stack.len() == self.max_history {
            self.undo_stack.pop_front();  // Remove the oldest state to maintain history limit
        }

        // Push the current state onto the undo stack
        self.undo_stack.push_back(state.clone());

        // Clear the redo stack because a new change invalidates the redo history
        self.redo_stack.clear();
    }

    /// Undoes the last change by reverting to the previous state in the undo stack.
    /// Moves the current state to the redo stack to enable redoing the action.
    pub fn undo(&mut self, current_state: &EditorState) -> Option<EditorState> {
        if let Some(previous_state) = self.undo_stack.pop_back() {
            // Move the current state to the redo stack
            self.redo_stack.push_back(current_state.clone());

            // Return the previous state for reverting
            return Some(previous_state);
        }
        None
    }

    /// Redoes the last undone change by restoring the next state in the redo stack.
    /// Moves the current state back to the undo stack.
    pub fn redo(&mut self, current_state: &EditorState) -> Option<EditorState> {
        if let Some(next_state) = self.redo_stack.pop_back() {
            // Move the current state back to the undo stack
            self.undo_stack.push_back(current_state.clone());

            // Return the next state for redoing
            return Some(next_state);
        }
        None
    }

    /// Sets a limit for the maximum number of states stored in history.
    pub fn set_max_history(&mut self, max_history: usize) {
        self.max_history = max_history;
    }

    /// Clears all stored history for undo and redo actions.
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

