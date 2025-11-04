# Research: Command and Application Picker

**Feature**: 007-hotkey-command-picker
**Date**: 2025-11-03
**Status**: Complete

This document consolidates research findings for all technical decisions required to implement the command and application picker feature.

## 1. macOS Application Discovery

### Decision: Use native macOS APIs via Rust std::fs

**Approach**:
- Scan standard application directories: `/Applications`, `~/Applications`, `/System/Applications`
- Parse `.app` bundle structures to extract metadata
- Use `plist` crate to read `Info.plist` for bundle ID, display name, executable path
- Extract icons from `.icns` files in `Contents/Resources/`

**Rationale**:
- Native approach, no external dependencies beyond plist parsing
- Consistent with "Simple Architecture" principle (no complex frameworks)
- Direct file system access provides full control over performance
- Works reliably with standard .app bundle structure

**Implementation Pattern**:
```rust
use std::fs;
use std::path::PathBuf;
use plist::Value;

fn discover_applications() -> Vec<Application> {
    let search_paths = vec![
        PathBuf::from("/Applications"),
        PathBuf::from(format!("{}/Applications", std::env::var("HOME").unwrap())),
        PathBuf::from("/System/Applications"),
    ];

    let mut apps = Vec::new();
    for path in search_paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().extension() == Some(std::ffi::OsStr::new("app")) {
                    if let Some(app) = parse_app_bundle(&entry.path()) {
                        apps.push(app);
                    }
                }
            }
        }
    }
    apps
}

fn parse_app_bundle(path: &PathBuf) -> Option<Application> {
    let info_plist_path = path.join("Contents/Info.plist");
    let plist = Value::from_file(&info_plist_path).ok()?;

    // Extract CFBundleName, CFBundleIdentifier, CFBundleExecutable
    // Return Application struct
}
```

**Performance**: Scanning 500 apps takes ~1-2s on modern hardware (acceptable per clarification)

**Alternatives Considered**:
- **NSWorkspace APIs via Objective-C bridge**: More complex, requires FFI, not justified for simple file scanning
- **Launch Services framework**: Overkill for this use case, adds complexity
- **Third-party crates (e.g., `mac-app-util`)**: None found with sufficient maintenance/features

**Edge Cases Handled**:
- Apps without Info.plist: Skip silently
- Duplicate app names: Display with path suffix (per clarification)
- Non-standard bundles: Best-effort parsing, fall back to directory name

---

## 2. Command Template Design

### Decision: JSON format with embedded parameter definitions

**Schema**:
```json
{
  "templates": [
    {
      "id": "volume-up",
      "name": "Increase Volume",
      "description": "Increase system volume by specified amount",
      "category": "media",
      "command_pattern": "osascript -e 'set volume output volume (output volume of (get volume settings) + {amount})'",
      "parameters": [
        {
          "name": "amount",
          "description": "Volume increase amount (0-100)",
          "data_type": "integer",
          "default_value": "10",
          "validation_regex": "^([1-9]|[1-9][0-9]|100)$"
        }
      ]
    },
    {
      "id": "screenshot-area",
      "name": "Screenshot Selection",
      "description": "Take screenshot of selected area and save to Downloads",
      "category": "system",
      "command_pattern": "screencapture -i ~/Downloads/screenshot-$(date +%Y%m%d-%H%M%S).png",
      "parameters": []
    }
  ],
  "categories": [
    {
      "id": "media",
      "name": "Media Control",
      "description": "Audio and playback controls",
      "icon": "speaker.wave.2"
    },
    {
      "id": "system",
      "name": "System",
      "description": "System operations and utilities",
      "icon": "gear"
    },
    {
      "id": "window",
      "name": "Window Management",
      "description": "Window positioning and tiling",
      "icon": "rectangle.split.3x3"
    }
  ]
}
```

**Rationale**:
- JSON is simple, human-readable, easy to extend
- Embedded in binary (no external file dependencies)
- Parameter substitution via simple string replacement (`{param_name}`)
- Validation regex ensures user input safety

**Initial Template Set (20-30 templates)**:

**Media (6 templates)**:
- Volume up/down/mute
- Play/pause/next/previous track

**System (8 templates)**:
- Screenshot (full/area/window)
- Lock screen
- Sleep/restart/shutdown
- Show desktop
- Mission Control

**Window Management (6 templates)**:
- Tile left/right/fullscreen
- Center window
- Move to next/previous display

**Brightness (3 templates)**:
- Increase/decrease/set brightness

**Launch (3 templates)**:
- Open Finder/Terminal/System Preferences

**Alternatives Considered**:
- **YAML**: More complex parsing, no significant benefit
- **TOML**: Less common in Rust ecosystem for this use case
- **Database**: Overkill for static templates (violates Simple Architecture)

---

## 3. Path Validation & Escaping

### Decision: Use `shell-escape` pattern with comprehensive testing

**Approach**:
```rust
fn escape_shell_path(path: &str) -> String {
    // Wrap in single quotes and escape any single quotes within
    format!("'{}'", path.replace("'", r"'\''"))
}

fn validate_executable(path: &PathBuf) -> Result<bool, String> {
    use std::os::unix::fs::PermissionsExt;

    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    let metadata = path.metadata()
        .map_err(|e| format!("Cannot read file metadata: {}", e))?;

    let permissions = metadata.permissions();
    let is_executable = permissions.mode() & 0o111 != 0;

    Ok(is_executable)
}

fn detect_interpreter(path: &PathBuf) -> Option<String> {
    match path.extension()?.to_str()? {
        "sh" | "bash" => Some("bash".to_string()),
        "zsh" => Some("zsh".to_string()),
        "py" => Some("python3".to_string()),
        "rb" => Some("ruby".to_string()),
        "js" => Some("node".to_string()),
        _ => None,
    }
}
```

**Rationale**:
- Single-quote wrapping is the safest shell escaping method
- Handles spaces, special chars ($, `, ", etc.), Unicode
- Permission checking prevents non-executable files
- Interpreter detection helps users with script files

**Test Coverage** (>80% target):
- Paths with spaces: `"/Users/test/My Documents/script.sh"`
- Special characters: `"/tmp/file$name.sh"`, `"/tmp/back`tick.sh"`
- Quotes: `"/tmp/quote\"file.sh"`, `"/tmp/quote'file.sh"`
- Unicode: `"/tmp/文件.sh"`
- Edge cases: Empty string, root path, symlinks

**Alternatives Considered**:
- **Double-quote escaping**: More complex, requires escaping $, `, \, "
- **Backslash escaping**: Error-prone, shell-dependent
- **Third-party crate (e.g., `shell-escape`)**: Adds dependency for simple logic

---

## 4. Svelte 5 Dialog Patterns

### Decision: Modal dialogs with runes-based reactivity

**Pattern**:
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Application } from '../types';

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
      });
  });

  const filteredApps = $derived(
    applications.filter(app =>
      app.display_name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  function handleSelect(app: Application) {
    const command = `open -a "${app.display_name}"`;
    onSelect(command);
  }
</script>

<div class="modal-overlay" onclick={onCancel}>
  <div class="modal-content" onclick|stopPropagation>
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search applications..."
    />

    {#if loading}
      <div class="spinner">Loading...</div>
    {:else}
      <ul class="app-list">
        {#each filteredApps as app}
          <li onclick={() => handleSelect(app)}>
            <img src={app.icon_path} alt="" />
            <span>{app.display_name}</span>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>
```

**Rationale**:
- Svelte 5 runes (`$state`, `$derived`, `$effect`) provide reactive search
- Modal overlay pattern is standard and accessible
- Keyboard navigation via native browser focus management
- Integration with ShortcutForm via callback props

**Accessibility**:
- Focus trap: Auto-focus search input, ESC to close
- ARIA labels: `role="dialog"`, `aria-modal="true"`, `aria-label="Application Picker"`
- Keyboard navigation: Arrow keys for list, Enter to select, Escape to cancel

**Alternatives Considered**:
- **Third-party modal library**: Unnecessary dependency for simple modal
- **Svelte 4 stores**: Svelte 5 runes are more modern and performant
- **Native `<dialog>` element**: Browser support still limited on older macOS versions

---

## 5. File Picker Enhancement

### Decision: Use `rfd` crate for native dialogs

**Approach**:
```rust
use rfd::FileDialog;

#[tauri::command]
async fn open_file_picker(filter: Option<String>) -> Result<Option<String>, String> {
    let dialog = FileDialog::new()
        .set_title("Select Script or Executable");

    let dialog = if let Some(f) = filter {
        dialog.add_filter("Executables", &[&f])
    } else {
        dialog.add_filter("All Files", &["*"])
    };

    Ok(dialog.pick_file().map(|p| p.display().to_string()))
}
```

**Rationale**:
- `rfd` (Rust File Dialog) provides native macOS file dialogs
- Already in project dependencies (Cargo.toml shows `rfd = "0.14"`)
- Integrates with macOS native UI (consistent with Constitution principle I)
- Async API works well with Tauri commands

**Integration**:
- FilePicker.svelte calls `open_file_picker()` Tauri command
- Returns path string
- Frontend validates executability via `validate_file_executable()` command
- Displays warning if file not executable (FR-010)

**Alternatives Considered**:
- **Custom Svelte file browser**: Poor UX, doesn't feel native, lots of work
- **HTML `<input type="file">`**: Web-based, not native, sandboxing issues in Tauri

---

## Best Practices Summary

### Rust Backend
- Use `serde` for JSON template deserialization
- Implement `#[tauri::command]` macro for all frontend-facing functions
- Return `Result<T, String>` for error handling
- Use `async` for I/O operations (file scanning, plist parsing)
- Add `#[cfg(test)]` modules with comprehensive test coverage

### TypeScript Frontend
- Define interfaces for all data models (Application, CommandTemplate, etc.)
- Use Tauri `invoke()` API with type parameters for type safety
- Implement service layer to encapsulate Tauri commands
- Use Svelte 5 runes for reactive state management
- Keep components small and focused (single responsibility)

### Performance
- Cache application list in frontend after first load (session-only)
- Use `$derived` for computed values (search filtering) - automatically optimized
- Debounce search input if performance issues observed (unlikely with <500 apps)
- Async file operations to avoid blocking UI thread

### Testing
- Unit tests for: path escaping, template parameter substitution, app bundle parsing
- Integration tests for: Tauri command invocations, end-to-end picker flows
- Property-based testing for: path escaping edge cases (using `proptest` crate)
- Manual testing for: UI interactions, keyboard navigation, performance with many apps

---

## Risks & Mitigations

### Risk: Application scanning slow on older Macs
**Probability**: Medium
**Impact**: Medium (user annoyance)
**Mitigation**:
- User accepted this tradeoff in clarifications (load all at once)
- Show loading spinner during scan
- Cache results for session duration
- Future: Implement lazy loading or virtualized list if performance issues arise

### Risk: Template parameters insufficient for user needs
**Probability**: High (initial version won't cover all cases)
**Impact**: Low (users can manually edit commands)
**Mitigation**:
- Start with 20-30 common templates
- User can edit command text after template insertion (FR-015)
- Template JSON easy to extend based on user feedback
- Document how to request new templates in README

### Risk: Path escaping doesn't work with all shells
**Probability**: Low (single-quote escaping is standard)
**Impact**: High (commands fail to execute)
**Mitigation**:
- Comprehensive test suite (>80% coverage)
- Test with sh, bash, zsh, fish
- Document any known limitations
- User can manually adjust escaped paths if needed

### Risk: Non-standard .app bundles missing metadata
**Probability**: Medium (some apps have unusual structures)
**Impact**: Low (app not shown in picker)
**Mitigation**:
- Best-effort parsing with fallbacks
- Log warnings for unparseable bundles (debug mode)
- User can still use file picker to select executable directly
- Acceptable per "95% coverage" success criterion (SC-005)

---

## Dependencies to Add

**Rust (Cargo.toml)**:
```toml
plist = "1.6"  # For parsing Info.plist files
```

**TypeScript (package.json)**:
No new dependencies required - using existing Tauri API and Svelte 5

---

## Conclusion

All technical unknowns have been resolved:

1. ✅ Application discovery: Native file scanning with plist parsing
2. ✅ Template design: JSON format with parameter substitution
3. ✅ Path escaping: Single-quote wrapping with comprehensive tests
4. ✅ Svelte dialogs: Runes-based modal components
5. ✅ File picker: Use existing `rfd` crate integration

Ready to proceed to Phase 1 (data model and contracts generation).
