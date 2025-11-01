/// Test result for shortcut execution
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// ID of the shortcut that was tested
    pub shortcut_id: String,

    /// The command that was tested
    pub command: String,

    /// Whether the command syntax is valid
    pub syntax_valid: bool,

    /// Syntax error message if invalid
    pub syntax_error: Option<String>,

    /// Preview of what the command would do
    pub preview: String,

    /// Timestamp when the test was executed
    pub timestamp: String,
}
