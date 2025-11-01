/// Atomic file I/O service for safe configuration file operations
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tempfile::NamedTempFile;

/// Safely read configuration file
///
/// Returns the file contents as a String
pub fn read_config_safe<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Atomically write configuration file using temp file + rename pattern
///
/// This ensures the original file is never corrupted even if the write fails
/// or the process is interrupted. The write either succeeds completely or fails
/// with no side effects.
///
/// # Arguments
/// * `path` - Destination file path
/// * `content` - Configuration content to write
///
/// # Returns
/// * `Ok(())` on successful atomic write
/// * `Err(io::Error)` if operation fails
///
/// # Safety
/// This uses the atomic rename pattern:
/// 1. Write to temporary file in same directory
/// 2. Sync temp file to disk
/// 3. Atomically rename temp file to target (POSIX atomic operation)
pub fn write_config_atomic<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let path = path.as_ref();

    // Get parent directory for temp file
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Path has no parent directory")
    })?;

    // Ensure parent directory exists
    fs::create_dir_all(parent)?;

    // Create temp file in same directory (required for atomic rename)
    let mut temp_file = NamedTempFile::new_in(parent)?;

    // Write content to temp file
    temp_file.write_all(content.as_bytes())?;

    // Sync to disk before rename
    temp_file.as_file().sync_all()?;

    // Atomically replace target file with temp file
    temp_file.persist(path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_and_read_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.conf");

        let content = "# Test config\ncmd - return : open -a Terminal\n";

        // Write
        write_config_atomic(&config_path, content).unwrap();

        // Verify file exists
        assert!(config_path.exists());

        // Read back
        let read_content = read_config_safe(&config_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_atomic_write_overwrites_existing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.conf");

        // Write initial content
        let content1 = "# Original\n";
        write_config_atomic(&config_path, content1).unwrap();

        // Overwrite with new content
        let content2 = "# Updated\n";
        write_config_atomic(&config_path, content2).unwrap();

        // Verify new content
        let read_content = read_config_safe(&config_path).unwrap();
        assert_eq!(read_content, content2);
    }

    #[test]
    fn test_atomic_write_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("a/b/c/test.conf");

        let content = "# Test\n";
        write_config_atomic(&nested_path, content).unwrap();

        assert!(nested_path.exists());
        let read_content = read_config_safe(&nested_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_config_safe("/nonexistent/path/file.conf");
        assert!(result.is_err());
    }
}
