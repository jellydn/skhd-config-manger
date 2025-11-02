use keybinder_lib::models::{LogEntry, LogLevel};
use regex::Regex;

/// Parse a single log line into structured LogEntry
/// Format: YYYY-MM-DD HH:MM:SS [LEVEL] message
fn parse_log_line(line: &str) -> Option<LogEntry> {
    let re = Regex::new(r"^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(ERROR|WARN|INFO|DEBUG)\] (.+)$")
        .unwrap();

    if let Some(caps) = re.captures(line) {
        let timestamp_str = caps.get(1)?.as_str();
        let level_str = caps.get(2)?.as_str();
        let message = caps.get(3)?.as_str();

        let timestamp = chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
            .ok()?
            .and_utc();

        let level = LogLevel::from_str(level_str)?;

        Some(LogEntry::new(
            timestamp,
            level,
            message.to_string(),
            line.to_string(),
        ))
    } else {
        // Return fallback entry for unparseable lines
        Some(LogEntry::from_raw(line.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_info_log() {
        let line = "2025-11-02 10:15:30 [INFO] skhd: configuration loaded successfully";
        let entry = parse_log_line(line).expect("Should parse valid log line");

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "skhd: configuration loaded successfully");
        assert_eq!(entry.raw, line);
    }

    #[test]
    fn test_parse_valid_error_log() {
        let line = "2025-11-02 10:16:01 [ERROR] skhd: failed to execute command";
        let entry = parse_log_line(line).expect("Should parse valid log line");

        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "skhd: failed to execute command");
    }

    #[test]
    fn test_parse_valid_warn_log() {
        let line = "2025-11-02 10:16:05 [WARN] skhd: duplicate keybinding detected";
        let entry = parse_log_line(line).expect("Should parse valid log line");

        assert_eq!(entry.level, LogLevel::Warn);
        assert_eq!(entry.message, "skhd: duplicate keybinding detected");
    }

    #[test]
    fn test_parse_valid_debug_log() {
        let line = "2025-11-02 10:15:31 [DEBUG] skhd: parsing keybindings";
        let entry = parse_log_line(line).expect("Should parse valid log line");

        assert_eq!(entry.level, LogLevel::Debug);
        assert_eq!(entry.message, "skhd: parsing keybindings");
    }

    #[test]
    fn test_parse_invalid_log_fallback() {
        let line = "Invalid log line without timestamp";
        let entry = parse_log_line(line).expect("Should return fallback entry");

        // Fallback creates INFO level with raw content
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, line);
        assert_eq!(entry.raw, line);
    }

    #[test]
    fn test_parse_empty_line() {
        let line = "";
        let entry = parse_log_line(line).expect("Should handle empty line");

        assert_eq!(entry.raw, "");
    }

    #[test]
    fn test_parse_log_with_special_characters() {
        let line = "2025-11-02 10:15:30 [INFO] skhd: command output: \"hello world\" & echo done";
        let entry = parse_log_line(line).expect("Should parse line with special chars");

        assert_eq!(
            entry.message,
            "skhd: command output: \"hello world\" & echo done"
        );
    }

    #[test]
    fn test_multiple_logs_from_fixture() {
        let fixture = include_str!("fixtures/sample_logs.txt");
        let lines: Vec<&str> = fixture.lines().collect();

        assert!(lines.len() > 5, "Fixture should have multiple log lines");

        for line in lines {
            let entry = parse_log_line(line);
            assert!(entry.is_some(), "Every line should parse");
        }
    }

    #[test]
    fn test_timestamp_parsing_accuracy() {
        let line = "2025-11-02 10:15:30 [INFO] test message";
        let entry = parse_log_line(line).unwrap();

        let expected_time = chrono::NaiveDateTime::parse_from_str("2025-11-02 10:15:30", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc();

        assert_eq!(entry.timestamp, expected_time);
    }
}
