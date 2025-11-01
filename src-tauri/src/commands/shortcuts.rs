/// Shortcut management Tauri commands
use crate::commands::config::ConfigState;
use crate::models::Shortcut;
use tauri::State;

/// Request to create a new shortcut
#[derive(serde::Deserialize)]
pub struct CreateShortcutRequest {
    pub modifiers: Vec<String>,
    pub key: String,
    pub command: String,
    pub mode: Option<String>,
    pub comment: Option<String>,
}

/// Request to update an existing shortcut
#[derive(serde::Deserialize)]
pub struct UpdateShortcutRequest {
    pub id: String,
    pub modifiers: Vec<String>,
    pub key: String,
    pub command: String,
    pub mode: Option<String>,
    pub comment: Option<String>,
}

/// Create a new shortcut
///
/// # Arguments
/// * `request` - Shortcut creation request
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Shortcut)` - Created shortcut
/// * `Err(String)` - Error message
#[tauri::command]
pub fn create_shortcut(
    request: CreateShortcutRequest,
    state: State<'_, ConfigState>,
) -> Result<Shortcut, String> {
    let mut config_guard = state.config.lock().unwrap();
    let config = config_guard.as_mut().ok_or("No config loaded")?;

    // Determine line number (append to end)
    let line_number = config
        .shortcuts
        .last()
        .map(|s| s.line_number + 1)
        .unwrap_or(1);

    // Create shortcut
    let mut shortcut = Shortcut::new(request.modifiers, request.key, request.command, line_number);

    shortcut.mode = request.mode;
    shortcut.comment = request.comment;

    // Validate
    shortcut
        .validate()
        .map_err(|e| format!("Invalid shortcut: {}", e))?;

    // Check for duplicates
    let duplicates = config.find_duplicates(&shortcut);
    if !duplicates.is_empty() {
        return Err(format!(
            "Duplicate key combination: {}",
            shortcut.key_combination_string()
        ));
    }

    // Add to config
    config.add_shortcut(shortcut.clone());
    config.is_modified = true;

    Ok(shortcut)
}

/// Update an existing shortcut
///
/// # Arguments
/// * `request` - Shortcut update request
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Shortcut)` - Updated shortcut
/// * `Err(String)` - Error message
#[tauri::command]
pub fn update_shortcut(
    request: UpdateShortcutRequest,
    state: State<'_, ConfigState>,
) -> Result<Shortcut, String> {
    let mut config_guard = state.config.lock().unwrap();
    let config = config_guard.as_mut().ok_or("No config loaded")?;

    // Find existing shortcut
    let existing = config
        .find_shortcut(&request.id)
        .ok_or("Shortcut not found")?;

    // Create updated shortcut (preserving line number)
    let mut updated = Shortcut::with_id(
        request.id.clone(),
        request.modifiers,
        request.key,
        request.command,
        existing.line_number,
    );

    updated.mode = request.mode;
    updated.comment = request.comment;

    // Validate
    updated
        .validate()
        .map_err(|e| format!("Invalid shortcut: {}", e))?;

    // Check for duplicates (excluding this shortcut)
    let duplicates = config.find_duplicates(&updated);
    if !duplicates.is_empty() {
        return Err(format!(
            "Duplicate key combination: {}",
            updated.key_combination_string()
        ));
    }

    // Update in config
    config.update_shortcut(updated.clone());
    config.is_modified = true;

    Ok(updated)
}

/// Delete a shortcut by ID
///
/// # Arguments
/// * `id` - Shortcut ID
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` on failure
#[tauri::command]
pub fn delete_shortcut(id: String, state: State<'_, ConfigState>) -> Result<(), String> {
    let mut config_guard = state.config.lock().unwrap();
    let config = config_guard.as_mut().ok_or("No config loaded")?;

    config.remove_shortcut(&id).ok_or("Shortcut not found")?;
    config.is_modified = true;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shortcut_request() {
        let request = CreateShortcutRequest {
            modifiers: vec!["cmd".to_string()],
            key: "return".to_string(),
            command: "open -a Terminal".to_string(),
            mode: None,
            comment: Some("Terminal shortcut".to_string()),
        };

        assert_eq!(request.modifiers, vec!["cmd"]);
        assert_eq!(request.key, "return");
    }

    #[test]
    fn test_update_shortcut_request() {
        let request = UpdateShortcutRequest {
            id: "test-id".to_string(),
            modifiers: vec!["cmd".to_string(), "shift".to_string()],
            key: "f".to_string(),
            command: "open ~".to_string(),
            mode: None,
            comment: None,
        };

        assert_eq!(request.id, "test-id");
        assert_eq!(request.modifiers.len(), 2);
    }
}
