/// Theme detection commands for macOS system theme integration
///
/// This module provides Tauri commands to detect and monitor macOS system theme
/// preferences (light/dark mode) and emit events when theme changes occur.
use crate::utils::detect_system_theme;

/// Detect the current macOS system theme preference (light or dark mode)
///
/// Reads `AppleInterfaceStyle` from macOS NSUserDefaults.
/// Returns "dark" if value is "Dark", "light" otherwise.
/// Falls back to "dark" if detection fails (maintains current app default).
///
/// # Returns
/// - `Ok("light")` if system is in light mode
/// - `Ok("dark")` if system is in dark mode or detection fails
#[tauri::command]
pub fn get_system_theme() -> Result<String, String> {
    detect_system_theme()
}

/// Start monitoring macOS system theme changes
///
/// Subscribes to theme changes via polling (every 2 seconds).
/// Emits 'theme-changed' Tauri event when system theme changes.
/// 
/// # Arguments
/// * `app_handle` - Tauri AppHandle for emitting events
/// * `state` - Shared ThemeMonitorState
#[tauri::command]
pub async fn start_theme_monitor(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, crate::services::ThemeMonitorState>,
) -> Result<(), String> {
    state.start_monitoring(app_handle).await
}

/// Stop monitoring macOS system theme changes
/// 
/// Stops polling and cleans up resources.
/// 
/// # Arguments
/// * `state` - Shared ThemeMonitorState
#[tauri::command]
pub async fn stop_theme_monitor(
    state: tauri::State<'_, crate::services::ThemeMonitorState>,
) -> Result<(), String> {
    state.stop_monitoring().await
}
