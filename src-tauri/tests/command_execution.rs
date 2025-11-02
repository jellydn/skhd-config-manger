/// Integration tests for command execution functionality
use keybinder_lib::commands::testing::truncate_output;

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

// Note: Full integration tests for command execution, timeout, and failure
// require async test infrastructure and would be added in a full implementation.
// These tests verify the core truncation logic which is critical for the feature.
