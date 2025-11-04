use crate::services::path_validator::{detect_interpreter, escape_shell_path, validate_file_executable};
use rfd::FileDialog;
use std::path::PathBuf;

/// Tauri command to open native file picker dialog
#[tauri::command]
pub async fn open_file_picker(start_directory: Option<String>) -> Result<Option<String>, String> {
    let mut dialog = FileDialog::new()
        .set_title("Select Script or Executable")
        .add_filter("All Files", &["*"])
        .add_filter("Scripts", &["sh", "bash", "py", "rb", "js", "pl"])
        .add_filter("Executables", &["app", ""]);

    // Set starting directory if provided
    if let Some(dir_path) = start_directory {
        dialog = dialog.set_directory(&dir_path);
    }

    // Open file picker dialog (blocking, but runs on async runtime)
    let file_handle = tokio::task::spawn_blocking(move || dialog.pick_file()).await
        .map_err(|e| format!("File picker task failed: {}", e))?;

    match file_handle {
        Some(path) => Ok(Some(path.display().to_string())),
        None => Ok(None), // User canceled
    }
}

/// Tauri command to validate if a file is executable
#[tauri::command]
pub async fn check_file_executable(file_path: String) -> Result<bool, String> {
    let path = PathBuf::from(&file_path);
    validate_file_executable(&path)
}

/// Tauri command to escape shell path for safe command execution
#[tauri::command]
pub async fn escape_path_for_shell(file_path: String) -> Result<String, String> {
    Ok(escape_shell_path(&file_path))
}

/// Tauri command to detect interpreter for script files
#[tauri::command]
pub async fn detect_script_interpreter(file_path: String) -> Result<Option<String>, String> {
    let path = PathBuf::from(&file_path);
    Ok(detect_interpreter(&path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_escape_path_for_shell() {
        let result = escape_path_for_shell("/tmp/normal.sh".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "'/tmp/normal.sh'");
    }

    #[tokio::test]
    async fn test_escape_path_with_spaces() {
        let result = escape_path_for_shell("/tmp/with spaces.sh".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "'/tmp/with spaces.sh'");
    }

    #[tokio::test]
    async fn test_detect_script_interpreter_sh() {
        let result = detect_script_interpreter("test.sh".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("bash".to_string()));
    }

    #[tokio::test]
    async fn test_detect_script_interpreter_py() {
        let result = detect_script_interpreter("test.py".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("python3".to_string()));
    }

    #[tokio::test]
    async fn test_detect_script_interpreter_unknown() {
        let result = detect_script_interpreter("test.txt".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}
