pub mod backups;
/// Tauri command modules
pub mod config;
pub mod logs;
pub mod service;
pub mod shortcuts;
pub mod testing;
pub mod validation;

// Re-export commands for easy access
pub use backups::{create_backup, list_backups, restore_backup};
pub use config::{detect_active_config, export_config, import_config, load_config, reload_config, save_config};
pub use logs::{is_log_stream_running, start_log_stream, stop_log_stream};
pub use service::{get_service_status, reload_service};
pub use shortcuts::{create_shortcut, delete_shortcut, update_shortcut};
pub use testing::{cancel_shortcut_execution, execute_shortcut_command, execute_test_command, test_shortcut};
pub use validation::{validate_config, validate_shortcut};
