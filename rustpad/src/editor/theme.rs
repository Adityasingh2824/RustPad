use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write};

/// Represents a theme configuration with customizable properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background_color: String,
    pub text_color: String,
    pub cursor_color: String,
    pub selection_color: String,
    pub font_family: String,
    pub font_size: String,
}

/// Manages theme selection and customization
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    active_theme: String,
    custom_themes_dir: String,
}

impl ThemeManager {
    /// Creates a new ThemeManager with predefined themes
    pub fn new(custom_themes_dir: &str) -> io::Result<Self> {
        let mut themes = HashMap::new();

        // Add predefined themes
        themes.insert(
            "Dracula".to_string(),
            Theme {
                name: "Dracula".to_string(),
                background_color: "#282a36".to_string(),
                text_color: "#f8f8f2".to_string(),
                cursor_color: "#ff79c6".to_string(),
                selection_color: "#44475a".to_string(),
                font_family: "Fira Code, monospace".to_string(),
                font_size: "14px".to_string(),
            },
        );

        themes.insert(
            "Monokai".to_string(),
            Theme {
                name: "Monokai".to_string(),
                background_color: "#272822".to_string(),
                text_color: "#f8f8f2".to_string(),
                cursor_color: "#f92672".to_string(),
                selection_color: "#49483e".to_string(),
                font_family: "Fira Code, monospace".to_string(),
                font_size: "14px".to_string(),
            },
        );

        let theme_manager = ThemeManager {
            themes,
            active_theme: "Dracula".to_string(), // Default active theme
            custom_themes_dir: custom_themes_dir.to_string(),
        };

        theme_manager.load_custom_themes()?;

        Ok(theme_manager)
    }

    /// Load custom themes from the filesystem
    fn load_custom_themes(&self) -> io::Result<()> {
        let paths = fs::read_dir(&self.custom_themes_dir)?;

        for path in paths {
            let path = path?;
            let file_content = fs::read_to_string(path.path())?;
            let theme: Theme = serde_json::from_str(&file_content)?;
            self.themes.insert(theme.name.clone(), theme);
        }

        Ok(())
    }

    /// Set the active theme
    pub fn set_active_theme(&mut self, theme_name: &str) -> Option<&Theme> {
        if self.themes.contains_key(theme_name) {
            self.active_theme = theme_name.to_string();
            self.themes.get(theme_name)
        } else {
            None
        }
    }

    /// Get the currently active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.active_theme)
    }

    /// Save a custom theme to the filesystem
    pub fn save_custom_theme(&self, theme: &Theme) -> io::Result<()> {
        let theme_path = format!("{}/{}.json", self.custom_themes_dir, theme.name);
        let mut file = File::create(theme_path)?;
        let serialized_theme = serde_json::to_string_pretty(&theme)?;
        file.write_all(serialized_theme.as_bytes())?;
        Ok(())
    }

    /// Add a custom theme to the theme manager and save it to the filesystem
    pub fn add_custom_theme(&mut self, theme: Theme) -> io::Result<()> {
        self.themes.insert(theme.name.clone(), theme.clone());
        self.save_custom_theme(&theme)
    }

    /// Get the list of available themes
    pub fn get_available_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    /// Modify an existing theme
    pub fn customize_theme(&mut self, theme_name: &str, new_values: ThemeCustomization) -> Option<&Theme> {
        if let Some(theme) = self.themes.get_mut(theme_name) {
            if let Some(background_color) = new_values.background_color {
                theme.background_color = background_color;
            }
            if let Some(text_color) = new_values.text_color {
                theme.text_color = text_color;
            }
            if let Some(cursor_color) = new_values.cursor_color {
                theme.cursor_color = cursor_color;
            }
            if let Some(selection_color) = new_values.selection_color {
                theme.selection_color = selection_color;
            }
            if let Some(font_family) = new_values.font_family {
                theme.font_family = font_family;
            }
            if let Some(font_size) = new_values.font_size {
                theme.font_size = font_size;
            }
            Some(theme)
        } else {
            None
        }
    }
}

/// Represents a set of customizations for an existing theme
#[derive(Debug, Clone)]
pub struct ThemeCustomization {
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub cursor_color: Option<String>,
    pub selection_color: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_active_theme() {
        let mut theme_manager = ThemeManager::new("./custom_themes").unwrap();

        // Set theme to Monokai
        let theme = theme_manager.set_active_theme("Monokai").unwrap();
        assert_eq!(theme.name, "Monokai");

        // Set theme to Dracula
        let theme = theme_manager.set_active_theme("Dracula").unwrap();
        assert_eq!(theme.name, "Dracula");
    }

    #[test]
    fn test_add_custom_theme() {
        let mut theme_manager = ThemeManager::new("./custom_themes").unwrap();
        
        let custom_theme = Theme {
            name: "CustomDark".to_string(),
            background_color: "#000000".to_string(),
            text_color: "#FFFFFF".to_string(),
            cursor_color: "#00FF00".to_string(),
            selection_color: "#333333".to_string(),
            font_family: "Arial, sans-serif".to_string(),
            font_size: "16px".to_string(),
        };

        theme_manager.add_custom_theme(custom_theme.clone()).unwrap();

        // Ensure custom theme was added
        let themes = theme_manager.get_available_themes();
        assert!(themes.contains(&"CustomDark".to_string()));

        // Set and retrieve custom theme
        let theme = theme_manager.set_active_theme("CustomDark").unwrap();
        assert_eq!(theme.background_color, "#000000");
    }

    #[test]
    fn test_customize_theme() {
        let mut theme_manager = ThemeManager::new("./custom_themes").unwrap();
        
        let custom_values = ThemeCustomization {
            background_color: Some("#123456".to_string()),
            text_color: Some("#abcdef".to_string()),
            cursor_color: None,
            selection_color: None,
            font_family: Some("Courier New, monospace".to_string()),
            font_size: Some("18px".to_string()),
        };

        // Customize the Dracula theme
        theme_manager.customize_theme("Dracula", custom_values).unwrap();
        let theme = theme_manager.get_active_theme().unwrap();

        assert_eq!(theme.background_color, "#123456");
        assert_eq!(theme.text_color, "#abcdef");
        assert_eq!(theme.font_family, "Courier New, monospace");
        assert_eq!(theme.font_size, "18px");
    }
}
