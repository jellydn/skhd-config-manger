# Research: Command Execution Test

**Feature**: Command Execution Test
**Date**: 2025-11-02
**Phase**: 0 - Research & Technical Decisions

## Overview

This document consolidates technical research and decisions for implementing command execution functionality in the skhd GUI test button feature.

## Research Areas

### 1. Async Command Execution in Rust/Tauri

**Decision**: Use tokio::process::Command with timeout via tokio::time::timeout

**Rationale**:
- Tauri v2 is built on tokio async runtime, natural integration
- tokio::process::Command provides non-blocking execution
- Built-in timeout support prevents hanging commands
- Process cancellation via child.kill() for user-initiated stops
- Allows UI to remain responsive during command execution

**Alternatives Considered**:
- std::process::Command (blocking): Rejected - would freeze UI during execution
- async-std: Rejected - Tauri already uses tokio, avoid mixing runtimes
- Manual thread spawning: Rejected - unnecessary complexity when tokio handles it

**Implementation Pattern**:
```rust
use tokio::process::Command;
use tokio::time::{timeout, Duration};

async fn execute_with_timeout(command: &str) -> Result<Output, Error> {
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    timeout(Duration::from_secs(30), child).await
}
```

**Reference**: [Tauri async commands](https://tauri.app/v2/guides/features/commands/#async-commands), [tokio::process documentation](https://docs.rs/tokio/latest/tokio/process/)

---

### 2. Destructive Command Detection

**Decision**: Pattern-based detection using regex with whitelisting for safe patterns

**Rationale**:
- Balance between security and usability
- Regex allows flexible pattern matching (e.g., rm with various flags)
- Whitelist approach for common safe paths (e.g., /tmp/, test directories)
- False positives acceptable - better to over-warn than under-warn

**Patterns to Match** (partial list, expand during implementation):
- `rm` with `-r`, `-rf`, or targeting system paths
- `sudo`, `doas` (privilege escalation)
- `kill`, `pkill`, `killall` (process termination)
- `shutdown`, `reboot`, `halt` (system control)
- `dd`, `mkfs`, `fdisk` (disk operations)
- `chmod`, `chown` on system paths
- File operations in `/System`, `/Library`, `/bin`, `/usr`

**Whitelisted Safe Patterns**:
- `rm` targeting `/tmp/*`, `~/Downloads/*`, project directories
- `kill` with user's own process IDs
- `open -a` (application launching - never destructive)

**Alternatives Considered**:
- Sandboxing: Rejected - complex setup, limited macOS support in Tauri context
- AST parsing of shell commands: Rejected - overkill for this use case, hard to maintain
- No detection: Rejected - violates safety principle

**Implementation Approach**:
```rust
fn is_destructive(command: &str) -> bool {
    let destructive_patterns = [
        r"sudo\s+",
        r"rm\s+(-[rf]+|\S*r\S*)\s+(?!/tmp/)",
        r"shutdown|reboot|halt",
        // ... more patterns
    ];

    // Check if command matches any destructive pattern
    // and doesn't match any safe pattern
}
```

---

### 3. Process Cancellation Strategy

**Decision**: Store Child handle in shared state (Arc<Mutex<Option<Child>>>), kill on cancel

**Rationale**:
- Tauri commands are stateless by default, need shared state for process tracking
- Arc<Mutex<>> provides thread-safe access to running process
- Child::kill() sends SIGTERM, then SIGKILL if needed
- Clean up process on both explicit cancel and timeout

**State Management**:
```rust
pub struct ExecutionState {
    running_process: Arc<Mutex<Option<Child>>>,
}

#[tauri::command]
async fn cancel_command(state: State<'_, ExecutionState>) -> Result<(), String> {
    let mut process = state.running_process.lock().unwrap();
    if let Some(child) = process.take() {
        child.kill().await?;
    }
    Ok(())
}
```

**Alternatives Considered**:
- Frontend tracking: Rejected - backend should own process lifecycle
- No cancellation: Rejected - violates user control requirement
- Process groups: Considered for future if zombie processes become issue

---

### 4. Output Truncation and Display

**Decision**: Truncate at 10,000 characters in backend, indicate truncation to user

**Rationale**:
- Prevents memory exhaustion from commands with large output
- 10K chars ≈ 200 lines of output, sufficient for debugging
- Backend truncation prevents large data over IPC bridge
- Clear indicator prevents user confusion

**Implementation**:
```rust
fn truncate_output(output: String, limit: usize) -> (String, bool) {
    if output.len() > limit {
        (output[..limit].to_string(), true)
    } else {
        (output, false)
    }
}
```

**Display Strategy**:
- Show first 10,000 characters
- Add banner: "⚠️ Output truncated (showing first 10,000 of N characters)"
- Provide option to save full output to file (future enhancement)

**Alternatives Considered**:
- Streaming output: Rejected - adds complexity for marginal benefit in desktop app
- No truncation: Rejected - memory safety concern
- Frontend truncation: Rejected - waste of IPC bandwidth

---

### 5. TestResult Model Extensions

**Decision**: Extend existing TestResult with execution-specific fields, maintain backward compatibility

**Current TestResult**:
```rust
pub struct TestResult {
    shortcut_id: String,
    command: String,
    syntax_valid: bool,
    syntax_error: Option<String>,
    preview: String,
    timestamp: String,
}
```

**Extended TestResult**:
```rust
pub struct TestResult {
    // Existing fields
    shortcut_id: String,
    command: String,
    syntax_valid: bool,
    syntax_error: Option<String>,
    preview: String,
    timestamp: String,

    // New execution fields
    executed: bool,                    // false for syntax-only tests
    exit_code: Option<i32>,           // None for syntax tests
    stdout: Option<String>,           // None for syntax tests
    stderr: Option<String>,           // None for syntax tests
    execution_duration_ms: Option<u64>, // None for syntax tests
    cancelled: bool,                   // true if user cancelled
    timed_out: bool,                   // true if hit 30s timeout
    output_truncated: bool,            // true if output exceeded limit
}
```

**Rationale**:
- Single model for both syntax validation and execution results
- Optional fields maintain backward compatibility
- `executed` flag disambiguates test type
- Clear semantic fields for all execution scenarios

---

### 6. Error Handling and User Feedback

**Decision**: Structured error types with user-friendly messages

**Error Scenarios**:
1. **Syntax validation failure**: Show parse error from shell
2. **Execution failure**: Show exit code and stderr
3. **Timeout**: "Command timed out after 30 seconds"
4. **Cancellation**: "Execution cancelled by user"
5. **Permission denied**: "Command requires elevated privileges"
6. **Command not found**: "Command not found in PATH"

**Error Display Strategy**:
- Red banner for failures (exit code ≠ 0)
- Yellow banner for warnings (destructive confirmation)
- Grey banner for cancelled/timeout
- Include actionable suggestions when possible

---

### 7. Frontend State Management

**Decision**: Reactive state in ShortcutItem component with loading/executing/complete states

**State Machine**:
```
idle → (click test) → confirming (if destructive)
                   → executing → success/error/cancelled/timeout
```

**UI States**:
- **idle**: Test button enabled
- **confirming**: Show confirmation dialog
- **executing**: Spinner + Cancel button
- **success**: Green checkmark + show results
- **error**: Red X + show error details
- **cancelled**: Grey dash + "Cancelled"
- **timeout**: Yellow warning + "Timed out"

**Implementation in Svelte**:
```typescript
let executionState: 'idle' | 'confirming' | 'executing' | 'complete' = 'idle';
let testResult: TestResult | null = null;
```

---

### 8. Security Considerations

**Decisions**:
1. **No shell injection prevention needed**: Commands are stored by user, not constructed from untrusted input
2. **Execute in user context**: Commands run as the user, not elevated
3. **Working directory**: Execute in user's home directory for consistency
4. **Environment**: Inherit user's PATH and environment variables

**Rationale**:
- User is testing their own shortcuts they created
- Security model: user trusts their own commands
- Destructive command confirmation is the primary safety measure
- macOS sandboxing provides OS-level protection

**Not Implemented** (out of scope):
- Command sandboxing
- Command allow/deny lists
- Execution in isolated environment

---

## Technology Stack Summary

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| Backend Runtime | Rust | 1.75+ | Command execution, validation |
| Async Runtime | tokio | Latest compatible with Tauri v2 | Non-blocking execution |
| Process Management | tokio::process | Built-in | Command spawning |
| Frontend Framework | Svelte | 5 | UI state and display |
| Type Safety | TypeScript | 5+ | Frontend type definitions |
| Testing (Rust) | cargo test | Built-in | Backend unit/integration tests |
| Testing (Frontend) | Vitest | 1+ | Frontend component tests |

---

## Best Practices Applied

### Rust Command Execution
- Use `sh -c` wrapper for complex commands with pipes/redirects
- Proper quote escaping for user commands
- Capture both stdout and stderr separately
- Check exit code for success determination

### Async/Await Patterns
- All Tauri commands that execute shell commands must be async
- Use `tokio::time::timeout` for bounded execution time
- Proper error handling with Result types
- Clean up resources in all execution paths

### macOS Integration
- Respect user's default shell (sh, zsh, bash)
- Use system PATH for command resolution
- Support macOS-specific commands (open -a, etc.)
- Handle macOS sandboxing constraints

### Error Handling
- Distinguish between syntax errors, runtime errors, timeouts, cancellations
- Provide actionable error messages
- Log errors for debugging without exposing to user
- Graceful degradation when commands fail

---

## Open Questions (Resolved)

None - all technical decisions finalized.

---

## Next Steps

Proceed to Phase 1:
1. Generate data-model.md with TestResult schema
2. Generate API contracts for new Tauri commands
3. Create quickstart guide for testing workflow
4. Update agent context files with new technologies
