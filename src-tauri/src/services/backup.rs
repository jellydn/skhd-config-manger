/// Backup service with SHA-256 checksums for configuration files
use crate::models::Backup;
use crate::services::file_io::write_config_atomic;
use crate::utils::path::{expand_path, get_backup_dir};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::Path;

/// Create a backup of a configuration file with SHA-256 checksum
///
/// # Arguments
/// * `source_path` - Path to file to backup
/// * `description` - Optional description of why backup was created
///
/// # Returns
/// * `Ok(Backup)` containing backup metadata
/// * `Err(io::Error)` if backup fails
pub fn create_backup<P: AsRef<Path>>(
    source_path: P,
    description: Option<String>,
) -> io::Result<Backup> {
    let source_path = expand_path(source_path);

    // Read source file
    let content = fs::read(&source_path)?;

    // Calculate checksum
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let checksum = format!("{:x}", hasher.finalize());

    // Create backup directory if it doesn't exist
    let backup_dir = get_backup_dir();
    fs::create_dir_all(&backup_dir)?;

    // Generate backup filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let source_filename = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("config");
    let backup_filename = format!("{}_{}.backup", source_filename, timestamp);
    let backup_path = backup_dir.join(backup_filename);

    // Write backup file
    fs::write(&backup_path, &content)?;

    // Create backup record
    let backup = if let Some(desc) = description {
        Backup::with_description(
            backup_path,
            source_path,
            checksum,
            content.len() as u64,
            desc,
        )
    } else {
        Backup::new(backup_path, source_path, checksum, content.len() as u64)
    };

    Ok(backup)
}

/// List all backups in the backup directory
pub fn list_backups() -> io::Result<Vec<Backup>> {
    let backup_dir = get_backup_dir();

    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();

    for entry in fs::read_dir(&backup_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("backup") {
            // Read file to calculate checksum
            let content = fs::read(&path)?;
            let mut hasher = Sha256::new();
            hasher.update(&content);
            let checksum = format!("{:x}", hasher.finalize());

            // Try to infer original path from filename
            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let original_name = filename.split('_').next().unwrap_or("skhdrc");
            let original_path = expand_path(format!("~/.config/skhd/{}", original_name));

            let backup = Backup::new(path, original_path, checksum, content.len() as u64);

            backups.push(backup);
        }
    }

    // Sort by creation time (newest first)
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(backups)
}

/// Restore a configuration file from a backup
///
/// # Arguments
/// * `backup` - Backup record to restore from
/// * `target_path` - Optional target path (defaults to backup's original_path)
///
/// # Returns
/// * `Ok(())` on successful restore
/// * `Err(io::Error)` if restore fails
pub fn restore_backup<P: AsRef<Path>>(backup: &Backup, target_path: Option<P>) -> io::Result<()> {
    let target = if let Some(path) = target_path {
        expand_path(path)
    } else {
        backup.original_path.clone()
    };

    // Read backup file
    let content = fs::read(&backup.file_path)?;

    // Verify checksum
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let checksum = format!("{:x}", hasher.finalize());

    if checksum != backup.checksum {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Backup checksum mismatch - file may be corrupted",
        ));
    }

    // Convert content to string for atomic write
    let content_str = String::from_utf8_lossy(&content);

    // Write to target atomically
    write_config_atomic(&target, &content_str)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to restore backup: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Digest;
    use tempfile::TempDir;

    #[test]
    #[ignore] // Ignore in parallel test runs due to shared backup directory
    fn test_create_and_restore_backup() {
        use std::thread;
        use std::time::Duration;

        let temp_dir = TempDir::new().unwrap();
        let source_path = temp_dir.path().join("test.conf");
        let target_path = temp_dir.path().join("restored.conf");
        let content = "# Test config\ncmd - return : open -a Terminal\n";

        // Create source file
        fs::write(&source_path, content).unwrap();

        // Create backup (goes to ~/.config/skhd/backups)
        let backup = create_backup(&source_path, Some("Test backup".to_string())).unwrap();

        // Small delay to ensure file is fully written
        thread::sleep(Duration::from_millis(10));

        // Verify backup exists and has correct checksum
        assert!(backup.file_path.exists());
        assert_eq!(backup.description, Some("Test backup".to_string()));

        // Verify checksum matches
        let backup_content = fs::read(&backup.file_path).unwrap();
        let mut hasher = sha2::Sha256::new();
        hasher.update(&backup_content);
        let actual_checksum = format!("{:x}", hasher.finalize());
        assert_eq!(actual_checksum, backup.checksum, "Backup checksum mismatch");

        // Restore from backup to a different location
        restore_backup(&backup, Some(&target_path)).unwrap();

        // Verify restored content
        let restored_content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(restored_content, content);

        // Cleanup backup file
        let _ = fs::remove_file(&backup.file_path);
    }

    #[test]
    fn test_backup_checksum_verification() {
        let temp_dir = TempDir::new().unwrap();
        let source_path = temp_dir.path().join("test.conf");
        let content = "# Original\n";

        fs::write(&source_path, content).unwrap();

        let mut backup = create_backup(&source_path, None).unwrap();

        // Corrupt the checksum
        backup.checksum = "invalid_checksum".to_string();

        // Restore should fail
        let result = restore_backup(&backup, Some(&source_path));
        assert!(result.is_err());
    }

    #[test]
    fn test_list_backups() {
        // This test depends on backup directory state, so we'll just check it runs
        let result = list_backups();
        assert!(result.is_ok());
    }
}
