/// Theme monitoring service for macOS system theme changes
/// 
/// This service monitors macOS system theme changes via NSDistributedNotificationCenter
/// and emits Tauri events when the system theme changes during application runtime.
use std::ffi::{CStr, c_char};
use std::sync::Arc;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::{Class, Object};
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

/// Shared state for theme monitoring
pub struct ThemeMonitorState {
    is_monitoring: Arc<Mutex<bool>>,
}

impl ThemeMonitorState {
    /// Create new theme monitor state
    pub fn new() -> Self {
        Self {
            is_monitoring: Arc::new(Mutex::new(false)),
        }
    }

    /// Start monitoring theme changes
    /// 
    /// This implementation uses polling as a fallback approach.
    /// A more efficient implementation using NSDistributedNotificationCenter
    /// would require complex Objective-C callback handling.
    pub async fn start_monitoring(&self, app_handle: AppHandle) -> Result<(), String> {
        let mut is_monitoring = self.is_monitoring.lock().await;
        
        if *is_monitoring {
            return Err("Monitoring already active".to_string());
        }

        *is_monitoring = true;
        drop(is_monitoring);

        // Spawn background task to poll theme changes
        let monitoring_flag = self.is_monitoring.clone();
        
        tokio::spawn(async move {
            let mut last_theme: Option<String> = None;
            let mut poll_interval = interval(Duration::from_millis(500)); // Poll every 500ms
            
            loop {
                poll_interval.tick().await;
                
                // Check if monitoring is still active
                let is_active = *monitoring_flag.lock().await;
                if !is_active {
                    break;
                }
                
                // Detect current theme (synchronous call)
                match detect_current_theme() {
                    Ok(current_theme) => {
                        // Emit event if theme changed
                        if last_theme.as_ref().map(|t| t != &current_theme).unwrap_or(true) {
                            if let Some(prev_theme) = &last_theme {
                                eprintln!("Theme changed from {} to {}", prev_theme, current_theme);
                            }
                            
                            // Emit theme-changed event
                            let payload = serde_json::json!({
                                "theme": current_theme,
                                "timestamp": chrono::Utc::now().to_rfc3339(),
                            });
                            
                            if let Err(e) = app_handle.emit("theme-changed", &payload) {
                                eprintln!("Failed to emit theme-changed event: {}", e);
                            }
                            
                            last_theme = Some(current_theme);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to detect theme during monitoring: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop monitoring theme changes
    pub async fn stop_monitoring(&self) -> Result<(), String> {
        let mut is_monitoring = self.is_monitoring.lock().await;
        *is_monitoring = false;
        Ok(())
    }

    /// Check if monitoring is active
    pub async fn is_monitoring(&self) -> bool {
        *self.is_monitoring.lock().await
    }
}

impl Default for ThemeMonitorState {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect current system theme (duplicated from theme.rs for internal use)
fn detect_current_theme() -> Result<String, String> {
    // Try using objc crate to access NSUserDefaults
    match detect_theme_via_objc() {
        Ok(theme) => Ok(theme),
        Err(_) => {
            // Fallback to defaults command-line tool
            match detect_theme_via_defaults() {
                Ok(theme) => Ok(theme),
                Err(_) => {
                    // Ultimate fallback: default to dark mode
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
