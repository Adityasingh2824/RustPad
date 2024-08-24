use log::{debug, error, info, warn, LevelFilter};
use simplelog::{Config, SimpleLogger, TermLogger, WriteLogger, CombinedLogger, TerminalMode};
use std::fs::File;
use std::path::Path;

/// Initializes the logger with console and optional file logging.
/// The log level and file path are configurable.
pub fn init_logger(log_to_file: bool, file_path: Option<&str>, level: LevelFilter) {
    let mut loggers = vec![];

    // Setup terminal logging (logs to the console)
    loggers.push(TermLogger::new(
        level,
        Config::default(),
        TerminalMode::Mixed, // Logs to both stderr for errors and stdout for others
        simplelog::ColorChoice::Auto,
    ));

    // If file logging is enabled, set up file logging
    if log_to_file {
        if let Some(path) = file_path {
            if let Ok(log_file) = File::create(Path::new(path)) {
                loggers.push(WriteLogger::new(level, Config::default(), log_file));
            } else {
                eprintln!("Failed to create log file at path: {}", path);
            }
        }
    }

    // Initialize the logger
    CombinedLogger::init(loggers).unwrap();
}

/// Logs a debug message.
pub fn log_debug(message: &str) {
    debug!("{}", message);
}

/// Logs an info message.
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Logs a warning message.
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Logs an error message.
pub fn log_error(message: &str) {
    error!("{}", message);
}
