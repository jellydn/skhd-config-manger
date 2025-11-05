/// Theme monitoring service for macOS system theme changes
///
/// This service monitors macOS system theme changes via polling
/// and emits Tauri events when the system theme changes during application runtime.
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use crate::utils::detect_system_theme;

/// Theme polling interval in seconds
///
/// Polls system theme every 2 seconds to detect changes.
/// This provides a maximum 2-second detection latency, meeting the <2s requirement.
/// More efficient event-based monitoring via NSDistributedNotificationCenter
/// would require complex Objective-C callback handling.
const THEME_POLL_INTERVAL_SECS: u64 = 2;

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
            let mut poll_interval = interval(Duration::from_secs(THEME_POLL_INTERVAL_SECS));
            
            loop {
                poll_interval.tick().await;
                
                // Check if monitoring is still active
                let is_active = *monitoring_flag.lock().await;
                if !is_active {
                    break;
                }
                
                // Detect current theme (synchronous call)
                match detect_system_theme() {
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

