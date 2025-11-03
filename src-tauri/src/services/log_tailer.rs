/// Service for tailing skhd service logs in real-time
///
/// This module provides the core functionality for:
/// - Parsing structured log entries from raw skhd log output
/// - Streaming log data in real-time using macOS `log stream`
/// - Managing the lifecycle of log streaming (start/stop)
/// - Event emission for new log entries to the frontend

use crate::models::{LogEntry, LogLevel};
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Parse a single log line into a structured LogEntry
///
/// skhd logs are plain text without timestamps or explicit log levels.
/// The log level is determined by which file the log came from:
/// - stderr file (/tmp/skhd_*.err.log) -> ERROR level
/// - stdout file (/tmp/skhd_*.out.log) -> INFO level
///
/// # Arguments
/// * `line` - Raw log line string from skhd service
/// * `is_error` - True if from stderr file, false if from stdout file
///
/// # Returns
/// * `Some(LogEntry)` - Successfully parsed log entry
///
/// # Examples
/// ```ignore
/// let line = "skhd: unable to find application named 'Visual Studio Code'";
/// let entry = parse_log_line(line, true); // from stderr = ERROR
/// assert!(entry.is_some());
/// assert_eq!(entry.unwrap().level, LogLevel::Error);
/// ```
pub fn parse_log_line(line: &str, is_error: bool) -> Option<LogEntry> {
    // Skip empty lines
    if line.trim().is_empty() {
        return None;
    }

    let timestamp = chrono::Utc::now();
    // Simple source-based level assignment:
    // stderr file -> ERROR, stdout file -> INFO
    let level = if is_error {
        LogLevel::Error
    } else {
        LogLevel::Info
    };

    Some(LogEntry::new(
        timestamp,
        level,
        line.to_string(),
        line.to_string(),
    ))
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

/// Internal handle to the running log stream processes and tasks
struct StreamHandle {
    stdout_process: Child,
    stdout_task: JoinHandle<()>,
    stderr_process: Child,
    stderr_task: JoinHandle<()>,
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
            return Err("Log stream is already running. Stop the current stream before starting a new one.".to_string());
        }

        // Get current username for log file paths
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        // Sanitize username to prevent path traversal attacks
        let sanitized_username = sanitize_username(&username);
        let stdout_log_file = format!("/tmp/skhd_{}.out.log", sanitized_username);
        let stderr_log_file = format!("/tmp/skhd_{}.err.log", sanitized_username);

        // Spawn tail process for stdout (INFO logs)
        let mut stdout_process = Command::new("tail")
            .arg("-f")
            .arg("-n")
            .arg("50") // Start with last 50 lines from stdout
            .arg(&stdout_log_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| {
                format!(
                    "Failed to start stdout log monitoring for {}: {}. \
                     The log file may not exist yet if skhd has never been started.",
                    stdout_log_file, e
                )
            })?;

        let stdout_stream = stdout_process.stdout.take().ok_or(
            "Failed to capture stdout log stream. \
             This is an internal error - please report this issue."
                .to_string(),
        )?;

        // Spawn tail process for stderr (ERROR logs)
        let mut stderr_process = Command::new("tail")
            .arg("-f")
            .arg("-n")
            .arg("50") // Start with last 50 lines from stderr
            .arg(&stderr_log_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| {
                format!(
                    "Failed to start stderr log monitoring for {}: {}. \
                     Make sure the skhd service is running and has generated logs.",
                    stderr_log_file, e
                )
            })?;

        let stderr_stream = stderr_process.stdout.take().ok_or(
            "Failed to capture stderr log stream. \
             This is an internal error - please report this issue."
                .to_string(),
        )?;

        // Spawn task to read stdout lines and emit events
        let app_handle_stdout = self.app_handle.clone();
        let stdout_task = tokio::spawn(async move {
            let reader = BufReader::new(stdout_stream);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                // Parse log line from stdout (INFO level)
                if let Some(entry) = parse_log_line(&line, false) {
                    // Emit event to frontend
                    let _ = app_handle_stdout.emit("log-entry", &entry);
                }
            }
        });

        // Spawn task to read stderr lines and emit events
        let app_handle_stderr = self.app_handle.clone();
        let stderr_task = tokio::spawn(async move {
            let reader = BufReader::new(stderr_stream);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                // Parse log line from stderr (ERROR level)
                if let Some(entry) = parse_log_line(&line, true) {
                    // Emit event to frontend
                    let _ = app_handle_stderr.emit("log-entry", &entry);
                }
            }
        });

        *handle = Some(StreamHandle {
            stdout_process,
            stdout_task,
            stderr_process,
            stderr_task,
        });

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
            // Kill the stdout process
            stream.stdout_process.kill().await.map_err(|e| {
                format!(
                    "Failed to stop stdout log stream process: {}. \
                     The process may have already terminated.",
                    e
                )
            })?;

            // Kill the stderr process
            stream.stderr_process.kill().await.map_err(|e| {
                format!(
                    "Failed to stop stderr log stream process: {}. \
                     The process may have already terminated.",
                    e
                )
            })?;

            // Abort both tasks
            stream.stdout_task.abort();
            stream.stderr_task.abort();

            Ok(())
        } else {
            Err("Log stream is not running. Start the stream before attempting to stop it.".to_string())
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
    fn test_parse_log_line_from_stderr() {
        let line = "skhd: unable to find application named 'Zed'";
        let entry = parse_log_line(line, true).expect("Should parse stderr log");

        assert_eq!(entry.level, LogLevel::Error); // stderr = ERROR
        assert_eq!(entry.message, line);
        assert_eq!(entry.raw, line);
    }

    #[test]
    fn test_parse_log_line_from_stdout() {
        let line = "yabai -m window --space next";
        let entry = parse_log_line(line, false).expect("Should parse stdout log");

        assert_eq!(entry.level, LogLevel::Info); // stdout = INFO
        assert_eq!(entry.message, line);
        assert_eq!(entry.raw, line);
    }

    #[test]
    fn test_parse_log_line_empty() {
        let line = "";
        let entry = parse_log_line(line, true);

        assert!(entry.is_none(), "Empty lines should return None");
    }

    #[test]
    fn test_parse_log_line_whitespace_only() {
        let line = "   \t\n   ";
        let entry = parse_log_line(line, false);

        assert!(
            entry.is_none(),
            "Whitespace-only lines should return None"
        );
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
