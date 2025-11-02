use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::sync::Mutex;

use crate::models::{ServiceState, ServiceStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceManager {
    #[serde(skip)]
    reload_lock: std::sync::Arc<Mutex<()>>,
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            reload_lock: std::sync::Arc::new(Mutex::new(())),
        }
    }

    /// Get the current status of the skhd service
    pub async fn get_status(&self) -> Result<ServiceStatus, String> {
        let output = Command::new("launchctl")
            .arg("list")
            .output()
            .map_err(|e| format!("Failed to execute launchctl: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Look for skhd service in the output
        for line in stdout.lines() {
            if line.contains("com.koekeishiya.skhd") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let pid_str = parts[0];
                    let status_code = parts[1];

                    let (state, pid, error_message) = if pid_str == "-" {
                        (ServiceState::Stopped, None, None)
                    } else if status_code != "0" {
                        (
                            ServiceState::Error,
                            pid_str.parse().ok(),
                            Some(format!("Service exited with code {}", status_code)),
                        )
                    } else {
                        (ServiceState::Running, pid_str.parse().ok(), None)
                    };

                    return Ok(ServiceStatus {
                        state,
                        pid,
                        last_updated: chrono::Utc::now(),
                        config_path: self.get_active_config_path().await.ok(),
                        error_message,
                    });
                }
            }
        }

        // Service not found in launchctl list
        Ok(ServiceStatus {
            state: ServiceState::Unknown,
            pid: None,
            last_updated: chrono::Utc::now(),
            config_path: None,
            error_message: Some("Service not found in launchctl list".to_string()),
        })
    }

    /// Stop the skhd service
    pub async fn stop_service(&self) -> Result<(), String> {
        let plist_path = self.get_plist_path()?;

        let output = Command::new("launchctl")
            .arg("unload")
            .arg(&plist_path)
            .output()
            .map_err(|e| format!("Failed to execute launchctl unload: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to stop service: {}", stderr));
        }

        Ok(())
    }

    /// Start the skhd service
    pub async fn start_service(&self) -> Result<(), String> {
        let plist_path = self.get_plist_path()?;

        let output = Command::new("launchctl")
            .arg("load")
            .arg(&plist_path)
            .output()
            .map_err(|e| format!("Failed to execute launchctl load: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to start service: {}", stderr));
        }

        // Wait for service to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Verify service started
        let status = self.get_status().await?;
        match status.state {
            ServiceState::Running => Ok(()),
            ServiceState::Error => Err(status
                .error_message
                .unwrap_or_else(|| "Service failed to start".to_string())),
            _ => Err(format!("Service in unexpected state: {:?}", status.state)),
        }
    }

    /// Reload the skhd service
    ///
    /// This method acquires a lock to prevent concurrent reloads.
    /// The lock is automatically released when the function returns (RAII pattern),
    /// even in case of errors or panics.
    pub async fn reload_service(&self) -> Result<(), String> {
        // Acquire lock to prevent concurrent reloads
        // The _lock guard will automatically release the mutex when dropped,
        // either at the end of this function or during error propagation
        let _lock = self.reload_lock.lock().await;

        // Stop the service
        self.stop_service().await?;

        // Start the service
        self.start_service().await?;

        // Lock is automatically released here when _lock goes out of scope
        Ok(())
    }

    /// Get the path to the skhd launchd plist file
    fn get_plist_path(&self) -> Result<String, String> {
        let home = std::env::var("HOME")
            .map_err(|_| "Failed to get HOME environment variable".to_string())?;

        Ok(format!(
            "{}/Library/LaunchAgents/com.koekeishiya.skhd.plist",
            home
        ))
    }

    /// Get the active skhd configuration path
    async fn get_active_config_path(&self) -> Result<String, String> {
        // Read the plist to find StandardErrorPath which usually contains the log
        // In a real implementation, you might want to parse the plist XML
        // For now, use the standard skhd config locations
        let home = std::env::var("HOME")
            .map_err(|_| "Failed to get HOME environment variable".to_string())?;

        let config_paths = vec![
            format!("{}/.config/skhd/skhdrc", home),
            format!("{}/.skhdrc", home),
        ];

        for path in config_paths {
            if std::path::Path::new(&path).exists() {
                return Ok(path);
            }
        }

        Err("No active configuration file found".to_string())
    }
}
