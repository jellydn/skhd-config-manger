/// Backup model for config file versioning
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a backup of a configuration file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    /// Absolute path to the backup file
    pub file_path: PathBuf,

    /// Path to the original config file
    pub original_path: PathBuf,

    /// Timestamp when backup was created (ISO 8601)
    pub created_at: String,

    /// SHA-256 checksum of backup content for integrity verification
    pub checksum: String,

    /// Human-readable description of backup reason
    pub description: Option<String>,

    /// File size in bytes
    pub size_bytes: u64,
}

impl Backup {
    /// Create a new backup record
    pub fn new(
        file_path: PathBuf,
        original_path: PathBuf,
        checksum: String,
        size_bytes: u64,
    ) -> Self {
        Self {
            file_path,
            original_path,
            created_at: chrono::Utc::now().to_rfc3339(),
            checksum,
            description: None,
            size_bytes,
        }
    }

    /// Create backup with description
    pub fn with_description(
        file_path: PathBuf,
        original_path: PathBuf,
        checksum: String,
        size_bytes: u64,
        description: String,
    ) -> Self {
        Self {
            file_path,
            original_path,
            created_at: chrono::Utc::now().to_rfc3339(),
            checksum,
            description: Some(description),
            size_bytes,
        }
    }

    /// Verify backup integrity by comparing checksums
    pub fn verify_checksum(&self, expected_checksum: &str) -> bool {
        self.checksum == expected_checksum
    }

    /// Get backup age in seconds
    pub fn age_seconds(&self) -> i64 {
        if let Ok(created) = chrono::DateTime::parse_from_rfc3339(&self.created_at) {
            let now = chrono::Utc::now();
            (now.timestamp() - created.timestamp()).abs()
        } else {
            0
        }
    }

    /// Check if backup exists on filesystem
    pub fn exists(&self) -> bool {
        self.file_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new_backup() {
        let backup = Backup::new(
            PathBuf::from("/tmp/backup.skhdrc"),
            PathBuf::from("/home/user/.config/skhd/skhdrc"),
            "abc123".to_string(),
            1024,
        );

        assert_eq!(backup.file_path, PathBuf::from("/tmp/backup.skhdrc"));
        assert_eq!(
            backup.original_path,
            PathBuf::from("/home/user/.config/skhd/skhdrc")
        );
        assert_eq!(backup.checksum, "abc123");
        assert_eq!(backup.size_bytes, 1024);
        assert!(backup.description.is_none());
        assert!(!backup.created_at.is_empty());
    }

    #[test]
    fn test_backup_with_description() {
        let backup = Backup::with_description(
            PathBuf::from("/tmp/backup.skhdrc"),
            PathBuf::from("/home/user/.config/skhd/skhdrc"),
            "abc123".to_string(),
            1024,
            "Before editing shortcuts".to_string(),
        );

        assert_eq!(
            backup.description,
            Some("Before editing shortcuts".to_string())
        );
    }

    #[test]
    fn test_verify_checksum() {
        let backup = Backup::new(
            PathBuf::from("/tmp/backup.skhdrc"),
            PathBuf::from("/home/user/.config/skhd/skhdrc"),
            "abc123".to_string(),
            1024,
        );

        assert!(backup.verify_checksum("abc123"));
        assert!(!backup.verify_checksum("xyz789"));
    }

    #[test]
    fn test_age_seconds() {
        let backup = Backup::new(
            PathBuf::from("/tmp/backup.skhdrc"),
            PathBuf::from("/home/user/.config/skhd/skhdrc"),
            "abc123".to_string(),
            1024,
        );

        // Age should be close to 0 for newly created backup
        let age = backup.age_seconds();
        assert!(age >= 0 && age < 2); // Allow 2 second margin
    }
}
