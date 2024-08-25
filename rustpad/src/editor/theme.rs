use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Theme structure, holding color values for different parts of the editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String, // Background color
    pub foreground: String, // Foreground text color
    pub keyword_color: String, // Keyword color
    pub string_color: String, // String color
    pub comment_color: String, // Comment color
    // Add more fields as needed for different code parts (functions, variables, etc.)
}

impl Theme {
    pub fn new(name: &str, background: &str, foreground: &str, keyword_color: &str, string_color: &str, comment_color: &str) -> Self {
        Theme {
            name: name.to_string(),
            background: background.to_string(),
            foreground: foreground.to_string(),
            keyword_color: keyword_color.to_string(),
            string_color: string_color.to_string(),
            comment_color: comment_color.to_string(),
        }
    }
}

/// Store for managing available themes and the currently selected theme
type ThemeStore = Arc<Mutex<HashMap<String, Theme>>>;

/// Initializes the store with predefined themes
pub fn initialize_themes() -> ThemeStore {
    let mut themes = HashMap::new();

    // Predefined light theme
    themes.insert(
        "light".to_string(),
        Theme::new(
            "Light",
            "#ffffff",  // Background color
            "#000000",  // Foreground color
            "#0000ff",  // Keyword color (blue)
            "#008000",  // String color (green)
            "#808080",  // Comment color (gray)
        ),
    );

    // Predefined dark theme
    themes.insert(
        "dark".to_string(),
        Theme::new(
            "Dark",
            "#1e1e1e",  // Background color
            "#d4d4d4",  // Foreground color
            "#569cd6",  // Keyword color (blue)
            "#ce9178",  // String color (brownish)
            "#6a9955",  // Comment color (green)
        ),
    );

    Arc::new(Mutex::new(themes))
}

/// Sets a new theme as the current theme
pub fn set_theme(theme_store: ThemeStore, theme_name: &str) -> Result<(), String> {
    let themes = theme_store.lock().unwrap();

    if themes.contains_key(theme_name) {
        Ok(())
    } else {
        Err(format!("Theme '{}' not found.", theme_name))
    }
}

/// Gets the currently selected theme's details
pub fn get_theme(theme_store: ThemeStore, theme_name: &str) -> Option<Theme> {
    let themes = theme_store.lock().unwrap();
    themes.get(theme_name).cloned()
}

/// Adds a custom theme to the store
pub fn add_custom_theme(
    theme_store: ThemeStore,
    theme: Theme,
) -> Result<(), String> {
    let mut themes = theme_store.lock().unwrap();

    if themes.contains_key(&theme.name) {
        Err(format!("A theme with the name '{}' already exists.", theme.name))
    } else {
        themes.insert(theme.name.clone(), theme);
        Ok(())
    }
}

/// Lists all available themes
pub fn list_themes(theme_store: ThemeStore) -> Vec<String> {
    let themes = theme_store.lock().unwrap();
    themes.keys().cloned().collect()
}
