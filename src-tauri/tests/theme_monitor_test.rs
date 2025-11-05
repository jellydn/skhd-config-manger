/// Unit tests for theme monitoring service
/// 
/// These tests verify the ThemeMonitorState service behaves correctly
/// for starting and stopping theme monitoring.

use keybinder_lib::services::ThemeMonitorState;
use std::time::Duration;
use tokio::time::sleep;

#[test]
#[cfg(target_os = "macos")]
fn test_theme_monitor_initial_state() {
    // Verify initial state is not monitoring
    let state = ThemeMonitorState::new();
    let is_monitoring = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(state.is_monitoring());
    
    assert!(!is_monitoring, "Initial state should not be monitoring");
}

#[test]
#[cfg(target_os = "macos")]
fn test_theme_monitor_start_stop() {
    // Test starting and stopping monitoring
    let state = ThemeMonitorState::new();
    
    // This test requires a Tauri AppHandle, which is complex to mock
    // For now, we test the state management logic
    
    // Verify we can check monitoring state
    let initial_state = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(state.is_monitoring());
    assert!(!initial_state, "Should not be monitoring initially");
    
    // Test stop when not monitoring (should not error)
    let stop_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(state.stop_monitoring());
    assert!(stop_result.is_ok(), "stop_monitoring should succeed even when not monitoring");
}

#[test]
#[cfg(target_os = "macos")]
fn test_theme_monitor_multiple_stop_calls() {
    // Verify multiple stop calls don't cause errors
    let state = ThemeMonitorState::new();
    
    let result1 = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(state.stop_monitoring());
    assert!(result1.is_ok(), "First stop should succeed");
    
    let result2 = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(state.stop_monitoring());
    assert!(result2.is_ok(), "Second stop should succeed");
}

#[test]
#[cfg(target_os = "macos")]
fn test_theme_monitor_state_persistence() {
    // Verify state persists across async operations
    let state = ThemeMonitorState::new();
    
    // Check state multiple times
    let state1 = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            sleep(Duration::from_millis(10)).await;
            state.is_monitoring().await
        });
    
    let state2 = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(state.is_monitoring());
    
    assert_eq!(state1, state2, "State should be consistent");
}
