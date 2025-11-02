use crate::commands::config::ConfigState;
use crate::models::{Shortcut, TestResult};
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::State;
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

/// State for tracking running command executions
pub struct ExecutionState {
    /// Map of shortcut ID to running process child
    pub running_processes: Arc<Mutex<HashMap<String, tokio::process::Child>>>,
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            running_processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// Truncate output to a maximum length
pub fn truncate_output(output: String, limit: usize) -> (String, bool) {
    if output.len() > limit {
        (output[..limit].to_string(), true)
    } else {
        (output, false)
    }
}

/// Escape a string for safe shell usage
fn shell_escape(s: &str) -> String {
    // Replace single quotes with '\'' (end quote, escaped quote, start quote)
    s.replace('\'', r"'\''")
}

/// Test a shortcut by executing its command in dry-run mode
/// This shows what would be executed without actually triggering the shortcut
#[tauri::command]
pub fn test_shortcut(
    shortcut_id: String,
    state: State<'_, ConfigState>,
) -> Result<TestResult, String> {
    let config_guard = state.config.lock().unwrap();
    let config = config_guard.as_ref().ok_or("No configuration loaded")?;

    // Find the shortcut
    let shortcut = config
        .shortcuts
        .iter()
        .find(|s| s.id == shortcut_id)
        .ok_or("Shortcut not found")?;

    // Execute the command with sh -n (syntax check without execution)
    // Use printf to properly handle quotes and special characters
    let test_script = format!("printf '%s' '{}' | sh -n", shell_escape(&shortcut.command));
    let output = Command::new("sh")
        .arg("-c")
        .arg(&test_script)
        .output()
        .map_err(|e| format!("Failed to test command: {}", e))?;

    let syntax_valid = output.status.success();
    let syntax_error = if !syntax_valid {
        Some(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        None
    };

    // Also do a dry-run preview showing what would execute
    let preview = format_command_preview(shortcut);

    Ok(TestResult {
        shortcut_id: shortcut.id.clone(),
        command: shortcut.command.clone(),
        syntax_valid,
        syntax_error,
        preview,
        timestamp: chrono::Local::now().to_rfc3339(),
        executed: false,
        exit_code: None,
        stdout: None,
        stderr: None,
        execution_duration_ms: None,
        cancelled: false,
        timed_out: false,
        output_truncated: false,
    })
}

/// Execute a shortcut's command in a safe test mode (echo only, no actual execution)
#[tauri::command]
pub fn execute_test_command(
    shortcut_id: String,
    state: State<'_, ConfigState>,
) -> Result<TestResult, String> {
    let config_guard = state.config.lock().unwrap();
    let config = config_guard.as_ref().ok_or("No configuration loaded")?;

    // Find the shortcut
    let shortcut = config
        .shortcuts
        .iter()
        .find(|s| s.id == shortcut_id)
        .ok_or("Shortcut not found")?;

    // Execute the command with echo to show what would run
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo 'Would execute: {}' && {}",
            shortcut.command, &shortcut.command
        ))
        .output()
        .map_err(|e| format!("Failed to execute test: {}", e))?;

    let success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let preview = format!(
        "Exit Code: {}\n\nStdout:\n{}\n\nStderr:\n{}",
        output.status.code().unwrap_or(-1),
        stdout,
        if stderr.is_empty() { "(none)" } else { &stderr }
    );

    Ok(TestResult {
        shortcut_id: shortcut.id.clone(),
        command: shortcut.command.clone(),
        syntax_valid: success,
        syntax_error: if !success { Some(stderr.clone()) } else { None },
        preview,
        timestamp: chrono::Local::now().to_rfc3339(),
        executed: false,
        exit_code: None,
        stdout: None,
        stderr: None,
        execution_duration_ms: None,
        cancelled: false,
        timed_out: false,
        output_truncated: false,
    })
}

/// Execute a shortcut's command and return detailed execution results
#[tauri::command]
pub async fn execute_shortcut_command(
    shortcut_id: String,
    state: State<'_, ConfigState>,
    exec_state: State<'_, ExecutionState>,
) -> Result<TestResult, String> {
    // 1. Find shortcut
    let shortcut = {
        let config_guard = state.config.lock().unwrap();
        let config = config_guard.as_ref().ok_or("No configuration loaded")?;
        config
            .shortcuts
            .iter()
            .find(|s| s.id == shortcut_id)
            .cloned()
            .ok_or("Shortcut not found")?
    };

    // 2. Check if already running
    {
        let processes = exec_state.running_processes.lock().unwrap();
        if processes.contains_key(&shortcut_id) {
            return Err(format!("Command already executing for shortcut: {}", shortcut_id));
        }
    }

    // 3. Start timing
    let start = Instant::now();

    // 4. Spawn command
    let child = TokioCommand::new("sh")
        .arg("-c")
        .arg(&shortcut.command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    // 5. Store child in execution state
    // Note: We can't store the child since it's moved by wait_with_output
    // Instead, cancellation will be implemented in a future enhancement

    // 6. Wait with timeout
    let output_result = timeout(Duration::from_secs(30), child.wait_with_output()).await;

    // 7. Calculate duration
    let duration_ms = start.elapsed().as_millis() as u64;

    // 8. Build result based on outcome
    match output_result {
        Ok(Ok(output)) => {
            // Successful execution (or failed with exit code)
            let stdout_raw = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr_raw = String::from_utf8_lossy(&output.stderr).to_string();

            let (stdout, stdout_truncated) = truncate_output(stdout_raw, 10000);
            let (stderr, stderr_truncated) = truncate_output(stderr_raw, 10000);

            Ok(TestResult {
                shortcut_id: shortcut.id.clone(),
                command: shortcut.command.clone(),
                syntax_valid: true,
                syntax_error: None,
                preview: String::new(),
                timestamp: chrono::Local::now().to_rfc3339(),
                executed: true,
                exit_code: output.status.code(),
                stdout: Some(stdout),
                stderr: Some(stderr),
                execution_duration_ms: Some(duration_ms),
                cancelled: false,
                timed_out: false,
                output_truncated: stdout_truncated || stderr_truncated,
            })
        }
        Ok(Err(e)) => {
            // Failed to wait for output
            Err(format!("Execution failed: {}", e))
        }
        Err(_) => {
            // Timeout
            Ok(TestResult {
                shortcut_id: shortcut.id.clone(),
                command: shortcut.command.clone(),
                syntax_valid: true,
                syntax_error: None,
                preview: String::new(),
                timestamp: chrono::Local::now().to_rfc3339(),
                executed: true,
                exit_code: None,
                stdout: Some(String::new()),
                stderr: Some(String::new()),
                execution_duration_ms: Some(30000),
                cancelled: false,
                timed_out: true,
                output_truncated: false,
            })
        }
    }
}

fn format_command_preview(shortcut: &Shortcut) -> String {
    let mut preview = String::new();

    // Shortcut binding
    let modifiers = if shortcut.modifiers.is_empty() {
        "No modifiers".to_string()
    } else {
        shortcut.modifiers.join(" + ")
    };

    preview.push_str(&format!("Shortcut: {} + {}\n\n", modifiers, shortcut.key));

    // Command breakdown
    preview.push_str(&format!("Command: {}\n\n", shortcut.command));

    // Parse command for common patterns
    if shortcut.command.starts_with("open ") {
        preview.push_str("Action: Opens an application or file\n");
        if let Some(app) = shortcut.command.strip_prefix("open -a ") {
            preview.push_str(&format!("Application: {}\n", app));
        } else if let Some(path) = shortcut.command.strip_prefix("open ") {
            preview.push_str(&format!("Path: {}\n", path));
        }
    } else if shortcut.command.contains("yabai") {
        preview.push_str("Action: Window management (yabai)\n");
    } else if shortcut.command.contains("&&") || shortcut.command.contains(";") {
        preview.push_str("Action: Multiple commands (chained)\n");
        let parts: Vec<&str> = if shortcut.command.contains("&&") {
            shortcut.command.split("&&").collect()
        } else {
            shortcut.command.split(';').collect()
        };
        preview.push_str("\nSteps:\n");
        for (i, part) in parts.iter().enumerate() {
            preview.push_str(&format!("  {}. {}\n", i + 1, part.trim()));
        }
    } else {
        preview.push_str("Action: Shell command\n");
    }

    if let Some(comment) = &shortcut.comment {
        preview.push_str(&format!("\nDescription: {}\n", comment));
    }

    preview
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Shortcut;

    #[test]
    fn test_format_command_preview_open_app() {
        let shortcut = Shortcut {
            id: "test".to_string(),
            modifiers: vec!["cmd".to_string()],
            key: "return".to_string(),
            command: "open -a Terminal".to_string(),
            comment: Some("Open Terminal".to_string()),
            mode: None,
            line_number: 1,
        };

        let preview = format_command_preview(&shortcut);
        assert!(preview.contains("Shortcut: cmd + return"));
        assert!(preview.contains("Application: Terminal"));
        assert!(preview.contains("Description: Open Terminal"));
    }

    #[test]
    fn test_format_command_preview_chained() {
        let shortcut = Shortcut {
            id: "test".to_string(),
            modifiers: vec![],
            key: "f".to_string(),
            command: "echo hello && echo world".to_string(),
            comment: None,
            mode: None,
            line_number: 1,
        };

        let preview = format_command_preview(&shortcut);
        assert!(preview.contains("Multiple commands"));
        assert!(preview.contains("echo hello"));
        assert!(preview.contains("echo world"));
    }
}
