# Tauri Command Contracts: Configuration Import/Export

**Feature**: 002-config-import-export
**Date**: 2025-11-01

This file defines the Tauri command signatures (Rust backend ↔ TypeScript frontend bridge).

## Table of Contents

- [Commands](#commands)
  - [import_config](#import_config)
  - [export_config](#export_config)
  - [reload_config](#reload_config)
- [TypeScript Service Interface](#typescript-service-interface)
- [Error Handling Contract](#error-handling-contract)
- [Testing Contracts](#testing-contracts)

---

## Commands

### import_config

**Description**: Opens native macOS file picker dialog for user to select an skhd configuration file. Loads and parses the selected file, updates the current configuration state, and returns the loaded ConfigFile.

**User Story**: US1 - Import Configuration from Custom Location (P1)

**Rust Signature**:
```rust
#[tauri::command]
async fn import_config(
    state: State<'_, ConfigState>
) -> Result<ConfigFile, String>
```

**Parameters**: None (file selection handled by dialog)

**Returns**:

- **Success** (`ConfigFile`): Complete configuration loaded from selected file, including:
  - `current_file_path`: Set to the selected file path
  - `shortcuts`: Parsed shortcuts from file
  - `parse_errors`: Any non-fatal parsing warnings
  - `is_modified`: false (freshly loaded)

- **Error** (`String`): User-friendly error message for failures:
  - `"Import cancelled"` - User closed dialog without selecting
  - `"Cannot access file..."` - Permission denied
  - `"Configuration file not found..."` - File doesn't exist
  - `"Invalid skhd configuration..."` - Parse errors

**Behavior**:
1. Opens AsyncFileDialog with filters: `["skhdrc", "conf", "txt"]`
2. Default directory: `~/.config/skhd/`
3. On user selection: Parse file with existing parser
4. Validate syntax before accepting
5. Update `ConfigState.config` with new ConfigFile
6. Set `current_file_path` to selected path
7. Return ConfigFile to frontend

**Side Effects**:
- Updates global ConfigState
- Previous unsaved changes are lost (frontend should warn)

**Performance**:
- File dialog: Native macOS (no latency)
- Parse time: <100ms for typical configs (<1000 lines)
- Total: <200ms for user-perceived operation

---

### export_config

**Description**: Opens native macOS save file dialog for user to select export destination. Serializes current in-memory configuration to skhd text format, validates syntax, and writes to selected file atomically.

**User Story**: US2 - Export Current Configuration (P2)

**Rust Signature**:
```rust
#[tauri::command]
async fn export_config(
    state: State<'_, ConfigState>
) -> Result<String, String>
```

**Parameters**: None (file destination handled by dialog)

**Returns**:

- **Success** (`String`): Absolute path to the exported file for user confirmation.
  - Example: `"/Users/alice/Desktop/my-config.skhdrc"`

- **Error** (`String`): User-friendly error message for failures:
  - `"Export cancelled"` - User closed dialog
  - `"Cannot write to directory..."` - Permission denied
  - `"Configuration validation failed..."` - Internal state invalid
  - `"File operation failed..."` - I/O error during write

**Behavior**:
1. Get current ConfigFile from state
2. Serialize to skhd text format
3. Validate by re-parsing serialized text
4. Open `AsyncFileDialog.save_file()`
5. Default filename: "skhdrc" or current filename
6. On user selection: Write atomically (temp + rename)
7. Return exported file path
8. Does NOT change `current_file_path` (export is copy operation)

**Side Effects**:
- Creates/overwrites file at user-selected path
- No changes to ConfigState (export-only operation)

**Performance**:
- Serialization: <10ms
- Validation: <50ms
- Write: <50ms
- Total: <200ms

---

### reload_config

**Description**: Reloads configuration from the currently tracked file path (`current_file_path`). Discards all in-memory unsaved changes. Frontend should show confirmation dialog before calling if `is_modified` is true.

**User Story**: US3 - Reload from Default Location (P1)

**Rust Signature**:
```rust
#[tauri::command]
async fn reload_config(
    state: State<'_, ConfigState>
) -> Result<ConfigFile, String>
```

**Parameters**: None (reloads from `current_file_path` in state)

**Returns**:

- **Success** (`ConfigFile`): Freshly parsed configuration from `current_file_path`:
  - `shortcuts`: Re-parsed from disk
  - `is_modified`: false (clean state)
  - `current_file_path`: Unchanged (still same source)

- **Error** (`String`): User-friendly error message for failures:
  - `"Configuration file not found..."` - File deleted externally
  - `"Cannot access file..."` - Permission changed
  - `"Invalid skhd configuration..."` - File corrupted externally

**Behavior**:
1. Read `current_file_path` from ConfigState
2. Re-parse file using existing load logic
3. Replace `ConfigState.config` with fresh parse
4. Set `is_modified = false`
5. Return new ConfigFile

**Side Effects**:
- **DESTRUCTIVE**: Discards all unsaved in-memory changes
- Updates global ConfigState

**Performance**:
- Parse time: <100ms
- State update: <10ms
- Total: <150ms

---

## TypeScript Service Interface

**File**: `src/services/tauri.ts`

### importConfig

**Signature**:
```typescript
export async function importConfig(): Promise<ConfigFile>
```

**Implementation**:
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { ConfigFile } from '../types';

export async function importConfig(): Promise<ConfigFile> {
  return invoke<ConfigFile>('import_config');
}
```

**Usage Example**:
```typescript
// In Svelte component
async function handleImport() {
  try {
    const config = await importConfig();
    // Config state updated automatically by Tauri
    // UI reactivity will update display
  } catch (error) {
    if (error !== "Import cancelled") {
      showError(error);
    }
  }
}
```

---

### exportConfig

**Signature**:
```typescript
export async function exportConfig(): Promise<string>
```

**Implementation**:
```typescript
export async function exportConfig(): Promise<string> {
  return invoke<string>('export_config');
}
```

**Usage Example**:
```typescript
async function handleExport() {
  try {
    const exportedPath = await exportConfig();
    showSuccess(`Configuration exported to: ${exportedPath}`);
  } catch (error) {
    if (error !== "Export cancelled") {
      showError(error);
    }
  }
}
```

---

### reloadConfig

**Signature**:
```typescript
export async function reloadConfig(): Promise<ConfigFile>
```

**Implementation**:
```typescript
export async function reloadConfig(): Promise<ConfigFile> {
  return invoke<ConfigFile>('reload_config');
}
```

**Usage Example**:
```typescript
async function handleReload() {
  // Check for unsaved changes first
  if (config.is_modified) {
    const confirmed = await showConfirmDialog(
      'Discard unsaved changes?',
      'This will reload from disk and lose your current edits.'
    );
    if (!confirmed) return;
  }

  try {
    const reloadedConfig = await reloadConfig();
    // State updated, UI will refresh
  } catch (error) {
    showError(error);
  }
}
```

---

## Error Handling Contract

### Philosophy

All errors are converted to user-friendly strings before crossing the Tauri bridge. Frontend displays errors in toast notifications or modals. "Cancelled" operations are not shown as errors to the user.

### Error Display

- **Location**: Toast notification component
- **Duration**: 5 seconds for errors, 3 seconds for success
- **Dismiss**: User can click to dismiss early

### Cancellation Handling

When user cancels a file dialog, commands return error string `"Import cancelled"` or `"Export cancelled"`. Frontend should NOT display these as errors - they are normal user actions.

---

## Testing Contracts

### Unit Tests

- Import with valid file → Returns ConfigFile
- Import with invalid syntax → Returns parse errors in ConfigFile
- Import cancelled → Returns error "Import cancelled"
- Export valid config → Creates file at path
- Export with validation failure → Returns error
- Reload when file exists → Returns fresh ConfigFile
- Reload when file missing → Returns error

### Integration Tests

- Import → Edit → Export → Import again → Verify round-trip
- Import custom → Save → Reload → Verify unchanged
- Reload with unsaved changes → Verify changes discarded

### Manual Tests

- File dialog appears native on macOS
- File filters work correctly
- Permission errors show clear messages
- Large files (1000+ lines) parse quickly
