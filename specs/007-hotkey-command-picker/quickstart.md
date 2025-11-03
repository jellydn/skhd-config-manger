# Quickstart Guide: Command and Application Picker

**Feature**: 007-hotkey-command-picker
**Date**: 2025-11-03
**Audience**: Developers implementing this feature

This guide helps developers get started with implementing and testing the command and application picker feature.

## Development Setup

### Prerequisites

- Existing project setup (Tauri v2, Rust 1.75+, Node/Bun, macOS 11+)
- Feature branch checked out: `007-hotkey-command-picker`
- Dependencies from research: `plist = "1.6"` added to Cargo.toml

### Add Required Dependency

```bash
cd src-tauri
echo 'plist = "1.6"' >> Cargo.toml
cargo build  # Verify dependency resolves
```

---

## Project Structure Overview

### Files to Create

**Backend (Rust)**:
```
src-tauri/src/
├── commands/
│   ├── applications.rs      # NEW
│   ├── templates.rs          # NEW
│   └── file_picker.rs        # NEW
├── models/
│   ├── application.rs        # NEW
│   ├── command_template.rs   # NEW
│   └── command_category.rs   # NEW
├── services/
│   ├── app_discovery.rs      # NEW
│   ├── template_loader.rs    # NEW
│   └── path_validator.rs     # NEW
└── data/
    └── command_templates.json # NEW
```

**Frontend (TypeScript/Svelte)**:
```
src/
├── components/pickers/
│   ├── ApplicationPicker.svelte   # NEW
│   ├── CommandPicker.svelte       # NEW
│   ├── FilePicker.svelte          # NEW
│   └── TemplateParameterForm.svelte # NEW
├── services/
│   ├── applicationService.ts  # NEW
│   ├── commandService.ts      # NEW
│   └── templateService.ts     # NEW
└── types.ts                   # MODIFY (add new interfaces)
```

### Files to Modify

- `src-tauri/src/lib.rs` - Register new command modules
- `src-tauri/src/main.rs` - Register Tauri commands
- `src/components/ShortcutForm.svelte` - Add picker buttons

---

## Quick Implementation Steps

### Step 1: Set Up Data Models (30 min)

**Create models/application.rs**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub display_name: String,
    pub app_path: String,
    pub bundle_id: String,
    pub executable_path: String,
    pub icon_path: Option<String>,
    pub version: Option<String>,
}
```

**Add to lib.rs**:
```rust
pub mod models {
    pub mod application;
    pub mod command_template;
    pub mod command_category;
}
```

Repeat for `command_template.rs` and `command_category.rs` (see [data-model.md](./data-model.md)).

---

### Step 2: Implement Application Discovery (2 hours)

**Create services/app_discovery.rs**:
```rust
use std::fs;
use std::path::PathBuf;
use plist::Value;
use crate::models::application::Application;

pub fn discover_applications() -> Result<Vec<Application>, String> {
    let search_paths = vec![
        PathBuf::from("/Applications"),
        PathBuf::from(format!("{}/Applications", std::env::var("HOME")
            .map_err(|_| "Cannot determine home directory")?)),
        PathBuf::from("/System/Applications"),
    ];

    let mut apps = Vec::new();

    for path in search_paths {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.extension() == Some(std::ffi::OsStr::new("app")) {
                    if let Ok(app) = parse_app_bundle(&entry_path) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    // Remove duplicates by bundle_id + app_path
    apps.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    apps.dedup_by(|a, b| a.bundle_id == b.bundle_id && a.app_path == b.app_path);

    Ok(apps)
}

fn parse_app_bundle(app_path: &PathBuf) -> Result<Application, String> {
    let info_plist_path = app_path.join("Contents/Info.plist");
    let plist: Value = Value::from_file(&info_plist_path)
        .map_err(|e| format!("Failed to parse Info.plist: {}", e))?;

    let dict = plist.as_dictionary()
        .ok_or("Info.plist is not a dictionary")?;

    let bundle_name = dict.get("CFBundleName")
        .or_else(|| dict.get("CFBundleDisplayName"))
        .and_then(|v| v.as_string())
        .ok_or("Missing CFBundleName")?;

    let bundle_id = dict.get("CFBundleIdentifier")
        .and_then(|v| v.as_string())
        .ok_or("Missing CFBundleIdentifier")?;

    let executable = dict.get("CFBundleExecutable")
        .and_then(|v| v.as_string())
        .ok_or("Missing CFBundleExecutable")?;

    let executable_path = app_path
        .join("Contents/MacOS")
        .join(executable)
        .display()
        .to_string();

    let icon_file = dict.get("CFBundleIconFile")
        .and_then(|v| v.as_string());

    let icon_path = icon_file.map(|icon| {
        let mut path = app_path.join("Contents/Resources").join(icon);
        if path.extension().is_none() {
            path.set_extension("icns");
        }
        path.display().to_string()
    });

    let version = dict.get("CFBundleShortVersionString")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());

    Ok(Application {
        display_name: bundle_name.to_string(),
        app_path: app_path.display().to_string(),
        bundle_id: bundle_id.to_string(),
        executable_path,
        icon_path,
        version,
    })
}
```

**Create commands/applications.rs**:
```rust
use crate::services::app_discovery;
use crate::models::application::Application;

#[tauri::command]
pub async fn get_installed_applications() -> Result<Vec<Application>, String> {
    app_discovery::discover_applications()
}
```

---

### Step 3: Create Command Templates JSON (1 hour)

**Create src-tauri/src/data/command_templates.json**:
```json
{
  "categories": [
    {
      "id": "media",
      "name": "Media Control",
      "description": "Audio playback and volume controls",
      "icon": "speaker.wave.2",
      "display_order": 1
    },
    {
      "id": "system",
      "name": "System",
      "description": "System operations and utilities",
      "icon": "gear",
      "display_order": 2
    }
  ],
  "templates": [
    {
      "id": "volume-up",
      "name": "Increase Volume",
      "description": "Increase system volume by specified amount",
      "category_id": "media",
      "command_pattern": "osascript -e 'set volume output volume (output volume of (get volume settings) + {amount})'",
      "parameters": [
        {
          "name": "amount",
          "description": "Volume increase amount (1-100)",
          "data_type": "integer",
          "default_value": "10",
          "validation_regex": "^([1-9]|[1-9][0-9]|100)$",
          "min_value": 1,
          "max_value": 100
        }
      ],
      "requires_admin": false
    },
    {
      "id": "screenshot-area",
      "name": "Screenshot Selection",
      "description": "Take screenshot of selected area",
      "category_id": "system",
      "command_pattern": "screencapture -i ~/Downloads/screenshot-$(date +%Y%m%d-%H%M%S).png",
      "parameters": [],
      "requires_admin": false
    }
  ]
}
```

Start with 2-3 templates, expand to 20-30 iteratively.

---

### Step 4: Implement Template Loading (1 hour)

**Create services/template_loader.rs**:
```rust
use serde::Deserialize;
use crate::models::{command_template::CommandTemplate, command_category::CommandCategory};

#[derive(Deserialize)]
struct TemplateData {
    categories: Vec<CommandCategory>,
    templates: Vec<CommandTemplate>,
}

pub fn load_templates() -> Result<TemplateData, String> {
    let json_data = include_str!("../data/command_templates.json");
    serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse templates: {}", e))
}

pub fn get_templates(category_id: Option<String>) -> Result<Vec<CommandTemplate>, String> {
    let data = load_templates()?;
    if let Some(cat_id) = category_id {
        Ok(data.templates.into_iter()
            .filter(|t| t.category_id == cat_id)
            .collect())
    } else {
        Ok(data.templates)
    }
}

pub fn get_categories() -> Result<Vec<CommandCategory>, String> {
    let data = load_templates()?;
    let mut categories = data.categories;
    categories.sort_by_key(|c| c.display_order);
    Ok(categories)
}
```

---

### Step 5: Implement Frontend Components (3 hours)

**Create components/pickers/ApplicationPicker.svelte**:
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Application } from '../../types';

  let { onSelect, onCancel } = $props<{
    onSelect: (command: string) => void;
    onCancel: () => void;
  }>();

  let applications = $state<Application[]>([]);
  let searchQuery = $state('');
  let loading = $state(true);

  $effect(() => {
    invoke<Application[]>('get_installed_applications')
      .then(apps => {
        applications = apps;
        loading = false;
      })
      .catch(err => {
        console.error('Failed to load applications:', err);
        loading = false;
      });
  });

  const filteredApps = $derived(
    searchQuery === ''
      ? applications
      : applications.filter(app =>
          app.display_name.toLowerCase().includes(searchQuery.toLowerCase())
        )
  );

  function handleSelect(app: Application) {
    const command = `open -a "${app.display_name}"`;
    onSelect(command);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onCancel();
    }
  }
</script>

<div class="modal-overlay" onclick={onCancel} onkeydown={handleKeydown}>
  <div class="modal-content" onclick|stopPropagation role="dialog" aria-modal="true" aria-label="Application Picker">
    <h2>Select Application</h2>

    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search applications..."
      class="search-input"
      autofocus
    />

    {#if loading}
      <div class="loading">Loading applications...</div>
    {:else if filteredApps.length === 0}
      <div class="empty">No results found</div>
    {:else}
      <ul class="app-list">
        {#each filteredApps as app (app.bundle_id + app.app_path)}
          <li onclick={() => handleSelect(app)} class="app-item">
            {#if app.icon_path}
              <img src={app.icon_path} alt="" class="app-icon" />
            {/if}
            <span class="app-name">{app.display_name}</span>
            {#if app.version}
              <span class="app-version">{app.version}</span>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}

    <button onclick={onCancel} class="cancel-btn">Cancel</button>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    border-radius: 8px;
    padding: 20px;
    width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .search-input {
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 14px;
  }

  .app-list {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
  }

  .app-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
    cursor: pointer;
    border-radius: 4px;
  }

  .app-item:hover {
    background: #f0f0f0;
  }

  .app-icon {
    width: 32px;
    height: 32px;
  }

  .app-name {
    flex: 1;
    font-weight: 500;
  }

  .app-version {
    color: #666;
    font-size: 12px;
  }

  .cancel-btn {
    padding: 8px 16px;
    background: #f0f0f0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .loading, .empty {
    text-align: center;
    padding: 40px;
    color: #666;
  }
</style>
```

Repeat similar pattern for `CommandPicker.svelte` and `FilePicker.svelte`.

---

### Step 6: Integrate with ShortcutForm (1 hour)

**Modify components/ShortcutForm.svelte**:
```svelte
<script lang="ts">
  import ApplicationPicker from './pickers/ApplicationPicker.svelte';
  import CommandPicker from './pickers/CommandPicker.svelte';
  import FilePicker from './pickers/FilePicker.svelte';

  let showAppPicker = $state(false);
  let showCmdPicker = $state(false);
  let showFilePicker = $state(false);

  let commandText = $state('');

  function handleCommandInsert(command: string) {
    commandText = command;
    closeAllPickers();
  }

  function closeAllPickers() {
    showAppPicker = false;
    showCmdPicker = false;
    showFilePicker = false;
  }
</script>

<!-- Existing ShortcutForm fields -->

<div class="command-input-group">
  <label for="command">Command</label>
  <textarea id="command" bind:value={commandText}></textarea>

  <div class="picker-buttons">
    <button onclick={() => showAppPicker = true}>Browse Applications</button>
    <button onclick={() => showCmdPicker = true}>Browse Commands</button>
    <button onclick={() => showFilePicker = true}>Browse Files</button>
  </div>
</div>

{#if showAppPicker}
  <ApplicationPicker
    onSelect={handleCommandInsert}
    onCancel={closeAllPickers}
  />
{/if}

{#if showCmdPicker}
  <CommandPicker
    onSelect={handleCommandInsert}
    onCancel={closeAllPickers}
  />
{/if}

{#if showFilePicker}
  <FilePicker
    onSelect={handleCommandInsert}
    onCancel={closeAllPickers}
  />
{/if}
```

---

## Testing Locally

### Test Application Discovery

```bash
cd src-tauri
cargo test app_discovery
```

Create test in `src-tauri/src/services/app_discovery.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_applications() {
        let apps = discover_applications().unwrap();
        assert!(apps.len() > 0, "Should find at least one application");
        println!("Found {} applications", apps.len());
        for app in apps.iter().take(5) {
            println!("- {} ({})", app.display_name, app.bundle_id);
        }
    }

    #[test]
    fn test_parse_safari_bundle() {
        use std::path::PathBuf;
        let safari_path = PathBuf::from("/Applications/Safari.app");
        if safari_path.exists() {
            let app = parse_app_bundle(&safari_path).unwrap();
            assert_eq!(app.display_name, "Safari");
            assert_eq!(app.bundle_id, "com.apple.Safari");
        }
    }
}
```

### Test Template Loading

```bash
cargo test template_loader
```

### Test Path Escaping

```bash
cargo test escape_shell_path
```

Create test in `src-tauri/src/services/path_validator.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_shell_path() {
        assert_eq!(escape_shell_path("/tmp/normal.sh"), "'/tmp/normal.sh'");
        assert_eq!(escape_shell_path("/tmp/with spaces.sh"), "'/tmp/with spaces.sh'");
        assert_eq!(escape_shell_path("/tmp/quote's.sh"), r"'/tmp/quote'\''s.sh'");
        assert_eq!(escape_shell_path("/tmp/$special`chars\".sh"), r"'/tmp/$special`chars".sh'");
    }
}
```

### Run Frontend in Dev Mode

```bash
bun run dev
```

Open app, click "Add Shortcut", test picker buttons.

---

## Adding New Command Templates

### Step 1: Edit command_templates.json

```json
{
  "id": "new-template-id",
  "name": "New Template Name",
  "description": "What this template does",
  "category_id": "existing-category-id",
  "command_pattern": "command with {param} placeholder",
  "parameters": [
    {
      "name": "param",
      "description": "Parameter description",
      "data_type": "integer",
      "default_value": "10",
      "min_value": 1,
      "max_value": 100
    }
  ],
  "requires_admin": false
}
```

### Step 2: Rebuild

```bash
cd src-tauri
cargo build  # Recompiles with new embedded JSON
```

### Step 3: Test

Open CommandPicker in dev mode, verify new template appears.

---

## Common Issues & Solutions

### Issue: Application list is empty

**Cause**: Permission denied on /Applications
**Solution**: Grant Full Disk Access to the app in System Preferences → Security & Privacy

### Issue: Template parameters not substituting

**Cause**: Parameter name mismatch between `parameters[]` and `{placeholder}`
**Solution**: Ensure parameter.name matches exactly (case-sensitive)

### Issue: Path escaping not working

**Cause**: Using wrong escaping method
**Solution**: Always use single-quote wrapping, not backslash escaping

### Issue: Icons not displaying

**Cause**: Icon path wrong or missing
**Solution**: Fallback to default icon if icon_path is None

---

## Performance Tips

### Cache Application List

```typescript
// In ApplicationPicker component
let cachedApplications: Application[] | null = null;

$effect(() => {
  if (!cachedApplications) {
    invoke<Application[]>('get_installed_applications')
      .then(apps => {
        cachedApplications = apps;
        applications = apps;
        loading = false;
      });
  } else {
    applications = cachedApplications;
    loading = false;
  }
});
```

### Debounce Search

```typescript
import { debounce } from '../utils/debounce';

const debouncedSearch = debounce((query: string) => {
  searchQuery = query;
}, 300);
```

---

## Next Steps

After implementing basic functionality:

1. **Expand Templates**: Add 20-30 templates across all categories
2. **Improve UI**: Add keyboard shortcuts, better styling
3. **Add Tests**: Comprehensive test coverage (>80% target)
4. **Handle Edge Cases**: Non-standard bundles, duplicate apps, etc.
5. **Performance**: Optimize for 500+ applications if needed

For detailed task breakdown, see [tasks.md](./tasks.md) (generated by `/speckit.tasks`).

---

## Resources

- **Tauri Commands**: [contracts/tauri-commands.md](./contracts/tauri-commands.md)
- **Data Models**: [data-model.md](./data-model.md)
- **Research**: [research.md](./research.md)
- **Tauri Docs**: https://tauri.app/v2/
- **Svelte 5 Docs**: https://svelte.dev/docs/svelte/overview
- **plist crate**: https://docs.rs/plist/latest/plist/
