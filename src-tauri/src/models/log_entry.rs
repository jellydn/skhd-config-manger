use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Log level categorization for visual distinction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "DEBUG")]
    Debug,
}

impl LogLevel {
    /// Parse log level from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "ERROR" | "ERR" => Some(LogLevel::Error),
            "WARN" | "WARNING" => Some(LogLevel::Warn),
            "INFO" => Some(LogLevel::Info),
            "DEBUG" | "DBG" => Some(LogLevel::Debug),
            _ => None,
        }
    }
}

/// Represents a single log line from skhd service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Unique identifier for deduplication and tracking
    pub id: String,

    /// When the log entry was generated (ISO 8601)
    pub timestamp: DateTime<Utc>,

    /// Severity level of the log entry
    pub level: LogLevel,

    /// The actual log message content
    pub message: String,

    /// Original unparsed log line (fallback for display)
    pub raw: String,
}

impl LogEntry {
    /// Create a new log entry with generated UUID
    pub fn new(timestamp: DateTime<Utc>, level: LogLevel, message: String, raw: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp,
            level,
            message,
            raw,
        }
    }

    /// Create a log entry from raw line with current timestamp (fallback parsing)
    pub fn from_raw(raw: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            level: LogLevel::Info,
            message: raw.clone(),
            raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("ERROR"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("WARN"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("INVALID"), None);
    }

    #[test]
    fn test_log_entry_new() {
        let now = Utc::now();
        let entry = LogEntry::new(
            now,
            LogLevel::Info,
            "Test message".to_string(),
            "Raw log line".to_string(),
        );

        assert!(!entry.id.is_empty());
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.raw, "Raw log line");
    }
}
