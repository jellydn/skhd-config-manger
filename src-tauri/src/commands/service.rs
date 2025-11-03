use crate::models::ServiceStatus;
use crate::services::ServiceManager;
use tauri::State;

/// Get the current status of the skhd service
#[tauri::command]
pub async fn get_service_status(
    service_manager: State<'_, ServiceManager>,
) -> Result<ServiceStatus, String> {
    service_manager.get_status().await
}

/// Reload the skhd service
#[tauri::command]
pub async fn reload_service(service_manager: State<'_, ServiceManager>) -> Result<(), String> {
    service_manager.reload_service().await
}
