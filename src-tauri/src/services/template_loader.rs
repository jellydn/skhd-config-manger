use crate::models::{command_category::CommandCategory, command_template::CommandTemplate};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct TemplateData {
    pub categories: Vec<CommandCategory>,
    pub templates: Vec<CommandTemplate>,
}

/// Load command templates from embedded JSON
pub fn load_templates() -> Result<TemplateData, String> {
    let json_data = include_str!("../data/command_templates.json");
    serde_json::from_str(json_data).map_err(|e| format!("Failed to parse templates: {}", e))
}

/// Get all command templates, optionally filtered by category
pub fn get_templates(category_id: Option<String>) -> Result<Vec<CommandTemplate>, String> {
    let data = load_templates()?;
    if let Some(cat_id) = category_id {
        Ok(data
            .templates
            .into_iter()
            .filter(|t| t.category_id == cat_id)
            .collect())
    } else {
        Ok(data.templates)
    }
}

/// Get all command categories, sorted by display_order
pub fn get_categories() -> Result<Vec<CommandCategory>, String> {
    let data = load_templates()?;
    let mut categories = data.categories;
    categories.sort_by_key(|c| c.display_order);
    Ok(categories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_templates() {
        let result = load_templates();
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.categories.len() > 0, "Should have categories");
        assert!(data.templates.len() > 0, "Should have templates");
    }

    #[test]
    fn test_get_templates_all() {
        let templates = get_templates(None).unwrap();
        assert!(templates.len() > 0, "Should return all templates");
    }

    #[test]
    fn test_get_templates_by_category() {
        let templates = get_templates(Some("media".to_string())).unwrap();
        assert!(templates.len() > 0, "Should have media templates");
        for template in templates {
            assert_eq!(template.category_id, "media");
        }
    }

    #[test]
    fn test_get_categories() {
        let categories = get_categories().unwrap();
        assert!(categories.len() > 0, "Should have categories");
        // Verify they're sorted by display_order
        for i in 1..categories.len() {
            assert!(categories[i - 1].display_order <= categories[i].display_order);
        }
    }
}
