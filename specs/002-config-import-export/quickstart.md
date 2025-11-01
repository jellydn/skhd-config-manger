# Quickstart Guide: Configuration Import/Export

**Feature**: 002-config-import-export
**For**: Developers implementing this feature
**Date**: 2025-11-01

## Overview

This guide walks through implementing and testing the configuration import/export feature. Follow these steps sequentially to build the feature correctly.

## Prerequisites

- Feature 001 (skhd GUI core) completed and merged to main
- Branch `002-config-import-export` created
- Development environment running: `bun run tauri dev`
- Test suite passing: `cargo test` (40 tests from Feature 001)

## Implementation Checklist

### Phase 1: Backend - Add Dependencies

**File**: `src-tauri/Cargo.toml`

```toml
[dependencies]
# Existing dependencies...
rfd = "0.14"  # Native file dialogs (may already be in Tauri deps)
```

**Verify**:

```bash
cd src-tauri
cargo check  # Should compile without errors
```

---

### Phase 2: Backend - Extend Data Model

**File**: `src-tauri/src/models/config_file.rs`

**Add field to ConfigFile**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    // ... existing fields ...

    /// Tracks the currently active file path (where saves will write)
    #[serde(default = "ConfigFile::default_file_path")]
    pub current_file_path: String,
}

impl ConfigFile {
    pub fn default_file_path() -> String {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join(".config/skhd/skhdrc")
            .to_string_lossy()
            .to_string()
    }

    // MODIFY existing new() or from_parse() to set current_file_path
    pub fn from_parse(path: &str, /* ... */) -> Self {
        Self {
            file_path: path.to_string(),
            current_file_path: path.to_string(), // NEW: Initialize
            // ... rest of fields ...
        }
    }
}
```

**Test**:

```bash
cargo test models::config_file
```

---

### Phase 3: Backend - Implement Import Command

**File**: `src-tauri/src/commands/config.rs`

**Add import_config function**:

```rust
use rfd::AsyncFileDialog;

#[tauri::command]
pub async fn import_config(state: State<'_, ConfigState>) -> Result<ConfigFile, String> {
    // 1. Show file picker
    let file = AsyncFileDialog::new()
        .add_filter("skhd Configuration", &["skhdrc", "conf", "txt"])
        .add_filter("All Files", &["*"])
        .set_title("Import skhd Configuration")
        .set_directory(
            dirs::home_dir()
                .unwrap_or_default()
                .join(".config/skhd")
        )
        .pick_file()
        .await;

    // 2. Handle user cancellation
    let file = match file {
        Some(f) => f,
        None => return Err("Import cancelled".to_string()),
    };

    let path = file.path().to_path_buf();

    // 3. Load config from selected path (reuse existing load_config_internal)
    let config = load_config_from_path(&path.to_string_lossy(), state).await?;

    Ok(config)
}

// Helper: Extract existing load logic for reuse
async fn load_config_from_path(
    file_path: &str,
    state: State<'_, ConfigState>
) -> Result<ConfigFile, String> {
    // Move existing load_config implementation here
    // Parse file, validate, update state, return ConfigFile
    // ...
}
```

**Test**:

```bash
# Unit test with mock file
cargo test import_config
```

---

### Phase 4: Backend - Implement Export Command

**File**: `src-tauri/src/commands/config.rs`

```rust
#[tauri::command]
pub async fn export_config(state: State<'_, ConfigState>) -> Result<String, String> {
    // 1. Get current config
    let config = state.config.lock().await.clone();

    // 2. Serialize to skhd format (use existing serializer from save_config)
    let skhd_text = serialize_to_skhd(&config)?;

    // 3. Validate by re-parsing
    validate_skhd_syntax(&skhd_text)?;

    // 4. Show save dialog
    let file = AsyncFileDialog::new()
        .set_file_name("skhdrc")
        .set_title("Export Configuration")
        .add_filter("skhd Configuration", &["skhdrc", "conf"])
        .save_file()
        .await;

    let file = match file {
        Some(f) => f,
        None => return Err("Export cancelled".to_string()),
    };

    let export_path = file.path().to_path_buf();

    // 5. Write atomically (reuse existing atomic write logic)
    write_config_atomic(&export_path, &skhd_text)?;

    // 6. Return path for user confirmation
    Ok(export_path.to_string_lossy().to_string())
}

// Helper: Validation before export
fn validate_skhd_syntax(text: &str) -> Result<(), String> {
    // Use existing parser to validate
    match crate::parser::parse_skhd_config(text) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Configuration validation failed: {}", e)),
    }
}
```

**Test**:

```bash
cargo test export_config
```

---

### Phase 5: Backend - Implement Reload Command

**File**: `src-tauri/src/commands/config.rs`

```rust
#[tauri::command]
pub async fn reload_config(state: State<'_, ConfigState>) -> Result<ConfigFile, String> {
    // 1. Get current file path
    let current_path = {
        let config = state.config.lock().await;
        config.current_file_path.clone()
    };

    // 2. Re-load from that path (reuse load_config_from_path)
    let fresh_config = load_config_from_path(&current_path, state).await?;

    Ok(fresh_config)
}
```

**Note**: Frontend will handle unsaved changes warning.

**Test**:

```bash
cargo test reload_config
```

---

### Phase 6: Backend - Register Commands

**File**: `src-tauri/src/commands/mod.rs`

```rust
// Exports
pub use config::{
    load_config,
    save_config,
    reload_config,      // EXISTING
    import_config,      // NEW
    export_config,      // NEW
    reload_config,      // NEW (if not already exported)
    // ... other commands
};
```

**File**: `src-tauri/src/lib.rs`

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(commands::config::ConfigState::default())
        .invoke_handler(tauri::generate_handler![
            commands::config::load_config,
            commands::config::save_config,
            commands::config::reload_config,
            commands::config::import_config,    // NEW
            commands::config::export_config,    // NEW
            // ... other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Test**:

```bash
cargo build  # Should compile cleanly
cargo test   # All tests should pass
```

---

### Phase 7: Frontend - Update TypeScript Types

**File**: `src/types.ts`

```typescript
export interface ConfigFile {
  file_path: string;
  shortcuts: Shortcut[];
  global_comments: string[];
  last_modified: string;
  is_modified: boolean;
  backup_path?: string;
  parse_errors: ParseError[];
  current_file_path: string; // NEW
}
```

---

### Phase 8: Frontend - Add Service Methods

**File**: `src/services/tauri.ts`

```typescript
/**
 * Import configuration from custom file location
 * Opens native file picker dialog
 */
export async function importConfig(): Promise<ConfigFile> {
  return invoke<ConfigFile>('import_config');
}

/**
 * Export current configuration to file
 * Opens native save dialog
 */
export async function exportConfig(): Promise<string> {
  return invoke<string>('export_config');
}

/**
 * Reload configuration from current file path
 * Discards unsaved changes
 */
export async function reloadConfig(): Promise<ConfigFile> {
  return invoke<ConfigFile>('reload_config');
}
```

---

### Phase 9: Frontend - Create Confirmation Dialog

**File**: `src/components/ConfirmDialog.svelte`

```svelte
<script lang="ts">
  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmText = 'Confirm',
    cancelText = 'Cancel',
    onConfirm,
    onCancel,
  }: Props = $props();
</script>

{#if open}
  <div class="modal-backdrop" role="dialog" aria-modal="true">
    <div class="confirm-dialog">
      <h2>{title}</h2>
      <p>{message}</p>
      <div class="actions">
        <button class="btn-cancel" onclick={onCancel}>
          {cancelText}
        </button>
        <button class="btn-confirm" onclick={onConfirm}>
          {confirmText}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    /* Same styling as Modal.svelte */
  }
  .confirm-dialog {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    max-width: 400px;
  }
  .btn-confirm {
    background: #ff3b30; /* Warning color for destructive action */
  }
</style>
```

---

### Phase 10: Frontend - Add UI Controls

**File**: `src/routes/+page.svelte`

**Add state**:

```svelte
<script lang="ts">
  import {
    importConfig as importConfigAPI,
    exportConfig as exportConfigAPI,
    reloadConfig as reloadConfigAPI,
  } from '../services/tauri';
  import ConfirmDialog from '../components/ConfirmDialog.svelte';

  // Existing state...
  let showReloadConfirm = $state(false);

  async function handleImport() {
    try {
      config = await importConfigAPI();
      error = null;
    } catch (err) {
      if (err !== 'Import cancelled') {
        error = err instanceof Error ? err.message : String(err);
      }
    }
  }

  async function handleExport() {
    try {
      const exportedPath = await exportConfigAPI();
      console.log(`Exported to: ${exportedPath}`);
      // TODO: Show success toast
    } catch (err) {
      if (err !== 'Export cancelled') {
        error = err instanceof Error ? err.message : String(err);
      }
    }
  }

  async function handleReloadClick() {
    if (config && config.is_modified) {
      showReloadConfirm = true;
    } else {
      await performReload();
    }
  }

  async function performReload() {
    try {
      config = await reloadConfigAPI();
      showReloadConfirm = false;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  function cancelReload() {
    showReloadConfirm = false;
  }
</script>

<header class="app-header">
  <h1>skhd Configuration Manager</h1>
  <div class="header-actions">
    <!-- NEW BUTTONS -->
    <button class="btn-import" onclick={handleImport}> Import... </button>
    <button class="btn-export" onclick={handleExport}> Export... </button>
    <!-- EXISTING BUTTONS -->
    {#if config && config.shortcuts.length > 0}
      <button class="btn-create" onclick={handleCreate}> + New Shortcut </button>
    {/if}
    {#if config && config.is_modified}
      <button class="btn-save" onclick={saveConfiguration}> Save Changes </button>
    {/if}
    <!-- EXISTING RELOAD (now with confirmation) -->
    <button class="btn-reload" onclick={handleReloadClick} disabled={loading}>
      {loading ? 'Loading...' : 'Reload'}
    </button>
  </div>
</header>

<!-- Reload confirmation dialog -->
<ConfirmDialog
  open={showReloadConfirm}
  title="Discard Unsaved Changes?"
  message="This will reload the configuration from disk and lose all your current edits."
  confirmText="Reload"
  cancelText="Cancel"
  onConfirm={performReload}
  onCancel={cancelReload}
/>
```

**Add styles**:

```svelte
<style>
  .btn-import,
  .btn-export {
    background: #f5f5f7;
    color: #1d1d1f;
    border: 1px solid #d2d2d7;
  }

  .btn-import:hover,
  .btn-export:hover {
    background: #e8e8ed;
  }
</style>
```

---

## Testing Guide

### Manual Testing

**Test 1: Import Custom Config**

1. Start app: `bun run tauri dev`
2. Click "Import..." button
3. Select `test-skhdrc` file from project root
4. Verify: All 21 shortcuts display correctly
5. Verify: Header shows current file path

**Test 2: Export Configuration**

1. Make edit to any shortcut
2. Click "Export..." button
3. Save to Desktop as `test-export.skhdrc`
4. Verify: Success message shows file path
5. Open exported file in text editor
6. Verify: Contains all edits correctly

**Test 3: Reload with Unsaved Changes**

1. Edit a shortcut (don't save)
2. Click "Reload" button
3. Verify: Confirmation dialog appears
4. Click "Cancel" → Changes remain
5. Click "Reload" again → "Reload" in dialog
6. Verify: Changes discarded, config reloaded

**Test 4: Round-Trip Integrity**

1. Import `test-skhdrc`
2. Export to `/tmp/roundtrip.skhdrc`
3. Import `/tmp/roundtrip.skhdrc`
4. Verify: Identical to original (diff files)

### Automated Testing

**Run all tests**:

```bash
# Backend
cargo test

# Frontend
bun run test

# Type checking
bun run typecheck
```

---

## Common Issues

### Issue: File dialog doesn't appear

**Solution**: Check macOS permissions in System Settings > Privacy & Security > Files and Folders

### Issue: "Permission denied" on export

**Solution**: Choose a directory you own (Documents, Desktop, not /etc)

### Issue: Import shows parse errors

**Solution**: Verify skhd syntax is valid - this is expected behavior for malformed configs

### Issue: TypeScript errors on ConfigFile

**Solution**: Run `bun run typecheck` and ensure `current_file_path` added to types

---

## Performance Benchmarks

**Expected timings** (from constitution):

- Import operation: <200ms (dialog + parse)
- Export operation: <200ms (serialize + write)
- Reload operation: <150ms (parse + state update)

**Test with large config**:

```bash
# Generate 1000-line test config
for i in {1..1000}; do
  echo "ctrl + alt - $i : echo \"test $i\"" >> large-test.skhdrc
done

# Import and measure
# Should still be <200ms
```

---

## Definition of Done

- [ ] All 3 commands implemented (import, export, reload)
- [ ] Cargo tests pass (40+ tests)
- [ ] TypeScript checks pass (no errors)
- [ ] Manual testing checklist complete
- [ ] UI buttons styled consistently
- [ ] File dialogs use native macOS appearance
- [ ] Error messages are user-friendly
- [ ] Unsaved changes warning works correctly
- [ ] Round-trip import → export maintains data integrity
- [ ] Performance meets constitutional standards (<200ms)
- [ ] Code committed to `002-config-import-export` branch

---

**Quickstart Complete**: Ready for `/speckit.tasks` command to generate implementation tasks.
