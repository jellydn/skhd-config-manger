# Data Model: Shortcut Duplicate

**Feature**: `003-shortcut-duplicate`
**Date**: 2025-11-01
**Status**: Complete

## Overview

This feature introduces **no new data models**. It reuses the existing `Shortcut` type and operates entirely within the established data structures.

## Existing Data Models (Reused)

### Shortcut Type

**Location**: `src/types.ts:4-11`

```typescript
export interface Shortcut {
  id: string;              // UUID v4
  modifiers: string[];     // e.g., ["cmd", "shift"]
  key: string;             // e.g., "return", "a", "f1"
  command: string;         // Shell command to execute
  mode?: string;           // Optional mode restriction
  comment?: string;        // Optional user comment
  line_number: number;     // Position in skhd config file
}
```

### Config Type

**Location**: `src/types.ts:13-17`

```typescript
export interface Config {
  shortcuts: Shortcut[];
  file_path: string;
  is_modified: boolean;
}
```

## Data Flow

### 1. Duplicate Creation

```text
User clicks Duplicate button
    ↓
ShortcutItem emits onDuplicate(shortcut)
    ↓
+page.svelte handleDuplicate() creates new object:
    {
      ...sourceShortcut,           // Spread all fields
      id: crypto.randomUUID(),     // New unique ID
      line_number: getNextLine()   // Next available line number
    }
    ↓
editingShortcut = duplicatedShortcut
showForm = true
    ↓
ShortcutForm receives pre-filled data via props
```

### 2. User Modification

```text
User edits fields in ShortcutForm
    ↓
Form state updates (Svelte reactivity)
    ↓
User clicks Save
    ↓
ShortcutForm emits onSave(modifiedShortcut)
    ↓
+page.svelte handleSave() processes save
```

### 3. Save to Config

```text
handleSave(modifiedShortcut)
    ↓
Update config.shortcuts array:
    config = {
      ...config,
      shortcuts: [...config.shortcuts, modifiedShortcut],
      is_modified: true
    }
    ↓
User clicks "Save Changes" in header
    ↓
Tauri command save_config() writes to disk
```

## Field-Level Specifications

### id (UUID)

**Generation**: `crypto.randomUUID()`

**Purpose**: Unique identifier for React-style keyed lists

**Constraints**:
- Must be globally unique
- Never reuse IDs from deleted shortcuts
- Format: UUID v4 standard (e.g., "550e8400-e29b-41d4-a716-446655440000")

**Validation**: None required (UUID v4 guarantees uniqueness)

---

### line_number (integer)

**Generation**: `Math.max(...config.shortcuts.map(s => s.line_number)) + 1`

**Purpose**: Determines order in skhd config file and UI display

**Constraints**:
- Must be unique within config
- Must be positive integer (>= 1)
- Duplicates always append to end (max + 1)

**Validation**: Automatically handled by getNextLineNumber() function

**Edge Cases**:
- Empty config: line_number = 1
- Single shortcut (line 5): next duplicate = 6
- Gaps allowed (lines 1, 2, 5, 10 → next = 11)

---

### modifiers (string array)

**Source**: Copied from original shortcut

**Purpose**: Keyboard modifier keys (cmd, alt, shift, ctrl, fn)

**Constraints**:
- Order doesn't matter (skhd normalizes)
- Empty array allowed (no modifiers)
- Valid values: "cmd", "alt", "shift", "ctrl", "fn"

**Validation**: Existing skhd parser validates on save

**Duplicate Behavior**: Identical to source unless user modifies

---

### key (string)

**Source**: Copied from original shortcut

**Purpose**: Main keyboard key to trigger shortcut

**Constraints**:
- Required (cannot be empty)
- Case-insensitive
- Special keys: "return", "space", "tab", "escape", etc.
- Alphanumeric: "a"-"z", "0"-"9"
- Function keys: "f1"-"f20"

**Validation**:
- Existing validation checks for duplicate key combinations
- **Critical for duplicates**: Must modify key OR mode to avoid "duplicate key combination" error

**Duplicate Behavior**:
- Initially identical to source
- User typically modifies this field (P2 user story)

---

### command (string)

**Source**: Copied from original shortcut

**Purpose**: Shell command executed when shortcut triggered

**Constraints**:
- Required (cannot be empty)
- Freeform text (any valid shell command)
- Common patterns: "open -a AppName", "yabai -m ..."

**Validation**: Syntax validation only (skhd parser)

**Duplicate Behavior**:
- Initially identical to source
- User may modify for variations (P3 user story)

---

### mode (optional string)

**Source**: Copied from original shortcut if present

**Purpose**: Restrict shortcut to specific skhd mode

**Constraints**:
- Optional field
- Freeform text (user-defined mode names)
- Examples: "default", "resize", "focus"

**Validation**: No validation (user responsibility)

**Duplicate Behavior**: Copied as-is from source

---

### comment (optional string)

**Source**: Copied from original shortcut if present

**Purpose**: User notes about shortcut purpose

**Constraints**:
- Optional field
- Freeform text
- Displayed in UI, saved to skhd config as comment

**Validation**: None

**Duplicate Behavior**:
- Initially identical to source
- User typically modifies to reflect duplicate's purpose (P1 acceptance scenario 2)

---

## State Transitions

### Shortcut Lifecycle

```text
[Original Shortcut]
    ↓ (User clicks Duplicate)
[Duplicate Created - In Memory]
    id: new UUID
    line_number: max + 1
    all other fields: copied
    ↓ (User opens form)
[Editing State]
    editingShortcut = duplicate
    showForm = true
    ↓ (User modifies and saves)
[Persisted to Config]
    config.shortcuts.push(duplicate)
    config.is_modified = true
    ↓ (User clicks Save Changes)
[Written to Disk]
    Tauri save_config() → atomic file write
    ↓
[Active in skhd]
    User reloads skhd service
```

### State Properties

| State | In Memory | On Disk | Active in skhd |
|-------|-----------|---------|----------------|
| Original | ✅ | ✅ | ✅ |
| Duplicate created | ✅ | ❌ | ❌ |
| Duplicate saved | ✅ | ✅ | ❌ |
| skhd reloaded | ✅ | ✅ | ✅ |

**Key Insight**: Duplicate exists independently immediately upon creation. No parent-child relationship with original.

---

## Validation Rules

### Pre-Save Validation (ShortcutForm)

1. **Required Fields**:
   - `key` must not be empty
   - `command` must not be empty

2. **Duplicate Detection**:
   - Check if `(modifiers + key + mode)` combination already exists
   - Allow if only `command` differs (same key, different command in different mode)

3. **Syntax Validation**:
   - Delegate to existing skhd parser
   - Show validation errors in form

### Post-Save Validation (skhd parser)

1. **Configuration Integrity**:
   - Valid skhd syntax
   - No conflicting key bindings within same mode

2. **Atomic Write**:
   - Write to temporary file
   - Validate temp file
   - Move to config location (atomic operation)

**No changes to existing validation logic required.**

---

## Invariants

### Data Integrity

1. **ID Uniqueness**: Every shortcut has globally unique UUID
2. **Line Number Uniqueness**: Every shortcut has unique line number within config
3. **Independence**: Duplicates have no reference to source shortcut
4. **Immutability of Source**: Duplication never modifies original shortcut

### Consistency Rules

1. **Config Modified Flag**: Set `is_modified = true` when duplicate saved
2. **Sorted Display**: UI displays shortcuts sorted by `line_number` ascending
3. **Validation Applies Universally**: Same validation for create/edit/duplicate

---

## Edge Cases

### Empty Config
```typescript
// config.shortcuts = []
getNextLineNumber() → 1
duplicate.line_number = 1
```

### Gap in Line Numbers
```typescript
// config.shortcuts = [{line_number: 1}, {line_number: 5}, {line_number: 10}]
getNextLineNumber() → 11  // max + 1, gaps are allowed
```

### Duplicate Without Modification
```typescript
// User duplicates but doesn't change key
handleSave() → validation error: "Duplicate key combination"
// Form shows error, save blocked
```

### Multiple Duplicates
```typescript
// User duplicates same shortcut 3 times
// Result: 3 independent shortcuts with line_numbers: max+1, max+2, max+3
// Each can be edited/deleted independently
```

---

## Performance Considerations

### Memory

- Each `Shortcut` object: ~200 bytes
- Typical config: 50 shortcuts = 10KB
- Duplicates add negligible memory overhead

### Computation

- Object spread: O(1) for flat object (6 fields)
- getNextLineNumber(): O(n) where n = number of shortcuts
- For n=100 shortcuts: <1ms

### Reactivity

- Svelte 5 reactivity triggers on object reassignment
- No deep watchers or computed properties
- Efficient re-rendering of only affected components

---

## Future Considerations (Out of Scope)

### Not Implemented in MVP

- **Duplicate History**: Track which shortcuts are duplicates of others
- **Batch Duplicate**: Duplicate multiple shortcuts at once
- **Smart Defaults**: Auto-increment keys (e.g., "a" → "b" → "c")
- **Template System**: Save duplicates as templates for reuse

**Rationale**: MVP focuses on single-shortcut duplication. Advanced features can be added if user demand justifies complexity.

---

## Summary

**Data Model Impact**: ✅ Zero new models, 100% reuse

**Changes Required**:
- Add `getNextLineNumber()` helper function
- Add `handleDuplicate()` handler in +page.svelte
- Pass `onDuplicate` prop through ShortcutList → ShortcutItem

**Complexity**: Minimal - operates entirely within existing type system

**Risk**: Low - no schema changes, no migration required
