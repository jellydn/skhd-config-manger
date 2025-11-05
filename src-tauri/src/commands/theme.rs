/// Theme detection commands for macOS system theme integration
/// 
/// This module provides Tauri commands to detect and monitor macOS system theme
/// preferences (light/dark mode) and emit events when theme changes occur.
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};
use std::ffi::{CStr, c_char};

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
pub async fn get_system_theme() -> Result<String, String> {
    // Try using objc crate to access NSUserDefaults
    match detect_theme_via_objc() {
        Ok(theme) => Ok(theme),
        Err(_) => {
            // Fallback to defaults command-line tool
            match detect_theme_via_defaults() {
                Ok(theme) => Ok(theme),
                Err(_) => {
                    // Ultimate fallback: default to dark mode
                    eprintln!("Theme detection failed, defaulting to dark mode");
                    Ok("dark".to_string())
                }
            }
        }
    }
}

/// Detect theme using objc crate and NSUserDefaults (primary method)
fn detect_theme_via_objc() -> Result<String, String> {
    unsafe {
        // Get NSUserDefaults class
        let user_defaults_class = Class::get("NSUserDefaults")
            .ok_or("NSUserDefaults class not available")?;
        
        // Get standard user defaults instance
        #[allow(unexpected_cfgs)]
        let standard_defaults: *mut Object = msg_send![user_defaults_class, standardUserDefaults];
        if standard_defaults.is_null() {
            return Err("Failed to get standard user defaults".to_string());
        }
        
        // Create NSString for the key "AppleInterfaceStyle"
        let nsstring_class = Class::get("NSString")
            .ok_or("NSString class not available")?;
        #[allow(unexpected_cfgs)]
        let key: *mut Object = msg_send![nsstring_class, stringWithUTF8String: c"AppleInterfaceStyle".as_ptr()];
        
        if key.is_null() {
            return Err("Failed to create NSString key".to_string());
        }
        
        // Read AppleInterfaceStyle value
        #[allow(unexpected_cfgs)]
        let style_obj: *mut Object = msg_send![standard_defaults, objectForKey: key];
        
        if style_obj.is_null() {
            // Key not set means light mode
            return Ok("light".to_string());
        }
        
        // Convert NSString to Rust string
        #[allow(unexpected_cfgs)]
        let utf8_string: *const c_char = msg_send![style_obj, UTF8String];
        if utf8_string.is_null() {
            return Ok("light".to_string());
        }
        
        let c_str = CStr::from_ptr(utf8_string);
        let style_str = c_str.to_str()
            .map_err(|_| "Failed to convert NSString to UTF-8")?;
        
        // "Dark" means dark mode, anything else (or missing) means light mode
        if style_str == "Dark" {
            Ok("dark".to_string())
        } else {
            Ok("light".to_string())
        }
    }
}

/// Detect theme using defaults command-line tool (fallback method)
fn detect_theme_via_defaults() -> Result<String, String> {
    let output = std::process::Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleInterfaceStyle")
        .output()
        .map_err(|e| format!("Failed to execute defaults command: {}", e))?;
    
    if !output.status.success() {
        // Command failed (likely means light mode - key doesn't exist)
        return Ok("light".to_string());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    
    if trimmed == "Dark" {
        Ok("dark".to_string())
    } else {
        Ok("light".to_string())
    }
}

/// Start monitoring macOS system theme changes
/// 
/// Subscribes to theme changes via polling (every 500ms).
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
