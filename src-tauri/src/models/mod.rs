/// Data models module
pub mod backup;
pub mod config;
pub mod log_entry;
pub mod service_status;
pub mod shortcut;
pub mod test_result;

// Re-export commonly used types
pub use backup::Backup;
pub use config::{ConfigFile, ParseError};
pub use log_entry::{LogEntry, LogLevel};
pub use service_status::{ServiceState, ServiceStatus};
pub use shortcut::Shortcut;
pub use test_result::TestResult;
