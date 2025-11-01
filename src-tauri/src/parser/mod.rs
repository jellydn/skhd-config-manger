/// skhd configuration file parser using pest
pub mod ast;

use pest::Parser;
use pest_derive::Parser;
use std::error::Error;
use std::fmt;

use ast::{ConfigLine, ParsedComment, ParsedConfig, ParsedShortcut};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct SkhdParser;

/// Parse errors
#[derive(Debug, Clone)]
pub struct ParseError {
    pub line_number: usize,
    pub column: Option<usize>,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(col) = self.column {
            write!(
                f,
                "Parse error at line {}, column {}: {}",
                self.line_number, col, self.message
            )
        } else {
            write!(
                f,
                "Parse error at line {}: {}",
                self.line_number, self.message
            )
        }
    }
}

impl Error for ParseError {}

/// Parse an skhd configuration file
pub fn parse_config(content: &str) -> Result<ParsedConfig, Vec<ParseError>> {
    let mut parsed_config = ParsedConfig::new();
    let mut errors = Vec::new();

    // Parse the entire file
    let pairs = match SkhdParser::parse(Rule::config_file, content) {
        Ok(pairs) => pairs,
        Err(e) => {
            let (line, col) = match e.line_col {
                pest::error::LineColLocation::Pos((l, c)) => (l, Some(c)),
                pest::error::LineColLocation::Span((l, _), _) => (l, None),
            };
            errors.push(ParseError {
                line_number: line,
                column: col,
                message: format!("Syntax error: {}", e.variant),
            });
            return Err(errors);
        }
    };

    // Process each line
    let mut line_num = 1;
    for pair in pairs {
        if pair.as_rule() == Rule::config_file {
            for line_pair in pair.into_inner() {
                match line_pair.as_rule() {
                    Rule::comment => {
                        let text = line_pair
                            .as_str()
                            .trim_end_matches('\n')
                            .trim_end_matches('\r')
                            .trim_start_matches('#')
                            .trim();
                        parsed_config.lines.push(ConfigLine::Comment(ParsedComment {
                            line_number: line_num,
                            text: text.to_string(),
                        }));
                        line_num += 1;
                    }
                    Rule::shortcut => {
                        match parse_shortcut(&line_pair, line_num) {
                            Ok(shortcut) => {
                                parsed_config.lines.push(ConfigLine::Shortcut(shortcut));
                            }
                            Err(e) => {
                                errors.push(e);
                            }
                        }
                        line_num += 1;
                    }
                    Rule::empty_line => {
                        parsed_config.lines.push(ConfigLine::Empty(line_num));
                        line_num += 1;
                    }
                    Rule::EOI => break,
                    _ => {}
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(parsed_config)
    } else {
        Err(errors)
    }
}

/// Parse a single shortcut line
fn parse_shortcut(
    pair: &pest::iterators::Pair<Rule>,
    line_num: usize,
) -> Result<ParsedShortcut, ParseError> {
    let mut modifiers = Vec::new();
    let mut key = String::new();
    let mut command = String::new();

    for inner_pair in pair.clone().into_inner() {
        match inner_pair.as_rule() {
            Rule::modifiers => {
                for modifier_pair in inner_pair.into_inner() {
                    if modifier_pair.as_rule() == Rule::modifier {
                        modifiers.push(modifier_pair.as_str().to_string());
                    }
                }
            }
            Rule::key => {
                key = inner_pair.as_str().to_string();
            }
            Rule::command => {
                command = inner_pair.as_str().trim().to_string();
            }
            _ => {}
        }
    }

    if key.is_empty() {
        return Err(ParseError {
            line_number: line_num,
            column: None,
            message: "Missing key specification".to_string(),
        });
    }

    if command.is_empty() {
        return Err(ParseError {
            line_number: line_num,
            column: None,
            message: "Missing command specification".to_string(),
        });
    }

    Ok(ParsedShortcut {
        line_number: line_num,
        modifiers,
        key,
        command,
        comment: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_shortcut() {
        let config = "cmd - return : open -a Terminal\n";
        let result = parse_config(config);
        if let Err(ref errors) = result {
            for e in errors {
                eprintln!("Parse error: {:?}", e);
            }
        }
        assert!(result.is_ok());

        let parsed = result.unwrap();
        let shortcuts = parsed.shortcuts();
        assert_eq!(shortcuts.len(), 1);
        assert_eq!(shortcuts[0].modifiers, vec!["cmd"]);
        assert_eq!(shortcuts[0].key, "return");
        assert_eq!(shortcuts[0].command, "open -a Terminal");
    }

    #[test]
    fn test_parse_multiple_modifiers() {
        let config = "cmd + shift - f : open ~\n";
        let result = parse_config(config);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        let shortcuts = parsed.shortcuts();
        assert_eq!(shortcuts.len(), 1);
        assert_eq!(shortcuts[0].modifiers, vec!["cmd", "shift"]);
        assert_eq!(shortcuts[0].key, "f");
    }

    #[test]
    fn test_parse_with_comments() {
        let config = "# This is a comment\ncmd - return : open -a Terminal\n";
        let result = parse_config(config);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.comments().len(), 1);
        assert_eq!(parsed.shortcuts().len(), 1);
    }
}
