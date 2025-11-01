/// Abstract Syntax Tree structures for skhd configuration parsing

use serde::{Deserialize, Serialize};

/// Represents a parsed keyboard shortcut from the skhd config
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedShortcut {
    /// Line number in the original config file
    pub line_number: usize,

    /// Modifier keys (cmd, alt, shift, ctrl, fn)
    pub modifiers: Vec<String>,

    /// Primary key being pressed
    pub key: String,

    /// Shell command to execute
    pub command: String,

    /// Optional inline comment
    pub comment: Option<String>,
}

/// Represents a comment line in the config
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedComment {
    /// Line number in the original config file
    pub line_number: usize,

    /// Comment text (without the # prefix)
    pub text: String,
}

/// Represents a line in the skhd config file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigLine {
    /// A keyboard shortcut mapping
    Shortcut(ParsedShortcut),

    /// A comment line
    Comment(ParsedComment),

    /// An empty line
    Empty(usize), // line number
}

/// Complete parsed configuration file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedConfig {
    /// All lines from the config file in order
    pub lines: Vec<ConfigLine>,
}

impl ParsedConfig {
    /// Create a new empty parsed config
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// Get all shortcuts from the config
    pub fn shortcuts(&self) -> Vec<&ParsedShortcut> {
        self.lines
            .iter()
            .filter_map(|line| match line {
                ConfigLine::Shortcut(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    /// Get all comments from the config
    pub fn comments(&self) -> Vec<&ParsedComment> {
        self.lines
            .iter()
            .filter_map(|line| match line {
                ConfigLine::Comment(c) => Some(c),
                _ => None,
            })
            .collect()
    }
}

impl Default for ParsedConfig {
    fn default() -> Self {
        Self::new()
    }
}
