/// Tauri commands for log streaming and management
///
/// This module exposes the following commands to the frontend:
/// - `start_log_stream`: Begin streaming logs from skhd service
/// - `stop_log_stream`: Stop the log stream
/// - `is_log_stream_running`: Check if stream is active
///
/// Events emitted:
/// - `log-entry`: Emitted for each new log entry (payload: LogEntry)

use crate::services::LogTailer;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

/// Shared state for LogTailer across all commands
pub struct LogStreamState {
    tailer: Arc<Mutex<Option<LogTailer>>>,
}

impl LogStreamState {
    /// Create new log stream state
    pub fn new() -> Self {
        Self {
            tailer: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for LogStreamState {
    fn default() -> Self {
        Self::new()
    }
}

/// Start streaming logs from the skhd service
///
/// This command:
/// - Spawns a background `log stream` process
/// - Begins emitting "log-entry" events to the frontend
/// - Returns immediately after starting the stream
///
/// # Arguments
/// * `app` - Tauri AppHandle for event emission
/// * `state` - Shared LogStreamState
///
/// # Returns
/// * `Ok(())` - Stream started successfully
/// * `Err(String)` - Failed to start (already running, spawn error)
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// try {
///   await invoke('start_log_stream');
///   console.log('Log stream started');
/// } catch (error) {
///   console.error('Failed to start log stream:', error);
/// }
/// ```
#[tauri::command]
pub async fn start_log_stream(
    app: AppHandle,
    state: State<'_, LogStreamState>,
) -> Result<(), String> {
    let mut tailer_lock = state.tailer.lock().await;

    // Check if already initialized
    if tailer_lock.is_some() {
        return Err("Log stream is already running".to_string());
    }

    // Create new tailer and start stream
    let tailer = LogTailer::new(app);
    tailer.start_stream().await?;

    *tailer_lock = Some(tailer);

    Ok(())
}

/// Stop the log stream and clean up resources
///
/// This command:
/// - Kills the background `log stream` process
/// - Stops emitting log-entry events
/// - Releases all resources
///
/// # Arguments
/// * `state` - Shared LogStreamState
///
/// # Returns
/// * `Ok(())` - Stream stopped successfully
/// * `Err(String)` - Failed to stop (not running, kill error)
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// try {
///   await invoke('stop_log_stream');
///   console.log('Log stream stopped');
/// } catch (error) {
///   console.error('Failed to stop log stream:', error);
/// }
/// ```
#[tauri::command]
pub async fn stop_log_stream(state: State<'_, LogStreamState>) -> Result<(), String> {
    let mut tailer_lock = state.tailer.lock().await;

    if let Some(tailer) = tailer_lock.take() {
        tailer.stop_stream().await?;
        Ok(())
    } else {
        Err("Log stream is not running".to_string())
    }
}

/// Check if the log stream is currently running
///
/// # Arguments
/// * `state` - Shared LogStreamState
///
/// # Returns
/// * `true` - Stream is active
/// * `false` - Stream is not running
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const isRunning = await invoke<boolean>('is_log_stream_running');
/// console.log('Stream running:', isRunning);
/// ```
#[tauri::command]
pub async fn is_log_stream_running(state: State<'_, LogStreamState>) -> Result<bool, String> {
    let tailer_lock = state.tailer.lock().await;

    Ok(if let Some(tailer) = tailer_lock.as_ref() {
        tailer.is_running().await
    } else {
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_stream_state_creation() {
        let state = LogStreamState::new();
        // State should be created successfully
        assert!(state.tailer.try_lock().is_ok());
    }

    #[test]
    fn test_log_stream_state_default() {
        let state = LogStreamState::default();
        // Default implementation should work
        assert!(state.tailer.try_lock().is_ok());
    }
}
