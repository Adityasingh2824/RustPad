pub mod renderer;
pub mod input_handler;

use crate::editor::state::EditorState;
use crate::editor::syntax_highlighting::SyntaxHighlighter;
use crate::ui::renderer::Renderer;
use crate::ui::input_handler::InputHandler;

/// `UI` is the central module for handling the rendering and user interactions in the editor.
pub struct UI {
    renderer: Renderer,
    input_handler: InputHandler,
    syntax_highlighter: SyntaxHighlighter,
}

impl UI {
    /// Creates a new `UI` instance with the required components.
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            input_handler: InputHandler::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
        }
    }

    /// Runs the main loop for handling input and rendering the editor UI.
    pub fn run(&mut self, editor_state: &mut EditorState) {
        loop {
            // Handle user input and update the editor state
            self.input_handler.handle_input(editor_state);

            // Apply syntax highlighting to the document
            self.syntax_highlighter.highlight(editor_state);

            // Render the updated state to the UI
            self.renderer.render(editor_state);
        }
    }
}
