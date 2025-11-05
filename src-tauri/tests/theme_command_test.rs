/// Unit tests for theme detection commands
/// 
/// These tests verify the theme detection functionality works correctly
/// on macOS systems. Tests may be skipped on non-macOS platforms.

use keybinder_lib::commands::theme::get_system_theme;

// Note: Tests in subdirectories need to be run with `cargo test --test <name>`
// or moved to tests/ root. For now, these tests will be discovered by cargo test.

#[test]
#[cfg(target_os = "macos")]
fn test_get_system_theme_returns_valid_theme() {
    // This test requires macOS to run
    // It verifies that get_system_theme returns either "light" or "dark"
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_system_theme());
    
    assert!(result.is_ok(), "get_system_theme should succeed on macOS");
    let theme = result.unwrap();
    assert!(
        theme == "light" || theme == "dark",
        "Theme should be either 'light' or 'dark', got: {}",
        theme
    );
}

#[test]
#[cfg(target_os = "macos")]
fn test_get_system_theme_performance() {
    // Performance test: verify theme detection completes within reasonable time
    // Using 200ms threshold to account for system variability and CI/CD environments
    use std::time::Instant;
    
    let start = Instant::now();
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_system_theme());
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "get_system_theme should succeed");
    assert!(
        duration.as_millis() < 200,
        "Theme detection should complete within 200ms, took: {}ms",
        duration.as_millis()
    );
}

#[test]
#[cfg(target_os = "macos")]
fn test_get_system_theme_consistency() {
    // Verify that multiple calls return consistent results
    let result1 = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_system_theme());
    let result2 = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_system_theme());
    
    assert_eq!(
        result1, result2,
        "Multiple calls to get_system_theme should return the same result"
    );
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_get_system_theme_defaults_on_non_macos() {
    // On non-macOS platforms, should default to dark mode
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_system_theme());
    
    // Should still return a valid theme (likely defaults to dark)
    assert!(result.is_ok(), "get_system_theme should not fail");
    let theme = result.unwrap();
    assert!(
        theme == "light" || theme == "dark",
        "Theme should be either 'light' or 'dark'"
    );
}
