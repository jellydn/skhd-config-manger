# Quickstart Guide: Command Execution Test

**Feature**: Command Execution Test
**Audience**: Developers implementing this feature
**Date**: 2025-11-02

## Overview

This quickstart guide provides a practical walkthrough for implementing the command execution test feature. Follow these steps to add command execution capabilities to the existing test button.

---

## Prerequisites

Before starting implementation:

✅ Read `spec.md` - Understand user requirements and success criteria
✅ Read `research.md` - Understand technical decisions and patterns
✅ Read `data-model.md` - Understand data structures
✅ Read `contracts/tauri-commands.md` - Understand API contracts
✅ Rust 1.75+ installed
✅ Tauri v2 project setup complete
✅ Existing skhd-gui codebase cloned and building

---

## Implementation Roadmap

### Phase 1: Backend - TestResult Model Extension (30 minutes)

**File**: `src-tauri/src/models/test_result.rs`

**Current State**:
```rust
pub struct TestResult {
    pub shortcut_id: String,
    pub command: String,
    pub syntax_valid: bool,
    pub syntax_error: Option<String>,
    pub preview: String,
    pub timestamp: String,
}
```

**Target State**:
```rust
pub struct TestResult {
    // Existing fields
    pub shortcut_id: String,
    pub command: String,
    pub syntax_valid: bool,
    pub syntax_error: Option<String>,
    pub preview: String,
    pub timestamp: String,

    // New execution fields
    pub executed: bool,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub execution_duration_ms: Option<u64>,
    pub cancelled: bool,
    pub timed_out: bool,
    pub output_truncated: bool,
}
```

**Steps**:
1. Add new fields to struct with proper types
2. Update Debug, Clone, Serialize, Deserialize derives (already present)
3. No constructor changes needed (fields are public)
4. Run `cargo test` to verify existing tests still pass

**Validation**: Existing syntax validation tests should compile and run without modification

---

### Phase 2: Backend - Destructive Command Detection (45 minutes)

**File**: `src-tauri/src/commands/testing.rs`

**Add**:
```rust
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DESTRUCTIVE_PATTERNS: Vec<(Regex, &'static str, &'static str)> = vec![
        (Regex::new(r"sudo\s+").unwrap(), "high", "Requires elevated privileges"),
        (Regex::new(r"rm\s+.*-[rf]").unwrap(), "critical", "Recursive file deletion"),
        (Regex::new(r"shutdown|reboot|halt").unwrap(), "critical", "System shutdown"),
        (Regex::new(r"kill(all)?\s+").unwrap(), "medium", "Process termination"),
        (Regex::new(r"dd\s+").unwrap(), "critical", "Disk write operation"),
        (Regex::new(r"mkfs|fdisk").unwrap(), "critical", "Disk formatting"),
        // Add more patterns as needed
    ];
}

#[derive(Serialize, Deserialize)]
pub struct DestructiveCheck {
    pub is_destructive: bool,
    pub matched_patterns: Vec<String>,
    pub severity: Option<String>,
    pub warning_message: Option<String>,
}

#[tauri::command]
pub fn check_destructive_command(command: String) -> Result<DestructiveCheck, String> {
    for (pattern, severity, description) in DESTRUCTIVE_PATTERNS.iter() {
        if pattern.is_match(&command) {
            return Ok(DestructiveCheck {
                is_destructive: true,
                matched_patterns: vec![pattern.to_string()],
                severity: Some(severity.to_string()),
                warning_message: Some(format!(
                    "{}. Are you sure you want to execute this?",
                    description
                )),
            });
        }
    }

    Ok(DestructiveCheck {
        is_destructive: false,
        matched_patterns: vec![],
        severity: None,
        warning_message: None,
    })
}
```

**Dependencies**: Add to `Cargo.toml`:
```toml
regex = "1.10"
lazy_static = "1.4"
```

**Steps**:
1. Add dependencies to Cargo.toml
2. Implement pattern matching function
3. Create DestructiveCheck struct
4. Add check_destructive_command Tauri command
5. Write unit tests for pattern matching

**Test**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_sudo() {
        let result = check_destructive_command("sudo apt-get install foo".to_string()).unwrap();
        assert!(result.is_destructive);
        assert_eq!(result.severity, Some("high".to_string()));
    }

    #[test]
    fn test_safe_command() {
        let result = check_destructive_command("echo hello".to_string()).unwrap();
        assert!(!result.is_destructive);
    }
}
```

---

### Phase 3: Backend - Command Execution (90 minutes)

**File**: `src-tauri/src/commands/testing.rs`

**Add ExecutionState**:
```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::process::{Command, Child};

pub struct ExecutionState {
    running_processes: Arc<Mutex<HashMap<String, Child>>>,
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            running_processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
```

**Add execute_shortcut_command**:
```rust
use tokio::time::{timeout, Duration};
use std::time::Instant;

#[tauri::command]
pub async fn execute_shortcut_command(
    shortcut_id: String,
    state: State<'_, ConfigState>,
    exec_state: State<'_, ExecutionState>,
) -> Result<TestResult, String> {
    // 1. Find shortcut
    let config_guard = state.config.lock().unwrap();
    let config = config_guard.as_ref().ok_or("No configuration loaded")?;
    let shortcut = config
        .shortcuts
        .iter()
        .find(|s| s.id == shortcut_id)
        .ok_or("Shortcut not found")?;

    // 2. Check if already running
    {
        let processes = exec_state.running_processes.lock().unwrap();
        if processes.contains_key(&shortcut_id) {
            return Err(format!("Command already executing for shortcut: {}", shortcut_id));
        }
    }

    // 3. Spawn command
    let start = Instant::now();
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&shortcut.command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    // 4. Store in execution state
    exec_state.running_processes.lock().unwrap().insert(shortcut_id.clone(), child);

    // 5. Wait with timeout
    let output_result = timeout(Duration::from_secs(30), child.wait_with_output()).await;

    // 6. Remove from execution state
    exec_state.running_processes.lock().unwrap().remove(&shortcut_id);

    // 7. Build result
    let duration_ms = start.elapsed().as_millis() as u64;

    match output_result {
        Ok(Ok(output)) => {
            // Successful execution or failure (non-zero exit code)
            let (stdout, stdout_truncated) = truncate_output(
                String::from_utf8_lossy(&output.stdout).to_string(),
                10000,
            );
            let (stderr, stderr_truncated) = truncate_output(
                String::from_utf8_lossy(&output.stderr).to_string(),
                10000,
            );

            Ok(TestResult {
                shortcut_id: shortcut.id.clone(),
                command: shortcut.command.clone(),
                syntax_valid: true,
                syntax_error: None,
                preview: String::new(),
                timestamp: chrono::Local::now().to_rfc3339(),
                executed: true,
                exit_code: output.status.code(),
                stdout: Some(stdout),
                stderr: Some(stderr),
                execution_duration_ms: Some(duration_ms),
                cancelled: false,
                timed_out: false,
                output_truncated: stdout_truncated || stderr_truncated,
            })
        }
        Ok(Err(e)) => {
            // Failed to wait for output
            Err(format!("Execution failed: {}", e))
        }
        Err(_) => {
            // Timeout
            Ok(TestResult {
                shortcut_id: shortcut.id.clone(),
                command: shortcut.command.clone(),
                syntax_valid: true,
                syntax_error: None,
                preview: String::new(),
                timestamp: chrono::Local::now().to_rfc3339(),
                executed: true,
                exit_code: None,
                stdout: Some(String::new()),
                stderr: Some(String::new()),
                execution_duration_ms: Some(30000),
                cancelled: false,
                timed_out: true,
                output_truncated: false,
            })
        }
    }
}

fn truncate_output(output: String, limit: usize) -> (String, bool) {
    if output.len() > limit {
        (output[..limit].to_string(), true)
    } else {
        (output, false)
    }
}
```

**Add cancel_shortcut_execution**:
```rust
#[tauri::command]
pub async fn cancel_shortcut_execution(
    shortcut_id: String,
    exec_state: State<'_, ExecutionState>,
) -> Result<(), String> {
    let mut processes = exec_state.running_processes.lock().unwrap();

    if let Some(mut child) = processes.remove(&shortcut_id) {
        child.kill()
            .await
            .map_err(|e| format!("Failed to kill process: {}", e))?;
        Ok(())
    } else {
        Err(format!("No execution running for shortcut: {}", shortcut_id))
    }
}
```

**Register Commands**: `src-tauri/src/lib.rs`
```rust
fn main() {
    tauri::Builder::default()
        .manage(ConfigState::default())
        .manage(ExecutionState::default()) // Add this
        .invoke_handler(tauri::generate_handler![
            // ... existing commands ...
            execute_shortcut_command,
            cancel_shortcut_execution,
            check_destructive_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### Phase 4: Frontend - TypeScript Types (15 minutes)

**File**: `src/types.ts`

**Add**:
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

export interface DestructiveCheck {
  is_destructive: boolean;
  matched_patterns: string[];
  severity: 'critical' | 'high' | 'medium' | null;
  warning_message: string | null;
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

### Phase 5: Frontend - Update ShortcutItem Component (60 minutes)

**File**: `src/components/ShortcutItem.svelte`

**Add State**:
```typescript
let executionState: ExecutionStatus = 'idle';
let executingShortcutId: string | null = null;

async function handleTestClick() {
  // Check if destructive
  const check = await invoke<DestructiveCheck>('check_destructive_command', {
    command: shortcut.command
  });

  if (check.is_destructive) {
    executionState = 'confirming';
    // Show confirmation dialog (handled below)
  } else {
    await executeCommand();
  }
}

async function executeCommand() {
  executionState = 'executing';
  executingShortcutId = shortcut.id;

  try {
    const result = await invoke<TestResult>('execute_shortcut_command', {
      shortcutId: shortcut.id
    });

    if (result.timed_out) {
      executionState = 'timeout';
    } else if (result.cancelled) {
      executionState = 'cancelled';
    } else if (result.exit_code === 0) {
      executionState = 'success';
    } else {
      executionState = 'error';
    }

    if (onTest) {
      onTest(result);
    }
  } catch (error) {
    console.error('Execution failed:', error);
    executionState = 'error';
  } finally {
    executingShortcutId = null;
  }
}

async function handleCancelClick() {
  if (!executingShortcutId) return;

  try {
    await invoke('cancel_shortcut_execution', {
      shortcutId: executingShortcutId
    });
    executionState = 'cancelled';
  } catch (error) {
    console.error('Cancellation failed:', error);
  }
}
```

**Update Template**:
```svelte
{#if executionState === 'executing'}
  <button type="button" class="btn-cancel" onclick={handleCancelClick}>
    <svg><!-- cancel icon --></svg>
    Cancel
  </button>
{:else}
  <button type="button" class="btn-test" onclick={handleTestClick}>
    <svg><!-- play icon --></svg>
    {executionState === 'idle' ? 'Test' : executionState}
  </button>
{/if}
```

---

### Phase 6: Frontend - Destructive Command Confirmation (30 minutes)

**File**: Reuse existing `src/components/ConfirmDialog.svelte`

**Usage in ShortcutItem**:
```typescript
import ConfirmDialog from './ConfirmDialog.svelte';

let showConfirmDialog = false;
let confirmMessage = '';

async function handleTestClick() {
  const check = await invoke<DestructiveCheck>('check_destructive_command', {
    command: shortcut.command
  });

  if (check.is_destructive) {
    confirmMessage = check.warning_message || 'This command may be destructive. Continue?';
    showConfirmDialog = true;
  } else {
    await executeCommand();
  }
}

function handleConfirm() {
  showConfirmDialog = false;
  executeCommand();
}

function handleCancel() {
  showConfirmDialog = false;
  executionState = 'idle';
}
```

**Template**:
```svelte
{#if showConfirmDialog}
  <ConfirmDialog
    message={confirmMessage}
    onConfirm={handleConfirm}
    onCancel={handleCancel}
  />
{/if}
```

---

### Phase 7: Frontend - Update TestResultDisplay (45 minutes)

**File**: `src/components/TestResultDisplay.svelte`

**Enhance to show execution results**:
```svelte
<script lang="ts">
  import type { TestResult } from '../types';

  interface Props {
    result: TestResult;
  }

  let { result }: Props = $props();
</script>

<div class="test-result">
  {#if result.executed}
    <!-- Execution Result -->
    <div class="result-header" class:success={result.exit_code === 0} class:error={result.exit_code !== 0}>
      {#if result.timed_out}
        <span class="status-icon">⏱️</span>
        <span>Timed out after {result.execution_duration_ms}ms</span>
      {:else if result.cancelled}
        <span class="status-icon">🚫</span>
        <span>Cancelled by user</span>
      {:else if result.exit_code === 0}
        <span class="status-icon">✅</span>
        <span>Success (exit code 0)</span>
      {:else}
        <span class="status-icon">❌</span>
        <span>Failed (exit code {result.exit_code})</span>
      {/if}
      <span class="duration">{result.execution_duration_ms}ms</span>
    </div>

    {#if result.stdout}
      <div class="output-section">
        <h4>Standard Output</h4>
        <pre><code>{result.stdout}</code></pre>
        {#if result.output_truncated}
          <div class="truncation-notice">⚠️ Output truncated (showing first 10,000 characters)</div>
        {/if}
      </div>
    {/if}

    {#if result.stderr}
      <div class="output-section error">
        <h4>Standard Error</h4>
        <pre><code>{result.stderr}</code></pre>
      </div>
    {/if}
  {:else}
    <!-- Syntax Validation Result -->
    {#if result.syntax_valid}
      <div class="preview">{result.preview}</div>
    {:else}
      <div class="syntax-error">{result.syntax_error}</div>
    {/if}
  {/if}
</div>

<style>
  .result-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    border-radius: 6px;
    font-weight: 500;
  }

  .result-header.success {
    background: #d4edda;
    color: #155724;
  }

  .result-header.error {
    background: #f8d7da;
    color: #721c24;
  }

  .output-section {
    margin-top: 1rem;
  }

  .output-section pre {
    background: #f5f5f5;
    padding: 1rem;
    border-radius: 4px;
    overflow-x: auto;
  }

  .truncation-notice {
    background: #fff3cd;
    color: #856404;
    padding: 0.5rem;
    border-radius: 4px;
    margin-top: 0.5rem;
  }
</style>
```

---

## Testing Workflow

### Backend Tests

**File**: `src-tauri/tests/command_execution.rs`

```rust
#[tokio::test]
async fn test_execute_simple_command() {
    // Setup
    let state = setup_test_state();

    // Execute
    let result = execute_shortcut_command("test-id".to_string(), state, exec_state)
        .await
        .unwrap();

    // Assert
    assert!(result.executed);
    assert_eq!(result.exit_code, Some(0));
    assert!(result.stdout.is_some());
}

#[tokio::test]
async fn test_command_timeout() {
    let result = execute_shortcut_command("sleep-60".to_string(), state, exec_state)
        .await
        .unwrap();

    assert!(result.timed_out);
    assert_eq!(result.execution_duration_ms, Some(30000));
}
```

### Frontend Tests

**File**: `src/components/__tests__/ShortcutItem.test.ts`

```typescript
import { render, fireEvent } from '@testing-library/svelte';
import ShortcutItem from '../ShortcutItem.svelte';
import { vi } from 'vitest';

test('shows confirmation for destructive command', async () => {
  const mockInvoke = vi.fn().mockResolvedValue({
    is_destructive: true,
    warning_message: 'This is destructive'
  });

  const { getByText } = render(ShortcutItem, {
    shortcut: { command: 'rm -rf /tmp/test', ... }
  });

  await fireEvent.click(getByText('Test'));
  expect(getByText('This is destructive')).toBeInTheDocument();
});
```

---

## Common Pitfalls & Solutions

### Pitfall 1: UI Freezing During Execution
**Cause**: Using synchronous Tauri command
**Solution**: Ensure `execute_shortcut_command` is `async`

### Pitfall 2: Zombie Processes
**Cause**: Not cleaning up ExecutionState on timeout/cancel
**Solution**: Always remove from HashMap in all execution paths

### Pitfall 3: Large Output Memory Issues
**Cause**: Not truncating output
**Solution**: Implement truncation in backend before sending to frontend

### Pitfall 4: Race Condition on Cancel
**Cause**: Process completes before cancel arrives
**Solution**: Check if process exists before killing, return Ok(()) if not found

---

## Performance Validation

After implementation, verify:

✅ Command execution doesn't block UI (async)
✅ Cancellation completes within 1 second
✅ Timeout enforced at 30 seconds
✅ Memory usage stays under 100MB during execution
✅ Output truncation prevents memory exhaustion

**Tools**:
- macOS Activity Monitor for memory/CPU profiling
- `cargo flamegraph` for performance profiling
- Browser DevTools for frontend performance

---

## Next Steps

After completing implementation:

1. Run full test suite: `cargo test && bun run test`
2. Manual testing with various commands
3. Test destructive command confirmation flow
4. Test cancellation and timeout scenarios
5. Generate tasks.md with `/speckit.tasks`
6. Begin implementation following tasks.md

---

## Resources

- [Tauri Async Commands](https://tauri.app/v2/guides/features/commands/#async-commands)
- [tokio::process Documentation](https://docs.rs/tokio/latest/tokio/process/)
- [Svelte 5 Reactivity](https://svelte.dev/docs/svelte/reactivity)
- [Project Constitution](./.specify/memory/constitution.md)

---

## Support

For questions during implementation:
- Refer back to `research.md` for technical decisions
- Check `contracts/tauri-commands.md` for API details
- Review existing `testing.rs` for patterns
- Test frequently to catch issues early
