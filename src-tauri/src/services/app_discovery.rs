use crate::models::application::Application;
use plist::Value;
use std::fs;
use std::path::PathBuf;

/// Discovers all installed macOS applications from standard directories
pub fn discover_applications() -> Result<Vec<Application>, String> {
    let search_paths = vec![
        PathBuf::from("/Applications"),
        PathBuf::from(format!(
            "{}/Applications",
            std::env::var("HOME").map_err(|_| "Cannot determine home directory")?
        )),
        PathBuf::from("/System/Applications"),
    ];

    let mut apps = Vec::new();

    for path in search_paths {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.extension() == Some(std::ffi::OsStr::new("app")) {
                    if let Ok(app) = parse_app_bundle(&entry_path) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    // Remove duplicates by bundle_id + app_path
    apps.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    apps.dedup_by(|a, b| a.bundle_id == b.bundle_id && a.app_path == b.app_path);

    Ok(apps)
}

/// Parses a macOS .app bundle to extract application metadata
pub fn parse_app_bundle(app_path: &PathBuf) -> Result<Application, String> {
    let info_plist_path = app_path.join("Contents/Info.plist");
    let plist: Value = Value::from_file(&info_plist_path)
        .map_err(|e| format!("Failed to parse Info.plist: {}", e))?;

    let dict = plist
        .as_dictionary()
        .ok_or("Info.plist is not a dictionary")?;

    let bundle_name = dict
        .get("CFBundleName")
        .or_else(|| dict.get("CFBundleDisplayName"))
        .and_then(|v| v.as_string())
        .ok_or("Missing CFBundleName")?;

    let bundle_id = dict
        .get("CFBundleIdentifier")
        .and_then(|v| v.as_string())
        .ok_or("Missing CFBundleIdentifier")?;

    let executable = dict
        .get("CFBundleExecutable")
        .and_then(|v| v.as_string())
        .ok_or("Missing CFBundleExecutable")?;

    let executable_path = app_path
        .join("Contents/MacOS")
        .join(executable)
        .display()
        .to_string();

    let icon_file = dict.get("CFBundleIconFile").and_then(|v| v.as_string());

    let icon_path = icon_file.map(|icon| {
        let mut path = app_path.join("Contents/Resources").join(icon);
        if path.extension().is_none() {
            path.set_extension("icns");
        }
        path.display().to_string()
    });

    let version = dict
        .get("CFBundleShortVersionString")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());

    Ok(Application {
        display_name: bundle_name.to_string(),
        app_path: app_path.display().to_string(),
        bundle_id: bundle_id.to_string(),
        executable_path,
        icon_path,
        version,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_applications() {
        let apps = discover_applications().unwrap();
        assert!(apps.len() > 0, "Should find at least one application");
        println!("Found {} applications", apps.len());
        for app in apps.iter().take(5) {
            println!("- {} ({})", app.display_name, app.bundle_id);
            println!("  Icon: {:?}", app.icon_path);
        }
    }

    #[test]
    fn test_parse_safari_bundle() {
        let safari_path = PathBuf::from("/Applications/Safari.app");
        if safari_path.exists() {
            let app = parse_app_bundle(&safari_path).unwrap();
            assert_eq!(app.display_name, "Safari");
            assert_eq!(app.bundle_id, "com.apple.Safari");
            assert!(app.executable_path.contains("Safari"));
        }
    }

    #[test]
    fn test_parse_app_bundle_validates_structure() {
        let nonexistent_path = PathBuf::from("/nonexistent/app.app");
        let result = parse_app_bundle(&nonexistent_path);
        assert!(result.is_err());
    }
}
