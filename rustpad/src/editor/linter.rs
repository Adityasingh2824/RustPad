use std::process::Command;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: String, // e.g., "error", "warning"
}

type LinterStore = Arc<Mutex<HashMap<String, Box<dyn Linter + Send>>>>;

/// Trait to define common linter functionality
pub trait Linter {
    fn lint_code(&self, code: &str) -> Vec<LintError>;
}

/// Linter for Rust using `cargo check`
pub struct RustLinter;

impl Linter for RustLinter {
    fn lint_code(&self, code: &str) -> Vec<LintError> {
        let mut errors = Vec::new();

        // Write code to a temporary file and run `cargo check` or another Rust linter tool.
        let output = Command::new("cargo")
            .arg("check")
            .output()
            .expect("Failed to execute cargo check");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                if let Some(error) = parse_rust_error(line) {
                    errors.push(error);
                }
            }
        }

        errors
    }
}

/// Linter for JavaScript using ESLint
pub struct JavaScriptLinter;

impl Linter for JavaScriptLinter {
    fn lint_code(&self, code: &str) -> Vec<LintError> {
        let mut errors = Vec::new();

        // Run ESLint as an external command
        let output = Command::new("eslint")
            .arg("--stdin")
            .output()
            .expect("Failed to execute ESLint");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                if let Some(error) = parse_js_error(line) {
                    errors.push(error);
                }
            }
        }

        errors
    }
}

/// Linter for Python using Pylint
pub struct PythonLinter;

impl Linter for PythonLinter {
    fn lint_code(&self, code: &str) -> Vec<LintError> {
        let mut errors = Vec::new();

        // Run Pylint as an external command
        let output = Command::new("pylint")
            .arg("--from-stdin")
            .output()
            .expect("Failed to execute Pylint");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                if let Some(error) = parse_python_error(line) {
                    errors.push(error);
                }
            }
        }

        errors
    }
}

/// Initializes available linters for various languages
pub fn initialize_linters() -> LinterStore {
    let mut linters: HashMap<String, Box<dyn Linter + Send>> = HashMap::new();
    linters.insert("rust".to_string(), Box::new(RustLinter));
    linters.insert("javascript".to_string(), Box::new(JavaScriptLinter));
    linters.insert("python".to_string(), Box::new(PythonLinter));
    
    Arc::new(Mutex::new(linters))
}

/// Lints code based on the selected language
pub fn lint_code(language: &str, code: &str, linter_store: LinterStore) -> Vec<LintError> {
    let linters = linter_store.lock().unwrap();
    
    if let Some(linter) = linters.get(language) {
        linter.lint_code(code)
    } else {
        vec![]
    }
}

/// Parses Rust linter errors from `cargo check`
fn parse_rust_error(line: &str) -> Option<LintError> {
    // Simplified example of error parsing
    if line.contains("error") {
        Some(LintError {
            line: 1, // Simplify this as an example
            column: 1,
            message: line.to_string(),
            severity: "error".to_string(),
        })
    } else {
        None
    }
}

/// Parses JavaScript linter errors from ESLint
fn parse_js_error(line: &str) -> Option<LintError> {
    // Simplified example of error parsing
    if line.contains("error") || line.contains("warning") {
        Some(LintError {
            line: 1, // Simplify this as an example
            column: 1,
            message: line.to_string(),
            severity: if line.contains("error") { "error".to_string() } else { "warning".to_string() },
        })
    } else {
        None
    }
}

/// Parses Python linter errors from Pylint
fn parse_python_error(line: &str) -> Option<LintError> {
    // Simplified example of error parsing
    if line.contains("error") || line.contains("warning") {
        Some(LintError {
            line: 1, // Simplify this as an example
            column: 1,
            message: line.to_string(),
            severity: if line.contains("error") { "error".to_string() } else { "warning".to_string() },
        })
    } else {
        None
    }
}
