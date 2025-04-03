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
/// ```
pub struct Logging {
    enabled: bool,
    level: LogLevel,
    destination: LogOutput,
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
