# Tauri Commands Contract

**Feature**: 007-hotkey-command-picker
**Date**: 2025-11-03
**Status**: Complete

This document defines all Tauri commands (backend API) for the command and application picker feature.

## Overview

Tauri commands provide the bridge between the Rust backend and TypeScript frontend. All commands are async and use the `#[tauri::command]` macro for automatic serialization.

**General Patterns**:
- All commands return `Result<T, String>` for error handling
- Success: `Ok(value)` → frontend receives value
- Error: `Err(message)` → frontend receives error string
- Async operations use `tokio` runtime
- All responses are JSON-serialized via `serde`

---

## Command Definitions

### 1. get_installed_applications

**Purpose**: Discover and return all installed macOS applications from standard directories.

**Signature**:
```rust
#[tauri::command]
async fn get_installed_applications() -> Result<Vec<Application>, String>
```

**Parameters**: None

**Returns**: `Vec<Application>` - List of discovered applications

**Errors**:
- "Failed to scan application directories: {error}" - File system access error
- "No applications found" - Empty result (unlikely but possible)

**Behavior**:
1. Scan `/Applications`, `~/Applications`, `/System/Applications`
2. For each `.app` bundle, parse `Contents/Info.plist`
3. Extract: CFBundleName, CFBundleIdentifier, CFBundleExecutable, CFBundleIconFile
4. Construct `Application` struct
5. Skip bundles with missing or invalid Info.plist
6. Return deduplicated list (by bundle_id + app_path)

**Performance**: ~1-2 seconds for 500 applications (acceptable per clarification)

**Frontend Usage**:
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Application } from '../types';

const applications = await invoke<Application[]>('get_installed_applications');
```

**Example Response**:
```json
[
  {
    "display_name": "Safari",
    "app_path": "/Applications/Safari.app",
    "bundle_id": "com.apple.Safari",
    "executable_path": "/Applications/Safari.app/Contents/MacOS/Safari",
    "icon_path": "/Applications/Safari.app/Contents/Resources/AppIcon.icns",
    "version": "16.3"
  },
  {
    "display_name": "Visual Studio Code",
    "app_path": "/Applications/Visual Studio Code.app",
    "bundle_id": "com.microsoft.VSCode",
    "executable_path": "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
    "icon_path": "/Applications/Visual Studio Code.app/Contents/Resources/Code.icns",
    "version": "1.85.0"
  }
]
```

---

### 2. get_command_templates

**Purpose**: Load command templates, optionally filtered by category.

**Signature**:
```rust
#[tauri::command]
async fn get_command_templates(category_id: Option<String>) -> Result<Vec<CommandTemplate>, String>
```

**Parameters**:
- `category_id` (optional): Filter templates by this category ID. If `None`, return all templates.

**Returns**: `Vec<CommandTemplate>` - List of command templates

**Errors**:
- "Failed to load command templates: {error}" - JSON parsing error
- "Invalid category ID: {id}" - Category doesn't exist (if filtering)

**Behavior**:
1. Load embedded `command_templates.json` via `include_str!`
2. Parse JSON into `Vec<CommandTemplate>`
3. If `category_id` provided, filter templates where `template.category_id == category_id`
4. Return filtered or full list

**Performance**: <10ms (embedded JSON, small dataset)

**Frontend Usage**:
```typescript
// Get all templates
const allTemplates = await invoke<CommandTemplate[]>('get_command_templates');

// Get media templates only
const mediaTemplates = await invoke<CommandTemplate[]>('get_command_templates', {
  categoryId: 'media'
});
```

**Example Response**:
```json
[
  {
    "id": "volume-up",
    "name": "Increase Volume",
    "description": "Increase system volume by specified amount (0-100)",
    "category_id": "media",
    "command_pattern": "osascript -e 'set volume output volume (output volume of (get volume settings) + {amount})'",
    "parameters": [
      {
        "name": "amount",
        "description": "Volume increase amount",
        "data_type": "integer",
        "default_value": "10",
        "validation_regex": "^([1-9]|[1-9][0-9]|100)$",
        "min_value": 1,
        "max_value": 100
      }
    ],
    "requires_admin": false
  }
]
```

---

### 3. get_command_categories

**Purpose**: Load all command categories for organizing templates.

**Signature**:
```rust
#[tauri::command]
async fn get_command_categories() -> Result<Vec<CommandCategory>, String>
```

**Parameters**: None

**Returns**: `Vec<CommandCategory>` - List of categories, sorted by `display_order`

**Errors**:
- "Failed to load command categories: {error}" - JSON parsing error

**Behavior**:
1. Load embedded `command_templates.json`
2. Parse `categories` array
3. Sort by `display_order` ascending
4. Return sorted list

**Performance**: <10ms (embedded JSON, small dataset)

**Frontend Usage**:
```typescript
const categories = await invoke<CommandCategory[]>('get_command_categories');
```

**Example Response**:
```json
[
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
  },
  {
    "id": "window",
    "name": "Window Management",
    "description": "Window positioning and tiling",
    "icon": "rectangle.split.3x3",
    "display_order": 3
  }
]
```

---

### 4. generate_command_from_template

**Purpose**: Substitute parameters into a command template to generate executable command string.

**Signature**:
```rust
#[tauri::command]
async fn generate_command_from_template(
    template_id: String,
    params: std::collections::HashMap<String, String>
) -> Result<String, String>
```

**Parameters**:
- `template_id`: ID of the template to use
- `params`: Map of parameter names to user-provided values

**Returns**: `String` - Generated shell command with parameters substituted

**Errors**:
- "Template not found: {id}" - Invalid template_id
- "Missing required parameter: {name}" - Parameter not provided
- "Invalid parameter value for '{name}': {reason}" - Validation failed
- "Parameter '{name}' does not exist in template" - Extra parameter provided

**Behavior**:
1. Load template by ID
2. Validate all required parameters are provided
3. Validate each parameter value against rules (regex, min/max, enum)
4. Replace `{param}` placeholders in `command_pattern` with validated values
5. Return final command string

**Validation Rules** (per parameter):
- Integer: Must parse as i32, respect min/max if present
- Float: Must parse as f64, respect min/max if present
- Boolean: Must be "true" or "false"
- Enum: Must be in `enum_values` list
- String: Must match `validation_regex` if present

**Performance**: <5ms (simple string substitution)

**Frontend Usage**:
```typescript
const command = await invoke<string>('generate_command_from_template', {
  templateId: 'volume-up',
  params: { amount: '25' }
});
// Returns: "osascript -e 'set volume output volume (output volume of (get volume settings) + 25)'"
```

**Example Request/Response**:
```typescript
// Request
{
  template_id: "screenshot-window",
  params: {
    "save_location": "~/Desktop",
    "format": "png"
  }
}

// Response (success)
"screencapture -w ~/Desktop/screenshot-$(date +%Y%m%d-%H%M%S).png"

// Response (error)
"Invalid parameter value for 'format': must be one of [png, jpg, pdf]"
```

---

### 5. validate_file_executable

**Purpose**: Check if a file path points to an executable file.

**Signature**:
```rust
#[tauri::command]
async fn validate_file_executable(path: String) -> Result<bool, String>
```

**Parameters**:
- `path`: Absolute file path to check

**Returns**: `bool` - `true` if file exists and is executable, `false` otherwise

**Errors**:
- "File does not exist: {path}" - Path not found
- "Cannot read file metadata: {error}" - Permission/access error

**Behavior**:
1. Check if file exists at path
2. Read file metadata
3. Check Unix permissions (0o111 mask)
4. Return true if executable bit set

**Performance**: <1ms (single stat call)

**Frontend Usage**:
```typescript
const isExecutable = await invoke<boolean>('validate_file_executable', {
  path: '/Users/test/scripts/my-script.sh'
});

if (!isExecutable) {
  alert('Warning: This file may not be executable');
}
```

**Example Response**:
```json
true  // File is executable
false // File exists but not executable
```

---

### 6. open_file_picker

**Purpose**: Open native macOS file picker dialog to select a script or executable.

**Signature**:
```rust
#[tauri::command]
async fn open_file_picker(filter: Option<String>) -> Result<Option<String>, String>
```

**Parameters**:
- `filter` (optional): File extension filter (e.g., "sh", "py", "rb"). If `None`, allow all files.

**Returns**: `Option<String>` - Selected file path, or `None` if user cancelled

**Errors**:
- "File picker error: {error}" - System dialog error

**Behavior**:
1. Create `FileDialog` with title "Select Script or Executable"
2. If filter provided, apply extension filter
3. Show native macOS file picker
4. Return selected path or `None` if cancelled

**Performance**: Instant (native dialog)

**Frontend Usage**:
```typescript
const filePath = await invoke<string | null>('open_file_picker', {
  filter: 'sh' // Optional: filter for .sh files only
});

if (filePath) {
  console.log('Selected:', filePath);
} else {
  console.log('User cancelled');
}
```

**Example Response**:
```json
"/Users/test/scripts/my-script.sh"  // User selected file
null                                 // User cancelled
```

---

### 7. escape_shell_path

**Purpose**: Safely escape a file path for use in shell commands.

**Signature**:
```rust
#[tauri::command]
fn escape_shell_path(path: String) -> Result<String, String>
```

**Parameters**:
- `path`: Raw file path to escape

**Returns**: `String` - Shell-escaped path safe for command execution

**Errors**:
- Never errors (always returns valid escape)

**Behavior**:
1. Wrap path in single quotes
2. Escape any single quotes within path: `'` → `'\''`
3. Return escaped string

**Performance**: <1ms (simple string manipulation)

**Escaping Rules**:
- Spaces: Handled by single quotes
- Special chars ($, `, ", \): Handled by single quotes
- Single quotes: Escaped to `'\''` pattern
- Unicode: Preserved as-is

**Frontend Usage**:
```typescript
const rawPath = "/Users/test/My Documents/file with spaces.sh";
const escapedPath = await invoke<string>('escape_shell_path', { path: rawPath });
// Returns: "'/Users/test/My Documents/file with spaces.sh'"

const command = `bash ${escapedPath}`;
// Safe command: bash '/Users/test/My Documents/file with spaces.sh'
```

**Example Transformations**:
```
Input:  /tmp/normal.sh
Output: '/tmp/normal.sh'

Input:  /tmp/with spaces.sh
Output: '/tmp/with spaces.sh'

Input:  /tmp/quote's.sh
Output: '/tmp/quote'\''s.sh'

Input:  /tmp/$special`chars".sh
Output: '/tmp/$special`chars".sh'
```

---

## Command Registration

All commands must be registered in `main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_installed_applications,
            get_command_templates,
            get_command_categories,
            generate_command_from_template,
            validate_file_executable,
            open_file_picker,
            escape_shell_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Error Handling Pattern

**Backend**:
```rust
#[tauri::command]
async fn example_command() -> Result<String, String> {
    // Do work...
    if success {
        Ok("result".to_string())
    } else {
        Err("User-friendly error message".to_string())
    }
}
```

**Frontend**:
```typescript
try {
  const result = await invoke<string>('example_command');
  // Handle success
} catch (error) {
  console.error('Command failed:', error);
  // Show error to user
}
```

---

## TypeScript Service Layer

Recommended pattern for frontend:

```typescript
// src/services/applicationService.ts
import { invoke } from '@tauri-apps/api/core';
import type { Application } from '../types';

export const applicationService = {
  async listApplications(): Promise<Application[]> {
    return invoke('get_installed_applications');
  },

  searchApplications(apps: Application[], query: string): Application[] {
    const lowerQuery = query.toLowerCase();
    return apps.filter(app =>
      app.display_name.toLowerCase().includes(lowerQuery)
    );
  },

  generateLaunchCommand(app: Application): string {
    return `open -a "${app.display_name}"`;
  }
};

// src/services/commandService.ts
import { invoke } from '@tauri-apps/api/core';
import type { CommandTemplate, CommandCategory } from '../types';

export const commandService = {
  async listTemplates(categoryId?: string): Promise<CommandTemplate[]> {
    return invoke('get_command_templates', { categoryId });
  },

  async listCategories(): Promise<CommandCategory[]> {
    return invoke('get_command_categories');
  },

  async generateCommand(
    templateId: string,
    params: Record<string, string>
  ): Promise<string> {
    return invoke('generate_command_from_template', {
      templateId,
      params
    });
  },

  async validateExecutable(path: string): Promise<boolean> {
    return invoke('validate_file_executable', { path });
  },

  async pickFile(filter?: string): Promise<string | null> {
    return invoke('open_file_picker', { filter });
  },

  async escapePath(path: string): Promise<string> {
    return invoke('escape_shell_path', { path });
  }
};
```

---

## Testing

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_escape_shell_path() {
        let result = escape_shell_path("/tmp/with spaces.sh".to_string()).unwrap();
        assert_eq!(result, "'/tmp/with spaces.sh'");

        let result = escape_shell_path("/tmp/quote's.sh".to_string()).unwrap();
        assert_eq!(result, r"'/tmp/quote'\''s.sh'");
    }

    #[tokio::test]
    async fn test_generate_command_from_template() {
        // Load test template
        let result = generate_command_from_template(
            "volume-up".to_string(),
            [("amount".to_string(), "25".to_string())].iter().cloned().collect()
        ).await;

        assert!(result.is_ok());
        assert!(result.unwrap().contains("+ 25"));
    }
}
```

### Integration Tests (Frontend)

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import { applicationService } from '../services/applicationService';

describe('Application Service', () => {
  let applications: Application[];

  beforeAll(async () => {
    applications = await applicationService.listApplications();
  });

  it('should discover applications', () => {
    expect(applications.length).toBeGreaterThan(0);
  });

  it('should search applications by name', () => {
    const results = applicationService.searchApplications(applications, 'safari');
    expect(results.length).toBeGreaterThan(0);
    expect(results[0].display_name.toLowerCase()).toContain('safari');
  });

  it('should generate launch command', () => {
    const app = applications[0];
    const command = applicationService.generateLaunchCommand(app);
    expect(command).toMatch(/^open -a ".+"$/);
  });
});
```

---

## Conclusion

All Tauri commands are well-defined with:
- ✅ Clear signatures and parameters
- ✅ Error handling patterns
- ✅ Performance expectations
- ✅ Frontend usage examples
- ✅ Test coverage approach

Ready to proceed to quickstart guide (Phase 1 final artifact).
