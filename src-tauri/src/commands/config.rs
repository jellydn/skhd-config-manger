/// Configuration management Tauri commands

use crate::models::{ConfigFile, Shortcut};
use crate::parser::parse_config;
use crate::services::file_io::{read_config_safe, write_config_atomic};
use crate::utils::path::{expand_path, get_default_config_path};
use std::sync::Mutex;
use tauri::State;

/// Global state for the current configuration
pub struct ConfigState {
    pub config: Mutex<Option<ConfigFile>>,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(None),
        }
    }
}

/// Load skhd configuration from file
///
/// # Arguments
/// * `file_path` - Optional custom path (defaults to ~/.config/skhd/skhdrc)
/// * `state` - Application state
///
/// # Returns
/// * `Ok(ConfigFile)` on success
/// * `Err(String)` on failure
#[tauri::command]
pub fn load_config(
    file_path: Option<String>,
    state: State<'_, ConfigState>,
) -> Result<ConfigFile, String> {
    let path = if let Some(p) = file_path {
        expand_path(p)
    } else {
        get_default_config_path()
    };

    // Read file
    let content = read_config_safe(&path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    // Parse content
    let parsed = parse_config(&content)
        .map_err(|errors| {
            format!("Failed to parse config: {} errors", errors.len())
        })?;

    // Convert parsed config to ConfigFile
    let mut config = ConfigFile::new(path.to_string_lossy().to_string());

    for parsed_shortcut in parsed.shortcuts() {
        let shortcut = Shortcut::new(
            parsed_shortcut.modifiers.clone(),
            parsed_shortcut.key.clone(),
            parsed_shortcut.command.clone(),
            parsed_shortcut.line_number,
        );
        config.add_shortcut(shortcut);
    }

    // Extract comments
    for parsed_comment in parsed.comments() {
        config.global_comments.push(parsed_comment.text.clone());
    }

    // Reset modified flag since we just loaded
    config.is_modified = false;

    // Update state
    *state.config.lock().unwrap() = Some(config.clone());

    Ok(config)
}

/// Save configuration to file
///
/// # Arguments
/// * `config` - Configuration to save
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` on failure
#[tauri::command]
pub fn save_config(
    config: ConfigFile,
    state: State<'_, ConfigState>,
) -> Result<(), String> {
    // Serialize config back to skhd format
    let content = serialize_config(&config);

    // Write atomically
    write_config_atomic(&config.file_path, &content)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    // Update state with saved version
    let mut saved_config = config.clone();
    saved_config.is_modified = false;
    *state.config.lock().unwrap() = Some(saved_config);

    Ok(())
}

/// Reload configuration from disk (discarding in-memory changes)
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// * `Ok(ConfigFile)` on success
/// * `Err(String)` on failure
#[tauri::command]
pub fn reload_config(state: State<'_, ConfigState>) -> Result<ConfigFile, String> {
    let current = state.config.lock().unwrap();
    let file_path = current
        .as_ref()
        .map(|c| c.file_path.clone())
        .ok_or("No config loaded")?;

    drop(current); // Release lock before calling load_config

    load_config(Some(file_path), state)
}

/// Serialize ConfigFile back to skhd configuration format
///
/// This ensures round-trip compatibility: parse → modify → serialize → parse
pub fn serialize_config(config: &ConfigFile) -> String {
    let mut output = String::new();

    // Add global comments at the top
    for comment in &config.global_comments {
        output.push_str("# ");
        output.push_str(comment);
        output.push('\n');
    }

    if !config.global_comments.is_empty() {
        output.push('\n');
    }

    // Sort shortcuts by line number to maintain order
    let mut shortcuts = config.shortcuts.clone();
    shortcuts.sort_by_key(|s| s.line_number);

    // Serialize each shortcut
    for shortcut in shortcuts {
        // Add comment if present
        if let Some(comment) = &shortcut.comment {
            output.push_str("# ");
            output.push_str(comment);
            output.push('\n');
        }

        // Build modifier string
        let modifier_str = if shortcut.modifiers.is_empty() {
            String::new()
        } else {
            let mut mods = shortcut.modifiers.clone();
            mods.sort(); // Ensure consistent ordering
            format!("{} + ", mods.join(" + "))
        };

        // Write shortcut line: [modifiers +] - key : command
        output.push_str(&format!(
            "{}- {} : {}\n",
            modifier_str, shortcut.key, shortcut.command
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_config() {
        let mut config = ConfigFile::new("/test/path".to_string());

        // Add global comment
        config.global_comments.push("Global config".to_string());

        // Add shortcuts
        let s1 = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "open -a Terminal".to_string(),
            1,
        );

        let s2 = Shortcut::new(
            vec!["cmd".to_string(), "shift".to_string()],
            "f".to_string(),
            "open ~".to_string(),
            2,
        );

        config.add_shortcut(s1);
        config.add_shortcut(s2);

        let serialized = serialize_config(&config);
        eprintln!("Serialized output:\n{}", serialized);

        // Verify format
        assert!(serialized.contains("# Global config"));
        assert!(serialized.contains("cmd + - return : open -a Terminal"));
        assert!(serialized.contains("cmd + shift + - f : open ~"));
    }

    #[test]
    fn test_serialize_config_no_modifiers() {
        let mut config = ConfigFile::new("/test/path".to_string());

        let shortcut = Shortcut::new(
            vec![],
            "f1".to_string(),
            "echo test".to_string(),
            1,
        );

        config.add_shortcut(shortcut);

        let serialized = serialize_config(&config);
        assert!(serialized.contains("- f1 : echo test"));
    }
}
