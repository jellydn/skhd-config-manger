use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::SkhdVariant;

/// Accessibility permission state verified from the skhd daemon itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityPermission {
    #[serde(rename = "Granted")]
    Granted,
    #[serde(rename = "Denied")]
    Denied,
    #[serde(rename = "Unknown")]
    Unknown,
}

/// Input Monitoring state reported by skhd.zig.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputMonitoringPermission {
    #[serde(rename = "Granted")]
    Granted,
    #[serde(rename = "Denied")]
    Denied,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "NotRequired")]
    NotRequired,
}

/// Represents skhd service lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceState {
    /// Service is not running
    #[serde(rename = "Stopped")]
    Stopped,

    /// Service start initiated, awaiting confirmation
    #[serde(rename = "Starting")]
    Starting,

    /// Service is active and operational
    #[serde(rename = "Running")]
    Running,

    /// Service stop initiated, awaiting confirmation
    #[serde(rename = "Stopping")]
    Stopping,

    /// Service restart in progress (stop → start sequence)
    #[serde(rename = "Reloading")]
    Reloading,

    /// Service encountered a failure (check error_message)
    #[serde(rename = "Error")]
    Error,

    /// Cannot determine service state (skhd not installed, permission issues)
    #[serde(rename = "Unknown")]
    Unknown,
}

/// Represents the current state of the skhd service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Variant whose service was inspected.
    pub variant: SkhdVariant,

    /// Current lifecycle state
    pub state: ServiceState,

    /// Process ID if running, None if stopped
    pub pid: Option<u32>,

    /// When status was last checked (ISO 8601)
    pub last_updated: DateTime<Utc>,

    /// Path to active configuration file
    pub config_path: Option<String>,

    /// Error details if state is Error
    pub error_message: Option<String>,

    /// Permission state based on daemon-owned status and recent logs
    pub accessibility_permission: AccessibilityPermission,

    /// Correct macOS permission target and recovery steps
    pub accessibility_guidance: String,

    /// Input Monitoring state. Classic skhd does not report this separately.
    pub input_monitoring_permission: InputMonitoringPermission,
}

impl ServiceStatus {
    /// Create a new service status
    pub fn new(state: ServiceState) -> Self {
        Self {
            variant: SkhdVariant::Original,
            state,
            pid: None,
            last_updated: Utc::now(),
            config_path: None,
            error_message: None,
            accessibility_permission: AccessibilityPermission::Unknown,
            accessibility_guidance: String::new(),
            input_monitoring_permission: InputMonitoringPermission::NotRequired,
        }
    }

    /// Create a service status with PID (for Running state)
    pub fn running(pid: u32) -> Self {
        Self {
            variant: SkhdVariant::Original,
            state: ServiceState::Running,
            pid: Some(pid),
            last_updated: Utc::now(),
            config_path: None,
            error_message: None,
            accessibility_permission: AccessibilityPermission::Granted,
            accessibility_guidance: String::new(),
            input_monitoring_permission: InputMonitoringPermission::NotRequired,
        }
    }

    /// Create an error status with message
    pub fn error(message: String) -> Self {
        Self {
            variant: SkhdVariant::Original,
            state: ServiceState::Error,
            pid: None,
            last_updated: Utc::now(),
            config_path: None,
            error_message: Some(message),
            accessibility_permission: AccessibilityPermission::Unknown,
            accessibility_guidance: String::new(),
            input_monitoring_permission: InputMonitoringPermission::NotRequired,
        }
    }

    /// Update the last_updated timestamp to current time
    pub fn refresh_timestamp(&mut self) {
        self.last_updated = Utc::now();
    }

    /// Check if the service is in a healthy running state
    pub fn is_running(&self) -> bool {
        matches!(self.state, ServiceState::Running) && self.pid.is_some()
    }

    /// Check if the service is in a transitional state
    pub fn is_transitioning(&self) -> bool {
        matches!(
            self.state,
            ServiceState::Starting | ServiceState::Stopping | ServiceState::Reloading
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_status_new() {
        let status = ServiceStatus::new(ServiceState::Stopped);
        assert_eq!(status.state, ServiceState::Stopped);
        assert_eq!(status.pid, None);
        assert_eq!(status.error_message, None);
    }

    #[test]
    fn test_service_status_running() {
        let status = ServiceStatus::running(1234);
        assert_eq!(status.state, ServiceState::Running);
        assert_eq!(status.pid, Some(1234));
        assert!(status.is_running());
    }

    #[test]
    fn test_service_status_error() {
        let status = ServiceStatus::error("Permission denied".to_string());
        assert_eq!(status.state, ServiceState::Error);
        assert_eq!(status.error_message, Some("Permission denied".to_string()));
        assert!(!status.is_running());
    }

    #[test]
    fn test_is_transitioning() {
        assert!(ServiceStatus::new(ServiceState::Starting).is_transitioning());
        assert!(ServiceStatus::new(ServiceState::Stopping).is_transitioning());
        assert!(ServiceStatus::new(ServiceState::Reloading).is_transitioning());
        assert!(!ServiceStatus::running(1234).is_transitioning());
        assert!(!ServiceStatus::new(ServiceState::Stopped).is_transitioning());
    }
}
