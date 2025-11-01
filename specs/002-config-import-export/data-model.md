# Data Model: Configuration Import/Export

**Feature**: 002-config-import-export
**Phase**: 1 - Design & Contracts
**Date**: 2025-11-01

## Overview

This document defines the data model changes required for configuration import/export functionality. The design extends the existing `ConfigFile` model with minimal changes to maintain simplicity (Constitutional Principle V).

## Entity Changes

### 1. ConfigFile (MODIFIED)

**Purpose**: Represents the complete skhd configuration with file source tracking

**Location**: `src-tauri/src/models/config_file.rs`

**Existing Fields** (unchanged):
```rust
pub struct ConfigFile {
    /// Absolute path to config file (historical, may differ from current_file_path)
    pub file_path: String,

    /// List of keyboard shortcuts (ordered by line number)
    pub shortcuts: Vec<Shortcut>,

    /// Global comment lines not associated with shortcuts
    pub global_comments: Vec<String>,

    /// Last modification timestamp (ISO 8601)
    pub last_modified: String,

    /// Whether in-memory state differs from file
    pub is_modified: bool,

    /// Path to latest backup (if any)
    pub backup_path: Option<String>,

    /// Parse errors encountered (if any)
    pub parse_errors: Vec<ParseError>,
}
```

**New Field**:
```rust
/// Tracks the currently active file path (where saves will write)
/// Differs from file_path when user imports from custom location
/// Default: ~/.config/skhd/skhdrc
#[serde(default = "ConfigFile::default_file_path")]
pub current_file_path: String,
```

**Field Semantics**:
- `file_path`: Historical field, remains for backward compatibility, represents original load location
- `current_file_path`: Active file being edited, target for save operations
- On default load: `file_path == current_file_path`
- After import: `current_file_path` updated to imported path
- After export: `current_file_path` optionally updated if user wants to switch

**Default Value Function**:
```rust
impl ConfigFile {
    fn default_file_path() -> String {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join(".config/skhd/skhdrc")
            .to_string_lossy()
            .to_string()
    }
}
```

**Validation Rules**:
- `current_file_path` MUST be an absolute path
- `current_file_path` MUST be writable (checked before save operations)
- `current_file_path` MAY not exist (allows exporting to new locations)

**State Transitions**:
```
[App Start]
  ↓ load_config()
  → current_file_path = ~/.config/skhd/skhdrc

[Import Custom Config]
  ↓ import_config(/path/to/custom.skhdrc)
  → current_file_path = /path/to/custom.skhdrc
  → is_modified = false (just loaded)

[Edit Shortcut]
  → is_modified = true

[Save]
  ↓ save_config()
  → writes to current_file_path
  → is_modified = false

[Export]
  ↓ export_config() → /backup/config.skhdrc
  → current_file_path = /backup/config.skhdrc (user choice)
  OR
  → current_file_path unchanged (export-only mode)

[Reload]
  ↓ reload_config()
  → re-parse current_file_path
  → is_modified = false
```

**Backward Compatibility**:
- `#[serde(default)]` ensures old serialized configs deserialize correctly
- On first load of legacy config: set `current_file_path = file_path`

---

### 2. ConfigError (NEW)

**Purpose**: Structured error types for file operations

**Location**: `src-tauri/src/models/config_file.rs` or new `src-tauri/src/models/errors.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    /// User cancelled file dialog (not a real error)
    Cancelled,

    /// File access permission denied
    PermissionDenied { path: String },

    /// File not found at expected location
    FileNotFound { path: String },

    /// Configuration syntax/parsing error
    InvalidFormat { message: String, line: Option<usize> },

    /// Generic I/O error
    IOError { message: String },
}
```

**Display Implementation**:
```rust
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::Cancelled => {
                write!(f, "Operation cancelled by user")
            }
            ConfigError::PermissionDenied { path } => {
                write!(
                    f,
                    "Cannot access file at '{}': Permission denied. \
                     Check System Settings > Privacy & Security > Files and Folders.",
                    path
                )
            }
            ConfigError::FileNotFound { path } => {
                write!(
                    f,
                    "Configuration file not found: '{}'. \
                     The file may have been moved or deleted.",
                    path
                )
            }
            ConfigError::InvalidFormat { message, line } => {
                if let Some(line_num) = line {
                    write!(f, "Invalid skhd configuration (line {}): {}", line_num, message)
                } else {
                    write!(f, "Invalid skhd configuration: {}", message)
                }
            }
            ConfigError::IOError { message } => {
                write!(f, "File operation failed: {}", message)
            }
        }
    }
}
```

**Usage in Commands**:
```rust
#[tauri::command]
async fn import_config() -> Result<ConfigFile, String> {
    match internal_import() {
        Ok(config) => Ok(config),
        Err(ConfigError::Cancelled) => {
            Err("Import cancelled".to_string()) // Not shown as error to user
        }
        Err(e) => Err(e.to_string()) // User-friendly message
    }
}
```

---

### 3. FileDialogOptions (NEW - Internal Only)

**Purpose**: Configuration for file picker dialogs

**Location**: `src-tauri/src/commands/config.rs` (not exported to frontend)

```rust
struct FileDialogOptions {
    title: String,
    default_name: Option<String>,
    default_dir: Option<PathBuf>,
    filters: Vec<(&'static str, Vec<&'static str>)>,
}

impl FileDialogOptions {
    fn for_import() -> Self {
        Self {
            title: "Import skhd Configuration".to_string(),
            default_name: None,
            default_dir: Some(
                dirs::home_dir()
                    .unwrap_or_default()
                    .join(".config/skhd")
            ),
            filters: vec![
                ("skhd Configuration", vec!["skhdrc", "conf", "txt"]),
                ("All Files", vec!["*"]),
            ],
        }
    }

    fn for_export(current_name: &str) -> Self {
        Self {
            title: "Export Configuration".to_string(),
            default_name: Some(current_name.to_string()),
            default_dir: Some(dirs::home_dir().unwrap_or_default()),
            filters: vec![
                ("skhd Configuration", vec!["skhdrc", "conf"]),
            ],
        }
    }
}
```

---

## Data Flow Diagrams

### Import Flow
```
User Click "Import"
  ↓
Frontend: importConfig()
  ↓
Backend: import_config()
  ↓
rfd::AsyncFileDialog::pick_file()
  ↓ (User selects file)
Parse file with existing parser
  ↓
Validate syntax
  ↓ (Success)
Create ConfigFile with:
  - current_file_path = selected path
  - is_modified = false
  - shortcuts = parsed data
  ↓
Update ConfigState
  ↓
Return ConfigFile to frontend
  ↓
Frontend: Update UI with new shortcuts
```

### Export Flow
```
User Click "Export"
  ↓
Frontend: exportConfig()
  ↓
Backend: export_config()
  ↓
Get current ConfigFile from state
  ↓
Serialize to skhd text format
  ↓
Validate by re-parsing
  ↓ (Valid)
rfd::AsyncFileDialog::save_file()
  ↓ (User selects destination)
Write to file atomically:
  - Create temp file
  - Write content
  - Rename to target (atomic)
  ↓
Return exported path to frontend
  ↓
Frontend: Show success notification
Optional: Update current_file_path
```

### Reload Flow
```
User Click "Reload"
  ↓
Frontend: Check is_modified
  ↓ (If true)
Show ConfirmDialog
  ↓ (User confirms)
Frontend: reloadConfig()
  ↓
Backend: reload_config()
  ↓
Get current_file_path from state
  ↓
Re-parse file at current_file_path
  ↓
Replace ConfigState with fresh parse
  ↓
Set is_modified = false
  ↓
Return new ConfigFile
  ↓
Frontend: Update UI with reloaded data
```

---

## Persistence

**No Database**: This feature maintains the file-based storage model (Constitutional Principle V).

**File Operations**:
- **Import**: Read from user-selected path → Parse → Store in memory
- **Export**: Serialize from memory → Write to user-selected path
- **Reload**: Re-read from current_file_path → Replace memory state
- **Save**: Write to current_file_path (existing behavior)

**Atomic Write Pattern** (reused from Feature 001):
```rust
use tempfile::NamedTempFile;
use std::fs;

fn write_config_atomic(path: &Path, content: &str) -> Result<(), ConfigError> {
    // 1. Create temp file in same directory
    let temp = NamedTempFile::new_in(path.parent().unwrap())
        .map_err(|e| ConfigError::IOError { message: e.to_string() })?;

    // 2. Write content
    fs::write(temp.path(), content)
        .map_err(|e| ConfigError::IOError { message: e.to_string() })?;

    // 3. Atomic rename (replaces target if exists)
    temp.persist(path)
        .map_err(|e| ConfigError::IOError { message: e.to_string() })?;

    Ok(())
}
```

---

## Relationships

```
ConfigFile (1)
  ↓ contains
Shortcuts (0..n)
  ↓ references
ParseErrors (0..n)

ConfigFile
  ↓ tracks
current_file_path (1) [String]
  ↓ points to
File System [skhdrc file]
```

**No new relationships**: This feature only adds a tracking field to existing ConfigFile entity.

---

## Frontend Types (TypeScript)

**Update to `src/types.ts`**:

```typescript
export interface ConfigFile {
  file_path: string;          // EXISTING
  shortcuts: Shortcut[];      // EXISTING
  global_comments: string[];  // EXISTING
  last_modified: string;      // EXISTING
  is_modified: boolean;       // EXISTING
  backup_path?: string;       // EXISTING
  parse_errors: ParseError[]; // EXISTING

  current_file_path: string;  // NEW: Currently active file being edited
}
```

**No new frontend types needed**: Import/export operations return existing `ConfigFile` type.

---

## Validation Summary

| Field | Validation Rule | Enforcement |
|-------|----------------|-------------|
| `current_file_path` | Must be absolute path | Backend (on import/save) |
| `current_file_path` | Must be writable | Backend (on save attempt) |
| Exported content | Must be valid skhd syntax | Backend (before write) |
| Import content | Must parse without fatal errors | Backend (before accept) |

---

## Testing Data

**Test Fixtures** (`src-tauri/tests/fixtures/test-configs/`):

1. **valid-simple.skhdrc**: Minimal valid config for quick tests
2. **valid-complex.skhdrc**: Full-featured config with all skhd features
3. **invalid-syntax.skhdrc**: Intentional syntax errors for error handling tests
4. **unicode-names.skhdrc**: Config with emoji and unicode characters
5. **large-config.skhdrc**: 500+ shortcuts for performance testing

---

**Phase 1 Data Model Complete**: Ready for contract generation.
