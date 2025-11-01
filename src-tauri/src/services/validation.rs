/// Validation service for shortcuts and configurations
use crate::models::{ConfigFile, Shortcut};
use crate::parser::parse_config;

/// Validation result containing errors and warnings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a single shortcut
///
/// Checks:
/// - Key and command are not empty
/// - Modifiers are valid (cmd, alt, shift, ctrl, fn)
/// - No duplicate modifiers
pub fn validate_shortcut(shortcut: &Shortcut) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Use the shortcut's own validation
    if let Err(e) = shortcut.validate() {
        result.add_error(e);
    }

    // Additional warnings
    if shortcut.command.len() > 500 {
        result.add_warning("Command is very long (>500 chars)".to_string());
    }

    if shortcut.modifiers.is_empty() {
        result.add_warning(
            "Shortcut has no modifiers - may conflict with system shortcuts".to_string(),
        );
    }

    result
}

/// Validate an entire configuration file
///
/// Checks:
/// - All shortcuts are individually valid
/// - No duplicate key combinations
/// - No system shortcuts conflicts (basic check)
pub fn validate_config(config: &ConfigFile) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Validate each shortcut
    for shortcut in &config.shortcuts {
        let shortcut_result = validate_shortcut(shortcut);
        result.errors.extend(shortcut_result.errors);
        result.warnings.extend(shortcut_result.warnings);
    }

    // Use config's own validation for duplicates
    if let Err(errors) = config.validate() {
        for error in errors {
            result.add_error(error);
        }
    }

    // Check for common system shortcuts conflicts
    let system_shortcuts = [
        ("cmd", "space"),     // Spotlight
        ("cmd", "tab"),       // App switcher
        ("cmd", "q"),         // Quit
        ("cmd + shift", "3"), // Screenshot
        ("cmd + shift", "4"), // Screenshot selection
        ("cmd + ctrl", "q"),  // Lock screen
    ];

    for shortcut in &config.shortcuts {
        let mods = shortcut.modifiers.join(" + ");
        for (sys_mod, sys_key) in &system_shortcuts {
            if mods == *sys_mod && &shortcut.key == sys_key {
                result.add_warning(format!(
                    "Shortcut {} - {} conflicts with system shortcut",
                    mods, shortcut.key
                ));
            }
        }
    }

    // Update validity flag
    result.is_valid = result.errors.is_empty();

    result
}

/// Validate skhd configuration text
///
/// Parses the text and validates the resulting configuration
pub fn validate_config_text(text: &str) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Try to parse
    match parse_config(text) {
        Ok(parsed) => {
            // Convert to ConfigFile
            let mut config = ConfigFile::new(String::from("<text>"));
            for parsed_shortcut in parsed.shortcuts() {
                let shortcut = Shortcut::new(
                    parsed_shortcut.modifiers.clone(),
                    parsed_shortcut.key.clone(),
                    parsed_shortcut.command.clone(),
                    parsed_shortcut.line_number,
                );
                config.add_shortcut(shortcut);
            }

            // Validate config
            let config_result = validate_config(&config);
            result.errors.extend(config_result.errors);
            result.warnings.extend(config_result.warnings);
        }
        Err(parse_errors) => {
            for error in parse_errors {
                result.add_error(format!("Line {}: {}", error.line_number, error.message));
            }
        }
    }

    result.is_valid = result.errors.is_empty();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_shortcut() {
        let shortcut = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "open -a Terminal".to_string(),
            1,
        );

        let result = validate_shortcut(&shortcut);
        assert!(result.is_valid);
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn test_validate_shortcut_empty_command() {
        let shortcut = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "".to_string(),
            1,
        );

        let result = validate_shortcut(&shortcut);
        assert!(!result.is_valid);
        assert!(result.errors.len() > 0);
    }

    #[test]
    fn test_validate_config_with_duplicates() {
        let mut config = ConfigFile::new("/test/path".to_string());

        let s1 = Shortcut::new(
            vec!["cmd".to_string()],
            "f".to_string(),
            "command1".to_string(),
            1,
        );

        let s2 = Shortcut::new(
            vec!["cmd".to_string()],
            "f".to_string(),
            "command2".to_string(),
            2,
        );

        config.add_shortcut(s1);
        config.add_shortcut(s2);

        let result = validate_config(&config);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("Duplicate")));
    }

    #[test]
    fn test_validate_config_text() {
        let text = "cmd - return : open -a Terminal\n";
        let result = validate_config_text(text);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_config_text_invalid() {
        let text = "invalid syntax here\n";
        let result = validate_config_text(text);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_system_shortcut_warning() {
        let mut config = ConfigFile::new("/test/path".to_string());

        // Add a shortcut that conflicts with Spotlight
        let shortcut = Shortcut::new(
            vec!["cmd".to_string()],
            "space".to_string(),
            "my_command".to_string(),
            1,
        );

        config.add_shortcut(shortcut);

        let result = validate_config(&config);
        assert!(result
            .warnings
            .iter()
            .any(|w| w.contains("system shortcut")));
    }
}
