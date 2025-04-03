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

implement Logging {
    pub fn new(enabled: bool, level: LogLevel, destination: LogOutput) -> Self {
        Logging {
            enabled,
            level,
            destination,
        }
    }

    pub fn log(&self, message: &str) {
        if self.enabled {
            match self.destination {
                LogOutput::stdout => println!("{}", message),
                LogOutput::stderr => eprintln!("{}", message),
                LogOutput::File(ref path) => {
                    // Implement file logging here
                    // For example, using std::fs::write or a logging library
                }
            }
        }
    }
}