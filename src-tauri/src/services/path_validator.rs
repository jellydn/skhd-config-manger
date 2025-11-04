use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Escapes a shell path by wrapping it in single quotes and escaping any single quotes within
pub fn escape_shell_path(path: &str) -> String {
    format!("'{}'", path.replace("'", r"'\''"))
}

/// Validates if a file exists and is executable
pub fn validate_file_executable(path: &Path) -> Result<bool, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }

    let metadata = path
        .metadata()
        .map_err(|e| format!("Cannot read file metadata: {}", e))?;

    let permissions = metadata.permissions();
    let is_executable = permissions.mode() & 0o111 != 0;

    Ok(is_executable)
}

/// Detects the appropriate interpreter for a script file based on its extension
pub fn detect_interpreter(path: &Path) -> Option<String> {
    match path.extension()?.to_str()? {
        "sh" | "bash" => Some("bash".to_string()),
        "zsh" => Some("zsh".to_string()),
        "py" => Some("python3".to_string()),
        "rb" => Some("ruby".to_string()),
        "js" => Some("node".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_escape_shell_path() {
        assert_eq!(escape_shell_path("/tmp/normal.sh"), "'/tmp/normal.sh'");
        assert_eq!(
            escape_shell_path("/tmp/with spaces.sh"),
            "'/tmp/with spaces.sh'"
        );
        assert_eq!(
            escape_shell_path("/tmp/quote's.sh"),
            r"'/tmp/quote'\''s.sh'"
        );
        assert_eq!(
            escape_shell_path(r#"/tmp/$special`chars".sh"#),
            r#"'/tmp/$special`chars".sh'"#
        );
    }

    #[test]
    fn test_detect_interpreter() {
        assert_eq!(
            detect_interpreter(&PathBuf::from("script.sh")),
            Some("bash".to_string())
        );
        assert_eq!(
            detect_interpreter(&PathBuf::from("script.py")),
            Some("python3".to_string())
        );
        assert_eq!(
            detect_interpreter(&PathBuf::from("script.rb")),
            Some("ruby".to_string())
        );
        assert_eq!(
            detect_interpreter(&PathBuf::from("script.js")),
            Some("node".to_string())
        );
        assert_eq!(detect_interpreter(&PathBuf::from("unknown.txt")), None);
    }
}
