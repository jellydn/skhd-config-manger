use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
}

impl ServiceStatus {
    /// Create a new service status
    pub fn new(state: ServiceState) -> Self {
        Self {
            state,
            pid: None,
            last_updated: Utc::now(),
            config_path: None,
            error_message: None,
        }
    }

    /// Create a service status with PID (for Running state)
    pub fn running(pid: u32) -> Self {
        Self {
            state: ServiceState::Running,
            pid: Some(pid),
            last_updated: Utc::now(),
            config_path: None,
            error_message: None,
        }
    }

    /// Create an error status with message
    pub fn error(message: String) -> Self {
        Self {
            state: ServiceState::Error,
            pid: None,
            last_updated: Utc::now(),
            config_path: None,
            error_message: Some(message),
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
