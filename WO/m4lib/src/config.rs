//! Module containing the configuration for the M4 library.

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub enum LogOutput {
    stdout,
    stderr,
    File(String),
}

/// Stuct for logging configuration
/// This struct contains the configuration for logging in the M4 library.
/// # Exaples :
/// ```
/// use m4lib::config::Logging;
/// let mut logging = Logging::new();
/// logging.enabled = true;
/// logging.level = LogLevel::Debug;
/// logging.destination = LogOutput::stdout;
/// // Set the logging level to debug
/// logging.level = LogLevel::Debug;
/// // Set the logging destination to stdout
/// logging.destination = LogOutput::stdout;
/// // Enable logging
/// logging.enabled = true;
/// // Disable logging
/// logging.enabled = false;
/// // Set the logging destination to a file
/// logging.destination = LogOutput::File("log.txt".to_string());
/// // Set the logging level to info
/// logging.level = LogLevel::Info;
/// // Set the logging level to warning
/// logging.level = LogLevel::Warning;
/// // Set the logging level to error
/// logging.level = LogLevel::Error;
/// // Set the logging level to debug
/// logging.level = LogLevel::Debug;
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Default for Logging {
    fn default() -> Self {
        Self::new()
    }
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            destination: LogOutput::stdout,
        }
    }
}
