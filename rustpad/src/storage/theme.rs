use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

// A struct representing a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background_color: String,
    pub text_color: String,
    // You can add more properties like syntax colors, font styles, etc.
}

// A type alias for storing multiple themes
pub type Themes = Arc<Mutex<HashMap<String, Theme>>>;

// Function to initialize default themes
pub fn initialize_themes() -> Themes {
    let mut themes = HashMap::new();

    // Adding some default themes
    themes.insert(
        "Light".to_string(),
        Theme {
            name: "Light".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#000000".to_string(),
        },
    );

    themes.insert(
        "Dark".to_string(),
        Theme {
            name: "Dark".to_string(),
            background_color: "#282a36".to_string(),
            text_color: "#f8f8f2".to_string(),
        },
    );

    Arc::new(Mutex::new(themes))
}

// Function to get a theme by its name
pub fn get_theme(themes: Themes, theme_name: &str) -> Option<Theme> {
    let themes = themes.lock().unwrap();
    themes.get(theme_name).cloned()
}

// Function to set or override a theme
pub fn set_theme(themes: Themes, new_theme: Theme) -> Result<(), &'static str> {
    let mut themes = themes.lock().unwrap();
    themes.insert(new_theme.name.clone(), new_theme);
    Ok(())
}

