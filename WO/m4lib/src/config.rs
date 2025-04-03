//! Module containing the configuration for the M4 library.

// Log Level Configs
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

pub struct Logging {
    enabled: bool,
    level: LogLevel,
    destination: LogOutput,
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
