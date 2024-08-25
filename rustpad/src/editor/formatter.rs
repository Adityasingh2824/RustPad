use std::process::{Command, Output};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported languages for code formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Rust,
    JavaScript,
    Python,
}

/// FormatterError to represent any errors during the formatting process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterError {
    pub message: String,
}

/// Trait that defines the behavior of a formatter
pub trait Formatter {
    fn format_code(&self, code: &str) -> Result<String, FormatterError>;
}

/// Formatter for Rust using `rustfmt`
pub struct RustFormatter;

impl Formatter for RustFormatter {
    fn format_code(&self, code: &str) -> Result<String, FormatterError> {
        run_formatter_command("rustfmt", code)
    }
}

/// Formatter for JavaScript using Prettier
pub struct JavaScriptFormatter;

impl Formatter for JavaScriptFormatter {
    fn format_code(&self, code: &str) -> Result<String, FormatterError> {
        run_formatter_command("prettier", code)
    }
}

/// Formatter for Python using Black
pub struct PythonFormatter;

impl Formatter for PythonFormatter {
    fn format_code(&self, code: &str) -> Result<String, FormatterError> {
        run_formatter_command("black", code)
    }
}

/// Runs a formatter command and returns the formatted code or an error
fn run_formatter_command(command: &str, code: &str) -> Result<String, FormatterError> {
    // Run the formatter command as an external process
    let output: Output = Command::new(command)
        .arg("--stdin")
        .output()
        .map_err(|e| FormatterError {
            message: format!("Failed to run formatter: {}", e),
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(FormatterError {
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

/// Initializes the available formatters for different languages
pub fn initialize_formatters() -> Arc<Mutex<HashMap<Language, Box<dyn Formatter + Send>>>> {
    let mut formatters: HashMap<Language, Box<dyn Formatter + Send>> = HashMap::new();
    formatters.insert(Language::Rust, Box::new(RustFormatter));
    formatters.insert(Language::JavaScript, Box::new(JavaScriptFormatter));
    formatters.insert(Language::Python, Box::new(PythonFormatter));

    Arc::new(Mutex::new(formatters))
}

/// Formats the code based on the language using the appropriate formatter
pub fn format_code(
    language: Language,
    code: &str,
    formatter_store: Arc<Mutex<HashMap<Language, Box<dyn Formatter + Send>>>>,
) -> Result<String, FormatterError> {
    let formatters = formatter_store.lock().unwrap();
    
    if let Some(formatter) = formatters.get(&language) {
        formatter.format_code(code)
    } else {
        Err(FormatterError {
            message: format!("Formatter for language {:?} not found", language),
        })
    }
}
