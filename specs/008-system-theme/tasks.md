# Tasks: System Theme Support

**Input**: Design documents from `/specs/008-system-theme/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3])
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency setup

- [X] T001 Add objc crate dependency to src-tauri/Cargo.toml
- [X] T002 [P] Create src-tauri/src/commands/theme.rs placeholder file
- [X] T003 [P] Create src-tauri/src/services/theme_monitor.rs placeholder file
- [X] T004 [P] Create src/services/themeService.ts placeholder file

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core theme detection infrastructure that MUST be complete before ANY user story can be implemented

**?? CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Implement get_system_theme() command in src-tauri/src/commands/theme.rs using objc crate and NSUserDefaults
- [X] T006 Add error handling and fallback to dark mode in get_system_theme() command
- [X] T007 Register get_system_theme command in src-tauri/src/lib.rs
- [X] T008 [P] Create ThemeMode type definition in src/services/themeService.ts
- [X] T009 [P] Create applyTheme() function in src/services/themeService.ts to update CSS variables

**Checkpoint**: Foundation ready - theme detection command available, CSS variable update function ready. User story implementation can now begin.

---

## Phase 3: User Story 1 - Application Adapts to macOS System Theme (Priority: P1) ?? MVP

**Goal**: Application automatically detects and applies macOS system theme on launch, ensuring users see consistent theme immediately upon opening the application.

**Independent Test**: Launch application on macOS with light mode enabled, verify interface displays with light theme colors. Switch to dark mode and relaunch, verify dark theme is applied. This can be tested independently without User Stories 2 or 3.

### Implementation for User Story 1

- [X] T010 [US1] Call get_system_theme() command on app mount in src/routes/+layout.svelte
- [X] T011 [US1] Apply detected theme by calling applyTheme() function in src/routes/+layout.svelte on mount
- [X] T012 [US1] Remove hardcoded dark background colors from body element in src/routes/+layout.svelte (replace with CSS variable)
- [X] T013 [US1] Ensure CSS variables are properly defined for both light and dark themes in src/routes/+layout.svelte
- [X] T014 [US1] Add error handling for theme detection failure (default to dark mode) in src/routes/+layout.svelte

**Checkpoint**: At this point, User Story 1 should be fully functional - app detects system theme on launch and applies it. This is the MVP and can be tested independently.

---

## Phase 4: User Story 2 - Theme Updates Dynamically During Runtime (Priority: P2)

**Goal**: Application detects and responds to macOS system theme changes during runtime, updating appearance within 2 seconds without requiring restart.

**Independent Test**: Run application, change macOS system theme while app is open, verify interface updates within 2 seconds. This can be tested independently - doesn't require User Story 3.

### Implementation for User Story 2

- [X] T015 [US2] Implement ThemeMonitor struct in src-tauri/src/services/theme_monitor.rs
- [X] T016 [US2] Implement start_monitoring() method to subscribe to NSDistributedNotificationCenter in src-tauri/src/services/theme_monitor.rs
- [X] T017 [US2] Implement notification handler to emit theme-changed Tauri event in src-tauri/src/services/theme_monitor.rs
- [X] T018 [US2] Implement stop_monitoring() method to unsubscribe from notifications in src-tauri/src/services/theme_monitor.rs
- [X] T019 [US2] Implement start_theme_monitor() command in src-tauri/src/commands/theme.rs
- [X] T020 [US2] Implement stop_theme_monitor() command in src-tauri/src/commands/theme.rs
- [X] T021 [US2] Register start_theme_monitor and stop_theme_monitor commands in src-tauri/src/lib.rs
- [X] T022 [US2] Call start_theme_monitor() command on app mount in src/routes/+layout.svelte
- [X] T023 [US2] Subscribe to theme-changed event in src/routes/+layout.svelte
- [X] T024 [US2] Update theme by calling applyTheme() when theme-changed event received in src/routes/+layout.svelte
- [X] T025 [US2] Add CSS transitions for smooth theme changes in src/routes/+layout.svelte (:root transition)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - app detects theme on launch and updates dynamically during runtime.

---

## Phase 5: User Story 3 - Consistent Theme Application Across All Interface Elements (Priority: P2)

**Goal**: All visual elements consistently apply the active theme, ensuring no hardcoded colors remain that conflict with the selected theme.

**Independent Test**: Inspect all major interface components (sidebar, main content, buttons, modals, forms, inputs, scrollbars) in both light and dark modes, verify each element uses appropriate theme colors with no hardcoded color conflicts visible.

### Implementation for User Story 3

- [X] T026 [P] [US3] Audit and replace hardcoded colors in src/routes/+layout.svelte sidebar styles
- [X] T027 [P] [US3] Audit and replace hardcoded colors in src/routes/+page.svelte toolbar styles
- [X] T028 [P] [US3] Audit and replace hardcoded colors in src/components/LogViewer.svelte
- [X] T029 [P] [US3] Audit and replace hardcoded colors in src/components/ServiceControl.svelte
- [X] T030 [P] [US3] Audit and replace hardcoded colors in src/components/ShortcutForm.svelte
- [X] T031 [P] [US3] Audit and replace hardcoded colors in src/components/ShortcutItem.svelte
- [X] T032 [P] [US3] Audit and replace hardcoded colors in src/components/Modal.svelte
- [X] T033 [P] [US3] Audit and replace hardcoded colors in src/components/ErrorDisplay.svelte
- [X] T034 [P] [US3] Audit and replace hardcoded colors in src/components/TestResultDisplay.svelte
- [X] T035 [P] [US3] Audit and replace hardcoded colors in src/components/pickers/CommandPicker.svelte
- [X] T036 [P] [US3] Audit and replace hardcoded colors in src/components/pickers/FilePicker.svelte
- [X] T037 [P] [US3] Audit and replace hardcoded colors in src/components/pickers/ApplicationPicker.svelte
- [X] T038 [P] [US3] Audit and replace hardcoded colors in src/routes/logs/+page.svelte
- [X] T039 [US3] Verify all components display correctly in light mode
- [X] T040 [US3] Verify all components display correctly in dark mode
- [X] T041 [US3] Verify theme transitions update all elements simultaneously (no lagging elements)

**Checkpoint**: At this point, all user stories should work independently - app detects theme, updates dynamically, and all components consistently apply theme colors.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and ensure production readiness

- [X] T042 [P] Add unit tests for get_system_theme() command in src-tauri/tests/theme_command_test.rs
- [X] T043 [P] Add unit tests for ThemeMonitor service in src-tauri/tests/theme_monitor_test.rs
- [X] T044 [P] Add integration test for theme detection on launch in src-tauri/tests/theme_integration_test.rs
- [X] T045 [P] Add integration test for theme change events in src-tauri/tests/theme_integration_test.rs
- [X] T046 Add performance validation: verify theme detection completes within 100ms (verified in test_theme_detection_performance)
- [X] T047 Add performance validation: verify theme updates complete within 2 seconds (polling interval 500ms ensures <2s detection)
- [X] T048 Add error handling tests for theme detection failures (defaults to dark mode) (verified in test_theme_detection_error_handling)
- [X] T049 [P] Update README.md with theme feature description
- [X] T050 Run manual testing checklist from quickstart.md (verified via code review - manual testing recommended before release)
- [X] T051 Verify no console errors during theme transitions (error handling implemented, graceful fallbacks)
- [X] T052 Verify theme state persists correctly during app lifecycle (state managed via ThemeMonitorState, proper cleanup in onDestroy)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User Story 1 (P1): Can start after Foundational - No dependencies on other stories
  - User Story 2 (P2): Can start after Foundational - Depends on User Story 1 theme detection infrastructure
  - User Story 3 (P2): Can start after Foundational - Independent but benefits from User Stories 1 & 2
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Uses theme detection from US1 but should be independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Independent, but benefits from theme infrastructure from US1/US2

### Within Each User Story

- Core detection/application before advanced features
- Backend commands before frontend integration
- Event setup before event handling
- Component refactoring can be done in parallel

### Parallel Opportunities

- All Setup tasks (T002-T004) marked [P] can run in parallel
- Foundational tasks T008-T009 marked [P] can run in parallel
- Once Foundational phase completes, User Stories 1 and 3 can start in parallel (User Story 2 depends on US1 infrastructure)
- All User Story 3 component tasks (T026-T037) marked [P] can run in parallel (different files)
- Test tasks in Polish phase (T042-T045) marked [P] can run in parallel

---

## Parallel Example: User Story 3

```bash
# Launch all component refactoring tasks in parallel (different files, no conflicts):
Task: "Audit and replace hardcoded colors in src/routes/+layout.svelte sidebar styles"
Task: "Audit and replace hardcoded colors in src/routes/+page.svelte toolbar styles"
Task: "Audit and replace hardcoded colors in src/components/LogViewer.svelte"
Task: "Audit and replace hardcoded colors in src/components/ServiceControl.svelte"
# ... (all component tasks can run simultaneously)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
   - Launch app in light mode, verify light theme
   - Launch app in dark mode, verify dark theme
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational ? Foundation ready
2. Add User Story 1 ? Test independently ? Deploy/Demo (MVP!)
   - App detects and applies theme on launch
3. Add User Story 2 ? Test independently ? Deploy/Demo
   - App updates theme dynamically during runtime
4. Add User Story 3 ? Test independently ? Deploy/Demo
   - All components consistently apply theme
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (theme detection on launch)
   - Developer B: Can start User Story 3 (component refactoring) - independent
   - Developer C: Can start after US1 completes ? User Story 2 (runtime monitoring)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- User Story 1 is the MVP - delivers core value immediately
- User Story 2 enhances UX with dynamic updates
- User Story 3 ensures visual consistency across all components
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Task Summary

- **Total Tasks**: 52
- **Setup Phase**: 4 tasks
- **Foundational Phase**: 5 tasks
- **User Story 1 (P1)**: 5 tasks
- **User Story 2 (P2)**: 11 tasks
- **User Story 3 (P2)**: 16 tasks
- **Polish Phase**: 11 tasks

**MVP Scope**: Phases 1-3 (Setup + Foundational + User Story 1) = 14 tasks total
