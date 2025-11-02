// Modules
pub mod commands;
pub mod models;
pub mod parser;
pub mod services;
pub mod utils;

use commands::config::ConfigState;
use commands::testing::ExecutionState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ConfigState::new())
        .manage(ExecutionState::default())
        .invoke_handler(tauri::generate_handler![
            commands::config::detect_active_config,
            commands::config::load_config,
            commands::config::save_config,
            commands::config::save_as_config,
            commands::config::reload_config,
            commands::config::import_config,
            commands::config::export_config,
            commands::shortcuts::create_shortcut,
            commands::shortcuts::update_shortcut,
            commands::shortcuts::delete_shortcut,
            commands::validation::validate_shortcut,
            commands::validation::validate_config,
            commands::backups::create_backup,
            commands::backups::list_backups,
            commands::backups::restore_backup,
            commands::testing::test_shortcut,
            commands::testing::execute_test_command,
            commands::testing::execute_shortcut_command,
            commands::testing::cancel_shortcut_execution,
            commands::testing::get_execution_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
