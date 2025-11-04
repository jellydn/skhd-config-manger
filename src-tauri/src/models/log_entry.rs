use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

impl FromStr for LogLevel {
    type Err = ();

    /// Parse log level from string (case-insensitive)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ERROR" | "ERR" => Ok(LogLevel::Error),
            "WARN" | "WARNING" => Ok(LogLevel::Warn),
            "INFO" => Ok(LogLevel::Info),
            "DEBUG" | "DBG" => Ok(LogLevel::Debug),
            _ => Err(()),
        }
    }
}

impl LogLevel {
    /// Parse log level from string (case-insensitive) - convenience method returning Option
    pub fn parse_opt(s: &str) -> Option<Self> {
        <Self as FromStr>::from_str(s).ok()
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
        assert_eq!(LogLevel::parse_opt("ERROR"), Some(LogLevel::Error));
        assert_eq!(LogLevel::parse_opt("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::parse_opt("WARN"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::parse_opt("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::parse_opt("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::parse_opt("INVALID"), None);
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
