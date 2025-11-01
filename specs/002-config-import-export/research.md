# Research: Configuration Import/Export

**Feature**: 002-config-import-export
**Phase**: 0 - Outline & Research
**Date**: 2025-11-01

## Overview

This document consolidates research findings for implementing file import/export functionality in the Tauri-based skhd GUI manager. All technical decisions are aligned with constitutional principles and existing codebase patterns.

## Key Research Areas

### 1. Tauri File Dialog Integration

**Decision**: Use `rfd` crate with Tauri's async runtime

**Rationale**:
- `rfd` (Rust File Dialog) is the standard for Tauri v2 file picking
- Provides native macOS file dialogs (NSOpenPanel/NSSavePanel under the hood)
- Fully async/non-blocking, integrates with Tauri's tokio runtime
- Already used implicitly by Tauri for file operations

**Implementation Pattern**:
```rust
use rfd::AsyncFileDialog;

#[tauri::command]
async fn import_config() -> Result<ConfigFile, String> {
    let file = AsyncFileDialog::new()
        .add_filter("skhd config", &["skhdrc", "conf", "txt"])
        .set_title("Import skhd Configuration")
        .pick_file()
        .await;

    match file {
        Some(file) => {
            let path = file.path().to_path_buf();
            // Use existing load_config logic
        }
        None => Err("User cancelled".to_string())
    }
}
```

**Alternatives Considered**:
- Native Swift dialogs via custom Tauri plugin → Rejected: Unnecessary complexity, rfd already provides native dialogs
- Web-based file input → Rejected: Not native macOS experience, violates constitution

**Best Practices**:
- Use `add_filter()` to restrict to valid skhd config file types
- Set descriptive `set_title()` for clarity
- Handle cancellation gracefully (user closes dialog without selecting)
- Use `set_directory()` to start in sensible default (e.g., `~/.config/skhd/`)

**Dependencies**:
- `rfd = "0.14"` (add to src-tauri/Cargo.toml)
- Already compatible with Tauri v2

**References**:
- [rfd documentation](https://docs.rs/rfd/)
- [Tauri file system guide](https://v2.tauri.app/develop/calling-rust/)

---

### 2. File Path State Management

**Decision**: Add `current_file_path` field to existing `ConfigFile` model

**Rationale**:
- Tracks which file is currently loaded (default vs custom location)
- Enables "save to current location" behavior (FR-005 requirement)
- Minimal change to existing data model (simple String field)
- Frontend can display current file path in UI (FR-003 requirement)

**Data Model Change**:
```rust
// src-tauri/src/models/config_file.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub file_path: String,           // EXISTING: Path to config file
    pub shortcuts: Vec<Shortcut>,    // EXISTING
    // ... other existing fields ...

    // NEW FIELD:
    #[serde(default = "default_file_path")]
    pub current_file_path: String,   // Tracks currently loaded file
}

fn default_file_path() -> String {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".config/skhd/skhdrc")
        .to_string_lossy()
        .to_string()
}
```

**Alternatives Considered**:
- Separate `ConfigSource` struct → Rejected: Over-engineering for simple path tracking
- Store in global state separate from ConfigFile → Rejected: Splits related data unnecessarily

**Migration Strategy**:
- Use `#[serde(default)]` so existing serialized configs remain compatible
- On first load, set `current_file_path = file_path`

---

### 3. Unsaved Changes Detection

**Decision**: Leverage existing `is_modified` field in `ConfigFile`

**Rationale**:
- `is_modified` already tracks when in-memory state differs from disk
- Implemented in Feature 001, tested and working
- Reload command can check this field before proceeding
- No additional state tracking needed

**Reload Warning Logic**:
```rust
#[tauri::command]
async fn reload_config(state: State<'_, ConfigState>) -> Result<ConfigFile, String> {
    let current_config = state.config.lock().await;

    // Check if unsaved changes exist
    if current_config.is_modified {
        // Frontend will show confirmation dialog before calling this
        // This is a backend validation check only
    }

    let current_path = current_config.current_file_path.clone();
    drop(current_config); // Release lock

    // Reload from current_file_path using existing load logic
    load_config_from_path(&current_path, state).await
}
```

**Frontend Confirmation**:
```svelte
async function handleReload() {
  if (config.is_modified) {
    const confirmed = await showConfirmDialog(
      "Discard unsaved changes?",
      "This will reload from disk and lose your current changes."
    );
    if (!confirmed) return;
  }
  await reloadConfigAPI();
}
```

**Alternatives Considered**:
- Backend-side confirmation dialog → Rejected: Violates separation of concerns, UI logic belongs in frontend
- Auto-save before reload → Rejected: User expects "reload" to mean "discard changes"

---

### 4. Export File Validation

**Decision**: Use existing skhd parser to validate before export

**Rationale**:
- Export should never write invalid skhd syntax (Configuration Safety principle)
- Existing parser in `src-tauri/src/parser/` already validates syntax
- Test export by parsing in-memory before writing to disk

**Validation Strategy**:
```rust
#[tauri::command]
async fn export_config(state: State<'_, ConfigState>) -> Result<String, String> {
    // 1. Get current config
    let config = state.config.lock().await.clone();

    // 2. Convert to skhd text format
    let skhd_text = serialize_config(&config)?;

    // 3. VALIDATE by parsing back
    match parse_skhd_config(&skhd_text) {
        Ok(_) => {
            // 4. Show save dialog
            let file = AsyncFileDialog::new()
                .set_file_name("skhdrc")
                .set_title("Export Configuration")
                .save_file()
                .await;

            match file {
                Some(file) => {
                    let path = file.path();
                    // 5. Write using atomic pattern (existing logic)
                    write_config_atomic(path, &skhd_text)?;
                    Ok(path.to_string_lossy().to_string())
                }
                None => Err("Export cancelled".to_string())
            }
        }
        Err(e) => Err(format!("Config validation failed: {}", e))
    }
}
```

**Best Practices**:
- Always validate before writing
- Use atomic writes (tempfile + rename)
- Return exported file path to frontend for user feedback
- Handle permission errors gracefully

**Alternatives Considered**:
- Skip validation → Rejected: Violates Configuration Safety principle
- Validate after writing → Rejected: Too late, file already corrupted if invalid

---

### 5. Error Handling Patterns

**Decision**: Structured error types with user-friendly messages

**Rationale**:
- File operations have many failure modes (permissions, missing files, invalid paths)
- FR-011 requires clear error messages for all failures
- Maintain consistency with existing error handling in Feature 001

**Error Categories**:
1. **User Cancellation**: Not an error, return early gracefully
2. **Permission Denied**: "Cannot access file: permission denied. Check System Settings > Privacy & Security."
3. **File Not Found**: "Configuration file not found at [path]. The file may have been moved or deleted."
4. **Invalid Format**: "Invalid skhd configuration: [specific syntax error]"
5. **I/O Errors**: "File operation failed: [system error message]"

**Implementation Pattern**:
```rust
pub enum ConfigError {
    PermissionDenied(String),
    FileNotFound(String),
    InvalidFormat(String),
    IOError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::PermissionDenied(path) => {
                write!(f, "Cannot access file at {}: Permission denied. Check System Settings > Privacy & Security.", path)
            }
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {}. The file may have been moved or deleted.", path)
            }
            // ... other variants
        }
    }
}
```

**Best Practices**:
- Convert Rust errors to user-friendly strings before returning to frontend
- Include actionable guidance in error messages (e.g., "Check System Settings")
- Log detailed errors to console for debugging
- Never expose internal Rust error types to frontend

---

## Technology Stack Confirmation

### Current Project Stack (from Feature 001)
- **Tauri**: v2.1.1
- **Rust**: 1.75+ (edition 2021)
- **Frontend**: Svelte 5 with TypeScript
- **Build**: Bun (package manager), Vite (bundler)
- **Testing**: cargo test (Rust), vitest (TypeScript)

### New Dependencies Required
```toml
# src-tauri/Cargo.toml additions
[dependencies]
rfd = "0.14"  # File dialogs (if not already present via tauri dependencies)
```

**Verification**: Check if `rfd` is already in Cargo.toml via Tauri's dependencies before adding explicitly.

---

## Integration Points with Existing Code

### 1. Commands Module (`src-tauri/src/commands/config.rs`)
- **Existing**: `load_config`, `save_config`, `reload_config`
- **New**: `import_config`, `export_config`
- **Shared Logic**: Reuse parsing, validation, atomic writes

### 2. ConfigState (`src-tauri/src/commands/config.rs`)
```rust
pub struct ConfigState {
    pub config: Arc<Mutex<ConfigFile>>,
}
```
- No changes needed to state structure
- All commands use this same state

### 3. Frontend Service (`src/services/tauri.ts`)
- **Pattern**: All Tauri commands wrapped in async functions
- **New Exports**: `importConfig()`, `exportConfig()`, `reloadConfig()`

### 4. UI Components (`src/routes/+page.svelte`)
- **Existing Pattern**: Buttons in header with onclick handlers
- **New Buttons**: Import, Export, Reload (alongside existing Save button)
- **New Component**: `ConfirmDialog.svelte` for reload warning

---

## Constitutional Compliance Summary

| Principle | Compliance | Evidence |
|-----------|------------|----------|
| Native macOS Experience | ✅ PASS | rfd provides NSOpenPanel/NSSavePanel native dialogs |
| Configuration Safety | ✅ PASS | Validation before export, atomic writes, unsaved change warnings |
| Test Coverage | ✅ PASS | Integration tests for import/export round-trips planned |
| Performance Standards | ✅ PASS | File dialogs async, operations <100ms target maintained |
| Simple Architecture | ✅ PASS | Extends existing command pattern, no new abstractions |

---

## Next Steps (Phase 1)

1. **Data Model**: Document `ConfigFile.current_file_path` addition
2. **Contracts**: Define Tauri command signatures (import/export/reload)
3. **Quickstart**: Create developer guide for testing file dialogs
4. **Agent Context**: Update with rfd crate and file dialog patterns

---

**Research Complete**: All technical unknowns resolved. Ready for Phase 1 design.
