/// Configuration File model

use serde::{Deserialize, Serialize};
use super::shortcut::Shortcut;

/// Represents a parse error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseError {
    pub line_number: usize,
    pub column: Option<usize>,
    pub error_type: String,
    pub message: String,
    pub line_content: String,
}

/// Represents the complete skhd configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    /// Absolute path to config file
    pub file_path: String,

    /// List of keyboard shortcuts (ordered by line number)
    pub shortcuts: Vec<Shortcut>,

    /// Global comment lines not associated with shortcuts
    pub global_comments: Vec<String>,

    /// Last modification timestamp (ISO 8601)
    pub last_modified: String,

    /// Whether in-memory state differs from file
    pub is_modified: bool,

    /// Path to latest backup (if any)
    pub backup_path: Option<String>,

    /// Parse errors encountered (if any)
    pub parse_errors: Vec<ParseError>,
}

impl ConfigFile {
    /// Create a new empty config file
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            shortcuts: Vec::new(),
            global_comments: Vec::new(),
            last_modified: chrono::Utc::now().to_rfc3339(),
            is_modified: false,
            backup_path: None,
            parse_errors: Vec::new(),
        }
    }

    /// Add a shortcut to the configuration
    pub fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
        self.is_modified = true;
    }

    /// Remove a shortcut by ID
    pub fn remove_shortcut(&mut self, id: &str) -> Option<Shortcut> {
        if let Some(index) = self.shortcuts.iter().position(|s| s.id == id) {
            self.is_modified = true;
            Some(self.shortcuts.remove(index))
        } else {
            None
        }
    }

    /// Update a shortcut
    pub fn update_shortcut(&mut self, updated: Shortcut) -> bool {
        if let Some(shortcut) = self.shortcuts.iter_mut().find(|s| s.id == updated.id) {
            *shortcut = updated;
            self.is_modified = true;
            true
        } else {
            false
        }
    }

    /// Find shortcut by ID
    pub fn find_shortcut(&self, id: &str) -> Option<&Shortcut> {
        self.shortcuts.iter().find(|s| s.id == id)
    }

    /// Check for duplicate key combinations
    pub fn find_duplicates(&self, shortcut: &Shortcut) -> Vec<&Shortcut> {
        self.shortcuts
            .iter()
            .filter(|s| s.id != shortcut.id && s.has_same_combination(shortcut))
            .collect()
    }

    /// Validate all shortcuts
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for shortcut in &self.shortcuts {
            if let Err(e) = shortcut.validate() {
                errors.push(format!("Line {}: {}", shortcut.line_number, e));
            }
        }

        // Check for duplicates
        for (i, shortcut) in self.shortcuts.iter().enumerate() {
            for other in self.shortcuts.iter().skip(i + 1) {
                if shortcut.has_same_combination(other) {
                    errors.push(format!(
                        "Duplicate key combination '{}' at lines {} and {}",
                        shortcut.key_combination_string(),
                        shortcut.line_number,
                        other.line_number
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Sort shortcuts by line number
    pub fn sort_by_line_number(&mut self) {
        self.shortcuts.sort_by_key(|s| s.line_number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::shortcut::Shortcut;

    #[test]
    fn test_new_config() {
        let config = ConfigFile::new("/test/path".to_string());
        assert_eq!(config.file_path, "/test/path");
        assert_eq!(config.shortcuts.len(), 0);
        assert!(!config.is_modified);
    }

    #[test]
    fn test_add_shortcut() {
        let mut config = ConfigFile::new("/test/path".to_string());
        let shortcut = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "open -a Terminal".to_string(),
            1,
        );

        config.add_shortcut(shortcut);
        assert_eq!(config.shortcuts.len(), 1);
        assert!(config.is_modified);
    }

    #[test]
    fn test_remove_shortcut() {
        let mut config = ConfigFile::new("/test/path".to_string());
        let shortcut = Shortcut::new(
            vec!["cmd".to_string()],
            "return".to_string(),
            "open -a Terminal".to_string(),
            1,
        );
        let id = shortcut.id.clone();

        config.add_shortcut(shortcut);
        let removed = config.remove_shortcut(&id);
        assert!(removed.is_some());
        assert_eq!(config.shortcuts.len(), 0);
    }

    #[test]
    fn test_find_duplicates() {
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

        config.add_shortcut(s1.clone());
        config.add_shortcut(s2.clone());

        let duplicates = config.find_duplicates(&s1);
        assert_eq!(duplicates.len(), 1);
    }
}
