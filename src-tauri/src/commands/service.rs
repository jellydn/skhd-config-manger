use crate::models::ServiceStatus;
use crate::services::ServiceManager;
use tauri::State;

const ACCESSIBILITY_SETTINGS_URL: &str =
    "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility";
const INPUT_MONITORING_SETTINGS_URL: &str =
    "x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent";

/// Get the current status of the skhd service
#[tauri::command]
pub async fn get_service_status(
    service_manager: State<'_, ServiceManager>,
) -> Result<ServiceStatus, String> {
    service_manager.get_status().await
}

/// Start the skhd service
#[tauri::command]
pub async fn start_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.start_service().await
}

/// Stop the skhd service
#[tauri::command]
pub async fn stop_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.stop_service().await
}

/// Restart the skhd service
#[tauri::command]
pub async fn restart_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.restart_service().await
}

/// Reload the skhd service configuration
#[tauri::command]
pub async fn reload_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.reload_service().await
}

/// Install the skhd service (skhd.zig only)
#[tauri::command]
pub async fn install_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.install_service().await
}

/// Uninstall the skhd service (skhd.zig only)
#[tauri::command]
pub async fn uninstall_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.uninstall_service().await
}

/// Open the macOS Accessibility pane without exposing arbitrary URL opening.
#[tauri::command]
pub fn open_accessibility_settings() -> Result<(), String> {
    tauri_plugin_opener::open_url(ACCESSIBILITY_SETTINGS_URL, None::<&str>)
        .map_err(|error| error.to_string())
}

/// Open the macOS Input Monitoring pane without exposing arbitrary URL opening.
#[tauri::command]
pub fn open_input_monitoring_settings() -> Result<(), String> {
    tauri_plugin_opener::open_url(INPUT_MONITORING_SETTINGS_URL, None::<&str>)
        .map_err(|error| error.to_string())
}
