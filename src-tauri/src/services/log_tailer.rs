/// Service for tailing skhd service logs in real-time
///
/// This module provides the core functionality for:
/// - Parsing structured log entries from raw skhd log output
/// - Streaming log data in real-time using macOS `log stream`
/// - Managing the lifecycle of log streaming (start/stop)
/// - Event emission for new log entries to the frontend

use crate::models::{LogEntry, LogLevel};
use chrono::NaiveDateTime;
use regex::Regex;
use std::process::Stdio;
use std::sync::{Arc, OnceLock};
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Regex pattern for parsing skhd log lines
/// Format: YYYY-MM-DD HH:MM:SS [LEVEL] message
static LOG_PATTERN: OnceLock<Regex> = OnceLock::new();

/// Get the compiled regex pattern for log parsing
fn get_log_pattern() -> &'static Regex {
    LOG_PATTERN.get_or_init(|| {
        Regex::new(r"^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(ERROR|WARN|INFO|DEBUG)\] (.+)$")
            .expect("Valid regex pattern")
    })
}

/// Parse a single log line into a structured LogEntry
///
/// # Arguments
/// * `line` - Raw log line string from skhd service
///
/// # Returns
/// * `Some(LogEntry)` - Successfully parsed log entry
/// * `None` - Line could not be parsed (should use fallback)
///
/// # Examples
/// ```ignore
/// let line = "2025-11-02 10:15:30 [INFO] skhd: configuration loaded";
/// let entry = parse_log_line(line);
/// assert!(entry.is_some());
/// ```
pub fn parse_log_line(line: &str) -> Option<LogEntry> {
    let re = get_log_pattern();

    if let Some(caps) = re.captures(line) {
        let timestamp_str = caps.get(1)?.as_str();
        let level_str = caps.get(2)?.as_str();
        let message = caps.get(3)?.as_str();

        // Parse timestamp
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
            .ok()?
            .and_utc();

        // Parse log level
        let level = LogLevel::from_str(level_str)?;

        Some(LogEntry::new(
            timestamp,
            level,
            message.to_string(),
            line.to_string(),
        ))
    } else {
        // Return fallback entry for unparseable lines
        Some(LogEntry::from_raw(line.to_string()))
    }
}

/// Sanitize username to prevent path traversal attacks
///
/// This function filters out any characters that could be used for path traversal
/// or other malicious purposes, allowing only alphanumeric characters, underscores, and hyphens.
///
/// # Arguments
/// * `username` - Raw username string from environment
///
/// # Returns
/// * Sanitized username string safe for use in file paths
///
/// # Examples
/// ```ignore
/// let safe = sanitize_username("user/../root");
/// assert_eq!(safe, "userroot");
/// ```
fn sanitize_username(username: &str) -> String {
    username
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

/// LogTailer manages the lifecycle of log streaming from the skhd service
///
/// This struct:
/// - Spawns a background `log stream` process to tail skhd logs
/// - Parses each log line and emits events to the frontend
/// - Provides start/stop controls for the streaming process
/// - Ensures proper cleanup of resources on stop
#[derive(Clone)]
pub struct LogTailer {
    app_handle: AppHandle,
    stream_handle: Arc<Mutex<Option<StreamHandle>>>,
}

/// Internal handle to the running log stream process and task
struct StreamHandle {
    process: Child,
    task: JoinHandle<()>,
}

impl LogTailer {
    /// Create a new LogTailer instance
    ///
    /// # Arguments
    /// * `app_handle` - Tauri AppHandle for event emission
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            stream_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Start streaming logs from the skhd service
    ///
    /// This method spawns a background `log stream` process that:
    /// - Filters for skhd-related logs
    /// - Parses each line into structured LogEntry
    /// - Emits "log-entry" events to the frontend
    ///
    /// # Returns
    /// * `Ok(())` - Stream started successfully
    /// * `Err(String)` - Failed to start stream (already running, spawn error)
    ///
    /// # Examples
    /// ```ignore
    /// let tailer = LogTailer::new(app_handle);
    /// tailer.start_stream().await?;
    /// ```
    pub async fn start_stream(&self) -> Result<(), String> {
        let mut handle = self.stream_handle.lock().await;

        // Check if already running
        if handle.is_some() {
            return Err("Log stream is already running".to_string());
        }

        // Get current username for log file path
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        // Sanitize username to prevent path traversal attacks
        let sanitized_username = sanitize_username(&username);
        let log_file = format!("/tmp/skhd_{}.err.log", sanitized_username);

        // Spawn tail process to follow skhd log file
        // skhd writes logs to /tmp/skhd_<username>.err.log
        let mut process = Command::new("tail")
            .arg("-f")
            .arg("-n")
            .arg("100") // Start with last 100 lines
            .arg(&log_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to spawn tail process for {}: {}", log_file, e))?;

        let stdout = process
            .stdout
            .take()
            .ok_or("Failed to capture stdout".to_string())?;

        // Spawn task to read lines and emit events
        let app_handle = self.app_handle.clone();
        let task = tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                // Parse log line
                if let Some(entry) = parse_log_line(&line) {
                    // Emit event to frontend
                    let _ = app_handle.emit("log-entry", &entry);
                }
            }
        });

        *handle = Some(StreamHandle { process, task });

        Ok(())
    }

    /// Stop the log stream and clean up resources
    ///
    /// This method:
    /// - Kills the background `log stream` process
    /// - Cancels the line-reading task
    /// - Releases all resources
    ///
    /// # Returns
    /// * `Ok(())` - Stream stopped successfully
    /// * `Err(String)` - Failed to stop stream (not running, kill error)
    ///
    /// # Examples
    /// ```ignore
    /// tailer.stop_stream().await?;
    /// ```
    pub async fn stop_stream(&self) -> Result<(), String> {
        let mut handle = self.stream_handle.lock().await;

        if let Some(mut stream) = handle.take() {
            // Kill the process
            stream
                .process
                .kill()
                .await
                .map_err(|e| format!("Failed to kill log stream process: {}", e))?;

            // Abort the task
            stream.task.abort();

            Ok(())
        } else {
            Err("Log stream is not running".to_string())
        }
    }

    /// Check if the log stream is currently running
    ///
    /// # Returns
    /// * `true` - Stream is active
    /// * `false` - Stream is not running
    pub async fn is_running(&self) -> bool {
        self.stream_handle.lock().await.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log() {
        let line = "2025-11-02 10:15:30 [INFO] skhd: configuration loaded successfully";
        let entry = parse_log_line(line).expect("Should parse valid log line");

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "skhd: configuration loaded successfully");
        assert_eq!(entry.raw, line);
    }

    #[test]
    fn test_parse_invalid_log_fallback() {
        let line = "Invalid log line without timestamp";
        let entry = parse_log_line(line).expect("Should return fallback entry");

        // Fallback creates INFO level with raw content
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, line);
        assert_eq!(entry.raw, line);
    }

    #[test]
    fn test_log_pattern_caching() {
        // First call initializes
        let pattern1 = get_log_pattern();
        // Second call returns cached instance
        let pattern2 = get_log_pattern();

        // Should be the same instance
        assert!(std::ptr::eq(pattern1, pattern2));
    }

    #[test]
    fn test_sanitize_username_valid() {
        assert_eq!(sanitize_username("john_doe"), "john_doe");
        assert_eq!(sanitize_username("user-123"), "user-123");
        assert_eq!(sanitize_username("TestUser"), "TestUser");
    }

    #[test]
    fn test_sanitize_username_path_traversal() {
        assert_eq!(sanitize_username("user/../root"), "userroot");
        assert_eq!(sanitize_username("../../../etc/passwd"), "etcpasswd");
        assert_eq!(sanitize_username("user/../../tmp"), "usertmp");
    }

    #[test]
    fn test_sanitize_username_special_chars() {
        assert_eq!(sanitize_username("user@host.com"), "userhostcom");
        assert_eq!(sanitize_username("user$name"), "username");
        assert_eq!(sanitize_username("user;name"), "username");
        assert_eq!(sanitize_username("user|name"), "username");
    }
}
