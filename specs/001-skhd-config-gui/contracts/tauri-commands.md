# Tauri Commands API Contract

**Feature**: 001-skhd-config-gui
**Created**: 2025-11-01
**Purpose**: Define the interface between Svelte frontend and Rust backend

## Overview

Tauri commands are Rust functions exposed to the frontend via the `#[tauri::command]` attribute. The frontend invokes these commands using the Tauri API, and they return `Result<T, String>` where errors are serialized as strings.

All commands are async and return JSON-serializable data structures.

---

## Commands

### 1. load_config

**Purpose**: Load the skhd configuration file from disk and parse it

**Frontend Invocation**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const config = await invoke<ConfigFile>('load_config', {
  customPath: null  // Optional: null uses default ~/.config/skhd/skhdrc
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn load_config(custom_path: Option<String>) -> Result<ConfigFile, String>
```

**Parameters**:
- `custom_path`: `Option<String>` - Optional custom path to config file. If None, uses `~/.config/skhd/skhdrc`

**Returns**: `ConfigFile`
```typescript
interface ConfigFile {
  file_path: string;
  shortcuts: Shortcut[];
  global_comments: string[];
  last_modified: string;  // ISO 8601 timestamp
  is_modified: boolean;
  backup_path: string | null;
  parse_errors: ParseError[];
}
```

**Errors**:
- `"File not found: {path}"` - Config file doesn't exist
- `"Permission denied: {path}"` - No read access to file
- `"Parse error at line {n}: {message}"` - Syntax error (still returns ConfigFile with parse_errors populated)

**Behavior**:
- If file doesn't exist, returns empty ConfigFile with parse_error indicating missing file
- If file exists but has syntax errors, returns ConfigFile with shortcuts that could be parsed + parse_errors array
- Always resolves ~ to user home directory

---

### 2. save_config

**Purpose**: Save the current configuration to disk with atomic write and automatic backup

**Frontend Invocation**:
```typescript
const result = await invoke<SaveResult>('save_config', {
  config: configFileObject
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn save_config(config: ConfigFile) -> Result<SaveResult, String>
```

**Parameters**:
- `config`: `ConfigFile` - The complete configuration to save

**Returns**: `SaveResult`
```typescript
interface SaveResult {
  success: boolean;
  backup_path: string;
  new_last_modified: string;  // ISO 8601 timestamp
}
```

**Errors**:
- `"Validation failed: {message}"` - Configuration has invalid shortcuts
- `"Permission denied: {path}"` - No write access
- `"Backup failed: {message}"` - Could not create backup (save aborted)
- `"Write failed: {message}"` - Error during file write (original preserved)

**Behavior**:
1. Validate all shortcuts in config
2. Create backup with timestamp
3. Serialize config to skhd format
4. Write to temporary file in same directory
5. Re-parse temp file to validate
6. Atomic rename temp → target
7. Return backup path and new timestamp

**Guarantees** (per Constitution II):
- Never corrupts existing file
- Always creates backup before writing
- Only writes if validation passes
- Atomic operation (no partial writes)

---

### 3. validate_shortcut

**Purpose**: Validate a single shortcut before adding/editing (real-time validation)

**Frontend Invocation**:
```typescript
const result = await invoke<ValidationResult>('validate_shortcut', {
  shortcut: newShortcut,
  existingShortcuts: currentShortcutsList,
  mode: null
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn validate_shortcut(
    shortcut: Shortcut,
    existing_shortcuts: Vec<Shortcut>,
    mode: Option<String>
) -> Result<ValidationResult, String>
```

**Parameters**:
- `shortcut`: `Shortcut` - The shortcut to validate
- `existing_shortcuts`: `Vec<Shortcut>` - Current list of shortcuts (for duplicate detection)
- `mode`: `Option<String>` - Current mode (null for global scope)

**Returns**: `ValidationResult`
```typescript
interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
```

**Validation Checks**:
- Command not empty
- Key is valid skhd key
- Modifiers are valid (cmd, alt, shift, ctrl, fn)
- No duplicate key combination in same mode
- Mode name valid format (if present)

**Errors vs Warnings**:
- **Errors**: Block saving (invalid syntax, duplicates)
- **Warnings**: Informational (e.g., "This overrides a system shortcut")

---

### 4. create_backup

**Purpose**: Manually create a backup of the current config file (for "Backup Now" feature)

**Frontend Invocation**:
```typescript
const backup = await invoke<Backup>('create_backup', {
  configPath: '/Users/username/.config/skhd/skhdrc'
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn create_backup(config_path: String) -> Result<Backup, String>
```

**Parameters**:
- `config_path`: `String` - Path to config file to backup

**Returns**: `Backup`
```typescript
interface Backup {
  original_path: string;
  backup_path: string;
  created_at: string;  // ISO 8601
  file_size: number;
  checksum: string;    // SHA-256 hex
}
```

**Errors**:
- `"File not found: {path}"` - Source file doesn't exist
- `"Permission denied"` - Can't read source or write to backup location
- `"Backup already exists: {path}"` - Timestamp collision (unlikely)

**Behavior**:
- Creates backup with format: `skhdrc.backup.YYYY-MM-DD-HHmmss`
- Computes SHA-256 checksum for integrity
- Returns full backup metadata

---

### 5. list_backups

**Purpose**: List all available backups for the config file

**Frontend Invocation**:
```typescript
const backups = await invoke<Backup[]>('list_backups', {
  configPath: '/Users/username/.config/skhd/skhdrc'
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn list_backups(config_path: String) -> Result<Vec<Backup>, String>
```

**Parameters**:
- `config_path`: `String` - Path to config file (backups are in same directory)

**Returns**: `Vec<Backup>` - Array of backup metadata, sorted by created_at (newest first)

**Errors**:
- `"Permission denied"` - Can't read backup directory

**Behavior**:
- Scans directory for files matching `skhdrc.backup.*` pattern
- Parses timestamps from filenames
- Validates each backup file exists and is readable
- Returns sorted list (newest first)

---

### 6. restore_backup

**Purpose**: Restore configuration from a backup file

**Frontend Invocation**:
```typescript
const restored = await invoke<ConfigFile>('restore_backup', {
  backupPath: '/Users/username/.config/skhd/skhdrc.backup.2025-11-01-143022',
  configPath: '/Users/username/.config/skhd/skhdrc'
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn restore_backup(
    backup_path: String,
    config_path: String
) -> Result<ConfigFile, String>
```

**Parameters**:
- `backup_path`: `String` - Path to backup file to restore from
- `config_path`: `String` - Path to config file to restore to

**Returns**: `ConfigFile` - The restored configuration

**Errors**:
- `"Backup not found: {path}"` - Backup file doesn't exist
- `"Permission denied"` - Can't read backup or write to config
- `"Backup corrupt: {message}"` - Backup file is not valid skhd config

**Behavior**:
1. Create backup of current config (before restoring)
2. Read and validate backup file
3. Copy backup to config file (atomic operation)
4. Parse and return restored config

**Safety**: Always backs up current state before restoring

---

### 7. watch_config_file

**Purpose**: Start watching the config file for external changes (FR-015)

**Frontend Invocation**:
```typescript
await invoke('watch_config_file', {
  configPath: '/Users/username/.config/skhd/skhdrc'
});

// Listen for change events
import { listen } from '@tauri-apps/api/event';

listen<ConfigChangeEvent>('config-file-changed', (event) => {
  console.log('Config changed externally:', event.payload);
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn watch_config_file(
    config_path: String,
    app_handle: tauri::AppHandle
) -> Result<(), String>
```

**Parameters**:
- `config_path`: `String` - Path to config file to watch
- `app_handle`: `tauri::AppHandle` - Injected by Tauri for emitting events

**Returns**: `()` - Success only

**Events Emitted**:
```typescript
interface ConfigChangeEvent {
  file_path: string;
  change_type: 'modified' | 'deleted';
  timestamp: string;  // ISO 8601
}
```

**Errors**:
- `"Permission denied"` - Can't watch file
- `"File not found"` - File doesn't exist

**Behavior**:
- Uses platform file watcher (FSEvents on macOS)
- Emits `config-file-changed` event when file modified externally
- Frontend should prompt user to reload or ignore
- Watcher stops when app closes

---

### 8. check_skhd_running

**Purpose**: Check if skhd service is currently running (for testing shortcuts feature)

**Frontend Invocation**:
```typescript
const running = await invoke<boolean>('check_skhd_running');
```

**Rust Signature**:
```rust
#[tauri::command]
async fn check_skhd_running() -> Result<bool, String>
```

**Parameters**: None

**Returns**: `boolean` - True if skhd is running, false otherwise

**Errors**: None (returns false on any error)

**Behavior**:
- Checks for `skhd` process in process list
- Uses `pgrep skhd` command
- Returns true only if process found

---

### 9. get_permissions_status

**Purpose**: Check current file system permission status (FR-016, FR-017)

**Frontend Invocation**:
```typescript
const status = await invoke<PermissionsStatus>('get_permissions_status', {
  configPath: '/Users/username/.config/skhd/skhdrc'
});
```

**Rust Signature**:
```rust
#[tauri::command]
async fn get_permissions_status(config_path: String) -> Result<PermissionsStatus, String>
```

**Parameters**:
- `config_path`: `String` - Path to check permissions for

**Returns**: `PermissionsStatus`
```typescript
interface PermissionsStatus {
  can_read: boolean;
  can_write: boolean;
  file_exists: boolean;
  parent_writable: boolean;  // Can create file if it doesn't exist
}
```

**Errors**: None (returns status structure even on errors)

**Behavior**:
- Checks file permissions using Rust std::fs metadata
- If file doesn't exist, checks parent directory permissions
- Returns comprehensive permission status for UI decisions

---

## Type Definitions

### Shortcut
```typescript
interface Shortcut {
  id: string;           // UUID
  modifiers: string[];  // ["cmd", "shift"], etc.
  key: string;          // "f", "return", etc.
  command: string;      // Shell command
  mode: string | null;  // Mode name or null
  comment: string | null;
  line_number: number;
}
```

### ParseError
```typescript
interface ParseError {
  line_number: number;
  column: number | null;
  error_type: 'InvalidModifier' | 'InvalidKey' | 'MissingCommand' | 'InvalidSyntax' | 'DuplicateShortcut' | 'InvalidMode';
  message: string;
  line_content: string;
}
```

---

## Error Handling

All commands return `Result<T, String>`:
- **Success**: Resolves with data
- **Failure**: Rejects with error message string

Frontend should catch errors and display user-friendly messages:

```typescript
try {
  const config = await invoke<ConfigFile>('load_config', { customPath: null });
  // Handle success
} catch (error) {
  // error is a string
  console.error('Failed to load config:', error);
  // Show error to user
}
```

---

## Security Considerations

1. **Path Validation**: All file paths are validated and canonicalized in Rust to prevent directory traversal
2. **Sandboxing**: Tauri v2 sandboxing limits file system access to requested scope
3. **No Command Execution**: Config commands are not executed by the GUI (display only)
4. **Atomic Operations**: No race conditions in file writes (atomic rename)

---

## Performance SLAs

Per Constitution IV:
- `load_config`: <100ms for typical files (<1000 lines)
- `save_config`: <500ms total (backup + write + validate)
- `validate_shortcut`: <10ms (real-time validation)
- `watch_config_file`: Event emission <50ms after file change
- All other commands: <100ms

---

## Testing Strategy

**Unit Tests** (Rust):
- Test each command with valid inputs
- Test error cases (file not found, permission denied, invalid data)
- Test atomic write behavior (file corruption scenarios)

**Integration Tests** (Rust):
- Test full load → modify → save cycle
- Test backup creation and restoration
- Test file watcher events

**Frontend Tests** (Vitest):
- Mock Tauri invoke calls
- Test command invocation with correct parameters
- Test error handling in UI
