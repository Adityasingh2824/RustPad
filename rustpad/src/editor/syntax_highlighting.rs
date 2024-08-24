use syntect::highlighting::{ThemeSet, HighlightLines, Style, Color};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::easy::HighlightFile;
use syntect::util::{LinesWithEndings};
use crate::editor::state::EditorState;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    theme_name: String,  // Store the current theme name (e.g., "base16-ocean.dark")
    syntax: Option<SyntaxReference>, // Stores the current syntax based on the language
}

impl SyntaxHighlighter {
    /// Creates a new SyntaxHighlighter with the default theme and syntax set.
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines(); // Load the default set of syntaxes
        let theme_set = ThemeSet::load_defaults(); // Load the default set of themes
        let theme_name = "base16-ocean.dark".to_string(); // Set the default theme

        Self {
            syntax_set,
            theme_set,
            theme_name,
            syntax: None,
        }
    }

    /// Sets the programming language syntax for the highlighter (e.g., Rust, Python).
    pub fn set_language(&mut self, file_extension: &str) {
        self.syntax = self.syntax_set.find_syntax_by_extension(file_extension);
    }

    /// Highlights the given text based on the current programming language and theme.
    /// This method will apply syntax highlighting to the EditorState's text.
    pub fn highlight(&self, state: &mut EditorState) {
        if let Some(syntax) = &self.syntax {
            let theme = &self.theme_set.themes[&self.theme_name];
            let mut highlighter = HighlightLines::new(syntax, theme);

            // Get the document text from the editor state
            let lines = state.get_text().lines();

            // Clear previous highlights
            state.clear_highlight();

            // Apply syntax highlighting to each line
            for (line_number, line) in lines.enumerate() {
                let regions = highlighter.highlight_line(line, &self.syntax_set).unwrap();

                // Store the highlighted styles in the editor state
                state.add_highlighted_line(line_number, regions);
            }
        }
    }

    /// Allows switching the theme of the syntax highlighting.
    pub fn set_theme(&mut self, theme_name: &str) {
        if self.theme_set.themes.contains_key(theme_name) {
            self.theme_name = theme_name.to_string();
        }
    }
}

