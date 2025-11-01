/// Path resolution utilities for skhd configuration files

use std::path::{Path, PathBuf};
use std::env;

/// Expand ~ in path to user's home directory
///
/// # Examples
/// ```
/// use skhd_gui_lib::utils::path::expand_path;
///
/// let path = expand_path("~/.config/skhd/skhdrc");
/// assert!(path.starts_with("/Users/") || path.starts_with("/home/"));
/// ```
pub fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_str = path.as_ref().to_string_lossy();

    if path_str.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            let home_path = PathBuf::from(home);
            let remainder = &path_str[2..]; // Skip "~/"
            return home_path.join(remainder);
        }
    } else if path_str == "~" {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home);
        }
    }

    // Return as-is if no expansion needed or HOME not available
    PathBuf::from(path.as_ref())
}

/// Get the default skhd configuration file path
///
/// Returns ~/.config/skhd/skhdrc (macOS standard location)
pub fn get_default_config_path() -> PathBuf {
    expand_path("~/.config/skhd/skhdrc")
}

/// Get the directory for skhd configuration files
pub fn get_config_dir() -> PathBuf {
    expand_path("~/.config/skhd")
}

/// Get the directory for application backups
pub fn get_backup_dir() -> PathBuf {
    expand_path("~/.config/skhd/backups")
}

/// Validate that a path is within the allowed skhd config directory
pub fn is_valid_config_path<P: AsRef<Path>>(path: P) -> bool {
    let expanded = expand_path(path);
    let config_dir = get_config_dir();

    expanded.starts_with(config_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_path("~/test/path");
        let home = env::var("HOME").unwrap();
        assert_eq!(expanded, PathBuf::from(home).join("test/path"));
    }

    #[test]
    fn test_expand_tilde_only() {
        let expanded = expand_path("~");
        let home = env::var("HOME").unwrap();
        assert_eq!(expanded, PathBuf::from(home));
    }

    #[test]
    fn test_no_expansion_needed() {
        let path = "/absolute/path";
        let expanded = expand_path(path);
        assert_eq!(expanded, PathBuf::from(path));
    }

    #[test]
    fn test_get_default_config_path() {
        let path = get_default_config_path();
        let home = env::var("HOME").unwrap();
        assert_eq!(
            path,
            PathBuf::from(home).join(".config/skhd/skhdrc")
        );
    }

    #[test]
    fn test_get_config_dir() {
        let dir = get_config_dir();
        let home = env::var("HOME").unwrap();
        assert_eq!(dir, PathBuf::from(home).join(".config/skhd"));
    }

    #[test]
    fn test_is_valid_config_path() {
        // Valid path within config directory
        assert!(is_valid_config_path("~/.config/skhd/skhdrc"));
        assert!(is_valid_config_path("~/.config/skhd/custom.conf"));

        // Invalid paths outside config directory
        assert!(!is_valid_config_path("~/Documents/file.txt"));
        assert!(!is_valid_config_path("/etc/passwd"));
    }
}
