# Tasks: Command and Application Picker for Hotkey Setup

**Input**: Design documents from `/specs/007-hotkey-command-picker/`
**Prerequisites**: plan.md, spec.md (user stories), research.md, data-model.md, contracts/tauri-commands.md

**Tests**: Test coverage required per Constitution Check (>80% for Rust backend logic). Tests are included in this task breakdown.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Tauri desktop app**: `src-tauri/src/` for Rust backend, `src/` for Svelte frontend
- **Tests**: `src-tauri/tests/` for Rust tests, `src/__tests__/` for frontend tests

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency setup

- [X] T001 Add `plist = "1.6"` dependency to src-tauri/Cargo.toml for Info.plist parsing
- [X] T002 [P] Create src-tauri/src/data/ directory for embedded JSON templates
- [X] T003 [P] Create src/components/pickers/ directory for picker components
- [X] T004 [P] Create src/services/ directory for frontend service layer
- [X] T005 Verify cargo build succeeds with new plist dependency

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core models and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 [P] Create Application model in src-tauri/src/models/application.rs with Serialize/Deserialize
- [X] T007 [P] Create CommandTemplate model in src-tauri/src/models/command_template.rs
- [X] T008 [P] Create CommandParameter model in src-tauri/src/models/command_template.rs
- [X] T009 [P] Create CommandCategory model in src-tauri/src/models/command_category.rs
- [X] T010 Export all models in src-tauri/src/models/mod.rs
- [X] T011 [P] Add TypeScript interfaces for Application in src/types.ts
- [X] T012 [P] Add TypeScript interfaces for CommandTemplate, CommandParameter, CommandCategory in src/types.ts
- [X] T013 Create path_validator service with escape_shell_path function in src-tauri/src/services/path_validator.rs
- [X] T014 Write unit tests for escape_shell_path in src-tauri/src/services/path_validator.rs (test spaces, quotes, special chars)
- [X] T015 Run cargo test to verify path_validator tests pass

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Browse and Select Applications (Priority: P1) 🎯 MVP

**Goal**: Users can browse installed macOS applications with icons and search, and select them to generate launch commands

**Independent Test**: Open hotkey editor, click "Browse Applications" button, search for an app (e.g., "Safari"), select it, verify `open -a "Safari"` is inserted into command field

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T016 [P] [US1] Unit test for discover_applications in src-tauri/src/services/app_discovery.rs (test finds Safari.app)
- [X] T017 [P] [US1] Unit test for parse_app_bundle in src-tauri/src/services/app_discovery.rs (test extracts correct bundle ID)
- [X] T018 [P] [US1] Integration test for get_installed_applications Tauri command in src-tauri/tests/integration/applications_test.rs

### Implementation for User Story 1

- [X] T019 [US1] Implement app_discovery service with discover_applications function in src-tauri/src/services/app_discovery.rs
- [X] T020 [US1] Implement parse_app_bundle helper in src-tauri/src/services/app_discovery.rs using plist crate
- [X] T021 [US1] Create get_installed_applications Tauri command in src-tauri/src/commands/applications.rs
- [X] T022 [US1] Register applications command module in src-tauri/src/lib.rs and src-tauri/src/main.rs
- [X] T023 [US1] Run cargo test to verify all US1 backend tests pass
- [X] T024 [US1] Create applicationService.ts with listApplications and searchApplications functions in src/services/applicationService.ts
- [X] T025 [US1] Create ApplicationPicker.svelte component with search input in src/components/pickers/ApplicationPicker.svelte
- [X] T026 [US1] Implement real-time search filtering using $derived in ApplicationPicker.svelte
- [X] T027 [US1] Add application list display with icons in ApplicationPicker.svelte
- [X] T028 [US1] Implement application selection handler (generates `open -a` command) in ApplicationPicker.svelte
- [X] T029 [US1] Add "Browse Applications" button to ShortcutForm.svelte
- [X] T030 [US1] Wire ApplicationPicker to ShortcutForm command field insertion in src/components/ShortcutForm.svelte
- [X] T031 [US1] Test ApplicationPicker manually: verify apps load, search works, selection inserts correct command

**Checkpoint**: At this point, User Story 1 should be fully functional - users can browse and select applications

---

## Phase 4: User Story 2 - Search and Select Shell Commands (Priority: P2)

**Goal**: Users can browse pre-configured command templates organized by category and select them with parameter customization

**Independent Test**: Open hotkey editor, click "Browse Commands" button, select "Increase Volume" template, customize amount parameter, verify generated command is inserted

### Tests for User Story 2

- [ ] T032 [P] [US2] Unit test for load_templates in src-tauri/src/services/template_loader.rs (verify JSON parsing)
- [ ] T033 [P] [US2] Unit test for generate_command_from_template in src-tauri/src/commands/templates.rs (test parameter substitution)
- [ ] T034 [P] [US2] Unit test for parameter validation in src-tauri/src/commands/templates.rs (test regex, min/max)

### Implementation for User Story 2

- [ ] T035 [P] [US2] Create initial command_templates.json with 5-10 sample templates in src-tauri/src/data/command_templates.json
- [ ] T036 [US2] Implement template_loader service with load_templates function in src-tauri/src/services/template_loader.rs
- [ ] T037 [US2] Create get_command_templates Tauri command in src-tauri/src/commands/templates.rs
- [ ] T038 [US2] Create get_command_categories Tauri command in src-tauri/src/commands/templates.rs
- [ ] T039 [US2] Create generate_command_from_template Tauri command with parameter substitution in src-tauri/src/commands/templates.rs
- [ ] T040 [US2] Register templates command module in src-tauri/src/lib.rs and src-tauri/src/main.rs
- [ ] T041 [US2] Run cargo test to verify all US2 backend tests pass
- [ ] T042 [US2] Create commandService.ts with listTemplates, listCategories, generateCommand functions in src/services/commandService.ts
- [ ] T043 [US2] Create TemplateParameterForm.svelte for parameter input in src/components/pickers/TemplateParameterForm.svelte
- [ ] T044 [US2] Implement parameter validation (regex, min/max, enum) in TemplateParameterForm.svelte
- [ ] T045 [US2] Create CommandPicker.svelte with category navigation in src/components/pickers/CommandPicker.svelte
- [ ] T046 [US2] Implement template selection and parameter form display in CommandPicker.svelte
- [ ] T047 [US2] Add search/filter functionality for templates in CommandPicker.svelte
- [ ] T048 [US2] Add "Browse Commands" button to ShortcutForm.svelte
- [ ] T049 [US2] Wire CommandPicker to ShortcutForm command field insertion in src/components/ShortcutForm.svelte
- [ ] T050 [US2] Expand command_templates.json to 20-30 templates across all categories (Media, System, Window, etc.)
- [ ] T051 [US2] Test CommandPicker manually: verify templates load by category, parameters validate, commands generate correctly

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Browse File System for Scripts and Executables (Priority: P3)

**Goal**: Users can browse file system using native macOS file picker to select custom scripts or executables

**Independent Test**: Open hotkey editor, click "Browse Files" button, navigate to a script file, select it, verify escaped path is inserted into command field

### Tests for User Story 3

- [ ] T052 [P] [US3] Unit test for validate_file_executable in src-tauri/src/services/path_validator.rs (test executable permission check)
- [ ] T053 [P] [US3] Integration test for open_file_picker Tauri command in src-tauri/tests/integration/file_picker_test.rs

### Implementation for User Story 3

- [ ] T054 [US3] Create validate_file_executable function in src-tauri/src/services/path_validator.rs
- [ ] T055 [US3] Create detect_interpreter function in src-tauri/src/services/path_validator.rs (for .sh, .py, .rb files)
- [ ] T056 [US3] Create open_file_picker Tauri command using rfd crate in src-tauri/src/commands/file_picker.rs
- [ ] T057 [US3] Create validate_file_executable Tauri command in src-tauri/src/commands/file_picker.rs
- [ ] T058 [US3] Create escape_shell_path Tauri command in src-tauri/src/commands/file_picker.rs
- [ ] T059 [US3] Register file_picker command module in src-tauri/src/lib.rs and src-tauri/src/main.rs
- [ ] T060 [US3] Run cargo test to verify all US3 backend tests pass
- [ ] T061 [US3] Create FilePicker.svelte component that calls open_file_picker in src/components/pickers/FilePicker.svelte
- [ ] T062 [US3] Implement file selection with executable validation in FilePicker.svelte
- [ ] T063 [US3] Add interpreter detection and path escaping in FilePicker.svelte
- [ ] T064 [US3] Display warning if selected file is not executable in FilePicker.svelte
- [ ] T065 [US3] Add "Browse Files" button to ShortcutForm.svelte
- [ ] T066 [US3] Wire FilePicker to ShortcutForm command field insertion in src/components/ShortcutForm.svelte
- [ ] T067 [US3] Test FilePicker manually: verify native dialog opens, executable validation works, paths are properly escaped

**Checkpoint**: All three user stories (US1, US2, US3) should now be independently functional

---

## Phase 6: User Story 4 - Quick Command Templates (Already Implemented in US2)

**Note**: User Story 4 (Quick Command Templates) is the same implementation as User Story 2 (Search and Select Shell Commands). The template picker functionality covers both stories. No additional tasks needed.

**Verification**:
- [ ] T068 [US4] Verify templates are organized by category (completed in T045)
- [ ] T069 [US4] Verify template preview shows command description (completed in T046)
- [ ] T070 [US4] Verify parameters can be customized (completed in T044)
- [ ] T071 [US4] Verify manual editing is preserved after template insertion (verify in ShortcutForm)

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and final validation

- [ ] T072 [P] Add "No results found" message for empty searches in ApplicationPicker.svelte
- [ ] T073 [P] Add "No results found" message for empty searches in CommandPicker.svelte
- [ ] T074 [P] Implement duplicate app name handling (append path/version) in ApplicationPicker.svelte
- [ ] T075 [P] Add loading spinner during application discovery in ApplicationPicker.svelte
- [ ] T076 [P] Add keyboard navigation (Escape to close, Enter to select) to all picker components
- [ ] T077 [P] Add ARIA labels and accessibility attributes to all picker dialogs
- [ ] T078 Verify all picker buttons are styled consistently with existing ShortcutForm design
- [ ] T079 Run full cargo test suite and verify >80% coverage for app_discovery, template_loader, path_validator
- [ ] T080 Run bun run typecheck to verify TypeScript types are correct
- [ ] T081 [P] Test all pickers with 100+ applications/templates to verify performance <200ms search response
- [ ] T082 Manual accessibility testing: verify keyboard navigation, screen reader compatibility, focus management
- [ ] T083 Cross-browser testing: verify pickers work in different rendering engines (Safari, Chrome rendering in Tauri)
- [ ] T084 Document new picker components in quickstart.md or project README

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - **US1 (P1)**: Can start after Foundational - No dependencies on other stories
  - **US2 (P2)**: Can start after Foundational - No dependencies on other stories
  - **US3 (P3)**: Can start after Foundational - Depends on path_validator from Foundational
  - **US4 (P2)**: Already covered by US2 implementation
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1 - Application Picker)**: Independent, can be deployed as MVP
- **User Story 2 (P2 - Command Templates)**: Independent, can be deployed separately
- **User Story 3 (P3 - File Picker)**: Independent, uses shared path_validator service
- **User Story 4 (P2 - Template Customization)**: Covered by US2 implementation

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Models/services before commands
- Commands before frontend services
- Frontend services before UI components
- UI components before ShortcutForm integration

### Parallel Opportunities

#### Phase 1 (Setup)
- T002, T003, T004 can run in parallel (different directories)

#### Phase 2 (Foundational)
- T006, T007, T008, T009 can run in parallel (different model files)
- T011, T012 can run in parallel (different interfaces)
- T014 runs after T013 (same file)

#### Phase 3 (US1)
- T016, T017, T018 can run in parallel (different test files)
- T024 can run in parallel with backend work (independent file)

#### Phase 4 (US2)
- T032, T033, T034 can run in parallel (different test concerns)
- T035, T042, T043 can run in parallel (different files)

#### Phase 5 (US3)
- T052, T053 can run in parallel (different test files)
- T061, T062, T063 can run in parallel (same component, but incremental features)

#### Phase 7 (Polish)
- T072, T073, T074, T075, T076, T077 can run in parallel (different files)
- T081, T082, T083 can run in parallel (different testing types)

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for discover_applications"
Task: "Unit test for parse_app_bundle"
Task: "Integration test for get_installed_applications"

# Then launch backend + frontend work in parallel:
Task: "Create applicationService.ts" (frontend)
Task: "Implement app_discovery service" (backend)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T005)
2. Complete Phase 2: Foundational (T006-T015) - CRITICAL, blocks all stories
3. Complete Phase 3: User Story 1 (T016-T031)
4. **STOP and VALIDATE**: Test application picker independently
5. Deploy/demo application picker as MVP

### Incremental Delivery

1. **Foundation**: Complete Setup + Foundational (T001-T015) → ~2-3 hours
2. **MVP**: Add User Story 1 (T016-T031) → Test independently → Deploy/Demo → ~4-6 hours
3. **Expansion**: Add User Story 2 (T032-T051) → Test independently → Deploy/Demo → ~4-5 hours
4. **Advanced**: Add User Story 3 (T052-T067) → Test independently → Deploy/Demo → ~3-4 hours
5. **Polish**: Complete Phase 7 (T072-T084) → Final validation → ~3-4 hours

**Total Estimated Time**: 18-24 hours (matches plan.md estimate)

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T015)
2. Once Foundational is done:
   - Developer A: User Story 1 (Application Picker) - T016-T031
   - Developer B: User Story 2 (Command Templates) - T032-T051
   - Developer C: User Story 3 (File Picker) - T052-T067
3. Stories complete and integrate independently
4. Team reviews and completes Polish phase together (T072-T084)

---

## Task Statistics

- **Total Tasks**: 84
- **Setup Phase**: 5 tasks
- **Foundational Phase**: 10 tasks (BLOCKING)
- **User Story 1 (P1)**: 16 tasks (MVP)
- **User Story 2 (P2)**: 20 tasks
- **User Story 3 (P3)**: 16 tasks
- **User Story 4 (P2)**: 4 verification tasks (covered by US2)
- **Polish Phase**: 13 tasks

**Parallel Opportunities**: 28 tasks marked [P] can run in parallel with other tasks

**Test Coverage**: 11 test tasks ensuring >80% coverage target for Rust backend

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests MUST fail before implementation begins
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Constitution compliance: >80% test coverage for Rust backend (app_discovery, template_loader, path_validator)
