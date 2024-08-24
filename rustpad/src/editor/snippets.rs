use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Snippet {
    pub language: String,
    pub trigger: String,
    pub content: String,
}

pub struct SnippetManager {
    predefined_snippets: HashMap<String, Vec<Snippet>>,  // Stores predefined snippets by language
    user_snippets_dir: PathBuf,                          // Directory for user-defined snippets
}

impl SnippetManager {
    /// Creates a new SnippetManager
    pub fn new(user_snippets_dir: &str) -> io::Result<Self> {
        let dir = PathBuf::from(user_snippets_dir);
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(Self {
            predefined_snippets: HashMap::new(),
            user_snippets_dir: dir,
        })
    }

    /// Adds a predefined snippet
    pub fn add_predefined_snippet(&mut self, snippet: Snippet) {
        self.predefined_snippets
            .entry(snippet.language.clone())
            .or_default()
            .push(snippet);
    }

    /// Loads user-defined snippets from the filesystem
    pub fn load_user_snippets(&self) -> io::Result<HashMap<String, Vec<Snippet>>> {
        let mut user_snippets: HashMap<String, Vec<Snippet>> = HashMap::new();

        for entry in fs::read_dir(&self.user_snippets_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(snippet) = Self::load_snippet_from_file(&path)? {
                    user_snippets
                        .entry(snippet.language.clone())
                        .or_default()
                        .push(snippet);
                }
            }
        }

        Ok(user_snippets)
    }

    /// Saves a user-defined snippet to the filesystem
    pub fn save_user_snippet(&self, snippet: Snippet) -> io::Result<()> {
        let snippet_file = self.user_snippets_dir.join(format!("{}.snippet", snippet.trigger));
        let mut file = fs::File::create(snippet_file)?;
        writeln!(file, "{}", snippet.content)?;
        Ok(())
    }

    /// Retrieves a snippet by its trigger
    pub fn get_snippet(&self, language: &str, trigger: &str) -> Option<&Snippet> {
        self.predefined_snippets
            .get(language)
            .and_then(|snippets| snippets.iter().find(|snippet| snippet.trigger == trigger))
    }

    /// Loads a snippet from a file (user-defined snippet)
    fn load_snippet_from_file(path: &Path) -> io::Result<Option<Snippet>> {
        if let Some(file_stem) = path.file_stem() {
            let trigger = file_stem.to_string_lossy().to_string();
            let content = fs::read_to_string(path)?;
            // Assuming the file is named as `<language>_<trigger>.snippet`
            if let Some((language, _)) = trigger.split_once('_') {
                return Ok(Some(Snippet {
                    language: language.to_string(),
                    trigger: trigger.clone(),
                    content,
                }));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predefined_snippets() {
        let mut snippet_manager = SnippetManager::new("./snippets_test").unwrap();
        let snippet = Snippet {
            language: "rust".to_string(),
            trigger: "fn_main".to_string(),
            content: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
        };

        snippet_manager.add_predefined_snippet(snippet.clone());

        let retrieved = snippet_manager.get_snippet("rust", "fn_main").unwrap();
        assert_eq!(retrieved.content, snippet.content);
    }

    #[test]
    fn test_save_and_load_user_snippet() {
        let snippet_manager = SnippetManager::new("./snippets_test").unwrap();
        let snippet = Snippet {
            language: "rust".to_string(),
            trigger: "fn_test".to_string(),
            content: "fn test() {\n    assert!(true);\n}".to_string(),
        };

        snippet_manager.save_user_snippet(snippet.clone()).unwrap();

        let user_snippets = snippet_manager.load_user_snippets().unwrap();
        let loaded_snippet = user_snippets
            .get("rust")
            .unwrap()
            .iter()
            .find(|s| s.trigger == "fn_test")
            .unwrap();

        assert_eq!(loaded_snippet.content, snippet.content);

        // Clean up
        fs::remove_file("./snippets_test/fn_test.snippet").unwrap();
    }
}
