use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub display_order: i32,
}
