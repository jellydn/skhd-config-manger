use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::Mutex;

use crate::models::{
    AccessibilityPermission, InputMonitoringPermission, ServiceState, ServiceStatus, SkhdVariant,
};
use crate::services::service_diagnostics::{
    accessibility_guidance, config_requires_grabber, current_accessibility_denial, parse_zig_status,
};
use crate::services::settings::{effective_variant_async, EffectiveVariantResult};
use crate::services::variant_detector::find_binary;
use crate::utils::path::get_config_path_for_variant;

fn parse_launchctl_service_line(line: &str) -> Option<(ServiceState, Option<u32>, Option<String>)> {
    let mut parts = line.split_whitespace();
    let pid = parts.next()?;
    let exit_code = parts.next()?;

    if exit_code != "0" {
        return Some((
            ServiceState::Error,
            pid.parse().ok(),
            Some(format!("Service exited with code {}", exit_code)),
        ));
    }

    if pid == "-" {
        Some((ServiceState::Stopped, None, None))
    } else {
        Some((ServiceState::Running, pid.parse().ok(), None))
    }
}

/// Error type for service operations
#[derive(Debug, Clone)]
pub struct ServiceError {
    pub variant: SkhdVariant,
    pub message: String,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self.variant {
            SkhdVariant::Original => "skhd",
            SkhdVariant::Zig => "skhd.zig",
        };
        write!(f, "{}: {}", variant_name, self.message)
    }
}

impl std::error::Error for ServiceError {}

/// Service manager that dispatches based on the effective skhd variant
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
        let effective = effective_variant_async().await;
        let status = match effective.variant {
            SkhdVariant::Original => self.get_status_original().await,
            SkhdVariant::Zig => self.get_status_zig(&effective).await,
        }?;
        if effective.variant == SkhdVariant::Original {
            Ok(self
                .with_accessibility_status(status, effective.variant)
                .await)
        } else {
            Ok(status)
        }
    }

    async fn with_accessibility_status(
        &self,
        mut status: ServiceStatus,
        variant: SkhdVariant,
    ) -> ServiceStatus {
        status.accessibility_guidance = accessibility_guidance(variant);

        if matches!(status.state, ServiceState::Running) {
            status.accessibility_permission = AccessibilityPermission::Granted;
            return status;
        }

        if let Some(diagnostic) = self.recent_accessibility_denial(variant).await {
            status.state = ServiceState::Error;
            status.accessibility_permission = AccessibilityPermission::Denied;
            status.error_message = Some(diagnostic.trim().to_string());
        }

        status
    }

    async fn recent_accessibility_denial(&self, variant: SkhdVariant) -> Option<String> {
        let path = match variant {
            SkhdVariant::Original => {
                let username = std::env::var("USER")
                    .or_else(|_| std::env::var("USERNAME"))
                    .ok()?;
                let safe_username: String = username
                    .chars()
                    .filter(|character| {
                        character.is_alphanumeric() || *character == '_' || *character == '-'
                    })
                    .collect();
                PathBuf::from(format!("/tmp/skhd_{}.err.log", safe_username))
            }
            SkhdVariant::Zig => dirs::home_dir()?.join("Library/Logs/skhd.log"),
        };

        let mut file = tokio::fs::File::open(path).await.ok()?;
        let metadata = file.metadata().await.ok()?;
        if metadata.modified().ok()?.elapsed().ok()? > Duration::from_secs(300) {
            return None;
        }

        const MAX_LOG_TAIL_BYTES: u64 = 16 * 1024;
        let start = metadata.len().saturating_sub(MAX_LOG_TAIL_BYTES);
        file.seek(std::io::SeekFrom::Start(start)).await.ok()?;

        let mut bytes = Vec::with_capacity((metadata.len() - start) as usize);
        file.take(MAX_LOG_TAIL_BYTES)
            .read_to_end(&mut bytes)
            .await
            .ok()?;
        let tail = String::from_utf8_lossy(&bytes);
        let complete_tail = if start == 0 {
            tail.as_ref()
        } else {
            tail.split_once('\n').map_or("", |(_, rest)| rest)
        };

        current_accessibility_denial(complete_tail)
    }

    /// Get status for original skhd (koekeishiya)
    async fn get_status_original(&self) -> Result<ServiceStatus, String> {
        self.get_status_from_launchctl("com.koekeishiya.skhd").await
    }

    /// Get status for skhd.zig (jackielii)
    async fn get_status_zig(
        &self,
        effective: &EffectiveVariantResult,
    ) -> Result<ServiceStatus, String> {
        // skhd.zig --status reports SMAppService registration, daemon PID, event-tap
        // health, Accessibility, and Input Monitoring in one authoritative probe.
        if let Some(ref detected) = effective.detected {
            if detected.variant == Some(SkhdVariant::Zig) {
                if let Some(ref binary_path) = detected.binary_path {
                    return self.get_status_from_skhd_command(binary_path).await;
                }
            }
        }

        let app_binary = "/Applications/skhd.app/Contents/MacOS/skhd";
        if std::path::Path::new(app_binary).exists() {
            return self.get_status_from_skhd_command(app_binary).await;
        }

        let mut launchctl_status = self
            .get_status_from_launchctl(SkhdVariant::Zig.service_label())
            .await?;
        if !matches!(launchctl_status.state, ServiceState::Unknown) {
            launchctl_status.accessibility_guidance = accessibility_guidance(SkhdVariant::Zig);
            return Ok(launchctl_status);
        }

        // Service not found
        Ok(ServiceStatus {
            variant: SkhdVariant::Zig,
            state: ServiceState::Unknown,
            pid: None,
            last_updated: chrono::Utc::now(),
            config_path: self.get_active_config_path_zig().await.ok(),
            error_message: Some(
                "skhd.zig service not found. Install skhd.zig and register the service with: \
                 skhd --install-service && skhd --start-service"
                    .to_string(),
            ),
            accessibility_permission: AccessibilityPermission::Unknown,
            accessibility_guidance: accessibility_guidance(SkhdVariant::Zig),
            input_monitoring_permission: InputMonitoringPermission::Unknown,
        })
    }

    /// Get status from launchctl list output for a specific label
    async fn get_status_from_launchctl(&self, label: &str) -> Result<ServiceStatus, String> {
        let output = Command::new("launchctl")
            .arg("list")
            .output()
            .map_err(|e| {
                format!(
                    "Failed to execute launchctl command: {}. \
                     Make sure you're running on macOS and launchctl is available.",
                    e
                )
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Look for the service in the output
        for line in stdout.lines() {
            if line.contains(label) {
                if let Some((state, pid, error_message)) = parse_launchctl_service_line(line) {
                    let config_path = match label {
                        "com.koekeishiya.skhd" => self.get_active_config_path_original().await.ok(),
                        _ => self.get_active_config_path_zig().await.ok(),
                    };

                    return Ok(ServiceStatus {
                        variant: if label == SkhdVariant::Zig.service_label() {
                            SkhdVariant::Zig
                        } else {
                            SkhdVariant::Original
                        },
                        state,
                        pid,
                        last_updated: chrono::Utc::now(),
                        config_path,
                        error_message,
                        accessibility_permission: AccessibilityPermission::Unknown,
                        accessibility_guidance: String::new(),
                        input_monitoring_permission: if label == SkhdVariant::Zig.service_label() {
                            InputMonitoringPermission::Unknown
                        } else {
                            InputMonitoringPermission::NotRequired
                        },
                    });
                }
            }
        }

        // Service not found in launchctl list
        Ok(ServiceStatus {
            variant: if label == SkhdVariant::Zig.service_label() {
                SkhdVariant::Zig
            } else {
                SkhdVariant::Original
            },
            state: ServiceState::Unknown,
            pid: None,
            last_updated: chrono::Utc::now(),
            config_path: None,
            error_message: Some(match label {
                "com.koekeishiya.skhd" => {
                    "skhd service not found. Start it with: brew services start skhd".to_string()
                }
                _ => format!(
                    "skhd service not found in launchctl list (label: {}).",
                    label
                ),
            }),
            accessibility_permission: AccessibilityPermission::Unknown,
            accessibility_guidance: String::new(),
            input_monitoring_permission: if label == SkhdVariant::Zig.service_label() {
                InputMonitoringPermission::Unknown
            } else {
                InputMonitoringPermission::NotRequired
            },
        })
    }

    /// Get status from skhd --status command (for skhd.zig)
    async fn get_status_from_skhd_command(
        &self,
        binary_path: &str,
    ) -> Result<ServiceStatus, String> {
        let output = Command::new(binary_path).arg("--status").output().ok();

        let (state, pid, accessibility_permission, input_monitoring_permission, error_message) =
            if let Some(output) = output {
                if output.status.success() {
                    let status_output = format!(
                        "{}\n{}",
                        String::from_utf8_lossy(&output.stderr),
                        String::from_utf8_lossy(&output.stdout)
                    );
                    parse_zig_status(&status_output)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    (
                        ServiceState::Error,
                        None,
                        AccessibilityPermission::Unknown,
                        InputMonitoringPermission::Unknown,
                        Some(format!("skhd --status failed: {}", stderr.trim())),
                    )
                }
            } else {
                (
                    ServiceState::Unknown,
                    None,
                    AccessibilityPermission::Unknown,
                    InputMonitoringPermission::Unknown,
                    Some("Failed to run skhd --status".to_string()),
                )
            };

        Ok(ServiceStatus {
            variant: SkhdVariant::Zig,
            state,
            pid,
            last_updated: chrono::Utc::now(),
            config_path: self.get_active_config_path_zig().await.ok(),
            error_message,
            accessibility_permission,
            accessibility_guidance: accessibility_guidance(SkhdVariant::Zig),
            input_monitoring_permission,
        })
    }

    /// Stop the skhd service
    pub async fn stop_service(&self) -> Result<(), String> {
        let effective = effective_variant_async().await;
        match effective.variant {
            SkhdVariant::Original => self.stop_service_original().await,
            SkhdVariant::Zig => self.stop_service_zig(&effective).await,
        }
    }

    /// Stop service for original skhd
    async fn stop_service_original(&self) -> Result<(), String> {
        let plist_path = self.get_plist_path_original()?;
        let domain = Self::gui_domain_target()?;

        let output = Command::new("launchctl")
            .arg("bootout")
            .arg(domain)
            .arg(&plist_path)
            .output()
            .map_err(|e| {
                format!(
                    "skhd: Failed to execute launchctl bootout: {}. \
                     Check that you have permission to control launchd services.",
                    e
                )
            })?;

        // Also try the older unload command for backwards compatibility
        if !output.status.success() {
            let _ = Command::new("launchctl")
                .arg("unload")
                .arg(&plist_path)
                .output();
        }

        // Verify the service was stopped
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let status = self.get_status_original().await?;

        if matches!(status.state, ServiceState::Running) {
            return Err("skhd: Failed to stop service. The service is still running.".to_string());
        }

        Ok(())
    }

    /// Stop service for skhd.zig
    async fn stop_service_zig(&self, effective: &EffectiveVariantResult) -> Result<(), String> {
        let binary_path = self.get_skhd_binary_path(effective).await?;

        let output = Command::new(&binary_path)
            .arg("--stop-service")
            .output()
            .map_err(|e| {
                format!(
                    "skhd.zig: Failed to execute skhd --stop-service: {}. \
                     Make sure skhd.zig is installed and available in PATH.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd.zig: Failed to stop service: {}. \
                 The service may not be running.",
                stderr.trim()
            ));
        }

        // Verify the service was stopped
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let status = self.get_status_zig(effective).await?;

        if matches!(status.state, ServiceState::Running) {
            return Err(
                "skhd.zig: Failed to stop service. The service is still running.".to_string(),
            );
        }

        Ok(())
    }

    /// Start the skhd service
    pub async fn start_service(&self) -> Result<(), String> {
        let effective = effective_variant_async().await;
        match effective.variant {
            SkhdVariant::Original => self.start_service_original().await,
            SkhdVariant::Zig => self.start_service_zig(&effective).await,
        }
    }

    /// Start service for original skhd
    async fn start_service_original(&self) -> Result<(), String> {
        let plist_path = self.get_plist_path_original()?;
        let domain = Self::gui_domain_target()?;
        let service_registered = !matches!(
            self.get_status_original().await?.state,
            ServiceState::Unknown
        );

        let output = if service_registered {
            Command::new("launchctl")
                .arg("kickstart")
                .arg("-k")
                .arg(format!("{domain}/com.koekeishiya.skhd"))
                .output()
        } else {
            Command::new("launchctl")
                .arg("bootstrap")
                .arg(&domain)
                .arg(&plist_path)
                .output()
        }
        .map_err(|error| {
            let action = if service_registered {
                "kickstart"
            } else {
                "bootstrap"
            };
            format!(
                "skhd: Failed to execute launchctl {action}: {error}. \
                 Check that you have permission to control launchd services."
            )
        })?;

        let mut command_error = (!output.status.success()).then(|| {
            let action = if service_registered {
                "kickstart"
            } else {
                "bootstrap"
            };
            format!(
                "skhd: launchctl {action} failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )
        });

        // Also try the older load command for backwards compatibility.
        if !service_registered && !output.status.success() {
            let fallback = Command::new("launchctl")
                .arg("load")
                .arg(&plist_path)
                .output();
            if let Ok(fallback) = fallback {
                if !fallback.status.success() {
                    command_error = Some(format!(
                        "skhd: launchctl bootstrap failed: {}. launchctl load also failed: {}",
                        String::from_utf8_lossy(&output.stderr).trim(),
                        String::from_utf8_lossy(&fallback.stderr).trim()
                    ));
                } else {
                    command_error = None;
                }
            }
        }

        // Wait for service to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Verify service started
        let status = self
            .with_accessibility_status(self.get_status_original().await?, SkhdVariant::Original)
            .await;
        match status.state {
            ServiceState::Running => Ok(()),
            ServiceState::Error => Err(status.error_message.unwrap_or_else(|| {
                command_error.unwrap_or_else(|| {
                    "skhd: Service failed to start. Check your skhd configuration and service logs."
                        .to_string()
                })
            })),
            _ => Err(command_error.unwrap_or_else(|| format!(
                "skhd: Service in unexpected state: {:?}. Try restarting the service manually with: brew services restart skhd",
                status.state
            ))),
        }
    }

    /// Start service for skhd.zig
    async fn start_service_zig(&self, effective: &EffectiveVariantResult) -> Result<(), String> {
        let binary_path = self.get_skhd_binary_path(effective).await?;

        if let Ok(config_path) = get_config_path_for_variant(SkhdVariant::Zig) {
            let content = std::fs::read_to_string(&config_path).map_err(|error| {
                format!("skhd.zig: Failed to inspect configuration at {config_path}: {error}")
            })?;
            if config_requires_grabber(&content) {
                return Err(
                    "skhd.zig: This configuration uses block-form .remap rules. Run \
                     'skhd --start-service' manually in Terminal to review privileged helper setup."
                        .to_string(),
                );
            }
        }

        let output = Command::new(&binary_path)
            .arg("--start-service")
            .output()
            .map_err(|e| {
                format!(
                    "skhd.zig: Failed to execute skhd --start-service: {}. \
                     Make sure skhd.zig is installed and available in PATH.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd.zig: Failed to start service: {}. \
                 Check that skhd.zig is installed and the service is registered.",
                stderr.trim()
            ));
        }

        // Wait for service to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Verify service started
        let status = self
            .with_accessibility_status(self.get_status_zig(effective).await?, SkhdVariant::Zig)
            .await;
        match status.state {
            ServiceState::Running => Ok(()),
            ServiceState::Error => Err(status.error_message.unwrap_or_else(|| {
                "skhd.zig: Service failed to start. Check your skhd configuration for syntax errors.".to_string()
            })),
            _ => Err(format!(
                "skhd.zig: Service in unexpected state: {:?}. Try restarting the service manually with: \
                 skhd --restart-service",
                status.state
            )),
        }
    }

    /// Restart the skhd service
    pub async fn restart_service(&self) -> Result<(), String> {
        let effective = effective_variant_async().await;
        match effective.variant {
            SkhdVariant::Original => self.restart_service_original().await,
            SkhdVariant::Zig => self.restart_service_zig(&effective).await,
        }
    }

    /// Restart service for original skhd
    async fn restart_service_original(&self) -> Result<(), String> {
        let output = Command::new("brew")
            .args(["services", "restart", "skhd"])
            .output()
            .map_err(|e| {
                format!(
                    "skhd: Failed to execute brew services restart: {}. \
                     Make sure Homebrew is installed and skhd was installed via Homebrew.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // If brew services fails, try manual stop/start
            if self.stop_service_original().await.is_ok() {
                return self.start_service_original().await;
            }
            return Err(format!(
                "skhd: Failed to restart service: {}.",
                stderr.trim()
            ));
        }

        // Wait for service to restart
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Verify service is running
        let status = self
            .with_accessibility_status(self.get_status_original().await?, SkhdVariant::Original)
            .await;
        if !matches!(status.state, ServiceState::Running) {
            return Err(status.error_message.unwrap_or_else(|| {
                "skhd: Service failed to restart. Check the service status and logs.".to_string()
            }));
        }

        Ok(())
    }

    /// Restart service for skhd.zig
    async fn restart_service_zig(&self, effective: &EffectiveVariantResult) -> Result<(), String> {
        let binary_path = self.get_skhd_binary_path(effective).await?;

        let output = Command::new(&binary_path)
            .arg("--restart-service")
            .output()
            .map_err(|e| {
                format!(
                    "skhd.zig: Failed to execute skhd --restart-service: {}. \
                     Make sure skhd.zig is installed and available in PATH.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd.zig: Failed to restart service: {}.",
                stderr.trim()
            ));
        }

        // Wait for service to restart
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Verify service is running
        let status = self
            .with_accessibility_status(self.get_status_zig(effective).await?, SkhdVariant::Zig)
            .await;
        if !matches!(status.state, ServiceState::Running) {
            return Err(status.error_message.unwrap_or_else(|| {
                "skhd.zig: Service failed to restart. Check the service status and logs."
                    .to_string()
            }));
        }

        Ok(())
    }

    /// Reload the skhd service configuration
    ///
    /// This method acquires a lock to prevent concurrent reloads.
    /// The lock is automatically released when the function returns (RAII pattern),
    /// even in case of errors or panics.
    pub async fn reload_service(&self) -> Result<(), String> {
        // Acquire lock to prevent concurrent reloads
        let _lock = self.reload_lock.lock().await;

        let effective = effective_variant_async().await;
        let result = match effective.variant {
            SkhdVariant::Original => self.reload_service_original().await,
            SkhdVariant::Zig => self.reload_service_zig(&effective).await,
        };

        // Lock is automatically released here when _lock goes out of scope
        result
    }

    /// Reload service for original skhd
    async fn reload_service_original(&self) -> Result<(), String> {
        let output = Command::new("skhd").arg("--reload").output().map_err(|e| {
            format!(
                "skhd: Failed to execute skhd --reload: {}. \
                     Make sure skhd is installed and available in PATH.",
                e
            )
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd: Failed to reload service: {}. \
                 The service may not be running. Try starting it first.",
                stderr.trim()
            ));
        }

        Ok(())
    }

    /// Reload service for skhd.zig
    async fn reload_service_zig(&self, effective: &EffectiveVariantResult) -> Result<(), String> {
        let binary_path = self.get_skhd_binary_path(effective).await?;

        let output = Command::new(&binary_path)
            .arg("--reload")
            .output()
            .map_err(|e| {
                format!(
                    "skhd.zig: Failed to execute skhd --reload: {}. \
                     Make sure skhd.zig is installed and available in PATH.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd.zig: Failed to reload service: {}. \
                 The service may not be running. Try starting it first.",
                stderr.trim()
            ));
        }

        Ok(())
    }

    /// Install the skhd.zig service
    /// This is only applicable for skhd.zig variant
    pub async fn install_service(&self) -> Result<(), String> {
        let effective = effective_variant_async().await;
        if matches!(effective.variant, SkhdVariant::Original) {
            return Err("skhd: Service installation is handled via the plist file. \
                 Install skhd via Homebrew and use 'brew services start skhd'."
                .to_string());
        }

        let binary_path = self.get_skhd_binary_path(&effective).await?;

        if let Ok(config_path) = get_config_path_for_variant(SkhdVariant::Zig) {
            if let Ok(content) = std::fs::read_to_string(config_path) {
                if config_requires_grabber(&content) {
                    return Err(
                        "skhd.zig: This configuration uses block-form .remap rules, so service \
                         installation may require the privileged skhd-grabber. Run \
                         'skhd --install-service' manually in a terminal to review that flow."
                            .to_string(),
                    );
                }
            }
        }

        let output = Command::new(&binary_path)
            .arg("--install-service")
            .output()
            .map_err(|e| {
                format!(
                    "skhd.zig: Failed to execute skhd --install-service: {}. \
                     Make sure skhd.zig is installed.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd.zig: Failed to install service: {}.",
                stderr.trim()
            ));
        }

        Ok(())
    }

    /// Uninstall the skhd.zig service
    /// This is only applicable for skhd.zig variant
    pub async fn uninstall_service(&self) -> Result<(), String> {
        let effective = effective_variant_async().await;
        if matches!(effective.variant, SkhdVariant::Original) {
            return Err(
                "skhd: Service uninstallation is handled via the plist file. \
                 Stop skhd with 'brew services stop skhd'."
                    .to_string(),
            );
        }

        let binary_path = self.get_skhd_binary_path(&effective).await?;

        let output = Command::new(&binary_path)
            .arg("--uninstall-service")
            .output()
            .map_err(|e| {
                format!(
                    "skhd.zig: Failed to execute skhd --uninstall-service: {}. \
                     Make sure skhd.zig is installed.",
                    e
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "skhd.zig: Failed to uninstall service: {}.",
                stderr.trim()
            ));
        }

        Ok(())
    }

    /// Get the path to the skhd binary
    async fn get_skhd_binary_path(
        &self,
        effective: &EffectiveVariantResult,
    ) -> Result<String, String> {
        // First, check if we have a detected binary path
        if let Some(ref detected) = effective.detected {
            if detected.variant == Some(effective.variant) {
                if let Some(ref binary_path) = detected.binary_path {
                    return Ok(binary_path.clone());
                }
            }
        }

        find_binary(effective.variant).ok_or_else(|| {
            format!(
                "{}: No matching executable was found. Install it or select another implementation in Settings.",
                effective.variant.display_name()
            )
        })
    }

    /// Get the path to the skhd launchd plist file for original skhd
    fn get_plist_path_original(&self) -> Result<String, String> {
        let home = std::env::var("HOME").map_err(|_| {
            "Failed to get HOME environment variable. \
             This is required to locate the skhd plist file."
                .to_string()
        })?;

        Ok(format!(
            "{}/Library/LaunchAgents/com.koekeishiya.skhd.plist",
            home
        ))
    }

    /// Get the launchd GUI domain for the current user.
    fn gui_domain_target() -> Result<String, String> {
        let output = Command::new("id")
            .arg("-u")
            .output()
            .map_err(|e| format!("Failed to determine current UID: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to determine current UID: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if uid.is_empty() || !uid.chars().all(|character| character.is_ascii_digit()) {
            return Err("Failed to determine current UID: invalid output from id -u".to_string());
        }

        Ok(format!("gui/{}", uid))
    }

    /// Get the active skhd configuration path for original skhd
    async fn get_active_config_path_original(&self) -> Result<String, String> {
        get_config_path_for_variant(SkhdVariant::Original)
    }

    /// Get the active skhd configuration path for skhd.zig
    async fn get_active_config_path_zig(&self) -> Result<String, String> {
        get_config_path_for_variant(SkhdVariant::Zig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SkhdVariant;

    #[test]
    fn test_service_manager_new() {
        let manager = ServiceManager::new();
        // Just verify it creates without error
        assert!(manager.reload_lock.try_lock().is_ok());
    }

    #[test]
    fn test_service_error_display() {
        let err = ServiceError {
            variant: SkhdVariant::Original,
            message: "Failed to start".to_string(),
        };
        assert_eq!(err.to_string(), "skhd: Failed to start");

        let err_zig = ServiceError {
            variant: SkhdVariant::Zig,
            message: "Binary not found".to_string(),
        };
        assert_eq!(err_zig.to_string(), "skhd.zig: Binary not found");
    }

    #[test]
    fn test_launchctl_nonzero_exit_without_pid_is_an_error() {
        let (state, pid, message) =
            parse_launchctl_service_line("- 1 com.koekeishiya.skhd").unwrap();

        assert_eq!(state, ServiceState::Error);
        assert_eq!(pid, None);
        assert_eq!(message.as_deref(), Some("Service exited with code 1"));

        let (state, pid, message) =
            parse_launchctl_service_line("- 0 com.koekeishiya.skhd").unwrap();
        assert_eq!(state, ServiceState::Stopped);
        assert_eq!(pid, None);
        assert_eq!(message, None);
    }

    #[test]
    fn test_gui_domain_target_contains_numeric_uid() {
        let target = ServiceManager::gui_domain_target().unwrap();
        let uid = target.strip_prefix("gui/").unwrap();
        assert!(!uid.is_empty());
        assert!(uid.chars().all(|character| character.is_ascii_digit()));
    }
}
