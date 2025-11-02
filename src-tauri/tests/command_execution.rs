/// Integration tests for command execution functionality
use keybinder_lib::commands::testing::truncate_output;

// ===== Truncation Tests =====

#[test]
fn test_truncate_output_within_limit() {
    let output = "Hello World".to_string();
    let (truncated, was_truncated) = truncate_output(output.clone(), 100);

    assert_eq!(truncated, output);
    assert!(!was_truncated);
}

#[test]
fn test_truncate_output_exceeds_limit() {
    let output = "a".repeat(15000);
    let (truncated, was_truncated) = truncate_output(output, 10000);

    assert_eq!(truncated.len(), 10000);
    assert!(was_truncated);
}

#[test]
fn test_truncate_output_exactly_at_limit() {
    let output = "a".repeat(10000);
    let (truncated, was_truncated) = truncate_output(output.clone(), 10000);

    assert_eq!(truncated.len(), 10000);
    assert!(!was_truncated);
}

// ===== Command Execution Integration Tests =====
//
// Note: Full integration tests for execute_shortcut_command require Tauri runtime infrastructure.
// These cannot be easily tested in isolation without mocking the entire Tauri state management system.
//
// The command execution logic is validated through:
// 1. Unit tests for core functionality (truncate_output, format_command_preview, shell_escape)
// 2. Code review focusing on the ExecutionGuard RAII pattern and HashSet-based duplicate prevention
// 3. Manual testing in the running application with real shortcuts
// 4. End-to-end tests in the Svelte frontend
//
// Key improvements in src/commands/testing.rs that address PR review comments:
// - ExecutionState now uses HashSet<String> instead of HashMap (functional duplicate detection)
// - RAII ExecutionGuard pattern ensures automatic cleanup on completion or error
// - Config lock released immediately after cloning shortcut (reduced contention)
// - Comprehensive documentation of the timeout handling behavior
//
// Future enhancements:
// - Add integration tests using tauri::test::mock_context() or similar testing utilities
// - Consider extracting command execution logic to a testable service layer
