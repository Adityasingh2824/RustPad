use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents a theme, which includes a name and a set of colors.
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>, // Map color names (e.g., "background") to hex codes (e.g., "#FFFFFF")
}

/// Type alias for storing themes in a thread-safe manner.
pub type Themes = Arc<Mutex<HashMap<String, Theme>>>;

/// Initializes the theme store with default themes.
pub fn initialize_themes() -> Themes {
    let mut themes: HashMap<String, Theme> = HashMap::new();

    // Example: Adding a default dark theme
    let mut dark_theme_colors = HashMap::new();
    dark_theme_colors.insert("background".to_string(), "#000000".to_string());
    dark_theme_colors.insert("text".to_string(), "#FFFFFF".to_string());

    let dark_theme = Theme {
        name: "dark".to_string(),
        colors: dark_theme_colors,
    };

    themes.insert(dark_theme.name.clone(), dark_theme);

    // Return the themes wrapped in `Arc<Mutex<>>`
    Arc::new(Mutex::new(themes))
}

/// Retrieves a theme by its name.
pub fn get_theme(themes: Themes, theme_name: &str) -> Option<Theme> {
    let themes = themes.lock().unwrap();
    themes.get(theme_name).cloned()
}

/// Sets a new theme or updates an existing one.
pub fn set_theme(themes: Themes, new_theme: Theme) -> Result<(), &'static str> {
    let mut themes = themes.lock().unwrap();
    themes.insert(new_theme.name.clone(), new_theme);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_management() {
        let themes = initialize_themes();

        // Test retrieving a theme
        let theme = get_theme(themes.clone(), "dark");
        assert!(theme.is_some());
        assert_eq!(theme.unwrap().name, "dark");

        // Test setting a new theme
        let mut new_colors = HashMap::new();
        new_colors.insert("background".to_string(), "#FFFFFF".to_string());
        new_colors.insert("text".to_string(), "#000000".to_string());

        let new_theme = Theme {
            name: "light".to_string(),
            colors: new_colors,
        };

        assert!(set_theme(themes.clone(), new_theme.clone()).is_ok());

        // Ensure the new theme was added
        let theme = get_theme(themes, "light");
        assert!(theme.is_some());
        assert_eq!(theme.unwrap().name, "light");
    }
}
