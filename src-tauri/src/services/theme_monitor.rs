/// Theme monitoring service for macOS system theme changes
///
/// This service monitors macOS system theme changes via NSDistributedNotificationCenter
/// and emits Tauri events when the system theme changes during application runtime.
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{Mutex, mpsc};
use tokio::time::{interval, Duration};
use crate::utils::detect_system_theme;

#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl, runtime::{Class, Object}};
#[cfg(target_os = "macos")]
use objc_foundation::{INSString, NSString};
#[cfg(target_os = "macos")]
use objc_id::{Id, Shared};
#[cfg(target_os = "macos")]
use block::ConcreteBlock;

/// Theme polling interval in seconds (fallback only)
///
/// Used only when NSDistributedNotificationCenter is unavailable.
/// Polls system theme every 2 seconds as fallback to detect changes.
const THEME_POLL_INTERVAL_SECS: u64 = 2;

/// Monitoring strategy used
#[derive(Debug, Clone, Copy, PartialEq)]
enum MonitoringStrategy {
    /// Using NSDistributedNotificationCenter notifications
    Notification,
    /// Using polling fallback
    Polling,
}

/// Shared state for theme monitoring
pub struct ThemeMonitorState {
    is_monitoring: Arc<Mutex<bool>>,
    observer: Arc<Mutex<Option<ObserverHandle>>>,
    strategy: Arc<Mutex<Option<MonitoringStrategy>>>,
}

/// Handle to the NSDistributedNotificationCenter observer
#[cfg(target_os = "macos")]
struct ObserverHandle {
    observer: Id<Object, Shared>,
    notification_center: Id<Object, Shared>,
}

#[cfg(not(target_os = "macos"))]
struct ObserverHandle;

impl ThemeMonitorState {
    /// Create new theme monitor state
    pub fn new() -> Self {
        Self {
            is_monitoring: Arc::new(Mutex::new(false)),
            observer: Arc::new(Mutex::new(None)),
            strategy: Arc::new(Mutex::new(None)),
        }
    }

    /// Start monitoring theme changes
    ///
    /// Attempts to use NSDistributedNotificationCenter for efficient event-based monitoring.
    /// Falls back to polling if notification setup fails.
    pub async fn start_monitoring(&self, app_handle: AppHandle) -> Result<(), String> {
        let mut is_monitoring = self.is_monitoring.lock().await;

        if *is_monitoring {
            return Err("Monitoring already active".to_string());
        }

        *is_monitoring = true;
        drop(is_monitoring);

        // Try notification-based monitoring first (macOS only)
        #[cfg(target_os = "macos")]
        {
            match self.try_notification_monitoring(app_handle.clone()).await {
                Ok(()) => {
                    eprintln!("Theme monitoring started using NSDistributedNotificationCenter");
                    *self.strategy.lock().await = Some(MonitoringStrategy::Notification);
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Failed to start notification-based monitoring: {}", e);
                    eprintln!("Falling back to polling...");
                }
            }
        }

        // Fallback to polling
        self.start_polling_monitoring(app_handle).await;
        *self.strategy.lock().await = Some(MonitoringStrategy::Polling);
        Ok(())
    }

    /// Try to set up notification-based monitoring using NSDistributedNotificationCenter
    #[cfg(target_os = "macos")]
    async fn try_notification_monitoring(&self, app_handle: AppHandle) -> Result<(), String> {
        use std::ffi::c_void;

        // Create a channel to communicate from Objective-C callback to async Rust
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        let monitoring_flag = self.is_monitoring.clone();

        unsafe {
            // Get NSDistributedNotificationCenter class
            let notification_center_class = Class::get("NSDistributedNotificationCenter")
                .ok_or("NSDistributedNotificationCenter class not available")?;

            // Get default center
            let notification_center: *mut Object = msg_send![notification_center_class, defaultCenter];
            if notification_center.is_null() {
                return Err("Failed to get default notification center".to_string());
            }
            let notification_center: Id<Object, Shared> = Id::from_retained_ptr(notification_center);

            // Create the notification name NSString
            let notification_name = NSString::from_str("AppleInterfaceThemeChangedNotification");

            // Create the block that will be called when notification fires
            let block = ConcreteBlock::new(move |_notification: *mut Object| {
                // Detect current theme
                match detect_system_theme() {
                    Ok(theme) => {
                        // Send theme through channel
                        let _ = tx.send(theme);
                    }
                    Err(e) => {
                        eprintln!("Failed to detect theme in notification callback: {}", e);
                    }
                }
            });
            let block = block.copy();

            // Add observer using the block-based API
            // addObserverForName:object:queue:usingBlock:
            let observer: *mut Object = msg_send![
                &*notification_center,
                addObserverForName: &*notification_name
                object: std::ptr::null::<Object>()
                queue: std::ptr::null::<Object>()
                usingBlock: &*block
            ];

            if observer.is_null() {
                return Err("Failed to add notification observer".to_string());
            }

            let observer: Id<Object, Shared> = Id::from_retained_ptr(observer);

            // Store the observer handle for cleanup
            *self.observer.lock().await = Some(ObserverHandle {
                observer,
                notification_center,
            });
        }

        // Spawn task to handle notifications from the channel
        tokio::spawn(async move {
            let mut last_theme: Option<String> = None;

            while let Some(current_theme) = rx.recv().await {
                // Check if monitoring is still active
                let is_active = *monitoring_flag.lock().await;
                if !is_active {
                    break;
                }

                // Emit event if theme changed
                if last_theme.as_ref().map(|t| t != &current_theme).unwrap_or(true) {
                    if let Some(prev_theme) = &last_theme {
                        eprintln!("Theme changed from {} to {} (notification)", prev_theme, current_theme);
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
        });

        Ok(())
    }

    /// Start polling-based monitoring as fallback
    async fn start_polling_monitoring(&self, app_handle: AppHandle) {
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

                // Detect current theme
                match detect_system_theme() {
                    Ok(current_theme) => {
                        // Emit event if theme changed
                        if last_theme.as_ref().map(|t| t != &current_theme).unwrap_or(true) {
                            if let Some(prev_theme) = &last_theme {
                                eprintln!("Theme changed from {} to {} (polling)", prev_theme, current_theme);
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
                        eprintln!("Failed to detect theme during polling: {}", e);
                    }
                }
            }
        });
    }

    /// Stop monitoring theme changes
    pub async fn stop_monitoring(&self) -> Result<(), String> {
        let mut is_monitoring = self.is_monitoring.lock().await;
        *is_monitoring = false;

        // Remove notification observer if using notification strategy
        let mut observer = self.observer.lock().await;
        if let Some(handle) = observer.take() {
            self.remove_observer(handle).await;
        }

        *self.strategy.lock().await = None;
        Ok(())
    }

    /// Remove the NSDistributedNotificationCenter observer
    #[cfg(target_os = "macos")]
    async fn remove_observer(&self, handle: ObserverHandle) {
        unsafe {
            let _: () = msg_send![&*handle.notification_center, removeObserver: &*handle.observer];
        }
    }

    #[cfg(not(target_os = "macos"))]
    async fn remove_observer(&self, _handle: ObserverHandle) {
        // No-op on non-macOS platforms
    }

    /// Check if monitoring is active
    pub async fn is_monitoring(&self) -> bool {
        *self.is_monitoring.lock().await
    }

    /// Get the current monitoring strategy (for testing/debugging)
    #[allow(dead_code)]
    pub async fn get_strategy(&self) -> Option<MonitoringStrategy> {
        *self.strategy.lock().await
    }
}

impl Default for ThemeMonitorState {
    fn default() -> Self {
        Self::new()
    }
}

// Ensure ObserverHandle is Send + Sync for Arc<Mutex<Option<ObserverHandle>>>
#[cfg(target_os = "macos")]
unsafe impl Send for ObserverHandle {}
#[cfg(target_os = "macos")]
unsafe impl Sync for ObserverHandle {}
