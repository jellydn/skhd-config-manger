# Data Model: Command Execution Test

**Feature**: Command Execution Test
**Date**: 2025-11-02
**Phase**: 1 - Design & Contracts

## Overview

This document defines the data structures and their relationships for the command execution test feature. All models are designed to be serializable for Tauri IPC communication between Rust backend and TypeScript frontend.

---

## Core Entities

### TestResult (Extended)

Represents the outcome of testing a keyboard shortcut command, supporting both syntax validation and actual execution.

**Location**: `src-tauri/src/models/test_result.rs`

**Schema**:

```rust
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

    /// Preview of what the command would do (for syntax validation)
    pub preview: String,

    /// Timestamp when the test was executed (RFC3339 format)
    pub timestamp: String,

    // ===== NEW EXECUTION FIELDS =====

    /// Whether this was an actual execution (true) or syntax validation only (false)
    pub executed: bool,

    /// Exit code from command execution (None for syntax-only tests)
    pub exit_code: Option<i32>,

    /// Standard output from command execution (None for syntax-only tests)
    pub stdout: Option<String>,

    /// Standard error from command execution (None for syntax-only tests)
    pub stderr: Option<String>,

    /// Execution duration in milliseconds (None for syntax-only tests)
    pub execution_duration_ms: Option<u64>,

    /// Whether the command was cancelled by the user
    pub cancelled: bool,

    /// Whether the command timed out
    pub timed_out: bool,

    /// Whether output was truncated due to size limit
    pub output_truncated: bool,
}
```

**Field Validation Rules**:

| Field | Validation | Notes |
|-------|------------|-------|
| shortcut_id | Non-empty string | UUID format from existing Shortcut model |
| command | Non-empty string | Original command string to execute |
| syntax_valid | Boolean | Always populated |
| syntax_error | Optional | Populated only when syntax_valid = false |
| preview | String | Empty string for execution tests, populated for syntax tests |
| timestamp | RFC3339 string | ISO 8601 format (e.g., "2025-11-02T10:30:00-08:00") |
| executed | Boolean | true = execution test, false = syntax test |
| exit_code | Optional i32 | None for syntax tests, Some(code) for executions |
| stdout | Optional String | None for syntax tests, Some(output) for executions (may be empty string) |
| stderr | Optional String | None for syntax tests, Some(output) for executions (may be empty string) |
| execution_duration_ms | Optional u64 | None for syntax tests, Some(ms) for executions |
| cancelled | Boolean | true only when user clicked cancel button |
| timed_out | Boolean | true only when execution exceeded 30-second timeout |
| output_truncated | Boolean | true if stdout or stderr exceeded 10,000 character limit |

**State Transitions**:

```
Syntax Validation Flow:
  → executed = false
  → exit_code, stdout, stderr, execution_duration_ms = None
  → cancelled, timed_out, output_truncated = false

Successful Execution Flow:
  → executed = true
  → exit_code = Some(0 or non-zero)
  → stdout, stderr = Some(string, possibly empty)
  → execution_duration_ms = Some(milliseconds)
  → cancelled = false, timed_out = false
  → output_truncated = true if output > 10,000 chars

Cancelled Execution Flow:
  → executed = true
  → exit_code = None (or Some(-1) if available)
  → stdout, stderr = Some(partial output before cancellation)
  → execution_duration_ms = Some(milliseconds until cancellation)
  → cancelled = true, timed_out = false

Timeout Execution Flow:
  → executed = true
  → exit_code = None (or Some(-1) if available)
  → stdout, stderr = Some(partial output before timeout)
  → execution_duration_ms = Some(30000) # exactly 30 seconds
  → cancelled = false, timed_out = true
```

**Backward Compatibility**:
- Existing TestResult consumers (syntax validation) continue to work unchanged
- New fields are optional or have sensible defaults
- `executed` flag allows consumers to handle both test types appropriately

---

### ExecutionState (New)

Manages running command process state for cancellation support.

**Location**: `src-tauri/src/commands/testing.rs` (internal state, not exposed via IPC)

**Schema**:

```rust
use std::sync::{Arc, Mutex};
use tokio::process::Child;
use std::collections::HashMap;

pub struct ExecutionState {
    /// Map of shortcut ID to running process
    running_processes: Arc<Mutex<HashMap<String, Child>>>,
}
```

**Purpose**: Track running command processes per shortcut to enable cancellation

**Lifecycle**:
1. Created when command execution starts
2. Stored in Tauri State manager
3. Updated when process spawns
4. Cleaned up when process completes or is cancelled

**Thread Safety**: Arc<Mutex<>> provides thread-safe access across async Tauri commands

---

### DestructivePattern (New)

Configuration for destructive command detection patterns.

**Location**: `src-tauri/src/commands/testing.rs` (internal, not exposed via IPC)

**Schema**:

```rust
pub struct DestructivePattern {
    /// Regex pattern to match
    pattern: regex::Regex,

    /// Human-readable description
    description: String,

    /// Severity level: critical, high, medium
    severity: DestructiveSeverity,
}

pub enum DestructiveSeverity {
    Critical,  // System-level operations (shutdown, rm -rf /)
    High,      // Privilege escalation (sudo, doas)
    Medium,    // Process/file operations in user space
}
```

**Default Patterns**:
```rust
const DESTRUCTIVE_PATTERNS: &[(&str, &str, DestructiveSeverity)] = &[
    (r"sudo\s+", "Requires elevated privileges", DestructiveSeverity::High),
    (r"rm\s+.*-[rf]", "Recursive file deletion", DestructiveSeverity::Critical),
    (r"shutdown|reboot|halt", "System shutdown command", DestructiveSeverity::Critical),
    (r"kill(all)?\s+", "Process termination", DestructiveSeverity::Medium),
    (r"dd\s+", "Disk write operation", DestructiveSeverity::Critical),
    (r"mkfs|fdisk", "Disk formatting", DestructiveSeverity::Critical),
    // ... more patterns added during implementation
];
```

---

## Frontend TypeScript Types

**Location**: `src/types.ts`

```typescript
export interface TestResult {
  shortcut_id: string;
  command: string;
  syntax_valid: boolean;
  syntax_error?: string;
  preview: string;
  timestamp: string;

  // Execution fields
  executed: boolean;
  exit_code?: number;
  stdout?: string;
  stderr?: string;
  execution_duration_ms?: number;
  cancelled: boolean;
  timed_out: boolean;
  output_truncated: boolean;
}

export type ExecutionStatus =
  | 'idle'
  | 'confirming'
  | 'executing'
  | 'success'
  | 'error'
  | 'cancelled'
  | 'timeout';
```

---

## Entity Relationships

```
Shortcut (existing)
    │
    │ 1:N (one shortcut can have many test results over time)
    │
    ├─► TestResult
    │       ├─ executed = false → Syntax Validation
    │       └─ executed = true → Execution Result
    │               ├─ Success (exit_code = 0)
    │               ├─ Failure (exit_code ≠ 0)
    │               ├─ Cancelled (cancelled = true)
    │               └─ Timeout (timed_out = true)
    │
    └─► ExecutionState (transient, 0:1)
            └─ Running process for cancellation
```

**Notes**:
- TestResult is ephemeral (not persisted across app restarts)
- ExecutionState exists only during active command execution
- One shortcut can have at most one running execution at a time
- Previous test results are replaced by new results (no history)

---

## Data Flow

```
Frontend (Svelte)
    │
    │ invoke('test_shortcut', { id, execute: false })
    ├──────────────────────────────────────────────────►
    │                                                     Backend (Rust)
    │                                                     ├─ Check if destructive
    │                                                     ├─ If execute=false: syntax validation
    │                                                     └─ If execute=true: async execution
    │                                                           ├─ Spawn process
    │                                                           ├─ Store in ExecutionState
    │                                                           ├─ Wait with timeout
    │                                                           └─ Build TestResult
    │ ◄──────────────────────────────────────────────────┤
    │ TestResult { executed: true, ... }
    │
    └─► Display in TestResultDisplay component
```

**Cancellation Flow**:
```
Frontend                                    Backend
    │                                          │
    │ invoke('cancel_command', { id })         │
    ├──────────────────────────────────────────►
    │                                          ├─ Lookup process in ExecutionState
    │                                          ├─ Send SIGTERM/SIGKILL
    │                                          └─ Remove from state
    │ ◄──────────────────────────────────────┤
    │ Ok(())
    │
    └─► Update UI to cancelled state
```

---

## Validation Rules Summary

| Validation | Rule | Enforcement |
|------------|------|-------------|
| Command non-empty | command.len() > 0 | Backend before execution |
| Shortcut exists | Find in config | Backend before execution |
| Output size limit | stdout/stderr ≤ 10,000 chars | Backend during execution |
| Timeout limit | execution time ≤ 30 seconds | Backend via tokio::timeout |
| Single execution | At most one process per shortcut | Backend via ExecutionState |
| Exit code range | -1 (error) to 255 (standard) | Backend from process |

---

## Performance Considerations

| Aspect | Consideration | Mitigation |
|--------|---------------|------------|
| Large output | Memory usage | Truncate at 10,000 characters |
| Long execution | UI blocking | Async commands with tokio |
| Multiple tests | Resource usage | One execution at a time per shortcut |
| Process cleanup | Zombie processes | Explicit kill on cancel/timeout |
| IPC overhead | Large result serialization | Send only truncated output |

---

## Testing Strategy

### Unit Tests
- TestResult serialization/deserialization
- DestructivePattern regex matching
- Output truncation logic
- State transitions (syntax → execution → complete)

### Integration Tests
- Execute simple command, verify stdout captured
- Execute failing command, verify stderr and exit code
- Cancel long-running command, verify process killed
- Timeout command, verify timeout flag set
- Large output command, verify truncation

### Property Tests
- All valid TestResults serialize/deserialize correctly
- Exit codes always in valid range
- Timestamps always parse as RFC3339

---

## Migration Plan

**Current State**: TestResult with 6 fields (syntax validation only)

**Target State**: TestResult with 14 fields (syntax + execution)

**Migration Steps**:
1. ✅ No database migration needed (data not persisted)
2. ✅ Add new fields with Option types (backward compatible)
3. ✅ Existing syntax validation code unchanged
4. ✅ New execution code populates new fields
5. ✅ Frontend handles both old and new result formats via `executed` flag

**Rollback**: Remove new fields, revert to 6-field TestResult (no data loss)

---

## Next Steps

1. Generate API contracts in `/contracts` directory
2. Implement TestResult extensions in Rust
3. Create TypeScript type definitions
4. Build test suite for data model validation
