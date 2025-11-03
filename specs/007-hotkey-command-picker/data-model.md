# Data Model: Command and Application Picker

**Feature**: 007-hotkey-command-picker
**Date**: 2025-11-03
**Status**: Complete

This document defines all data entities and their relationships for the command and application picker feature.

## Entity Definitions

### 1. Application

Represents an installed macOS application discovered from standard application directories.

**Properties**:

| Field | Type | Required | Description | Validation |
|-------|------|----------|-------------|------------|
| `display_name` | String | Yes | Human-readable application name (e.g., "Safari", "Visual Studio Code") | Non-empty, max 255 chars |
| `app_path` | String | Yes | Full path to .app bundle (e.g., "/Applications/Safari.app") | Valid file path, must exist, must end in .app |
| `bundle_id` | String | Yes | macOS bundle identifier (e.g., "com.apple.Safari") | Reverse domain notation pattern |
| `executable_path` | String | Yes | Path to actual executable within bundle (e.g., "/Applications/Safari.app/Contents/MacOS/Safari") | Valid file path, must be executable |
| `icon_path` | Option<String> | No | Path to application icon file (e.g., "/Applications/Safari.app/Contents/Resources/AppIcon.icns") | Valid file path if present |
| `version` | Option<String> | No | Application version (e.g., "16.3") | Semver or arbitrary version string |

**Rust Implementation**:
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

**TypeScript Interface**:
```typescript
export interface Application {
  display_name: string;
  app_path: string;
  bundle_id: string;
  executable_path: string;
  icon_path?: string;
  version?: string;
}
```

**Uniqueness**: Combination of `bundle_id` + `app_path` (handles multiple versions)

**State Transitions**: Immutable (read-only from file system)

**Relationships**:
- No direct relationships (standalone entity)
- Used by frontend to generate `open -a "{display_name}"` commands

**Example**:
```json
{
  "display_name": "Visual Studio Code",
  "app_path": "/Applications/Visual Studio Code.app",
  "bundle_id": "com.microsoft.VSCode",
  "executable_path": "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
  "icon_path": "/Applications/Visual Studio Code.app/Contents/Resources/Code.icns",
  "version": "1.85.0"
}
```

---

### 2. CommandTemplate

Represents a pre-configured command pattern with optional parameters for common tasks.

**Properties**:

| Field | Type | Required | Description | Validation |
|-------|------|----------|-------------|------------|
| `id` | String | Yes | Unique template identifier (e.g., "volume-up", "screenshot-area") | Lowercase, hyphen-separated, max 64 chars |
| `name` | String | Yes | Human-readable template name (e.g., "Increase Volume") | Non-empty, max 128 chars |
| `description` | String | Yes | Explanation of what the command does | Non-empty, max 512 chars |
| `category_id` | String | Yes | Reference to CommandCategory.id | Must match existing category |
| `command_pattern` | String | Yes | Shell command with `{param}` placeholders | Non-empty, valid shell syntax |
| `parameters` | Vec<CommandParameter> | Yes | List of parameters (empty if none) | Unique parameter names |
| `requires_admin` | bool | No (default: false) | Whether command needs elevated privileges | Boolean |

**Rust Implementation**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category_id: String,
    pub command_pattern: String,
    pub parameters: Vec<CommandParameter>,
    #[serde(default)]
    pub requires_admin: bool,
}
```

**TypeScript Interface**:
```typescript
export interface CommandTemplate {
  id: string;
  name: string;
  description: string;
  category_id: string;
  command_pattern: string;
  parameters: CommandParameter[];
  requires_admin?: boolean;
}
```

**Uniqueness**: `id` field (globally unique within templates)

**State Transitions**: Immutable (defined in embedded JSON)

**Relationships**:
- **Many-to-One** with `CommandCategory` (via `category_id`)
- **One-to-Many** with `CommandParameter` (embedded array)

**Example**:
```json
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
```

---

### 3. CommandParameter

Represents a configurable value within a CommandTemplate.

**Properties**:

| Field | Type | Required | Description | Validation |
|-------|------|----------|-------------|------------|
| `name` | String | Yes | Parameter name (matches `{name}` in command_pattern) | Lowercase, alphanumeric+underscore, max 32 chars |
| `description` | String | Yes | User-facing explanation of parameter | Non-empty, max 256 chars |
| `data_type` | String | Yes | Parameter type: "string", "integer", "float", "boolean", "enum" | One of defined types |
| `default_value` | String | Yes | Default parameter value | Must match data_type |
| `validation_regex` | Option<String> | No | Regex pattern for validation (if data_type = "string" or "integer") | Valid regex pattern |
| `min_value` | Option<i32> | No | Minimum value (if data_type = "integer" or "float") | Must be less than max_value |
| `max_value` | Option<i32> | No | Maximum value (if data_type = "integer" or "float") | Must be greater than min_value |
| `enum_values` | Option<Vec<String>> | No | Allowed values (if data_type = "enum") | Non-empty array |

**Rust Implementation**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    pub name: String,
    pub description: String,
    pub data_type: String,
    pub default_value: String,
    pub validation_regex: Option<String>,
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
    pub enum_values: Option<Vec<String>>,
}
```

**TypeScript Interface**:
```typescript
export interface CommandParameter {
  name: string;
  description: string;
  data_type: 'string' | 'integer' | 'float' | 'boolean' | 'enum';
  default_value: string;
  validation_regex?: string;
  min_value?: number;
  max_value?: number;
  enum_values?: string[];
}
```

**Uniqueness**: `name` field (unique within parent CommandTemplate)

**State Transitions**: Immutable (defined in embedded JSON)

**Relationships**:
- **Many-to-One** with `CommandTemplate` (embedded within template)

**Validation Rules**:
1. If `data_type = "integer"`, `default_value` must be parseable as integer
2. If `data_type = "float"`, `default_value` must be parseable as float
3. If `data_type = "boolean"`, `default_value` must be "true" or "false"
4. If `data_type = "enum"`, `default_value` must be in `enum_values`
5. If `validation_regex` present, `default_value` must match pattern
6. If `min_value`/`max_value` present, `default_value` must be in range

**Example**:
```json
{
  "name": "direction",
  "description": "Window tiling direction",
  "data_type": "enum",
  "default_value": "left",
  "enum_values": ["left", "right", "top", "bottom", "center"],
  "validation_regex": null,
  "min_value": null,
  "max_value": null
}
```

---

### 4. CommandCategory

Represents a logical grouping of command templates.

**Properties**:

| Field | Type | Required | Description | Validation |
|-------|------|----------|-------------|------------|
| `id` | String | Yes | Unique category identifier (e.g., "media", "system", "window") | Lowercase, hyphen-separated, max 32 chars |
| `name` | String | Yes | Human-readable category name (e.g., "Media Control") | Non-empty, max 64 chars |
| `description` | String | Yes | Category explanation | Non-empty, max 256 chars |
| `icon` | Option<String> | No | SF Symbol name or emoji (e.g., "speaker.wave.2", "🔊") | Valid SF Symbol or single emoji |
| `display_order` | i32 | No (default: 0) | Sort order in UI | Non-negative integer |

**Rust Implementation**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub display_order: i32,
}
```

**TypeScript Interface**:
```typescript
export interface CommandCategory {
  id: string;
  name: string;
  description: string;
  icon?: string;
  display_order?: number;
}
```

**Uniqueness**: `id` field (globally unique)

**State Transitions**: Immutable (defined in embedded JSON)

**Relationships**:
- **One-to-Many** with `CommandTemplate` (via `category_id` foreign key)

**Example**:
```json
{
  "id": "media",
  "name": "Media Control",
  "description": "Audio playback and volume controls",
  "icon": "speaker.wave.2",
  "display_order": 1
}
```

---

## Entity Relationships

```text
CommandCategory
    └──< CommandTemplate (many templates per category)
            └──< CommandParameter (many parameters per template)

Application (standalone, no relationships)
```

**Diagram**:
```
┌─────────────────────┐
│  CommandCategory    │
│  - id               │
│  - name             │
│  - description      │
└─────────────────────┘
          │ 1
          │
          │ *
┌─────────────────────┐
│  CommandTemplate    │
│  - id               │
│  - category_id (FK) │
│  - command_pattern  │
│  - parameters[]     │
└─────────────────────┘
          │ 1
          │
          │ *
┌─────────────────────┐
│  CommandParameter   │
│  - name             │
│  - data_type        │
│  - default_value    │
└─────────────────────┘

┌─────────────────────┐
│  Application        │  (Independent)
│  - bundle_id        │
│  - display_name     │
│  - executable_path  │
└─────────────────────┘
```

---

## Data Flow

### Application Discovery Flow

```
1. User opens ApplicationPicker component
2. Frontend calls invoke('get_installed_applications')
3. Backend scans /Applications, ~/Applications, /System/Applications
4. Backend parses each .app bundle's Info.plist
5. Backend returns Vec<Application>
6. Frontend caches results (session-only)
7. Frontend displays list with search filtering
8. User selects application
9. Frontend generates command: `open -a "{display_name}"`
10. Command inserted into ShortcutForm
```

### Template Selection Flow

```
1. User opens CommandPicker component
2. Frontend calls invoke('get_command_templates', category?)
3. Backend loads embedded command_templates.json
4. Backend filters by category if specified
5. Backend returns Vec<CommandTemplate>
6. Frontend displays categorized list
7. User selects template
8. If template has parameters:
   a. Frontend shows TemplateParameterForm
   b. User inputs parameter values
   c. Frontend validates against parameter rules
   d. Frontend calls invoke('generate_command_from_template', id, params)
   e. Backend substitutes {param} placeholders
   f. Backend returns generated command string
9. Command inserted into ShortcutForm
```

---

## Storage

### Application Data

**Storage Type**: None (ephemeral, generated on-demand)
**Caching**: Frontend caches results in component state (session-only)
**Persistence**: Not persisted (regenerated each session)

### Command Templates

**Storage Type**: Embedded JSON file (`src-tauri/src/data/command_templates.json`)
**Loading**: Compiled into binary via `include_str!` macro
**Format**:
```json
{
  "categories": [
    { "id": "...", "name": "...", "description": "...", "icon": "..." }
  ],
  "templates": [
    { "id": "...", "category_id": "...", "command_pattern": "...", "parameters": [...] }
  ]
}
```

**Rationale**: Embedded for simplicity (no external files), easy to extend, version-controlled

---

## Validation Rules Summary

### Application Validation

- ✅ `app_path` must exist on file system
- ✅ `app_path` must end with ".app"
- ✅ `executable_path` must exist and be executable (chmod +x)
- ✅ `bundle_id` must follow reverse domain notation
- ✅ `display_name` must be non-empty
- ⚠️ Duplicates: Allow multiple apps with same `display_name` (show with path suffix)

### CommandTemplate Validation

- ✅ `id` must be unique across all templates
- ✅ `category_id` must reference existing CommandCategory
- ✅ `command_pattern` must contain valid shell syntax
- ✅ All `{param}` placeholders in `command_pattern` must have matching parameter in `parameters[]`
- ✅ All parameters in `parameters[]` must be used in `command_pattern`

### CommandParameter Validation

- ✅ `name` must be unique within parent template
- ✅ `default_value` must match `data_type` constraints
- ✅ If `validation_regex` present, must be valid regex
- ✅ If `min_value`/`max_value` present, must form valid range
- ✅ If `enum_values` present, must be non-empty array

---

## Conclusion

All data entities are well-defined with clear:
- ✅ Field types and constraints
- ✅ Validation rules
- ✅ Uniqueness constraints
- ✅ Relationships and foreign keys
- ✅ State transitions (immutable for all entities)
- ✅ Storage strategy (ephemeral for apps, embedded JSON for templates)

Ready to proceed to contract definitions (Phase 1 continued).
