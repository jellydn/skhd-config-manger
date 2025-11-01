/// Data models module

pub mod backup;
pub mod config;
pub mod shortcut;
pub mod test_result;

// Re-export commonly used types
pub use backup::Backup;
pub use config::{ConfigFile, ParseError};
pub use shortcut::Shortcut;
pub use test_result::TestResult;
