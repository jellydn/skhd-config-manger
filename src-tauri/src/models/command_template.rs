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
