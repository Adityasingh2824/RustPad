use std::process::{Command, Output};
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Formatter {
    formatters: HashMap<String, Box<dyn Fn(&str) -> io::Result<String>>>,
}

impl Formatter {
    /// Creates a new Formatter with predefined formatters for different languages
    pub fn new() -> Self {
        let mut formatters = HashMap::new();

        // Add a JavaScript formatter using Prettier
        formatters.insert("javascript".to_string(), Box::new(Self::format_javascript as _));

        // Add an HTML formatter using Prettier
        formatters.insert("html".to_string(), Box::new(Self::format_html as _));

        // Add a CSS formatter using Prettier
        formatters.insert("css".to_string(), Box::new(Self::format_css as _));

        // Add a Rust formatter using rustfmt
        formatters.insert("rust".to_string(), Box::new(Self::format_rust as _));

        Formatter { formatters }
    }

    /// Formats code based on the language
    pub fn format(&self, language: &str, code: &str) -> io::Result<String> {
        if let Some(formatter_fn) = self.formatters.get(language) {
            formatter_fn(code)
        } else {
            Ok(code.to_string()) // Return the original code if no formatter is available
        }
    }

    /// JavaScript formatter using Prettier
    fn format_javascript(code: &str) -> io::Result<String> {
        run_prettier(code, "javascript")
    }

    /// HTML formatter using Prettier
    fn format_html(code: &str) -> io::Result<String> {
        run_prettier(code, "html")
    }

    /// CSS formatter using Prettier
    fn format_css(code: &str) -> io::Result<String> {
        run_prettier(code, "css")
    }

    /// Rust formatter using rustfmt
    fn format_rust(code: &str) -> io::Result<String> {
        let output = run_command("rustfmt", vec!["--emit=stdout"], Some(code))?;
        Ok(output)
    }
}

/// Helper function to run Prettier with the appropriate parser
fn run_prettier(code: &str, parser: &str) -> io::Result<String> {
    run_command("prettier", vec!["--stdin-filepath", &format!("file.{}", parser), "--parser", parser], Some(code))
}

/// Helper function to run a formatter command
fn run_command(command: &str, args: Vec<&str>, input: Option<&str>) -> io::Result<String> {
    let mut cmd = Command::new(command);
    
    for arg in args {
        cmd.arg(arg);
    }

    if let Some(input_text) = input {
        let mut process = cmd.stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn()?;
        {
            let stdin = process.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(input_text.as_bytes())?;
        }
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
    fn test_format_javascript() {
        let formatter = Formatter::new();
        let code = "function test(){console.log('hello')}";
        let result = formatter.format("javascript", code).unwrap();
        
        assert_eq!(result, "function test() {\n  console.log('hello');\n}\n");
    }

    #[test]
    fn test_format_rust() {
        let formatter = Formatter::new();
        let code = "fn main() {println!(\"Hello, world!\");}";
        let result = formatter.format("rust", code).unwrap();
        
        assert_eq!(result, "fn main() {\n    println!(\"Hello, world!\");\n}\n");
    }

    #[test]
    fn test_format_html() {
        let formatter = Formatter::new();
        let code = "<div><h1>Hello</h1></div>";
        let result = formatter.format("html", code).unwrap();
        
        assert_eq!(result, "<div>\n  <h1>Hello</h1>\n</div>\n");
    }
}
