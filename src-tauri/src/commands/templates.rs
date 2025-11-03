use crate::models::{CommandCategory, CommandParameter, CommandTemplate};
use crate::services::template_loader;

/// Tauri command to get all command templates, optionally filtered by category
#[tauri::command]
pub async fn get_command_templates(
    category_id: Option<String>,
) -> Result<Vec<CommandTemplate>, String> {
    template_loader::get_templates(category_id)
}

/// Tauri command to get all command categories
#[tauri::command]
pub async fn get_command_categories() -> Result<Vec<CommandCategory>, String> {
    template_loader::get_categories()
}

/// Tauri command to generate a command from a template with parameter substitution
#[tauri::command]
pub async fn generate_command_from_template(
    template_id: String,
    parameter_values: std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let templates = template_loader::get_templates(None)?;
    let template = templates
        .iter()
        .find(|t| t.id == template_id)
        .ok_or_else(|| format!("Template not found: {}", template_id))?;

    let mut command = template.command_pattern.clone();

    // Substitute each parameter
    for param in &template.parameters {
        let value = parameter_values
            .get(&param.name)
            .unwrap_or(&param.default_value)
            .clone();

        // Validate parameter value
        validate_parameter_value(&param, &value)?;

        // Replace {param_name} with value
        let placeholder = format!("{{{}}}", param.name);
        command = command.replace(&placeholder, &value);
    }

    Ok(command)
}

/// Validate parameter value against parameter constraints
fn validate_parameter_value(param: &CommandParameter, value: &str) -> Result<(), String> {
    // Check regex validation if present
    if let Some(regex_pattern) = &param.validation_regex {
        let regex = regex::Regex::new(regex_pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        if !regex.is_match(value) {
            return Err(format!(
                "Invalid value for {}: must match pattern {}",
                param.name, regex_pattern
            ));
        }
    }

    // Check integer constraints
    if param.data_type == "integer" {
        let int_value: i32 = value
            .parse()
            .map_err(|_| format!("Invalid integer value for {}", param.name))?;

        if let Some(min) = param.min_value {
            if int_value < min {
                return Err(format!(
                    "{} must be at least {}",
                    param.name, min
                ));
            }
        }

        if let Some(max) = param.max_value {
            if int_value > max {
                return Err(format!(
                    "{} must be at most {}",
                    param.name, max
                ));
            }
        }
    }

    // Check enum values if present
    if let Some(enum_values) = &param.enum_values {
        if !enum_values.contains(&value.to_string()) {
            return Err(format!(
                "{} must be one of: {}",
                param.name,
                enum_values.join(", ")
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_command_templates_all() {
        let result = get_command_templates(None).await;
        assert!(result.is_ok());
        let templates = result.unwrap();
        assert!(templates.len() > 0, "Should have templates");
    }

    #[tokio::test]
    async fn test_get_command_templates_by_category() {
        let result = get_command_templates(Some("media".to_string())).await;
        assert!(result.is_ok());
        let templates = result.unwrap();
        assert!(templates.len() > 0, "Should have media templates");
        for template in templates {
            assert_eq!(template.category_id, "media");
        }
    }

    #[tokio::test]
    async fn test_get_command_categories() {
        let result = get_command_categories().await;
        assert!(result.is_ok());
        let categories = result.unwrap();
        assert!(categories.len() > 0, "Should have categories");
        // Verify they're sorted by display_order
        for i in 1..categories.len() {
            assert!(categories[i - 1].display_order <= categories[i].display_order);
        }
    }

    #[tokio::test]
    async fn test_generate_command_from_template() {
        let mut params = std::collections::HashMap::new();
        params.insert("amount".to_string(), "15".to_string());

        let result = generate_command_from_template("volume-up".to_string(), params).await;
        assert!(result.is_ok());
        let command = result.unwrap();
        assert!(command.contains("15"), "Should contain the parameter value");
        assert!(
            !command.contains("{amount}"),
            "Should not contain placeholder"
        );
    }

    #[tokio::test]
    async fn test_generate_command_with_defaults() {
        let params = std::collections::HashMap::new(); // Use defaults

        let result = generate_command_from_template("volume-up".to_string(), params).await;
        assert!(result.is_ok());
        let command = result.unwrap();
        assert!(command.contains("10"), "Should use default value");
    }

    #[tokio::test]
    async fn test_generate_command_invalid_template() {
        let params = std::collections::HashMap::new();
        let result = generate_command_from_template("nonexistent".to_string(), params).await;
        assert!(result.is_err());
    }
}
