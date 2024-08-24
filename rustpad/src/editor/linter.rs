use std::process::{Command, Output};
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct LintResult {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: LintSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
}

pub struct Linter {
    linters: HashMap<String, Box<dyn Fn(&str) -> io::Result<Vec<LintResult>>>>,
}

impl Linter {
    /// Creates a new Linter with predefined language linter functions
    pub fn new() -> Self {
        let mut linters = HashMap::new();

        // Add a JavaScript linter using ESLint
        linters.insert("javascript".to_string(), Box::new(Self::lint_javascript as _));

        // Add a Rust linter using rustc
        linters.insert("rust".to_string(), Box::new(Self::lint_rust as _));

        // Add other linters as needed...
        
        Linter { linters }
    }

    /// Lint code based on the language
    pub fn lint(&self, language: &str, code: &str) -> io::Result<Vec<LintResult>> {
        if let Some(linter_fn) = self.linters.get(language) {
            linter_fn(code)
        } else {
            Ok(vec![]) // No linter found for the given language
        }
    }

    /// JavaScript linter using ESLint
    fn lint_javascript(code: &str) -> io::Result<Vec<LintResult>> {
        let output = run_command("eslint", vec!["--stdin", "--format", "json"], Some(code))?;
        Self::parse_eslint_output(&output)
    }

    /// Rust linter using rustc
    fn lint_rust(code: &str) -> io::Result<Vec<LintResult>> {
        let output = run_command("rustc", vec!["--edition=2018", "--error-format=json", "-"], Some(code))?;
        Self::parse_rustc_output(&output)
    }

    /// Parse ESLint JSON output
    fn parse_eslint_output(output: &str) -> io::Result<Vec<LintResult>> {
        let mut lint_results = Vec::new();
        let parsed: serde_json::Value = serde_json::from_str(output)?;

        if let Some(array) = parsed.as_array() {
            for file in array {
                if let Some(messages) = file["messages"].as_array() {
                    for message in messages {
                        let lint_result = LintResult {
                            line: message["line"].as_u64().unwrap_or(1) as usize,
                            column: message["column"].as_u64().unwrap_or(1) as usize,
                            message: message["message"].as_str().unwrap_or("Unknown error").to_string(),
                            severity: match message["severity"].as_u64() {
                                Some(2) => LintSeverity::Error,
                                Some(1) => LintSeverity::Warning,
                                _ => LintSeverity::Info,
                            },
                        };
                        lint_results.push(lint_result);
                    }
                }
            }
        }

        Ok(lint_results)
    }

    /// Parse rustc JSON output
    fn parse_rustc_output(output: &str) -> io::Result<Vec<LintResult>> {
        let mut lint_results = Vec::new();
        let parsed: serde_json::Value = serde_json::from_str(output)?;

        if let Some(array) = parsed.as_array() {
            for message in array {
                let lint_result = LintResult {
                    line: message["spans"][0]["line_start"].as_u64().unwrap_or(1) as usize,
                    column: message["spans"][0]["column_start"].as_u64().unwrap_or(1) as usize,
                    message: message["message"].as_str().unwrap_or("Unknown error").to_string(),
                    severity: match message["level"].as_str() {
                        Some("error") => LintSeverity::Error,
                        Some("warning") => LintSeverity::Warning,
                        _ => LintSeverity::Info,
                    },
                };
                lint_results.push(lint_result);
            }
        }

        Ok(lint_results)
    }
}

/// Helper function to run a linter command
fn run_command(command: &str, args: Vec<&str>, input: Option<&str>) -> io::Result<String> {
    let mut cmd = Command::new(command);

    for arg in args {
        cmd.arg(arg);
    }

    if let Some(input_text) = input {
        let process = cmd.stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn()?;
        let mut stdin = process.stdin.unwrap();
        stdin.write_all(input_text.as_bytes())?;
        let output = process.wait_with_output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_javascript() {
        let linter = Linter::new();
        let code = "var x = ;"; // This code has a syntax error
        let result = linter.lint("javascript", code).unwrap();

        assert!(result.len() > 0);
        assert_eq!(result[0].severity, LintSeverity::Error);
    }

    #[test]
    fn test_lint_rust() {
        let linter = Linter::new();
        let code = "fn main() { let x = 5; println!(\"Hello World\"); }"; // Valid Rust code
        let result = linter.lint("rust", code).unwrap();

        assert_eq!(result.len(), 0); // No errors
    }
}
