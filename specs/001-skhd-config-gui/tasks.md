# Tasks: skhd Configuration GUI

**Input**: Design documents from `/specs/001-skhd-config-gui/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included as this is a configuration safety-critical application (Constitution II requires robust testing).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Tauri structure**: `src-tauri/` for Rust backend, `src/` for Svelte frontend
- **Tests**: `src-tauri/tests/` for Rust, `src/__tests__/` for frontend
- Paths are absolute to repository root

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Initialize Tauri project with Svelte + TypeScript using `npm create tauri-app@latest`
- [x] T002 Configure tauri.conf.json for macOS 11+ target and Universal binary in src-tauri/tauri.conf.json
- [x] T003 [P] Add Rust dependencies to src-tauri/Cargo.toml (tauri, serde, pest, tempfile, sha2, uuid, chrono, notify)
- [x] T004 [P] Add frontend dependencies (vitest, @testing-library/svelte, @testing-library/jest-dom) via npm
- [x] T005 [P] Configure file system permissions in src-tauri/tauri.conf.json for $HOME/.config/skhd/\* access
- [x] T006 [P] Setup ESLint and Prettier for frontend in package.json
- [x] T007 [P] Create project directory structure per plan.md (src-tauri/src/commands, models, services, utils, tests)
- [x] T008 [P] Create frontend directory structure (src/components, services, hooks, styles, **tests**)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Parser Foundation

- [x] T009 Create pest grammar file for skhd syntax in src-tauri/src/parser/grammar.pest
- [x] T010 Implement pest parser infrastructure in src-tauri/src/parser/mod.rs
- [x] T011 [P] Define AST (Abstract Syntax Tree) structures in src-tauri/src/parser/ast.rs
- [x] T012 Implement skhd config parsing function using pest in src-tauri/src/parser/mod.rs
- [x] T013 Add parser error handling and line number tracking in src-tauri/src/parser/mod.rs

### Data Models

- [x] T014 [P] Create Keyboard Shortcut model with serde in src-tauri/src/models/shortcut.rs
- [x] T015 [P] Create Configuration File model with serde in src-tauri/src/models/config.rs
- [x] T016 [P] Create ParseError model in src-tauri/src/models/mod.rs
- [x] T017 [P] Create Backup model in src-tauri/src/models/backup.rs

### Core Services

- [x] T018 Implement path resolution utility (~/.config/skhd/skhdrc expansion) in src-tauri/src/utils/path.rs
- [x] T019 Implement atomic file write service using tempfile in src-tauri/src/services/file_io.rs
- [x] T020 Implement backup creation service with SHA-256 checksums in src-tauri/src/services/backup.rs
- [x] T021 Implement validation service for shortcuts and config in src-tauri/src/services/validation.rs

### Frontend Type Definitions

- [x] T022 [P] Create TypeScript type definitions matching Rust models in src/types.ts
- [x] T023 [P] Create Tauri API wrapper service in src/services/tauri.ts

### Testing Infrastructure

- [x] T024 [P] Setup Vitest configuration for frontend in vitest.config.ts
- [x] T025 [P] Create test fixtures directory with sample skhd configs in src-tauri/tests/fixtures/
- [x] T026 [P] Configure cargo test for Rust in src-tauri/Cargo.toml

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - View Existing Configuration (Priority: P1) 🎯 MVP

**Goal**: Users can view their skhd configuration in a readable format within the application

**Independent Test**: Launch app, verify existing config file loads and displays correctly. App delivers value as a config viewer even without editing.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T027 [P] [US1] Unit test for parse_config with valid skhd syntax in src-tauri/tests/unit/parser_tests.rs
- [ ] T028 [P] [US1] Unit test for parse_config with invalid syntax in src-tauri/tests/unit/parser_tests.rs
- [ ] T029 [P] [US1] Unit test for parse_config with comments preservation in src-tauri/tests/unit/parser_tests.rs
- [ ] T030 [P] [US1] Integration test for load_config with existing file in src-tauri/tests/integration/config_lifecycle.rs
- [ ] T031 [P] [US1] Integration test for load_config with missing file in src-tauri/tests/integration/config_lifecycle.rs
- [ ] T032 [P] [US1] Integration test for load_config with permission errors in src-tauri/tests/integration/config_lifecycle.rs

### Implementation for User Story 1

- [ ] T033 [US1] Implement load_config Tauri command in src-tauri/src/commands/config.rs (FR-001)
- [ ] T034 [US1] Implement config serialization back to skhd format in src-tauri/src/commands/config.rs (for round-trip verification)
- [ ] T035 [US1] Register load_config command in src-tauri/src/main.rs
- [ ] T036 [P] [US1] Create ShortcutItem component to display single shortcut in src/components/ShortcutItem.svelte
- [ ] T037 [P] [US1] Create ShortcutList component to display all shortcuts in src/components/ShortcutList.svelte (FR-002)
- [ ] T038 [P] [US1] Create EmptyState component for no config scenario in src/components/EmptyState.svelte
- [ ] T039 [P] [US1] Create ErrorDisplay component for parse errors in src/components/ErrorDisplay.svelte (FR-010)
- [ ] T040 [US1] Implement App.svelte to load and display config on mount
- [ ] T041 [US1] Add error handling and loading states to App.svelte
- [ ] T042 [P] [US1] Create basic CSS styling for native macOS feel in src/styles/main.css
- [ ] T043 [P] [US1] Frontend test for ShortcutList component rendering in src/**tests**/components/ShortcutList.test.ts
- [ ] T044 [P] [US1] Frontend test for EmptyState component in src/**tests**/components/EmptyState.test.ts
- [ ] T045 [US1] Manual test: Launch app and verify config loads within 2 seconds (SC-001, SC-007)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Users can view their skhd config in the GUI.

---

## Phase 4: User Story 2 - Edit Keyboard Shortcuts (Priority: P2)

**Goal**: Users can add, edit, and delete keyboard shortcuts through the GUI and save changes safely

**Independent Test**: Open app, make changes (add/edit/delete shortcuts), save, and verify changes persisted correctly to config file with backup created.

### Tests for User Story 2

- [ ] T046 [P] [US2] Unit test for validate_shortcut with valid input in src-tauri/tests/unit/validation_tests.rs
- [ ] T047 [P] [US2] Unit test for validate_shortcut with duplicate detection in src-tauri/tests/unit/validation_tests.rs
- [ ] T048 [P] [US2] Unit test for validate_shortcut with invalid modifiers/keys in src-tauri/tests/unit/validation_tests.rs
- [ ] T049 [P] [US2] Integration test for save_config with backup creation in src-tauri/tests/integration/backup_restore.rs
- [ ] T050 [P] [US2] Integration test for save_config atomic write behavior in src-tauri/tests/integration/config_lifecycle.rs
- [ ] T051 [P] [US2] Integration test for save_config validation failure in src-tauri/tests/integration/config_lifecycle.rs

### Implementation for User Story 2

- [ ] T052 [US2] Implement validate_shortcut Tauri command in src-tauri/src/commands/config.rs (FR-006, FR-008)
- [ ] T053 [US2] Implement save_config Tauri command with backup logic in src-tauri/src/commands/config.rs (FR-007)
- [ ] T054 [US2] Register validate_shortcut and save_config commands in src-tauri/src/main.rs
- [ ] T055 [P] [US2] Create ShortcutEditor component (add/edit form) in src/components/ShortcutEditor.svelte (FR-003, FR-004)
- [ ] T056 [P] [US2] Create ModifierSelector component for keyboard modifier selection in src/components/ModifierSelector.svelte (FR-011)
- [ ] T057 [P] [US2] Create KeyInput component for key capture in src/components/KeyInput.svelte
- [ ] T058 [US2] Add real-time validation to ShortcutEditor using validate_shortcut command
- [ ] T059 [US2] Implement add shortcut functionality in ShortcutList component (FR-003)
- [ ] T060 [US2] Implement edit shortcut functionality in ShortcutItem component (FR-004)
- [ ] T061 [US2] Implement delete shortcut functionality in ShortcutItem component (FR-005)
- [ ] T062 [US2] Create Svelte store for config state management in src/stores/config.ts
- [ ] T063 [US2] Implement undo/redo stack in config store in src/stores/config.ts (FR-009)
- [ ] T064 [US2] Add save button and save logic to App.svelte
- [ ] T065 [US2] Implement unsaved changes detection and prompt before close in App.svelte (FR-014)
- [ ] T066 [P] [US2] Frontend test for ShortcutEditor component in src/**tests**/components/ShortcutEditor.test.ts
- [ ] T067 [P] [US2] Frontend test for undo/redo functionality in src/**tests**/stores/config.test.ts
- [ ] T068 [US2] Manual test: Complete full edit workflow (add → edit → save) in under 30 seconds (SC-002)
- [ ] T069 [US2] Manual test: Verify backup created before save with correct timestamp (SC-009)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Users can view AND edit their skhd config safely.

---

## Phase 5: User Story 3 - Test Shortcuts Before Saving (Priority: P3)

**Goal**: Users can test keyboard shortcuts before committing to active configuration

**Independent Test**: Create/modify shortcut, test it, verify it executes without affecting active config file.

### Implementation for User Story 3

- [ ] T070 [P] [US3] Implement check_skhd_running Tauri command in src-tauri/src/commands/config.rs
- [ ] T071 [US3] Register check_skhd_running command in src-tauri/src/main.rs
- [ ] T072 [P] [US3] Create TestShortcut component with test button in src/components/TestShortcut.svelte
- [ ] T073 [US3] Implement shortcut testing logic (keyboard event listening) in TestShortcut component
- [ ] T074 [US3] Add test feedback UI (success/failure indicators) to TestShortcut component
- [ ] T075 [US3] Integrate TestShortcut component into ShortcutEditor
- [ ] T076 [P] [US3] Frontend test for TestShortcut component in src/**tests**/components/TestShortcut.test.ts
- [ ] T077 [US3] Manual test: Test shortcut triggers correctly and shows feedback

**Checkpoint**: All user stories should now be independently functional. Full feature set complete.

---

## Phase 6: Advanced Features & Polish

**Purpose**: Additional requirements and cross-cutting concerns

### Search and Filter (FR-013)

- [ ] T078 [P] Create SearchBar component in src/components/SearchBar.svelte
- [ ] T079 Implement search/filter logic in ShortcutList component
- [ ] T080 [P] Frontend test for SearchBar component in src/**tests**/components/SearchBar.test.ts

### File Watching (FR-015, FR-016, FR-017)

- [ ] T081 Implement watch_config_file Tauri command using notify crate in src-tauri/src/commands/config.rs
- [ ] T082 Implement get_permissions_status Tauri command in src-tauri/src/commands/config.rs
- [ ] T083 Register watch_config_file and get_permissions_status in src-tauri/src/main.rs
- [ ] T084 Add file watcher setup to App.svelte with reload prompt on external changes
- [ ] T085 Implement permissions check on app launch with user messaging
- [ ] T086 [P] Add read-only mode UI state when permissions unavailable

### Backup Management (Future Enhancement)

- [ ] T087 [P] Implement create_backup Tauri command in src-tauri/src/commands/config.rs
- [ ] T088 [P] Implement list_backups Tauri command in src-tauri/src/commands/config.rs
- [ ] T089 [P] Implement restore_backup Tauri command in src-tauri/src/commands/config.rs
- [ ] T090 Register backup commands in src-tauri/src/main.rs
- [ ] T091 [P] Create BackupManager component in src/components/BackupManager.svelte
- [ ] T092 [P] Integration test for backup restoration in src-tauri/tests/integration/backup_restore.rs

### Code Quality & Performance

- [ ] T093 [P] Run cargo clippy and fix all warnings in src-tauri/
- [ ] T094 [P] Run cargo fmt to format Rust code in src-tauri/
- [ ] T095 [P] Run ESLint and Prettier on frontend code in src/
- [ ] T096 [P] Measure and optimize bundle size (target <20MB)
- [ ] T097 [P] Run cargo tarpaulin for code coverage report (target >80% for parser/file_io)
- [ ] T098 [P] Performance test: Verify <100ms parsing for 1000-line config (SC-003)
- [ ] T099 [P] Performance test: Verify <50MB memory usage during editing (SC-008)

### Documentation & Distribution

- [ ] T100 [P] Create README.md with setup and usage instructions
- [ ] T101 [P] Add keyboard shortcuts reference documentation
- [ ] T102 [P] Create release build configuration for Universal binary
- [ ] T103 [P] Setup code signing for macOS distribution (when ready)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion - No dependencies on other stories
- **User Story 2 (Phase 4)**: Depends on Foundational phase completion - Builds on US1 but is independently testable
- **User Story 3 (Phase 5)**: Depends on Foundational phase completion - Integrates with US2 but is independently testable
- **Advanced Features (Phase 6)**: Depends on desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Benefits from US1 existing but technically independent
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Integrates with US2 editor but still independently testable

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Models before services
- Services before Tauri commands
- Tauri commands before frontend components
- Core components before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T003, T004, T005, T006, T007, T008)
- All Foundational models marked [P] can run in parallel (T014, T015, T016, T017)
- All Foundational frontend tasks marked [P] can run in parallel (T022, T023, T024, T025, T026)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- All components within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all unit tests for User Story 1 together:
Task: T027 - Unit test for parse_config with valid skhd syntax
Task: T028 - Unit test for parse_config with invalid syntax
Task: T029 - Unit test for parse_config with comments preservation
Task: T030 - Integration test for load_config with existing file
Task: T031 - Integration test for load_config with missing file
Task: T032 - Integration test for load_config with permission errors

# After tests, launch all frontend components together:
Task: T036 - Create ShortcutItem component
Task: T037 - Create ShortcutList component
Task: T038 - Create EmptyState component
Task: T039 - Create ErrorDisplay component
Task: T042 - Create basic CSS styling

# Launch all frontend tests together:
Task: T043 - Frontend test for ShortcutList
Task: T044 - Frontend test for EmptyState
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (T027-T045)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Can deploy/demo as config viewer

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add Advanced Features → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (T027-T045)
   - Developer B: User Story 2 (T046-T069)
   - Developer C: User Story 3 (T070-T077)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Run cargo clippy, cargo fmt, and npm run lint before committing
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Success Criteria Validation

After implementation, verify each success criterion from spec.md:

- **SC-001**: Users can view their complete skhd configuration within 2 seconds → Validate with T045
- **SC-002**: Users can add or edit a keyboard shortcut and save in under 30 seconds → Validate with T068
- **SC-003**: 100% of valid skhd configurations are parsed correctly → Validate with T027-T029
- **SC-004**: Zero user-reported incidents of configuration file corruption → Ensured by T050 (atomic writes)
- **SC-005**: Application detects 100% of duplicate keyboard combinations → Validate with T047
- **SC-006**: 95% of users can complete first edit without documentation → Manual usability testing
- **SC-007**: Application launch time is under 2 seconds → Validate with T045
- **SC-008**: Application memory footprint remains under 50MB → Validate with T099
- **SC-009**: All configuration changes are backed up → Validate with T069

---

## Total Task Count: 103 tasks

- **Setup**: 8 tasks
- **Foundational**: 18 tasks
- **User Story 1 (P1)**: 19 tasks (6 tests + 13 implementation)
- **User Story 2 (P2)**: 24 tasks (6 tests + 18 implementation)
- **User Story 3 (P3)**: 8 tasks (0 explicit tests, testing integrated)
- **Advanced Features**: 26 tasks

**Parallel Opportunities**: 47 tasks marked [P] can run in parallel within their phase

**Suggested MVP Scope**: Phase 1 + Phase 2 + Phase 3 (User Story 1) = 45 tasks for a functional config viewer
