# Tasks: Shortcut Duplicate

**Input**: Design documents from `/specs/003-shortcut-duplicate/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Manual UI testing only (no automated tests requested in specification)

**Organization**: Tasks grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Desktop Application**: Tauri v2 structure
- **Frontend**: `src/` (Svelte 5 + TypeScript)
- **Backend**: `src-tauri/src/` (Rust - no changes required)
- **Tests**: Manual UI testing (no automated test files)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify project is ready for implementation

**Note**: Project structure already exists. This phase validates readiness.

- [x] T001 Verify existing project structure matches plan.md (src/components/, src/routes/, src/types.ts)
- [x] T002 Verify development server runs successfully (bun run dev)
- [x] T003 [P] Verify TypeScript compilation passes (bun run typecheck)
- [x] T004 [P] Verify linter passes (bun run lint or eslint)

**Checkpoint**: Development environment ready for feature implementation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

**Note**: This feature has NO foundational requirements. All infrastructure already exists (ShortcutForm, validation, save flow).

- [x] T005 Read and understand existing ShortcutItem.svelte component (src/components/ShortcutItem.svelte:1-180)
- [x] T006 Read and understand existing ShortcutList.svelte component (src/components/ShortcutList.svelte:1-148)
- [x] T007 Read and understand existing +page.svelte handlers (src/routes/+page.svelte:100-180)
- [x] T008 Verify ShortcutForm supports pre-population by examining Props interface (src/components/ShortcutForm.svelte:1-30)

**Checkpoint**: Foundation ready - all existing components understood, user story implementation can begin

---

## Phase 3: User Story 1 - Quick Shortcut Duplication (Priority: P1) 🎯 MVP

**Goal**: Users can duplicate an existing shortcut and edit it to create similar shortcuts with slight variations, reducing data entry time from 45+ seconds to under 15 seconds.

**Independent Test**: Duplicate a shortcut → verify form opens with all fields pre-filled → modify key field → save → verify both original and duplicate exist as separate entries in the list.

**Why P1**: Core value of the feature - delivers immediate, measurable time savings. Independently testable without other stories.

### Implementation for User Story 1

- [x] T009 [P] [US1] Add onDuplicate prop to ShortcutItem Props interface in src/components/ShortcutItem.svelte:4-9
- [x] T010 [P] [US1] Add Duplicate button to ShortcutItem template in src/components/ShortcutItem.svelte:37-47 (between Test and Edit buttons)
- [x] T011 [P] [US1] Add Duplicate button styling (.btn-duplicate class) in src/components/ShortcutItem.svelte:154-180
- [x] T012 [P] [US1] Add onDuplicate prop to ShortcutList Props interface in src/components/ShortcutList.svelte:5-13
- [x] T013 [P] [US1] Pass onDuplicate to ShortcutItem in ShortcutList template in src/components/ShortcutList.svelte:41-43
- [x] T014 [US1] Add getNextLineNumber() helper function in src/routes/+page.svelte (~line 100, after existing helper functions)
- [x] T015 [US1] Add handleDuplicate() handler function in src/routes/+page.svelte (~line 110, after getNextLineNumber)
- [x] T016 [US1] Pass onDuplicate={handleDuplicate} to ShortcutList in src/routes/+page.svelte (~line 360)
- [ ] T017 [US1] Manual test: Duplicate shortcut with all fields → verify form pre-filled
- [ ] T018 [US1] Manual test: Save duplicated shortcut without changes → verify validation error "Duplicate key combination"
- [ ] T019 [US1] Manual test: Modify key and save → verify new shortcut created, original unchanged

**Checkpoint**: User Story 1 COMPLETE - Users can duplicate shortcuts and modify fields. This is a shippable MVP.

---

## Phase 4: User Story 2 - Duplicate with Modified Key (Priority: P2)

**Goal**: Enhance duplication workflow for the most common use case - changing only the key while keeping the command the same (e.g., mapping same command to multiple key combinations).

**Independent Test**: Duplicate a shortcut → change only the key field → save → verify new shortcut created with new key but same command. Test validation prevents duplicate key combinations.

**Why P2**: Builds on P1 functionality. Common use case but not essential for basic duplication. Tests validation edge cases.

**Note**: This story is already implemented by User Story 1! No additional tasks required. The validation and save flow automatically handle this scenario.

### Validation Testing for User Story 2

- [ ] T020 [US2] Manual test: Duplicate shortcut → change key from "a" to "b" → save → verify new shortcut with key "b"
- [ ] T021 [US2] Manual test: Duplicate shortcut → set key to existing combination → verify error message shown
- [ ] T022 [US2] Manual test: Create 3 duplicates with different keys (a, b, c) → verify all appear in list sorted by line number
- [ ] T023 [US2] Manual test: Duplicate shortcut with existing key but different mode → verify saves successfully (modes isolate key bindings)

**Checkpoint**: User Story 2 VALIDATED - Key modification workflow tested and confirmed working.

---

## Phase 5: User Story 3 - Duplicate with Modified Command (Priority: P3)

**Goal**: Support advanced use cases where users want the same key for different contexts (different modes) or create similar commands with different parameters.

**Independent Test**: Duplicate a shortcut → change the command field → save → verify new shortcut created with modified command.

**Why P3**: Less common than P1/P2 but completes the duplication workflow. Advanced use case.

**Note**: This story is already implemented by User Story 1! No additional tasks required. The form pre-population handles all fields including command and comment.

### Validation Testing for User Story 3

- [ ] T024 [US3] Manual test: Duplicate shortcut → change command from "open -a Terminal" to "open -a iTerm" → save → verify new command
- [ ] T025 [US3] Manual test: Duplicate shortcut with comments → verify comment field pre-filled → modify comment → verify saved
- [ ] T026 [US3] Manual test: Duplicate shortcut → change mode field → save → verify mode updated independently

**Checkpoint**: User Story 3 VALIDATED - Command modification workflow tested and confirmed working.

---

## Phase 6: Edge Cases & Polish

**Purpose**: Validate edge cases and improve user experience

### Edge Case Testing

- [ ] T027 [P] Manual test: Duplicate shortcut then click Cancel → verify no new shortcut created, original unchanged
- [ ] T028 [P] Manual test: Duplicate when config has unsaved changes → verify duplicate works normally (in-memory operation)
- [ ] T029 [P] Manual test: Duplicate with exactly matching fields as existing shortcut → verify validation prevents save
- [ ] T030 [P] Manual test: Duplicate the last shortcut in list → verify new duplicate appears at end with correct line number
- [ ] T031 [P] Manual test: Empty config (0 shortcuts) → create one → duplicate it → verify line_number = 2
- [ ] T032 [P] Manual test: Config with gaps in line numbers (1, 5, 10) → duplicate → verify line_number = 11 (max + 1)

### UI/UX Polish

- [ ] T033 [P] Verify Duplicate button color matches macOS secondary gray (#8e8e93) in light mode
- [ ] T034 [P] Verify Duplicate button hover state darkens correctly
- [ ] T035 [P] Test dark mode: verify button colors adapt appropriately
- [ ] T036 [P] Test keyboard navigation: Tab through buttons → verify focus order Test → Duplicate → Edit → Delete
- [ ] T037 [P] Test accessibility: verify screen reader announces "Duplicate button" correctly
- [ ] T038 Remove console.log debugging statements if present (or keep if useful for debugging)

### Performance Validation

- [ ] T039 Test form opening speed: Duplicate → measure time to form appearing (target: <16ms)
- [ ] T040 Test with large config (50+ shortcuts): Duplicate → verify UI remains responsive
- [ ] T041 Verify getNextLineNumber() performance with 100 shortcuts (target: <5ms)

### Documentation

- [ ] T042 Update README.md with Duplicate feature in Features section
- [ ] T043 Add screenshot or GIF demonstrating duplicate workflow (optional but recommended)

**Checkpoint**: All edge cases tested, UI polished, documentation updated. Feature ready for final review.

---

## Phase 7: Final Validation & Deployment

**Purpose**: Comprehensive validation before merging to main

### Type Checking & Linting

- [ ] T044 Run TypeScript type checking (bun run typecheck) → verify no errors
- [ ] T045 Run linter (bun run lint) → verify no errors
- [ ] T046 Run formatter if applicable (bun run format or biome format) → verify code style

### Integration Testing

- [ ] T047 Full workflow test: Load config → Duplicate → Modify → Save → Save Changes → Reload app → Verify persisted
- [ ] T048 Multi-duplicate test: Duplicate same shortcut 5 times with different keys → Save all → Verify all persist
- [ ] T049 Independence test: Duplicate shortcut → Delete original → Verify duplicate unaffected

### Code Review Preparation

- [ ] T050 Review all modified files for code quality (ShortcutItem.svelte, ShortcutList.svelte, +page.svelte)
- [ ] T051 Verify all tasks completed (checklist above)
- [ ] T052 Verify no temporary files or debugging code left in codebase

### Commit & Merge

- [ ] T053 Stage all changes (git add src/components/ src/routes/)
- [ ] T054 Create commit with descriptive message following project conventions
- [ ] T055 Push feature branch (git push origin 003-shortcut-duplicate)
- [ ] T056 Create pull request or merge to main (if solo project)

**Checkpoint**: Feature complete, tested, and ready for production use.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - Validates existing infrastructure
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User Story 1 (P1) MUST be completed first (core functionality)
  - User Story 2 (P2) and 3 (P3) are validation-only (already implemented by US1)
- **Edge Cases (Phase 6)**: Depends on User Story 1 completion (validates P1 implementation)
- **Final Validation (Phase 7)**: Depends on all previous phases

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Depends on User Story 1 - Validates key modification workflow
- **User Story 3 (P3)**: Depends on User Story 1 - Validates command modification workflow

**Key Insight**: US1 implements the complete feature. US2 and US3 are validation phases confirming the implementation works for their specific use cases.

### Within Each User Story

**User Story 1 (Implementation)**:
1. Component prop additions (T009, T012) - Can run in parallel
2. Template changes (T010, T013) - Can run in parallel
3. Styling (T011) - Can run in parallel with above
4. Handler functions (T014, T015) - Sequential (getNextLineNumber before handleDuplicate)
5. Wire up (T016) - Depends on handler functions existing
6. Manual testing (T017-T019) - After implementation complete

**User Stories 2 & 3 (Validation)**:
- All testing tasks can run in parallel (different test scenarios)

### Parallel Opportunities

**Phase 1 (Setup)**:
- All tasks can run in parallel (T001-T004)

**Phase 2 (Foundational)**:
- All reading tasks can run in parallel (T005-T008)

**Phase 3 (User Story 1) - Implementation**:
```bash
# Parallel group 1: Component modifications
Task: "T009 Add onDuplicate prop to ShortcutItem"
Task: "T010 Add Duplicate button to template"
Task: "T011 Add button styling"
Task: "T012 Add onDuplicate prop to ShortcutList"
Task: "T013 Pass onDuplicate to ShortcutItem"

# Sequential: Handler functions
Task: "T014 Add getNextLineNumber()"
Task: "T015 Add handleDuplicate()"

# Sequential: Wire up
Task: "T016 Pass handler to ShortcutList"

# Sequential: Testing (after implementation)
Task: "T017-T019 Manual tests"
```

**Phase 4-5 (User Stories 2-3)**:
- All testing tasks can run in parallel

**Phase 6 (Edge Cases)**:
- All testing tasks (T027-T032) can run in parallel
- All UI/UX tasks (T033-T038) can run in parallel
- All performance tasks (T039-T041) can run in parallel

**Phase 7 (Final)**:
- Validation tasks (T044-T046) can run in parallel
- Integration tests (T047-T049) should run sequentially
- Commit tasks (T053-T056) must run sequentially

---

## Parallel Example: User Story 1 Implementation

```bash
# Launch all component modifications together:
Task: "Add onDuplicate prop to ShortcutItem Props interface in src/components/ShortcutItem.svelte"
Task: "Add Duplicate button to ShortcutItem template in src/components/ShortcutItem.svelte"
Task: "Add Duplicate button styling in src/components/ShortcutItem.svelte"
Task: "Add onDuplicate prop to ShortcutList Props interface in src/components/ShortcutList.svelte"
Task: "Pass onDuplicate to ShortcutItem in ShortcutList template in src/components/ShortcutList.svelte"

# Wait for completion, then sequential:
Task: "Add getNextLineNumber() helper function in src/routes/+page.svelte"
Task: "Add handleDuplicate() handler function in src/routes/+page.svelte"
Task: "Pass onDuplicate={handleDuplicate} to ShortcutList in src/routes/+page.svelte"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only) - RECOMMENDED

1. Complete Phase 1: Setup (T001-T004) - ~10 minutes
2. Complete Phase 2: Foundational (T005-T008) - ~15 minutes
3. Complete Phase 3: User Story 1 (T009-T019) - ~1.5 hours
4. **STOP and VALIDATE**: Test User Story 1 independently
5. If working correctly, this is a shippable MVP!

**Total MVP Time**: ~2 hours

**MVP Deliverable**: Users can duplicate shortcuts and edit all fields, reducing data entry time by 60%+

### Incremental Delivery

1. Complete Setup + Foundational → ~25 minutes
2. Add User Story 1 → Test independently → **Deploy MVP** (2 hours total)
3. Add User Story 2 validation → Confirm key modification works → 30 minutes
4. Add User Story 3 validation → Confirm command modification works → 30 minutes
5. Add Edge Cases & Polish → Comprehensive validation → 1 hour
6. Final validation and merge → 30 minutes

**Total Time**: ~4-5 hours for complete, polished feature

### Single Developer Strategy (Recommended)

**Day 1 (MVP)**:
1. Morning: Complete Setup + Foundational + User Story 1 implementation (T001-T016)
2. Afternoon: Complete User Story 1 testing (T017-T019)
3. **Result**: Working duplicate feature ready to use

**Day 2 (Polish - Optional)**:
1. Morning: User Story 2 & 3 validation (T020-T026)
2. Afternoon: Edge cases, polish, documentation (T027-T043)
3. Evening: Final validation and merge (T044-T056)

### Parallel Team Strategy (If Multiple Developers)

**Not applicable** - This is a small feature (~50 lines of code across 3 files). Single developer is more efficient than coordination overhead.

---

## Task Summary

**Total Tasks**: 56 tasks
- Phase 1 (Setup): 4 tasks (~10 min)
- Phase 2 (Foundational): 4 tasks (~15 min)
- Phase 3 (User Story 1 - P1): 11 tasks (~1.5 hours) **← MVP**
- Phase 4 (User Story 2 - P2): 4 tasks (~30 min)
- Phase 5 (User Story 3 - P3): 3 tasks (~30 min)
- Phase 6 (Edge Cases & Polish): 16 tasks (~1.5 hours)
- Phase 7 (Final Validation): 14 tasks (~30 min)

**Parallel Opportunities**: 35 tasks marked [P] (62% parallelizable)

**MVP Scope**: Phases 1-3 only (19 tasks, ~2 hours)

**Full Feature**: All phases (56 tasks, ~4-5 hours)

**Independent Test Criteria**:
- US1: Duplicate → modify → save → verify both exist ✅
- US2: Duplicate → change key → save → verify validation works ✅
- US3: Duplicate → change command → save → verify independence ✅

---

## Notes

- [P] tasks = different files or independent test scenarios, no dependencies
- [Story] label maps task to specific user story for traceability
- User Story 1 implements the complete feature - US2 and US3 validate specific workflows
- All testing is manual (no automated tests requested in spec)
- Commit after logical groups (e.g., after T016, after T019, after Phase 6)
- Stop at Phase 3 checkpoint for MVP validation
- Feature has zero backend changes - all frontend UI modifications
- Reuses 100% of existing validation, form, and save infrastructure
