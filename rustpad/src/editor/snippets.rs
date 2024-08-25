use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub name: String,       // The name of the snippet (e.g., "for-loop")
    pub description: String, // A short description of the snippet
    pub content: String,    // The actual code snippet
}

impl Snippet {
    pub fn new(name: &str, description: &str, content: &str) -> Self {
        Snippet {
            name: name.to_string(),
            description: description.to_string(),
            content: content.to_string(),
        }
    }
}

// Store for predefined and user-defined snippets.
type SnippetStore = Arc<Mutex<HashMap<String, Snippet>>>;

/// Adds a new snippet to the store.
pub fn add_snippet(store: SnippetStore, snippet: Snippet) -> Result<(), String> {
    let mut snippets = store.lock().unwrap();
    
    if snippets.contains_key(&snippet.name) {
        return Err("A snippet with this name already exists.".to_string());
    }

    snippets.insert(snippet.name.clone(), snippet);
    Ok(())
}

/// Updates an existing snippet in the store.
pub fn update_snippet(store: SnippetStore, name: &str, new_content: &str) -> Result<(), String> {
    let mut snippets = store.lock().unwrap();
    
    if let Some(snippet) = snippets.get_mut(name) {
        snippet.content = new_content.to_string();
        Ok(())
    } else {
        Err("Snippet not found.".to_string())
    }
}

/// Deletes a snippet from the store.
pub fn delete_snippet(store: SnippetStore, name: &str) -> Result<(), String> {
    let mut snippets = store.lock().unwrap();

    if snippets.remove(name).is_some() {
        Ok(())
    } else {
        Err("Snippet not found.".to_string())
    }
}

/// Retrieves a snippet by name.
pub fn get_snippet(store: SnippetStore, name: &str) -> Option<Snippet> {
    let snippets = store.lock().unwrap();
    snippets.get(name).cloned()
}

/// Lists all snippets.
pub fn list_snippets(store: SnippetStore) -> Vec<Snippet> {
    let snippets = store.lock().unwrap();
    snippets.values().cloned().collect()
}

/// Initializes the store with predefined snippets.
pub fn initialize_snippets(store: SnippetStore) {
    let predefined_snippets = vec![
        Snippet::new(
            "for-loop",
            "A basic for-loop in Rust",
            "for i in 0..10 {\n    println!(\"{}\", i);\n}",
        ),
        Snippet::new(
            "if-else",
            "An if-else conditional in Rust",
            "if condition {\n    // do something\n} else {\n    // do something else\n}",
        ),
        Snippet::new(
            "function",
            "A basic function in Rust",
            "fn my_function() -> i32 {\n    // function body\n    return 42;\n}",
        ),
    ];

    let mut snippets = store.lock().unwrap();
    for snippet in predefined_snippets {
        snippets.insert(snippet.name.clone(), snippet);
    }
}
