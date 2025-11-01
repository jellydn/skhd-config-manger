/// Keyboard Shortcut model
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single skhd configuration entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shortcut {
    /// Unique identifier for tracking during editing
    pub id: String,

    /// Modifier keys (cmd, alt, shift, ctrl, fn) - order-independent
    pub modifiers: Vec<String>,

    /// Primary key being pressed
    pub key: String,

    /// Shell command to execute
    pub command: String,

    /// Optional mode name for modal shortcuts
    pub mode: Option<String>,

    /// Optional inline comment
    pub comment: Option<String>,

    /// Original line number from config file
    pub line_number: usize,
}

impl Shortcut {
    /// Create a new shortcut with generated UUID
    pub fn new(modifiers: Vec<String>, key: String, command: String, line_number: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            modifiers,
            key,
            command,
            mode: None,
            comment: None,
            line_number,
        }
    }

    /// Create shortcut with explicit ID (for loading from config)
    pub fn with_id(
        id: String,
        modifiers: Vec<String>,
        key: String,
        command: String,
        line_number: usize,
    ) -> Self {
        Self {
            id,
            modifiers,
            key,
            command,
            mode: None,
            comment: None,
            line_number,
        }
    }

    /// Get the key combination as a string for display
    pub fn key_combination_string(&self) -> String {
        if self.modifiers.is_empty() {
            self.key.clone()
        } else {
            let mut sorted_modifiers = self.modifiers.clone();
            sorted_modifiers.sort();
            format!("{} - {}", sorted_modifiers.join(" + "), self.key)
        }
    }

    /// Check if this shortcut has the same key combination as another
    pub fn has_same_combination(&self, other: &Shortcut) -> bool {
        if self.key != other.key {
            return false;
        }

        let mut self_mods = self.modifiers.clone();
        let mut other_mods = other.modifiers.clone();
        self_mods.sort();
        other_mods.sort();

        self_mods == other_mods && self.mode == other.mode
    }

    /// Validate the shortcut
    pub fn validate(&self) -> Result<(), String> {
        // Check command is not empty
        if self.command.trim().is_empty() {
            return Err("Command cannot be empty".to_string());
        }

        // Check key is not empty
        if self.key.trim().is_empty() {
            return Err("Key cannot be empty".to_string());
        }

        // Check modifiers are valid
        let valid_modifiers = ["cmd", "alt", "shift", "ctrl", "fn"];
        for modifier in &self.modifiers {
            if !valid_modifiers.contains(&modifier.as_str()) {
                return Err(format!("Invalid modifier: {}", modifier));
            }
        }

        // Check for duplicate modifiers
        let mut unique_mods = self.modifiers.clone();
        unique_mods.sort();
        unique_mods.dedup();
        if unique_mods.len() != self.modifiers.len() {
            return Err("Duplicate modifiers not allowed".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shortcut() {
        let shortcut = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "open -a Terminal".to_string(),
            1,
        );

        assert!(!shortcut.id.is_empty());
        assert_eq!(shortcut.modifiers, vec!["cmd"]);
        assert_eq!(shortcut.key, "return");
        assert_eq!(shortcut.command, "open -a Terminal");
    }

    #[test]
    fn test_key_combination_string() {
        let shortcut = Shortcut::new(
            vec!["cmd".to_string(), "shift".to_string()],
            "f".to_string(),
            "open ~".to_string(),
            1,
        );

        assert_eq!(shortcut.key_combination_string(), "cmd + shift - f");
    }

    #[test]
    fn test_has_same_combination() {
        let s1 = Shortcut::new(
            vec!["cmd".to_string(), "shift".to_string()],
            "f".to_string(),
            "cmd1".to_string(),
            1,
        );

        let s2 = Shortcut::new(
            vec!["shift".to_string(), "cmd".to_string()],
            "f".to_string(),
            "cmd2".to_string(),
            2,
        );

        assert!(s1.has_same_combination(&s2));
    }

    #[test]
    fn test_validate() {
        let valid = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "open -a Terminal".to_string(),
            1,
        );
        assert!(valid.validate().is_ok());

        let invalid_empty_cmd = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "".to_string(),
            1,
        );
        assert!(invalid_empty_cmd.validate().is_err());
    }
}
