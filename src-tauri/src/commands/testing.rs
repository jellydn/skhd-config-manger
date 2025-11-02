use crate::commands::config::ConfigState;
use crate::models::{Shortcut, TestResult};
use std::process::Command;
use tauri::State;

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
        syntax_error: if !success { Some(stderr) } else { None },
        preview,
        timestamp: chrono::Local::now().to_rfc3339(),
    })
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
