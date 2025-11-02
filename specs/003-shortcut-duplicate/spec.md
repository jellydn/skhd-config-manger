# Feature Specification: Shortcut Duplicate for Quick Clone and Edit

**Feature Branch**: `003-shortcut-duplicate`
**Created**: 2025-11-01
**Status**: Draft
**Input**: User description: "add duplicate to quick clone then edit"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Quick Shortcut Duplication (Priority: P1)

Users need to create similar shortcuts with slight variations (e.g., same command for different keys, or similar keys for different applications). Currently, users must manually re-enter all shortcut details from scratch, which is time-consuming and error-prone.

**Why this priority**: This is the core value of the feature - reducing repetitive data entry when creating similar shortcuts. It delivers immediate, measurable time savings.

**Independent Test**: Can be fully tested by duplicating an existing shortcut, verifying all fields are copied, and confirming the duplicate can be edited and saved independently. Delivers standalone value even without other features.

**Acceptance Scenarios**:

1. **Given** a shortcut exists in the list, **When** user clicks the "Duplicate" button on that shortcut, **Then** the shortcut form opens with all fields pre-filled with the same values (modifiers, key, command, mode, comment)
2. **Given** the duplicate shortcut form is open with pre-filled values, **When** user modifies any field and clicks save, **Then** a new shortcut is created with the modified values without affecting the original
3. **Given** user has duplicated a shortcut, **When** viewing the shortcut list, **Then** both the original and the duplicated shortcut appear as separate entries

---

### User Story 2 - Duplicate with Modified Key (Priority: P2)

Users creating keyboard shortcut schemes often map the same command to multiple key combinations (e.g., opening different browsers with ctrl+alt+b, ctrl+alt+g, ctrl+alt+f).

**Why this priority**: Enhances the duplication workflow for the most common use case - changing only the key while keeping the command the same. Builds on P1 functionality.

**Independent Test**: Can be tested by duplicating a shortcut and only changing the key field, verifying the save validation prevents duplicate key combinations.

**Acceptance Scenarios**:

1. **Given** a duplicated shortcut form is open, **When** user changes only the key field and clicks save, **Then** the new shortcut is created with the new key but same command
2. **Given** user is editing a duplicated shortcut, **When** the new key combination already exists in the configuration, **Then** system prevents save and shows error message "Duplicate key combination"
3. **Given** user has created multiple duplicates with different keys, **When** viewing the list, **Then** shortcuts are displayed in line number order showing all variations

---

### User Story 3 - Duplicate with Modified Command (Priority: P3)

Users want to use the same key combination in different modes or create similar commands with different parameters (e.g., "open -a Terminal" vs "open -a iTerm").

**Why this priority**: Supports advanced use cases where users want the same key for different contexts. Less common than P1/P2 but completes the duplication workflow.

**Independent Test**: Can be tested by duplicating a shortcut and modifying the command field, verifying the new shortcut saves correctly.

**Acceptance Scenarios**:

1. **Given** a duplicated shortcut form is open, **When** user changes the command field and clicks save, **Then** the new shortcut is created with the new command
2. **Given** user is duplicating a shortcut with comments, **When** the duplicate form opens, **Then** the comment field is also pre-filled allowing user to modify it for the new context

---

### Edge Cases

- What happens when user duplicates a shortcut then clicks "Cancel" instead of save? (The original shortcut should remain unchanged, no new shortcut created)
- How does the system handle duplicating a shortcut when the configuration file has unsaved changes? (Should work normally since duplicates are in-memory operations until explicit save)
- What if user duplicates a shortcut, modifies it to match an existing shortcut exactly, then tries to save? (Duplicate detection should prevent save with appropriate error message)
- What happens when user duplicates the last shortcut in the list? (New duplicate should appear at the end with appropriate line number)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide a "Duplicate" action button on each shortcut item in the list
- **FR-002**: When duplicate action is triggered, system MUST open the shortcut edit form with all fields pre-populated from the source shortcut (modifiers, key, command, mode, comment)
- **FR-003**: System MUST treat duplicated shortcuts as new shortcuts with unique IDs
- **FR-004**: System MUST assign appropriate line numbers to duplicated shortcuts (next available line number after the last shortcut)
- **FR-005**: System MUST preserve all validation rules for duplicated shortcuts (duplicate key detection, required fields, syntax validation)
- **FR-006**: Users MUST be able to modify any field in the duplicated shortcut before saving
- **FR-007**: System MUST mark the configuration as modified when a duplicated shortcut is saved
- **FR-008**: System MUST allow users to cancel duplication without affecting the original shortcut or creating a new one

### Non-Functional Requirements

#### Accessibility Requirements
- **NFR-A01**: All icon-only buttons MUST include descriptive `aria-label` attributes for screen reader accessibility
- **NFR-A02**: Decorative SVG icons MUST include `aria-hidden="true"` to prevent redundant screen reader announcements
- **NFR-A03**: Interactive elements MUST be keyboard accessible with clear focus indicators
- **NFR-A04**: Button labels MUST clearly describe the action (e.g., "Duplicate shortcut" not just "Duplicate")

#### Performance Requirements
- **NFR-P01**: Array comparisons MUST use efficient element-by-element comparison, not JSON serialization
- **NFR-P02**: Form change detection MUST complete in under 1ms for typical shortcut configurations
- **NFR-P03**: Duplicate action MUST respond within 100ms to maintain perceived instant feedback

#### Code Quality Requirements
- **NFR-Q01**: Conditional rendering logic MUST use object lookup patterns instead of nested ternary operators when mapping modes to configuration
- **NFR-Q02**: Form configuration (titles, button text) MUST be centralized in a single source of truth
- **NFR-Q03**: Helper functions MUST be created for reusable logic (e.g., array comparison) rather than inline implementations

### Key Entities

- **Shortcut Duplicate**: A new shortcut instance initialized with values copied from an existing shortcut, maintaining independence from the source

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can duplicate a shortcut and create a new variation in under 15 seconds (compared to 45+ seconds for manual entry)
- **SC-002**: Duplication reduces data entry errors by 80% (no manual retyping of complex commands or modifier combinations)
- **SC-003**: 90% of users successfully create duplicate shortcuts on first attempt without errors
- **SC-004**: Users creating multiple similar shortcuts (3+) report time savings of 60% or more compared to manual creation

## Assumptions

- Duplicate button will be visually grouped with other shortcut actions (Test, Edit, Delete)
- Duplicated shortcuts do not automatically save - user must explicitly save like any new shortcut
- Line number assignment follows existing logic (appends to end)
- No limit on number of times a shortcut can be duplicated
- Duplicates are independent - deleting the original does not affect duplicates

## Out of Scope

- Batch duplication (duplicating multiple shortcuts at once)
- Duplicate with automatic key/command transformation (e.g., auto-increment key codes)
- Duplication across different configuration files
- Undo/redo for duplication operations
