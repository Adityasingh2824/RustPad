use crate::editor::syntax_highlighting::HighlightedStyle;

/// Represents a color in hex format (e.g., "#FFFFFF" for white).
pub type Color = String;

/// `Theme` represents the color scheme and style settings for the editor's UI.
pub struct Theme {
    pub background_color: Color,
    pub text_color: Color,
    pub keyword_color: Color,
    pub string_color: Color,
    pub comment_color: Color,
    pub cursor_color: Color,
    pub selection_color: Color,
    pub gutter_background_color: Color,
    pub gutter_text_color: Color,
    pub line_highlight_color: Color,
}

impl Theme {
    /// Creates a default light theme.
    pub fn light() -> Self {
        Self {
            background_color: "#FFFFFF".to_string(),
            text_color: "#000000".to_string(),
            keyword_color: "#0000FF".to_string(),
            string_color: "#008000".to_string(),
            comment_color: "#808080".to_string(),
            cursor_color: "#000000".to_string(),
            selection_color: "#ADD8E6".to_string(),
            gutter_background_color: "#F0F0F0".to_string(),
            gutter_text_color: "#000000".to_string(),
            line_highlight_color: "#F0F8FF".to_string(),
        }
    }

    /// Creates a default dark theme.
    pub fn dark() -> Self {
        Self {
            background_color: "#1E1E1E".to_string(),
            text_color: "#DCDCDC".to_string(),
            keyword_color: "#569CD6".to_string(),
            string_color: "#D69D85".to_string(),
            comment_color: "#6A9955".to_string(),
            cursor_color: "#FFFFFF".to_string(),
            selection_color: "#264F78".to_string(),
            gutter_background_color: "#2E2E2E".to_string(),
            gutter_text_color: "#858585".to_string(),
            line_highlight_color: "#333333".to_string(),
        }
    }

    /// Applies the theme to the editor's syntax highlighting styles.
    pub fn apply_syntax_highlighting(&self) -> Vec<HighlightedStyle> {
        vec![
            HighlightedStyle {
                color: self.keyword_color.clone(),
                bold: true,
                italic: false,
            },
            HighlightedStyle {
                color: self.string_color.clone(),
                bold: false,
                italic: false,
            },
            HighlightedStyle {
                color: self.comment_color.clone(),
                bold: false,
                italic: true,
            },
            // Additional syntax styles can be added here...
        ]
    }
}
