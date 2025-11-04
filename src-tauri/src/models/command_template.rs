use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category_id: String,
    pub command_pattern: String,
    pub parameters: Vec<CommandParameter>,
    #[serde(default)]
    pub requires_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    pub name: String,
    pub description: String,
    pub data_type: String,
    pub default_value: String,
    pub validation_regex: Option<String>,
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
    pub enum_values: Option<Vec<String>>,
}

impl CommandParameter {
    /// Validates a parameter value against its constraints
    pub fn validate_value(&self, value: &str) -> Result<(), String> {
        // Check regex validation if present
        if let Some(regex_pattern) = &self.validation_regex {
            let regex = regex::Regex::new(regex_pattern)
                .map_err(|e| format!("Invalid regex pattern: {}", e))?;
            if !regex.is_match(value) {
                return Err(format!(
                    "Invalid value for {}: must match pattern {}",
                    self.name, regex_pattern
                ));
            }
        }

        // Check integer constraints
        if self.data_type == "integer" {
            let int_value: i32 = value
                .parse()
                .map_err(|_| format!("Invalid integer value for {}", self.name))?;

            if let Some(min) = self.min_value {
                if int_value < min {
                    return Err(format!("{} must be at least {}", self.name, min));
                }
            }

            if let Some(max) = self.max_value {
                if int_value > max {
                    return Err(format!("{} must be at most {}", self.name, max));
                }
            }
        }

        // Check enum values if present
        if let Some(enum_values) = &self.enum_values {
            if !enum_values.contains(&value.to_string()) {
                return Err(format!(
                    "{} must be one of: {}",
                    self.name,
                    enum_values.join(", ")
                ));
            }
        }

        Ok(())
    }
}
