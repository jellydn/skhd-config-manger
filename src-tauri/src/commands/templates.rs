use crate::models::{CommandCategory, CommandTemplate};
use crate::services::template_loader;

/// Tauri command to get all command templates, optionally filtered by category
#[tauri::command]
pub async fn get_command_templates(
    category_id: Option<String>,
) -> Result<Vec<CommandTemplate>, String> {
    template_loader::get_templates(category_id).or_else(|e| {
        eprintln!("Template loading failed: {}", e);
        // Return empty list on error to prevent app crash
        Ok(Vec::new())
    })
}

/// Tauri command to get all command categories
#[tauri::command]
pub async fn get_command_categories() -> Result<Vec<CommandCategory>, String> {
    template_loader::get_categories().or_else(|e| {
        eprintln!("Category loading failed: {}", e);
        // Return empty list on error to prevent app crash
        Ok(Vec::new())
    })
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

        // Validate parameter value using CommandParameter's validate method
        param.validate_value(&value)?;

        // Replace {param_name} with value
        let placeholder = format!("{{{}}}", param.name);
        command = command.replace(&placeholder, &value);
    }

    Ok(command)
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
