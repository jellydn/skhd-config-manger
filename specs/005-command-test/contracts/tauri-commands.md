# Tauri Command Contracts: Command Execution Test

**Feature**: Command Execution Test
**Date**: 2025-11-02
**Phase**: 1 - Design & Contracts

## Overview

This document defines the API contracts for Tauri commands that enable command execution testing. These commands are exposed from the Rust backend to the TypeScript frontend via Tauri's IPC mechanism.

---

## Commands

### 1. execute_shortcut_command

**Purpose**: Execute a shortcut's command and return detailed execution results.

**Signature**:
```rust
#[tauri::command]
async fn execute_shortcut_command(
    shortcut_id: String,
    state: State<'_, ConfigState>,
    exec_state: State<'_, ExecutionState>,
) -> Result<TestResult, String>
```

**Frontend Invocation**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke<TestResult>('execute_shortcut_command', {
  shortcutId: 'shortcut-uuid-here'
});
```

**Request Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| shortcutId | String | Yes | UUID of the shortcut to execute |

**Response**: `TestResult` (see data-model.md)

**Success Response Example**:
```json
{
  "shortcut_id": "abc-123",
  "command": "echo 'Hello World'",
  "syntax_valid": true,
  "syntax_error": null,
  "preview": "",
  "timestamp": "2025-11-02T10:30:00-08:00",
  "executed": true,
  "exit_code": 0,
  "stdout": "Hello World\n",
  "stderr": "",
  "execution_duration_ms": 45,
  "cancelled": false,
  "timed_out": false,
  "output_truncated": false
}
```

**Error Response Examples**:

**Shortcut Not Found**:
```json
Error: "Shortcut not found: abc-123"
```

**Execution Failed**:
```json
{
  "shortcut_id": "abc-123",
  "command": "invalid-command",
  "syntax_valid": false,
  "syntax_error": "command not found: invalid-command",
  "preview": "",
  "timestamp": "2025-11-02T10:30:00-08:00",
  "executed": true,
  "exit_code": 127,
  "stdout": "",
  "stderr": "sh: invalid-command: command not found\n",
  "execution_duration_ms": 12,
  "cancelled": false,
  "timed_out": false,
  "output_truncated": false
}
```

**Timeout**:
```json
{
  "shortcut_id": "abc-123",
  "command": "sleep 60",
  "syntax_valid": true,
  "syntax_error": null,
  "preview": "",
  "timestamp": "2025-11-02T10:30:00-08:00",
  "executed": true,
  "exit_code": null,
  "stdout": "",
  "stderr": "",
  "execution_duration_ms": 30000,
  "cancelled": false,
  "timed_out": true,
  "output_truncated": false
}
```

**Behavior**:
1. Lookup shortcut by ID in ConfigState
2. Check if command is destructive (handled by frontend confirmation)
3. Spawn command execution via `tokio::process::Command`
4. Store Child process in ExecutionState for cancellation support
5. Wait for completion with 30-second timeout
6. Capture stdout, stderr, exit code
7. Truncate output if exceeds 10,000 characters
8. Build and return TestResult
9. Clean up process from ExecutionState

**Performance Guarantees**:
- Non-blocking: Uses async/await, doesn't block Tauri event loop
- Timeout: Always completes within 30 seconds (success, failure, or timeout)
- Memory: Output truncated to prevent memory exhaustion

**Error Conditions**:
- `"Shortcut not found: {id}"`: Invalid shortcut ID
- `"No configuration loaded"`: Configuration state not initialized
- `"Command already executing for shortcut: {id}"`: Previous execution still running

---

### 2. cancel_shortcut_execution

**Purpose**: Cancel a currently executing shortcut command.

**Signature**:
```rust
#[tauri::command]
async fn cancel_shortcut_execution(
    shortcut_id: String,
    exec_state: State<'_, ExecutionState>,
) -> Result<(), String>
```

**Frontend Invocation**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

await invoke('cancel_shortcut_execution', {
  shortcutId: 'shortcut-uuid-here'
});
```

**Request Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| shortcutId | String | Yes | UUID of the shortcut execution to cancel |

**Response**: `Ok(())` on success, `Err(String)` on failure

**Success Response**:
```json
null  // void return, handled as Ok(())
```

**Error Response Examples**:

**No Execution Running**:
```json
Error: "No execution running for shortcut: abc-123"
```

**Behavior**:
1. Lookup running process in ExecutionState by shortcut ID
2. Send SIGTERM to child process
3. Wait briefly for graceful shutdown
4. Send SIGKILL if process doesn't terminate
5. Remove process from ExecutionState
6. Return success

**Timing**:
- Cancellation completes within 1 second
- Process killed immediately if doesn't respond to SIGTERM

**Error Conditions**:
- `"No execution running for shortcut: {id}"`: No active process for this shortcut
- `"Failed to kill process: {error}"`: OS-level error killing process

**Notes**:
- Safe to call even if execution already completed (no-op)
- Original `execute_shortcut_command` will return TestResult with `cancelled: true`

---

### 3. test_shortcut (Existing - Enhanced)

**Purpose**: Validate shortcut command syntax without executing (existing functionality).

**Signature**:
```rust
#[tauri::command]
pub fn test_shortcut(
    shortcut_id: String,
    state: State<'_, ConfigState>,
) -> Result<TestResult, String>
```

**Frontend Invocation**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke<TestResult>('test_shortcut', {
  shortcutId: 'shortcut-uuid-here'
});
```

**Request Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| shortcutId | String | Yes | UUID of the shortcut to validate |

**Response**: `TestResult` with `executed: false`

**Success Response Example**:
```json
{
  "shortcut_id": "abc-123",
  "command": "open -a Terminal",
  "syntax_valid": true,
  "syntax_error": null,
  "preview": "Shortcut: cmd + return\n\nCommand: open -a Terminal\n\nAction: Opens an application\nApplication: Terminal\n",
  "timestamp": "2025-11-02T10:30:00-08:00",
  "executed": false,
  "exit_code": null,
  "stdout": null,
  "stderr": null,
  "execution_duration_ms": null,
  "cancelled": false,
  "timed_out": false,
  "output_truncated": false
}
```

**Behavior**:
1. Lookup shortcut by ID
2. Validate syntax using `sh -n` (syntax check without execution)
3. Generate preview description
4. Return TestResult with `executed: false`

**No Changes**: This existing command behavior is preserved, only TestResult schema extended with optional execution fields.

---

### 4. check_destructive_command

**Purpose**: Check if a command matches destructive patterns (for frontend confirmation).

**Signature**:
```rust
#[tauri::command]
pub fn check_destructive_command(
    command: String,
) -> Result<DestructiveCheck, String>
```

**Frontend Invocation**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const check = await invoke<DestructiveCheck>('check_destructive_command', {
  command: 'rm -rf /tmp/test'
});
```

**Request Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| command | String | Yes | Command string to check |

**Response**: `DestructiveCheck`

```typescript
interface DestructiveCheck {
  is_destructive: boolean;
  matched_patterns: string[];
  severity: 'critical' | 'high' | 'medium' | null;
  warning_message: string | null;
}
```

**Success Response Examples**:

**Safe Command**:
```json
{
  "is_destructive": false,
  "matched_patterns": [],
  "severity": null,
  "warning_message": null
}
```

**Destructive Command**:
```json
{
  "is_destructive": true,
  "matched_patterns": ["rm -rf"],
  "severity": "critical",
  "warning_message": "This command performs recursive file deletion. Files cannot be recovered. Are you sure you want to execute this?"
}
```

**Behavior**:
1. Check command against destructive pattern list
2. Return first matching pattern if any
3. Provide severity level and user-friendly warning
4. Return immediately (synchronous, no I/O)

**Error Conditions**: None (always returns a result)

---

## Frontend Usage Patterns

### Basic Execution Flow

```typescript
import { invoke } from '@tauri-apps/api/tauri';

async function executeShortcut(shortcutId: string) {
  try {
    // Check if destructive
    const check = await invoke<DestructiveCheck>('check_destructive_command', {
      command: shortcut.command
    });

    if (check.is_destructive) {
      const confirmed = await showConfirmDialog(check.warning_message);
      if (!confirmed) return;
    }

    // Execute command
    executionState = 'executing';
    const result = await invoke<TestResult>('execute_shortcut_command', {
      shortcutId
    });

    // Handle result
    if (result.timed_out) {
      executionState = 'timeout';
    } else if (result.cancelled) {
      executionState = 'cancelled';
    } else if (result.exit_code === 0) {
      executionState = 'success';
    } else {
      executionState = 'error';
    }

    testResult = result;
  } catch (error) {
    console.error('Execution failed:', error);
    executionState = 'error';
  }
}
```

### Cancellation Flow

```typescript
async function cancelExecution(shortcutId: string) {
  try {
    await invoke('cancel_shortcut_execution', { shortcutId });
    executionState = 'cancelled';
  } catch (error) {
    console.error('Cancellation failed:', error);
  }
}
```

### Syntax Validation Flow (Existing)

```typescript
async function validateSyntax(shortcutId: string) {
  try {
    const result = await invoke<TestResult>('test_shortcut', {
      shortcutId
    });

    if (result.syntax_valid) {
      console.log('Command is valid:', result.preview);
    } else {
      console.error('Syntax error:', result.syntax_error);
    }
  } catch (error) {
    console.error('Validation failed:', error);
  }
}
```

---

## State Management

### ExecutionState (Backend)

Tracks running processes for cancellation support.

```rust
pub struct ExecutionState {
    running_processes: Arc<Mutex<HashMap<String, Child>>>,
}

impl ExecutionState {
    pub fn store_process(&self, shortcut_id: String, child: Child) {
        self.running_processes.lock().unwrap().insert(shortcut_id, child);
    }

    pub fn remove_process(&self, shortcut_id: &str) -> Option<Child> {
        self.running_processes.lock().unwrap().remove(shortcut_id)
    }

    pub fn is_running(&self, shortcut_id: &str) -> bool {
        self.running_processes.lock().unwrap().contains_key(shortcut_id)
    }
}
```

**Initialization**: Register in Tauri app state during setup

```rust
fn main() {
    tauri::Builder::default()
        .manage(ConfigState::default())
        .manage(ExecutionState::default())
        .invoke_handler(tauri::generate_handler![
            execute_shortcut_command,
            cancel_shortcut_execution,
            test_shortcut,
            check_destructive_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Error Handling Standards

All commands follow consistent error handling:

**Success**: Return `Ok(T)` with result data
**Error**: Return `Err(String)` with user-friendly error message

**Error Message Format**:
- Concise and actionable
- No stack traces or technical jargon in user-facing errors
- Include context (shortcut ID, command, etc.) for debugging

**Frontend Error Display**:
- Show error message in red banner
- Log full error to console for debugging
- Provide retry option when appropriate

---

## Performance Contracts

| Command | Max Latency | Memory Impact | Notes |
|---------|-------------|---------------|-------|
| execute_shortcut_command | 30 seconds (timeout) | <10MB per execution | Async, non-blocking |
| cancel_shortcut_execution | 1 second | Negligible | Kills process immediately |
| test_shortcut | <100ms | <1MB | Synchronous syntax check |
| check_destructive_command | <10ms | Negligible | Pattern matching only |

---

## Security Considerations

**No Input Sanitization**: Commands are user-created, not constructed from untrusted input
**Execution Context**: Commands run as the user, not elevated
**Sandboxing**: Rely on macOS system-level sandboxing
**Destructive Check**: Frontend responsibility to confirm before executing

**Trust Model**: User trusts their own shortcuts, app doesn't modify commands

---

## Testing Strategy

### Contract Tests
- Verify command registration in Tauri
- Test serialization/deserialization of TestResult
- Validate error responses match contract

### Integration Tests
- Execute command end-to-end, verify result structure
- Cancel command, verify process killed and state cleaned
- Timeout command, verify timeout flag set
- Destructive check returns correct patterns

### Performance Tests
- Execute 10 concurrent commands (different shortcuts), all complete within timeout
- Cancel command completes within 1 second
- Syntax validation completes within 100ms

---

## Versioning

**Current Version**: 1.0.0 (Initial implementation)

**Future Enhancements** (not in scope):
- Streaming output via Tauri events
- Command history persistence
- Custom timeout configuration
- Process resource limits

---

## Next Steps

1. Implement Tauri commands in `src-tauri/src/commands/testing.rs`
2. Register commands in `src-tauri/src/lib.rs`
3. Create TypeScript wrappers in frontend
4. Build contract test suite
