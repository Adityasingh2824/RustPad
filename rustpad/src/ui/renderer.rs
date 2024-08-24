use crate::editor::state::EditorState;
use crate::editor::syntax_highlighting::HighlightedRegion;

/// `Renderer` is responsible for rendering the text, syntax highlighting, and cursor to the UI.
pub struct Renderer;

impl Renderer {
    /// Creates a new `Renderer` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Renders the text and highlighted syntax to the UI. This method is agnostic to the
    /// specific platform (web or desktop) and assumes the caller will handle the final rendering.
    pub fn render(&self, state: &EditorState) -> Vec<RenderedLine> {
        let mut rendered_lines = Vec::new();

        // Iterate through each line in the document, applying syntax highlighting
        for (line_index, line) in state.get_text().lines().enumerate() {
            let highlighted_regions = state.get_highlighted_regions_for_line(line_index);
            let rendered_line = self.render_line(line, highlighted_regions);

            rendered_lines.push(rendered_line);
        }

        rendered_lines
    }

    /// Renders a single line of text, applying any highlighted regions.
    fn render_line(&self, line: &str, highlighted_regions: Vec<HighlightedRegion>) -> RenderedLine {
        let mut rendered_line = RenderedLine::new();

        let mut current_position = 0;
        for region in highlighted_regions {
            // Get the unhighlighted text before the highlighted region
            if current_position < region.start {
                let unhighlighted_text = &line[current_position..region.start];
                rendered_line.add_segment(RenderedSegment {
                    text: unhighlighted_text.to_string(),
                    style: None, // No special style
                });
            }

            // Get the highlighted text within the region
            let highlighted_text = &line[region.start..region.end];
            rendered_line.add_segment(RenderedSegment {
                text: highlighted_text.to_string(),
                style: Some(region.style.clone()), // Apply the style from the syntax highlighter
            });

            current_position = region.end;
        }

        // Add any remaining unhighlighted text after the last region
        if current_position < line.len() {
            let unhighlighted_text = &line[current_position..];
            rendered_line.add_segment(RenderedSegment {
                text: unhighlighted_text.to_string(),
                style: None,
            });
        }

        rendered_line
    }
}

/// Represents a line of rendered text, consisting of segments with optional styles.
pub struct RenderedLine {
    segments: Vec<RenderedSegment>,
}

impl RenderedLine {
    /// Creates a new, empty `RenderedLine`.
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Adds a segment of text to the rendered line.
    pub fn add_segment(&mut self, segment: RenderedSegment) {
        self.segments.push(segment);
    }

    /// Returns the segments of the rendered line for further processing or rendering.
    pub fn get_segments(&self) -> &Vec<RenderedSegment> {
        &self.segments
    }
}

/// Represents a segment of rendered text with an optional style (for syntax highlighting).
#[derive(Clone)]
pub struct RenderedSegment {
    pub text: String,
    pub style: Option<HighlightedStyle>,
}

/// Represents the style applied to a highlighted region, such as color and font style.
#[derive(Clone)]
pub struct HighlightedStyle {
    pub color: String,  // Hex color code (e.g., "#ff0000" for red)
    pub bold: bool,
    pub italic: bool,
}
