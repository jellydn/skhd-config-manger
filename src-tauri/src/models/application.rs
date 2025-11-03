use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub display_name: String,
    pub app_path: String,
    pub bundle_id: String,
    pub executable_path: String,
    pub icon_path: Option<String>,
    pub version: Option<String>,
}
