# Quickstart Guide: skhd Configuration GUI

**Feature**: 001-skhd-config-gui
**Audience**: Developers implementing this feature
**Purpose**: Step-by-step guide to build and validate the application

## Prerequisites

Before starting implementation, ensure you have:

- **macOS**: 11 (Big Sur) or later (required for Tauri v2)
- **Rust**: 1.75 or later (`rustup update stable`)
- **Node.js**: 18+ (`node --version`)
- **Tauri CLI**: `cargo install tauri-cli`
- **skhd**: Installed and configured (`brew install koekeishiya/formulae/skhd`)

Optional but recommended:

- **rust-analyzer**: For IDE support
- **VS Code** with Rust and Svelte extensions

---

## Project Setup

### Step 1: Initialize Tauri Project

```bash
# Create new Tauri project with Svelte
npm create tauri-app@latest

# Choose:
# - Project name: skhd-gui
# - Frontend template: Svelte + TypeScript
# - Package manager: npm

cd skhd-gui
```

### Step 2: Configure Tauri for macOS

Edit `src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "skhd GUI",
    "version": "0.1.0"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.skhd.gui",
      "targets": ["dmg", "app"],
      "macOS": {
        "minimumSystemVersion": "11.0"
      }
    },
    "allowlist": {
      "fs": {
        "scope": ["$HOME/.config/skhd/*"]
      },
      "dialog": {
        "all": true
      }
    },
    "windows": []
  }
}
```

### Step 3: Add Rust Dependencies

Edit `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.0", features = ["macos-private-api"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pest = "2.7"
pest_derive = "2.7"
tempfile = "3.8"
sha2 = "0.10"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
notify = "6.1"  # For file watching

[dev-dependencies]
mockall = "0.12"
```

### Step 4: Add Frontend Dependencies

```bash
npm install
npm install --save-dev vitest @testing-library/svelte @testing-library/jest-dom
```

---

## Implementation Phases

### Phase 1: Backend - skhd Parser

**Goal**: Parse skhd config files into structured data

**Files to Create**:

- `src-tauri/src/parser/mod.rs`
- `src-tauri/src/parser/grammar.pest`
- `src-tauri/src/parser/ast.rs`

**Steps**:

1. Define pest grammar for skhd syntax in `grammar.pest`:

```pest
config_file = { SOI ~ (line ~ NEWLINE)* ~ EOI }
line = _{ comment | shortcut | WHITESPACE* }
comment = { "#" ~ (!NEWLINE ~ ANY)* }
shortcut = { modifiers ~ "-" ~ key ~ ":" ~ command }
modifiers = { modifier ~ ("+" ~ modifier)* }
modifier = { "cmd" | "alt" | "shift" | "ctrl" | "fn" }
key = @{ (ASCII_ALPHA | ASCII_DIGIT | "_")+ }
command = { (!NEWLINE ~ ANY)+ }
WHITESPACE = _{ " " | "\t" }
```

2. Implement parser in `mod.rs`:

```rust
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct SkhdParser;

pub fn parse_config(content: &str) -> Result<Vec<Shortcut>, ParseError> {
    // Implementation
}
```

3. Write unit tests in `src-tauri/tests/unit/parser_tests.rs`:

```rust
#[test]
fn test_parse_valid_shortcut() {
    let content = "cmd - return : open -a Terminal";
    let result = parse_config(content);
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_modifier() {
    let content = "command - f : open ~";
    let result = parse_config(content);
    assert!(result.is_err());
}
```

**Validation**: Run `cargo test --lib` - all parser tests should pass

---

### Phase 2: Backend - File Operations

**Goal**: Implement safe file I/O with atomic writes and backups

**Files to Create**:

- `src-tauri/src/services/file_io.rs`
- `src-tauri/src/services/backup.rs`

**Steps**:

1. Implement atomic write pattern:

```rust
use tempfile::NamedTempFile;
use std::fs;

pub fn save_config_atomic(path: &Path, content: &str) -> Result<(), Error> {
    let temp_file = NamedTempFile::new_in(path.parent().unwrap())?;
    temp_file.write_all(content.as_bytes())?;

    // Validate before committing
    parse_config(content)?;

    // Atomic rename
    temp_file.persist(path)?;
    Ok(())
}
```

2. Implement backup creation:

```rust
pub fn create_backup(config_path: &Path) -> Result<Backup, Error> {
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let backup_path = config_path.with_extension(format!("backup.{}", timestamp));

    fs::copy(config_path, &backup_path)?;

    // Calculate checksum
    let content = fs::read(&backup_path)?;
    let checksum = sha256::digest(&content);

    Ok(Backup {
        original_path: config_path.to_path_buf(),
        backup_path,
        created_at: chrono::Local::now(),
        checksum,
    })
}
```

3. Write integration tests in `src-tauri/tests/integration/config_lifecycle.rs`:

```rust
#[test]
fn test_save_creates_backup() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("skhdrc");
    fs::write(&config_path, "cmd - return : test").unwrap();

    let backup = create_backup(&config_path).unwrap();

    assert!(backup.backup_path.exists());
    assert!(backup.backup_path.to_str().unwrap().contains("backup"));
}
```

**Validation**: Run `cargo test` - all file operation tests should pass

---

### Phase 3: Backend - Tauri Commands

**Goal**: Expose Rust functions to frontend via Tauri commands

**Files to Create**:

- `src-tauri/src/commands/config.rs`
- `src-tauri/src/commands/mod.rs`

**Steps**:

1. Implement commands from contract specification:

```rust
#[tauri::command]
async fn load_config(custom_path: Option<String>) -> Result<ConfigFile, String> {
    let path = custom_path.unwrap_or_else(|| {
        shellexpand::tilde("~/.config/skhd/skhdrc").to_string()
    });

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("File not found: {}", e))?;

    let shortcuts = parse_config(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(ConfigFile {
        file_path: path,
        shortcuts,
        // ... other fields
    })
}

#[tauri::command]
async fn save_config(config: ConfigFile) -> Result<SaveResult, String> {
    // Validate
    validate_config(&config)?;

    // Backup
    let backup = create_backup(&config.file_path)
        .map_err(|e| format!("Backup failed: {}", e))?;

    // Serialize and save
    let content = serialize_config(&config);
    save_config_atomic(&config.file_path, &content)
        .map_err(|e| format!("Write failed: {}", e))?;

    Ok(SaveResult {
        success: true,
        backup_path: backup.backup_path.to_string_lossy().to_string(),
        new_last_modified: chrono::Local::now().to_rfc3339(),
    })
}
```

2. Register commands in `src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::validate_shortcut,
            commands::create_backup,
            // ... other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Validation**: Run `cargo tauri dev` - app should launch without errors

---

### Phase 4: Frontend - Tauri API Wrapper

**Goal**: Create type-safe wrappers for Tauri commands

**Files to Create**:

- `src/services/tauri.ts`
- `src/types.ts`

**Steps**:

1. Define TypeScript types matching Rust structures (`types.ts`):

```typescript
export interface Shortcut {
  id: string;
  modifiers: string[];
  key: string;
  command: string;
  mode: string | null;
  comment: string | null;
  line_number: number;
}

export interface ConfigFile {
  file_path: string;
  shortcuts: Shortcut[];
  global_comments: string[];
  last_modified: string;
  is_modified: boolean;
  backup_path: string | null;
  parse_errors: ParseError[];
}

// ... other types
```

2. Create Tauri command wrappers (`tauri.ts`):

```typescript
import { invoke } from '@tauri-apps/api/tauri';
import type { ConfigFile, SaveResult, Shortcut, ValidationResult } from './types';

export async function loadConfig(customPath: string | null = null): Promise<ConfigFile> {
  return invoke<ConfigFile>('load_config', { customPath });
}

export async function saveConfig(config: ConfigFile): Promise<SaveResult> {
  return invoke<SaveResult>('save_config', { config });
}

export async function validateShortcut(
  shortcut: Shortcut,
  existingShortcuts: Shortcut[],
  mode: string | null
): Promise<ValidationResult> {
  return invoke<ValidationResult>('validate_shortcut', {
    shortcut,
    existingShortcuts,
    mode,
  });
}

// ... other wrappers
```

**Validation**: TypeScript should compile without errors (`npm run check`)

---

### Phase 5: Frontend - UI Components

**Goal**: Build Svelte components for viewing and editing shortcuts

**Files to Create**:

- `src/components/ShortcutList.svelte`
- `src/components/ShortcutEditor.svelte`
- `src/components/ShortcutItem.svelte`
- `src/components/SearchBar.svelte`

**Steps**:

1. Create main App component (`App.svelte`):

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { loadConfig } from './services/tauri';
  import ShortcutList from './components/ShortcutList.svelte';
  import type { ConfigFile } from './types';

  let config: ConfigFile | null = null;
  let loading = true;
  let error: string | null = null;

  onMount(async () => {
    try {
      config = await loadConfig();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });
</script>

{#if loading}
  <div>Loading configuration...</div>
{:else if error}
  <div class="error">{error}</div>
{:else if config}
  <ShortcutList {config} />
{/if}
```

2. Create ShortcutList component:

```svelte
<script lang="ts">
  import type { ConfigFile } from '../types';
  import ShortcutItem from './ShortcutItem.svelte';

  export let config: ConfigFile;
</script>

<div class="shortcut-list">
  {#each config.shortcuts as shortcut (shortcut.id)}
    <ShortcutItem {shortcut} />
  {/each}
</div>

<style>
  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 16px;
  }
</style>
```

3. Create ShortcutEditor component (add/edit form)

**Validation**: Run `npm run dev` and visually inspect components

---

### Phase 6: Testing & Validation

**Goal**: Ensure all functionality works and meets success criteria

**Backend Tests**:

```bash
# Run all Rust tests
cargo test

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
# Should show >80% coverage for parser and file_io modules
```

**Frontend Tests**:

```bash
# Run Vitest tests
npm run test

# Run with coverage
npm run test:coverage
```

**Manual Testing Checklist**:

- [ ] App launches in <2 seconds
- [ ] Existing config file loads correctly
- [ ] Can add new shortcut
- [ ] Can edit existing shortcut
- [ ] Can delete shortcut
- [ ] Duplicate shortcuts are detected
- [ ] Save creates backup before writing
- [ ] Invalid configs show clear error messages
- [ ] Undo/redo works correctly
- [ ] Search/filter works
- [ ] Dark mode follows system preference
- [ ] VoiceOver navigation works (accessibility)

---

## Verification Against Success Criteria

After implementation, verify each success criterion:

- **SC-001**: Launch app and time from click to config display (<2s) ✅
- **SC-002**: Time full edit workflow (add shortcut → save) (<30s) ✅
- **SC-003**: Test with various skhd configs (100% parse success) ✅
- **SC-004**: Test file corruption scenarios (zero corruption) ✅
- **SC-005**: Try adding duplicate shortcuts (100% detection) ✅
- **SC-006**: Have new user try editing without docs (95% success) ✅
- **SC-007**: Measure launch time on macOS 11+ (<2s) ✅
- **SC-008**: Monitor memory usage during editing (<50MB) ✅
- **SC-009**: Verify backup created for every save ✅

---

## Distribution

### Build for Release

```bash
# Build optimized release
cargo tauri build --target universal-apple-darwin

# Output location
ls src-tauri/target/universal-apple-darwin/release/bundle/dmg/
```

### Code Signing (for distribution)

```bash
# Sign the app (requires Apple Developer account)
codesign --sign "Developer ID Application: Your Name" \
  --options runtime \
  --entitlements src-tauri/entitlements.plist \
  src-tauri/target/release/bundle/macos/skhd-gui.app

# Notarize for macOS Gatekeeper
xcrun notarytool submit skhd-gui.dmg \
  --apple-id "your-email@example.com" \
  --team-id "YOUR_TEAM_ID" \
  --wait
```

---

## Troubleshooting

**Issue**: Tauri commands not found

- **Fix**: Ensure commands are registered in `main.rs` with `generate_handler![]`

**Issue**: File permissions errors

- **Fix**: Check `tauri.conf.json` fs.scope includes `$HOME/.config/skhd/*`

**Issue**: Parser errors

- **Fix**: Validate pest grammar with `pest_debugger` tool

**Issue**: Build fails on M1 Mac

- **Fix**: Use `--target universal-apple-darwin` for Universal binary

---

## Next Steps

After completing quickstart:

1. Run `/speckit.tasks` to generate detailed implementation tasks
2. Implement tasks in priority order (P1 → P2 → P3)
3. Create feature branch per task for code reviews
4. Submit PRs incrementally (one user story at a time)

**Ready for Implementation!** 🚀
