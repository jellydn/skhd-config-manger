# Quickstart: Implementing Shortcut Duplicate

**Feature**: `003-shortcut-duplicate`
**Estimated Time**: 2-4 hours
**Difficulty**: Easy
**Prerequisites**: Familiarity with Svelte 5, TypeScript

## Overview

Add a "Duplicate" button to each shortcut item that pre-fills the edit form with the shortcut's data, allowing users to quickly create similar shortcuts.

## Implementation Checklist

- [ ] Step 1: Add Duplicate button to ShortcutItem component (15 min)
- [ ] Step 2: Add handler functions in +page.svelte (20 min)
- [ ] Step 3: Wire up props through ShortcutList (10 min)
- [ ] Step 4: Add styling for Duplicate button (10 min)
- [ ] Step 5: Manual testing of duplicate flow (30 min)
- [ ] Step 6: Edge case testing (15 min)
- [ ] Total: ~1.5 hours core implementation + testing

## Step-by-Step Implementation

### Step 1: Add Duplicate Button to ShortcutItem

**File**: `src/components/ShortcutItem.svelte`

**Location**: Lines 37-47 (action buttons section)

#### 1.1 Add prop to Props interface

```diff
 interface Props {
   shortcut: Shortcut;
   onEdit?: (shortcut: Shortcut) => void;
   onDelete?: (id: string) => void;
   onTest?: (id: string) => void;
+  onDuplicate?: (shortcut: Shortcut) => void;
 }

-let { shortcut, onEdit, onDelete, onTest }: Props = $props();
+let { shortcut, onEdit, onDelete, onTest, onDuplicate }: Props = $props();
```

#### 1.2 Add button to template

Insert between Test and Edit buttons:

```svelte
<div class="shortcut-actions">
  {#if onTest}
    <button type="button" class="btn-test" onclick={() => onTest(shortcut.id)}> Test </button>
  {/if}
  {#if onDuplicate}
    <button type="button" class="btn-duplicate" onclick={() => onDuplicate(shortcut)}> Duplicate </button>
  {/if}
  {#if onEdit}
    <button type="button" class="btn-edit" onclick={() => onEdit(shortcut)}> Edit </button>
  {/if}
  {#if onDelete}
    <button type="button" class="btn-delete" onclick={() => onDelete(shortcut.id)}> Delete </button>
  {/if}
</div>
```

#### 1.3 Add styling

Add after existing button styles (around line 154):

```css
.btn-duplicate {
  background: #8e8e93;
  color: white;
}

.btn-duplicate:hover {
  background: #636366;
}

@media (prefers-color-scheme: dark) {
  .btn-duplicate {
    background: #636366;
  }

  .btn-duplicate:hover {
    background: #8e8e93;
  }
}
```

---

### Step 2: Add Handler Functions in +page.svelte

**File**: `src/routes/+page.svelte`

**Location**: After existing handler functions (around line 100-150)

#### 2.1 Add getNextLineNumber helper

```typescript
function getNextLineNumber(): number {
  if (!config || config.shortcuts.length === 0) return 1;
  return Math.max(...config.shortcuts.map(s => s.line_number)) + 1;
}
```

**Why**: Assigns unique line number to duplicated shortcuts (appends to end)

#### 2.2 Add handleDuplicate function

```typescript
function handleDuplicate(shortcut: Shortcut) {
  console.log('handleDuplicate called with:', shortcut);

  editingShortcut = {
    ...shortcut,
    id: crypto.randomUUID(),
    line_number: getNextLineNumber()
  };

  showForm = true;
}
```

**Why**: Creates independent copy with new ID and line number, opens form for editing

**Note**: Uses `crypto.randomUUID()` (available in all modern browsers, Tauri environment)

---

### Step 3: Wire Up Props Through ShortcutList

**File**: `src/components/ShortcutList.svelte`

#### 3.1 Add prop to Props interface

```diff
 interface Props {
   shortcuts: Shortcut[];
   onEdit?: (shortcut: Shortcut) => void;
   onDelete?: (id: string) => void;
   onTest?: (id: string) => void;
   onCreate?: () => void;
   onSave?: () => void;
   isModified?: boolean;
+  onDuplicate?: (shortcut: Shortcut) => void;
 }

-let { shortcuts, onEdit, onDelete, onTest, onCreate, onSave, isModified }: Props = $props();
+let { shortcuts, onEdit, onDelete, onTest, onCreate, onSave, isModified, onDuplicate }: Props = $props();
```

#### 3.2 Pass to ShortcutItem

```diff
 {#each sortedShortcuts as shortcut (shortcut.id)}
-  <ShortcutItem {shortcut} {onEdit} {onDelete} {onTest} />
+  <ShortcutItem {shortcut} {onEdit} {onDelete} {onTest} {onDuplicate} />
 {/each}
```

---

### Step 4: Connect Handler in +page.svelte

**File**: `src/routes/+page.svelte`

**Location**: ShortcutList component usage (around line 360)

```diff
 <ShortcutList
   shortcuts={config.shortcuts}
   {onEdit}
   {onDelete}
   {onTest}
   {onCreate}
   {onSave}
   {isModified}
+  onDuplicate={handleDuplicate}
 />
```

---

## Testing Guide

### Manual Test Plan

#### Test 1: Basic Duplication ✅

1. Launch app, load config with existing shortcuts
2. Click "Duplicate" on any shortcut
3. **Verify**: Form opens with all fields pre-filled
4. **Verify**: Console shows `handleDuplicate called with: {shortcut data}`
5. Click "Save" without changes
6. **Expect**: Error message "Duplicate key combination"
7. Modify the `key` field (e.g., change "a" to "b")
8. Click "Save"
9. **Verify**: New shortcut appears in list
10. **Verify**: Original shortcut unchanged
11. **Verify**: "Save Changes" button enabled (config.is_modified = true)

#### Test 2: Multiple Duplicates ✅

1. Duplicate same shortcut 3 times (modifying key each time)
2. **Verify**: All duplicates have unique IDs (check in React DevTools)
3. **Verify**: Line numbers increment: max+1, max+2, max+3
4. **Verify**: Each appears as separate item in list
5. Delete one duplicate
6. **Verify**: Others remain unaffected

#### Test 3: Edge Cases ✅

**Empty Config**:
1. Start with empty config
2. Create one shortcut
3. Duplicate it
4. **Verify**: line_number = 2

**Cancel After Duplicate**:
1. Click Duplicate
2. Form opens
3. Click "Cancel" (close form without saving)
4. **Verify**: No new shortcut created
5. **Verify**: config.is_modified = false

**Duplicate Then Save Config**:
1. Duplicate shortcut, modify, save to config
2. Click "Save Changes" button
3. **Verify**: Config written to disk
4. Reload app
5. **Verify**: Duplicated shortcut persists

#### Test 4: Validation ✅

**Duplicate Key Detection**:
1. Duplicate shortcut with key="a", modifiers=["cmd"]
2. Attempt to save without modifying
3. **Verify**: Error "Duplicate key combination: cmd+a"

**Same Key, Different Mode**:
1. Duplicate shortcut with mode="default"
2. Change mode to "resize"
3. Keep same key
4. **Verify**: Saves successfully (different modes allow same keys)

#### Test 5: UI/UX ✅

**Button Appearance**:
- **Verify**: Duplicate button has gray background (secondary action color)
- **Verify**: Hover state darkens button
- **Verify**: Button positioned between Test and Edit

**Dark Mode**:
- Switch to dark mode (macOS System Preferences)
- **Verify**: Button colors adapt appropriately

**Keyboard Navigation**:
- Tab through shortcut items
- **Verify**: Focus order: Test → Duplicate → Edit → Delete
- Press Enter on Duplicate button
- **Verify**: Form opens

---

## Development Workflow

### 1. Start Development Server

```bash
bun run dev
```

### 2. Make Changes

Edit files according to implementation steps above.

### 3. Hot Reload Verification

Changes should reflect immediately in the running app (Vite HMR).

### 4. Type Checking

```bash
bun run typecheck
```

**Expect**: No TypeScript errors

### 5. Linting

```bash
bun run lint
```

**Expect**: No lint errors (may need to add `.btn-duplicate` to CSS if using CSS modules)

### 6. Manual Testing

Follow manual test plan above.

### 7. Commit

```bash
git add src/components/ShortcutItem.svelte \
        src/components/ShortcutList.svelte \
        src/routes/+page.svelte

git commit -m "Add duplicate button to quickly clone and edit shortcuts

- Add Duplicate button to ShortcutItem component (gray secondary action)
- Implement handleDuplicate() to create independent copy with new UUID
- Pre-fill form with all shortcut fields for easy modification
- Support duplicate → modify key → save workflow
- Line numbers auto-assigned (max + 1) for duplicates

Closes #003-shortcut-duplicate"
```

---

## Troubleshooting

### Issue: Duplicate button not appearing

**Check**:
1. Is `onDuplicate` prop passed from +page.svelte → ShortcutList → ShortcutItem?
2. Console errors about missing props?

**Fix**: Verify prop chain is complete (all 3 files updated)

---

### Issue: Form opens but fields are empty

**Check**:
1. Is `editingShortcut` assigned correctly in `handleDuplicate()`?
2. Console log `editingShortcut` value

**Fix**: Ensure object spread syntax is correct: `{ ...shortcut, id: newUUID }`

---

### Issue: Duplicate saves without error (duplicate key allowed)

**Check**:
1. Validation logic in ShortcutForm
2. Is duplicate detection checking `(modifiers + key + mode)` combination?

**Fix**: Verify existing validation logic is active (should already work, no changes needed)

---

### Issue: TypeScript errors on `crypto.randomUUID()`

**Check**: TypeScript lib configuration

**Fix**: Ensure `tsconfig.json` includes `"DOM"` and `"DOM.Iterable"` in lib array

---

### Issue: Line numbers wrong (duplicates get same number)

**Check**:
1. Is `getNextLineNumber()` using `Math.max()`?
2. Are shortcuts being added to `config.shortcuts` array?

**Fix**: Verify `Math.max(...config.shortcuts.map(s => s.line_number))` syntax

---

## Performance Validation

### Check 1: Form Opens Quickly

**Test**: Click Duplicate → measure time to form appearing

**Target**: <16ms (60fps)

**Measure**: Use browser DevTools Performance tab

**Expected**: Should be ~5-10ms (object duplication + Svelte reactivity)

### Check 2: No UI Freezing

**Test**: Duplicate 10 shortcuts rapidly

**Target**: UI remains responsive

**Measure**: Observe cursor, button hover states

**Expected**: No jank, smooth interactions

### Check 3: Memory Usage

**Test**: Duplicate 50 shortcuts

**Target**: <5MB additional memory

**Measure**: Browser DevTools Memory profiler

**Expected**: ~200 bytes per duplicate = 10KB total

---

## Code Review Checklist

Before requesting review:

- [ ] All 3 files modified (ShortcutItem, ShortcutList, +page.svelte)
- [ ] TypeScript compilation passes
- [ ] Linter passes
- [ ] Manual test plan completed
- [ ] Dark mode tested
- [ ] Keyboard navigation works
- [ ] Console.log debugging statements removed (or keep if useful)
- [ ] Comments added for complex logic (optional, code is self-documenting)
- [ ] Commit message follows project convention

---

## Next Steps

After implementation complete:

1. **Generate Tasks**: Run `/speckit.tasks` to create detailed implementation tasks
2. **Execute Tasks**: Follow task breakdown systematically
3. **Test**: Complete manual testing checklist
4. **Commit**: Atomic commit with descriptive message
5. **Merge**: Merge feature branch to main (if approved)

---

## Reference Files

### Key Code Locations

- **ShortcutItem**: `src/components/ShortcutItem.svelte:37-47` (action buttons)
- **ShortcutList**: `src/components/ShortcutList.svelte:5-15` (props), `41-43` (template)
- **+page.svelte**: `~100-150` (handlers), `~360` (ShortcutList usage)
- **Types**: `src/types.ts:4-11` (Shortcut interface)

### Related Documentation

- **Spec**: `specs/003-shortcut-duplicate/spec.md`
- **Data Model**: `specs/003-shortcut-duplicate/data-model.md`
- **Contracts**: `specs/003-shortcut-duplicate/contracts/ui-interface.md`
- **Research**: `specs/003-shortcut-duplicate/research.md`

---

## FAQ

**Q: Why pass entire `shortcut` object instead of just `id`?**

A: We need all fields for form pre-population. Passing object directly avoids lookup in config array.

**Q: Why use `crypto.randomUUID()` instead of incrementing IDs?**

A: UUIDs guarantee global uniqueness without coordination. No risk of ID collisions across sessions/imports.

**Q: Can users duplicate a shortcut that's already a duplicate?**

A: Yes! Each duplicate is independent. Duplicating a duplicate creates a new independent shortcut.

**Q: What if user deletes the original after creating duplicates?**

A: Duplicates are unaffected. They're independent objects with no parent-child relationship.

**Q: Should we track duplicate history (which came from which)?**

A: Out of scope for MVP. Could be added later if users request it.

---

## Success Criteria

Implementation is complete when:

✅ User can click Duplicate on any shortcut
✅ Form opens with all fields pre-filled
✅ User can modify fields and save as new shortcut
✅ Original shortcut remains unchanged
✅ Duplicate detection prevents saving unchanged duplicates
✅ Line numbers auto-assign correctly
✅ UI matches macOS design language (gray button, proper hover states)
✅ Keyboard navigation works (Tab to Duplicate, Enter to activate)
✅ Dark mode works correctly

**Estimated Value**: 60% time savings for creating similar shortcuts (45s → 15s)
