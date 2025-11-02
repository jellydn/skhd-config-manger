/// Service modules
pub mod backup;
pub mod file_io;
pub mod log_tailer;
pub mod service_manager;
pub mod validation;

pub use backup::{create_backup, list_backups, restore_backup};
pub use file_io::{read_config_safe, write_config_atomic};
pub use log_tailer::{parse_log_line, LogTailer};
pub use service_manager::ServiceManager;
pub use validation::{validate_config, validate_shortcut};
