// Modules
pub mod commands;
pub mod models;
pub mod parser;
pub mod services;
pub mod utils;

use commands::config::ConfigState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ConfigState::new())
        .invoke_handler(tauri::generate_handler![
            commands::config::load_config,
            commands::config::save_config,
            commands::config::reload_config,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
