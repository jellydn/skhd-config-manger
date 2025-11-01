/// Validation Tauri commands
use crate::models::{ConfigFile, Shortcut};
use crate::services::validation::{
    validate_config as validate_config_service, validate_shortcut as validate_shortcut_service,
    ValidationResult,
};

/// Validate a shortcut
///
/// # Arguments
/// * `shortcut` - Shortcut to validate
///
/// # Returns
/// * `ValidationResult` with errors and warnings
#[tauri::command]
pub fn validate_shortcut(shortcut: Shortcut) -> Result<ValidationResult, String> {
    Ok(validate_shortcut_service(&shortcut))
}

/// Validate entire configuration
///
/// # Arguments
/// * `config` - Configuration to validate
///
/// # Returns
/// * `ValidationResult` with errors and warnings
#[tauri::command]
pub fn validate_config(config: ConfigFile) -> Result<ValidationResult, String> {
    Ok(validate_config_service(&config))
}
