# Tasks: Configuration Import/Export

**Input**: Design documents from `/specs/002-config-import-export/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are NOT explicitly requested in the feature specification. Test tasks are omitted - manual testing will be performed per quickstart.md.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Tauri project**: `src-tauri/src/` (Rust backend), `src/` (Svelte frontend)
- **Tests**: `src-tauri/tests/` (Rust), `tests/` (frontend if needed)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add dependencies and prepare for file dialog implementation

- [ ] T001 Add rfd dependency to src-tauri/Cargo.toml for native file dialogs
- [ ] T002 Verify Cargo.toml builds successfully with new dependency

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Extend data model that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T003 Add current_file_path field to ConfigFile struct in src-tauri/src/models/config_file.rs
- [ ] T004 Add default_file_path() function to ConfigFile impl in src-tauri/src/models/config_file.rs
- [ ] T005 Update ConfigFile::new() or from_parse() to initialize current_file_path in src-tauri/src/models/config_file.rs
- [ ] T006 Update current_file_path field in ConfigFile TypeScript interface in src/types.ts

**Checkpoint**: Data model extended - user story implementation can now begin

---

## Phase 3: User Story 1 - Import Configuration from Custom Location (Priority: P1) 🎯 MVP

**Goal**: Allow users to load skhd configuration files from any location via native file picker

**Independent Test**: Select test-skhdrc file from project root → All 21 shortcuts display → Header shows current file path

### Implementation for User Story 1

- [ ] T007 [US1] Extract existing load_config logic into reusable load_config_from_path helper in src-tauri/src/commands/config.rs
- [ ] T008 [US1] Implement import_config command with AsyncFileDialog in src-tauri/src/commands/config.rs
- [ ] T009 [US1] Configure file dialog filters for skhd file types in import_config
- [ ] T010 [US1] Handle user cancellation gracefully in import_config
- [ ] T011 [US1] Update ConfigState with imported config and current_file_path in import_config
- [ ] T012 [US1] Add import_config to command exports in src-tauri/src/commands/mod.rs
- [ ] T013 [US1] Register import_config in invoke_handler in src-tauri/src/lib.rs
- [ ] T014 [P] [US1] Add importConfig() TypeScript service function in src/services/tauri.ts
- [ ] T015 [P] [US1] Add handleImport() function to +page.svelte in src/routes/+page.svelte
- [ ] T016 [P] [US1] Add "Import..." button to header-actions in src/routes/+page.svelte
- [ ] T017 [P] [US1] Add btn-import styles to +page.svelte
- [ ] T018 [US1] Test: Verify cargo build succeeds
- [ ] T019 [US1] Test: Verify bun run typecheck passes
- [ ] T020 [US1] Manual test: Import test-skhdrc and verify all shortcuts display

**Checkpoint**: User Story 1 complete - users can import configurations from any location

---

## Phase 4: User Story 2 - Export Current Configuration (Priority: P2)

**Goal**: Allow users to export current configuration to any writable location

**Independent Test**: Make edits → Click Export → Save to Desktop → Verify exported file contains all changes

### Implementation for User Story 2

- [ ] T021 [US2] Create serialize_to_skhd helper function in src-tauri/src/commands/config.rs (reuse existing save logic)
- [ ] T022 [US2] Create validate_skhd_syntax helper using existing parser in src-tauri/src/commands/config.rs
- [ ] T023 [US2] Implement export_config command with AsyncFileDialog.save_file() in src-tauri/src/commands/config.rs
- [ ] T024 [US2] Add serialization and validation before file dialog in export_config
- [ ] T025 [US2] Implement atomic write using existing write_config_atomic pattern in export_config
- [ ] T026 [US2] Return exported file path for user confirmation in export_config
- [ ] T027 [US2] Add export_config to command exports in src-tauri/src/commands/mod.rs
- [ ] T028 [US2] Register export_config in invoke_handler in src-tauri/src/lib.rs
- [ ] T029 [P] [US2] Add exportConfig() TypeScript service function in src/services/tauri.ts
- [ ] T030 [P] [US2] Add handleExport() function to +page.svelte in src/routes/+page.svelte
- [ ] T031 [P] [US2] Add "Export..." button to header-actions in src/routes/+page.svelte
- [ ] T032 [P] [US2] Add btn-export styles to +page.svelte
- [ ] T033 [US2] Test: Verify cargo build succeeds
- [ ] T034 [US2] Test: Verify bun run typecheck passes
- [ ] T035 [US2] Manual test: Export configuration and verify file validity

**Checkpoint**: User Stories 1 AND 2 complete - import and export both functional

---

## Phase 5: User Story 3 - Reload from Default Location (Priority: P1)

**Goal**: Allow users to reload configuration from disk with unsaved changes warning

**Independent Test**: Make edits without saving → Click Reload → Confirm dialog appears → Confirm → Changes discarded

### Implementation for User Story 3

- [ ] T036 [P] [US3] Create ConfirmDialog.svelte component in src/components/ConfirmDialog.svelte
- [ ] T037 [P] [US3] Add modal backdrop and dialog styles to ConfirmDialog.svelte
- [ ] T038 [P] [US3] Add confirmation button styles (warning color for destructive action) to ConfirmDialog.svelte
- [ ] T039 [US3] Implement reload_config command in src-tauri/src/commands/config.rs
- [ ] T040 [US3] Read current_file_path from state in reload_config
- [ ] T041 [US3] Reuse load_config_from_path helper to reload in reload_config
- [ ] T042 [US3] Set is_modified = false after reload in reload_config
- [ ] T043 [US3] Add reload_config to command exports in src-tauri/src/commands/mod.rs (if not already exported)
- [ ] T044 [US3] Register reload_config in invoke_handler in src-tauri/src/lib.rs
- [ ] T045 [P] [US3] Add reloadConfig() TypeScript service function in src/services/tauri.ts
- [ ] T046 [US3] Add showReloadConfirm state to +page.svelte in src/routes/+page.svelte
- [ ] T047 [US3] Update existing handleReloadClick to check is_modified in src/routes/+page.svelte
- [ ] T048 [US3] Add performReload() function to +page.svelte
- [ ] T049 [US3] Add cancelReload() function to +page.svelte
- [ ] T050 [US3] Add ConfirmDialog component to template in src/routes/+page.svelte
- [ ] T051 [US3] Test: Verify cargo build succeeds
- [ ] T052 [US3] Test: Verify bun run typecheck passes
- [ ] T053 [US3] Manual test: Make edits, click Reload, verify confirmation dialog and reload behavior

**Checkpoint**: All user stories (US1, US2, US3) are complete and independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T054 [P] Run full Rust test suite: cargo test in src-tauri/
- [ ] T055 [P] Run TypeScript type checking: bun run typecheck
- [ ] T056 [P] Run Rust linting: cargo clippy --all-targets --all-features in src-tauri/
- [ ] T057 [P] Run Rust formatting check: cargo fmt --check in src-tauri/
- [ ] T058 Manual test: Complete quickstart.md manual testing checklist
- [ ] T059 Manual test: Verify round-trip integrity (import → export → import → diff)
- [ ] T060 Manual test: Test with large config file (1000+ lines) for performance
- [ ] T061 Manual test: Test permission errors (read-only directory export)
- [ ] T062 Code review: Verify all error messages are user-friendly
- [ ] T063 Code review: Verify atomic write pattern used for all file operations
- [ ] T064 Update CLAUDE.md agent context if new patterns emerged during implementation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3, 4, 5)**: All depend on Foundational phase completion
  - User stories can proceed in parallel if desired (independent implementations)
  - Or sequentially in priority order: US1 (P1) → US3 (P1) → US2 (P2)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories (export is independent of import)
- **User Story 3 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories (reload uses existing infrastructure)

**Independence Validation**: Each user story can be tested independently:
- US1 alone: Import works without export or reload
- US2 alone: Export works without import or reload
- US3 alone: Reload works without import or export

### Within Each User Story

- Backend commands before frontend integration
- TypeScript types before service functions
- Service functions before UI components
- UI components before manual testing

### Parallel Opportunities

**Within User Story 1**:
- T014 (TypeScript service) + T015-T017 (UI components) can run in parallel

**Within User Story 2**:
- T029 (TypeScript service) + T030-T032 (UI components) can run in parallel

**Within User Story 3**:
- T036-T038 (ConfirmDialog component) can run in parallel with T039-T044 (backend command)
- T045 (TypeScript service) can run after backend is complete

**Across User Stories**:
- Once Foundational (Phase 2) is complete, all three user stories (US1, US2, US3) can be implemented in parallel

**Polish Phase**:
- T054-T057 (all test/lint commands) can run in parallel

---

## Parallel Example: User Story 1

```bash
# After T013 completes (backend registered):
# Launch frontend tasks in parallel:
Task: "Add importConfig() TypeScript service function in src/services/tauri.ts"
Task: "Add handleImport() function to +page.svelte"
Task: "Add 'Import...' button to header-actions in src/routes/+page.svelte"
Task: "Add btn-import styles to +page.svelte"
```

## Parallel Example: Across User Stories

```bash
# After Phase 2 (Foundational) completes:
# Launch all user stories in parallel:
Story 1: "Implement import_config command" (T007-T020)
Story 2: "Implement export_config command" (T021-T035)
Story 3: "Implement reload_config command" (T036-T053)
```

---

## Implementation Strategy

### MVP First (Recommended)

1. Complete Phase 1: Setup (T001-T002)
2. Complete Phase 2: Foundational (T003-T006) - CRITICAL
3. Complete Phase 3: User Story 1 (T007-T020)
4. **STOP and VALIDATE**: Test import functionality independently
5. Demo import feature (MVP with single P1 story)

### Full Feature Delivery

1. Complete Setup + Foundational → Foundation ready
2. Implement User Story 1 → Test independently (Import working)
3. Implement User Story 3 → Test independently (Reload working)
4. Implement User Story 2 → Test independently (Export working)
5. Complete Polish phase → All features validated
6. Final integration testing across all stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T006)
2. Once Foundational is done:
   - Developer A: User Story 1 - Import (T007-T020)
   - Developer B: User Story 3 - Reload (T036-T053)
   - Developer C: User Story 2 - Export (T021-T035)
3. Stories complete independently, integrate naturally
4. Team completes Polish phase together (T054-T064)

---

## Notes

- [P] tasks = different files, no dependencies, can run in parallel
- [Story] label (US1, US2, US3) maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group of related tasks
- Stop at any checkpoint to validate story independently
- Manual testing per quickstart.md is required (no automated test generation requested)
- Constitutional compliance verified in plan.md (all principles satisfied)
- Performance target: All file operations <200ms (validated during manual testing)

---

## Summary

- **Total Tasks**: 64 tasks
- **User Story 1 (Import)**: 14 tasks (T007-T020)
- **User Story 2 (Export)**: 15 tasks (T021-T035)
- **User Story 3 (Reload)**: 18 tasks (T036-T053)
- **Setup + Foundational**: 6 tasks (T001-T006)
- **Polish**: 11 tasks (T054-T064)
- **Parallel Opportunities**: 13 tasks can run in parallel across phases
- **MVP Scope**: Phase 1-3 (User Story 1 only) = 22 tasks
- **Independent Testing**: Each user story has clear test criteria and can be validated independently
