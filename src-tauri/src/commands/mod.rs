/// Tauri command modules

pub mod config;
pub mod shortcuts;
pub mod validation;
pub mod backups;
pub mod testing;

// Re-export commands for easy access
pub use config::{load_config, save_config, reload_config};
pub use shortcuts::{create_shortcut, update_shortcut, delete_shortcut};
pub use validation::{validate_shortcut, validate_config};
pub use backups::{create_backup, list_backups, restore_backup};
pub use testing::{test_shortcut, execute_test_command};
