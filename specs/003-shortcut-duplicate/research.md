# Research: Shortcut Duplicate

**Feature**: `003-shortcut-duplicate`
**Date**: 2025-11-01
**Status**: Complete

## Research Overview

This feature has minimal research requirements as it reuses existing components and patterns. All technical approaches are well-established in the codebase.

## Decisions & Rationale

### 1. Component Reuse Strategy

**Decision**: Reuse existing ShortcutForm component without modifications

**Rationale**:
- ShortcutForm already supports pre-population via props
- Form validation logic is centralized and will apply to duplicates automatically
- Maintaining single form component ensures consistency
- Zero risk of diverging behavior between create/edit/duplicate flows

**Alternatives Considered**:
- Create separate DuplicateShortcutForm: Rejected due to code duplication and maintenance burden
- Inline duplicate form in ShortcutItem: Rejected due to poor UX (no full-screen edit experience)

**Evidence**: ShortcutForm.svelte:5-15 shows Props interface with all fields optional, allowing pre-population

---

### 2. Button Placement

**Decision**: Place Duplicate button between Test and Edit buttons in ShortcutItem actions

**Rationale**:
- Follows visual workflow: Test → Duplicate → Edit → Delete
- Groups related actions (Duplicate and Edit both modify shortcuts)
- Maintains existing button styling and spacing patterns
- Consistent with macOS HIG for action button grouping

**Alternatives Considered**:
- Place after Edit button: Rejected because Duplicate is more common than Delete
- Add to list header as global action: Rejected because duplication is per-shortcut operation
- Context menu (right-click): Rejected for discoverability concerns on macOS

**Evidence**: ShortcutItem.svelte:37-47 shows existing button pattern with Test, Edit, Delete

---

### 3. Data Flow Pattern

**Decision**: Use object spread operator to create independent duplicate

**Rationale**:
- Svelte 5 reactivity requires object reassignment for state updates
- Spread operator creates shallow copy, breaking reference to original
- Matches existing pattern in handleDelete (line 147-151 in +page.svelte)
- Simple, readable, performant (negligible cost for small objects)

**Implementation**:
```typescript
function handleDuplicate(shortcut: Shortcut) {
  editingShortcut = {
    ...shortcut,
    id: crypto.randomUUID(), // New unique ID
    line_number: getNextLineNumber() // Assign next available line
  };
  showForm = true;
}
```

**Alternatives Considered**:
- Deep clone with structuredClone(): Rejected as overkill for flat shortcut objects
- Immutability library (Immer.js): Rejected to avoid new dependency for simple operation

**Evidence**: +page.svelte:147-151 shows Svelte 5 reactivity pattern with object spread

---

### 4. Line Number Assignment

**Decision**: Assign next available line number (max + 1) to duplicates

**Rationale**:
- Matches existing create behavior (shortcuts append to end)
- Preserves user's mental model (duplicates appear at bottom of list)
- No risk of line number collisions
- Simple implementation (single Math.max() call)

**Implementation**:
```typescript
function getNextLineNumber(): number {
  if (!config || config.shortcuts.length === 0) return 1;
  return Math.max(...config.shortcuts.map(s => s.line_number)) + 1;
}
```

**Alternatives Considered**:
- Insert after source shortcut: Rejected due to complexity of renumbering all subsequent shortcuts
- Let user choose position: Rejected as over-engineering for MVP

**Evidence**: Shortcut type definition in types.ts:4-11 shows line_number is required field

---

### 5. Validation Strategy

**Decision**: Reuse existing validation without modifications

**Rationale**:
- Duplicates are just new shortcuts from validation perspective
- Existing duplicate key detection will catch if user doesn't modify key
- Same validation rules apply (required fields, syntax checks)
- No special cases needed

**Evidence**: Validation logic in src-tauri/src/commands/validation.rs handles all shortcut validation

**Edge Cases Handled**:
- User duplicates and saves without changes → validation error "duplicate key combination"
- User modifies only command → saves successfully (different shortcuts, same key if in different modes)
- User modifies only key → saves successfully (duplicate command is allowed)

---

### 6. UI/UX Considerations

**Decision**: Use existing button styling with "Duplicate" label

**Icon**: Consider using document copy icon (⎘) or leave as text-only button

**Rationale**:
- Text label "Duplicate" is clearer than icon-only for discoverability
- Matches existing Test/Edit/Delete buttons (text labels)
- Icon is optional enhancement, not MVP requirement

**Accessibility**: Button will have same keyboard navigation as existing actions (Tab key)

**Visual Feedback**: Form opening with pre-filled data provides immediate confirmation

---

## Technical Stack Validation

### Frontend
- **Svelte 5**: Confirmed reactive patterns match existing codebase
- **TypeScript 5**: No new types needed, Shortcut interface covers all fields
- **Vite**: No build configuration changes

### Backend
- **Rust/Tauri**: No changes required
- **Commands**: Reuses existing save_config, no new Tauri commands

### Testing
- **Vitest**: Manual UI testing sufficient (no unit test infrastructure for Svelte components)
- **Integration**: Manual test: duplicate → modify → save → verify both exist

---

## Performance Analysis

### Object Duplication
- **Operation**: Object spread with 5-6 fields
- **Cost**: <1ms (negligible)
- **Memory**: ~200 bytes per shortcut object

### Form Pre-population
- **Operation**: Set reactive variables in Svelte
- **Cost**: <5ms (same as existing edit flow)
- **Impact**: Zero user-perceptible delay

### Validation
- **Operation**: Reuses existing validation (no additional cost)
- **Performance**: Same as create/edit operations

---

## Risk Assessment

### Low Risk Items ✅
- Component reuse (proven pattern)
- Svelte reactivity (matches existing code)
- Validation (no changes to existing logic)
- Line number assignment (simple arithmetic)

### Medium Risk Items ⚠️
- None identified

### High Risk Items 🚨
- None identified

---

## Open Questions

**None** - All technical decisions resolved. Feature is ready for design phase.

---

## References

1. Existing codebase patterns:
   - src/components/ShortcutItem.svelte (button actions)
   - src/components/ShortcutForm.svelte (form pre-population)
   - src/routes/+page.svelte (Svelte 5 reactivity)
   - src/types.ts (Shortcut type definition)

2. External documentation:
   - Svelte 5 runes: https://svelte.dev/docs/svelte/$state
   - Apple HIG button patterns (macOS native look)

---

## Next Steps

Proceed to Phase 1: Design & Contracts
- Generate data-model.md (minimal - reuses existing Shortcut type)
- Generate contracts/ (N/A - no API changes)
- Generate quickstart.md (developer implementation guide)
