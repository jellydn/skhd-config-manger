/// Backup management Tauri commands

use crate::commands::config::ConfigState;
use crate::models::Backup;
use crate::services::backup::{
    create_backup as create_backup_service,
    list_backups as list_backups_service,
    restore_backup as restore_backup_service,
};
use tauri::State;

/// Create a backup of the current configuration
///
/// # Arguments
/// * `description` - Optional description for the backup
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Backup)` - Created backup metadata
/// * `Err(String)` - Error message
#[tauri::command]
pub fn create_backup(
    description: Option<String>,
    state: State<'_, ConfigState>,
) -> Result<Backup, String> {
    let file_path = {
        let config_guard = state.config.lock().unwrap();
        let config = config_guard
            .as_ref()
            .ok_or("No config loaded")?;
        config.file_path.clone()
    }; // Lock released here

    create_backup_service(&file_path, description)
        .map_err(|e| format!("Failed to create backup: {}", e))
}

/// List all available backups
///
/// # Returns
/// * `Ok(Vec<Backup>)` - List of backups
/// * `Err(String)` - Error message
#[tauri::command]
pub fn list_backups() -> Result<Vec<Backup>, String> {
    list_backups_service()
        .map_err(|e| format!("Failed to list backups: {}", e))
}

/// Restore configuration from a backup
///
/// # Arguments
/// * `backup_path` - Path to backup file
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` on failure
#[tauri::command]
pub fn restore_backup(
    backup_path: String,
    state: State<'_, ConfigState>,
) -> Result<(), String> {
    let target_path = {
        let config_guard = state.config.lock().unwrap();
        let config = config_guard
            .as_ref()
            .ok_or("No config loaded")?;
        config.file_path.clone()
    }; // Lock released here

    // Find the backup
    let backups = list_backups_service()
        .map_err(|e| format!("Failed to list backups: {}", e))?;

    let backup = backups
        .iter()
        .find(|b| b.file_path.to_string_lossy() == backup_path)
        .ok_or("Backup not found")?;

    // Restore
    restore_backup_service(backup, Some(&target_path))
        .map_err(|e| format!("Failed to restore backup: {}", e))?;

    Ok(())
}
