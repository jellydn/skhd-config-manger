# Tasks: Service Log Viewer and Reload

**Input**: Design documents from `/specs/006-service-log-reload/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Test tasks included per constitution requirement for >80% coverage of parsing and file operations

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Tauri application structure: `src-tauri/src/` (Rust backend), `src/` (Svelte frontend)
- Tests: `src-tauri/tests/` (Rust), `src/__tests__/` (Svelte/TypeScript)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and file structure for new feature components

- [x] T001 Create backend directory structure: `src-tauri/src/models/`, `src-tauri/src/services/`, `src-tauri/src/commands/`
- [x] T002 Create frontend directory structure: `src/components/`, `src/services/`, `src/routes/logs/`
- [x] T003 Create test directory structure: `src-tauri/tests/unit/`, `src-tauri/tests/integration/`, `src/__tests__/components/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data models and shared types that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 [P] Create LogLevel enum in `src-tauri/src/models/log_entry.rs` with values: ERROR, WARN, INFO, DEBUG
- [x] T005 [P] Create ServiceState enum in `src-tauri/src/models/service_status.rs` with states: Stopped, Starting, Running, Stopping, Reloading, Error, Unknown
- [x] T006 [P] Create LogEntry struct in `src-tauri/src/models/log_entry.rs` with fields: id, timestamp, level, message, raw
- [x] T007 [P] Create ServiceStatus struct in `src-tauri/src/models/service_status.rs` with fields: state, pid, last_updated, config_path, error_message
- [x] T008 Add Serde derive macros to all models for JSON serialization
- [x] T009 Export models from `src-tauri/src/models/mod.rs`
- [x] T010 Create TypeScript interfaces in `src/types.ts` matching Rust models: LogEntry, ServiceStatus, LogLevel, ServiceState

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - View Service Logs (Priority: P1) 🎯 MVP

**Goal**: Users can view real-time and historical skhd service logs with visual distinction for log levels

**Independent Test**: Start application, navigate to log viewer, verify skhd logs display in chronological order with color coding for error/warn/info/debug levels

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T011 [P] [US1] Create unit test `src-tauri/tests/log_parser.rs` for log line parsing (valid formats, invalid formats, edge cases)
- [x] T012 [P] [US1] Create unit test fixtures in `src-tauri/tests/fixtures/sample_logs.txt` with various log formats and levels
- [x] T013 [P] [US1] Create component test `src/__tests__/components/LogViewer.test.ts` for log rendering, color coding, and scrolling

### Implementation for User Story 1

#### Backend - Log Parsing and Streaming

- [x] T014 [US1] Implement log parsing function in `src-tauri/src/services/log_tailer.rs` using regex to extract timestamp, level, message from log lines
- [x] T015 [US1] Implement `LogTailer` struct in `src-tauri/src/services/log_tailer.rs` with `start_stream()` and `stop_stream()` methods
- [x] T016 [US1] Add tokio async process spawning for `log stream --predicate 'process == "skhd"'` command in `log_tailer.rs`
- [x] T017 [US1] Add line-by-line reading with BufReader in `log_tailer.rs` to stream log output
- [x] T018 [US1] Add Tauri event emission for each parsed log entry in `log_tailer.rs` (event: `log-entry`)
- [x] T019 [US1] Implement error handling for permission denied, skhd not running, stream already active

#### Backend - Tauri Commands

- [x] T020 [P] [US1] Create `start_log_stream` command in `src-tauri/src/commands/logs.rs` that calls `LogTailer::start_stream()`
- [x] T021 [P] [US1] Create `stop_log_stream` command in `src-tauri/src/commands/logs.rs` that calls `LogTailer::stop_stream()`
- [ ] T022 [P] [US1] Create `get_recent_logs` command in `src-tauri/src/commands/logs.rs` using `log show --predicate 'process == "skhd"' --last 1h`
- [x] T023 [US1] Register log commands in `src-tauri/src/lib.rs` Tauri builder

#### Frontend - Service Client

- [x] T024 [US1] Create `src/services/logService.ts` with typed wrappers for `start_log_stream`, `stop_log_stream` commands
- [x] T025 [US1] Add event subscription helper `onLogEntry()` in `src/services/logService.ts` for `log-entry` events
- [x] T026 [US1] Create log management in `LogViewer.svelte`: `logs` (array), `isStreaming` (boolean)

#### Frontend - Log Viewer UI

- [x] T027 [US1] Create `LogViewer.svelte` component in `src/components/` with basic log entry list display
- [x] T028 [US1] Add CSS styling to `LogViewer.svelte`: Monaco font, color coding per log level (red/yellow/blue/gray)
- [x] T029 [US1] Implement virtual scrolling in `LogViewer.svelte` to handle large log volumes (max 1000 entries in memory)
- [x] T030 [US1] Add auto-scroll to bottom for new entries with pause-on-user-scroll in `LogViewer.svelte`
- [x] T031 [US1] Add log entry timestamp formatting in `LogViewer.svelte` (ISO 8601 display)
- [x] T031a [US1] Display logs in descending order (newest first) by default in `LogViewer.svelte` (removed user toggle for cleaner UI)
- [x] T032 [US1] Create log viewer page `src/routes/logs/+page.svelte` that uses `LogViewer` component
- [x] T033 [US1] Add onMount hook in `src/routes/logs/+page.svelte` to load recent logs and start streaming
- [x] T034 [US1] Add onDestroy hook in `src/routes/logs/+page.svelte` to stop log stream and cleanup

**Checkpoint**: At this point, User Story 1 should be fully functional - log viewer displays real-time skhd logs with color coding

---

## Phase 4: User Story 2 - Reload Service with Active Configuration (Priority: P2)

**Goal**: Users can reload the skhd service with the currently active configuration via GUI button

**Independent Test**: Click reload service button, verify service restarts successfully, check log viewer for restart confirmation

### Tests for User Story 2

- [ ] T035 [P] [US2] Create unit test `src-tauri/tests/unit/service_manager_test.rs` for service control operations (get_status, stop, start, reload)
- [ ] T036 [P] [US2] Create integration test `src-tauri/tests/integration/service_control_test.rs` for end-to-end reload flow with mock launchctl
- [ ] T037 [P] [US2] Create component test `src/__tests__/components/ServiceControl.test.ts` for button states, loading indicators, error displays

### Implementation for User Story 2

#### Backend - Service Manager

- [ ] T038 [US2] Create `ServiceManager` struct in `src-tauri/src/services/service_manager.rs` with tokio Mutex for reload lock
- [ ] T039 [US2] Implement `get_status()` method in `service_manager.rs` using `launchctl list | grep skhd` to query service state and PID
- [ ] T040 [US2] Implement `stop_service()` method in `service_manager.rs` using `launchctl unload ~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
- [ ] T041 [US2] Implement `start_service()` method in `service_manager.rs` using `launchctl load ~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
- [ ] T042 [US2] Implement `reload_service()` method in `service_manager.rs` with validation, backup, stop, write config, start, verify, rollback on failure
- [ ] T043 [US2] Add 2-second grace period and status check after service start in `reload_service()`
- [ ] T044 [US2] Add error handling for permission denied, service not found, config validation failed, service failed to start
- [ ] T045 [US2] Add Tauri event emission in `reload_service()` for success (`service-reload-success`) and error (`service-reload-error`)

#### Backend - Service Commands

- [ ] T046 [P] [US2] Create `get_service_status` command in `src-tauri/src/commands/service.rs` that calls `ServiceManager::get_status()`
- [ ] T047 [P] [US2] Create `reload_service` command in `src-tauri/src/commands/service.rs` with config_id parameter, calls `ServiceManager::reload_service()`
- [ ] T048 [US2] Register service commands in `src-tauri/src/lib.rs` Tauri builder

#### Frontend - Service Client Extensions

- [ ] T049 [US2] Add `getServiceStatus()` wrapper in `src/services/service.ts` for service status command
- [ ] T050 [US2] Add `reloadService(configId)` wrapper in `src/services/service.ts` for reload command
- [ ] T051 [US2] Add event subscriptions in `src/services/service.ts` for `service-reload-success` and `service-reload-error`
- [ ] T052 [US2] Create Svelte store in `src/services/service.ts`: `serviceStatus` (reactive ServiceStatus object)

#### Frontend - Service Control UI

- [ ] T053 [US2] Create `ServiceControl.svelte` component in `src/components/` with reload button and status indicator
- [ ] T054 [US2] Add service status display in `ServiceControl.svelte` with colored dot (green/yellow/red/gray) based on state
- [ ] T055 [US2] Add reload button with loading state in `ServiceControl.svelte` (disabled during reload)
- [ ] T056 [US2] Implement reload click handler in `ServiceControl.svelte` that calls `reloadService()` with active config
- [ ] T057 [US2] Add success/error toast notifications in `ServiceControl.svelte` for reload results
- [ ] T058 [US2] Add periodic status polling (every 5 seconds) in `ServiceControl.svelte` when mounted
- [ ] T059 [US2] Integrate `ServiceControl` component into `src/routes/logs/+page.svelte` above log viewer

**Checkpoint**: At this point, both user stories work independently - logs viewable and service reloadable

---

## Phase 5: User Story 3 - Import Configuration Before Reload (Priority: P3)

**Goal**: Users can import a different configuration file before reloading the service

**Independent Test**: Click "Import Config" button, select a config file, see active config path displayed, reload service, verify service uses imported config

**Note**: This uses existing `import_config` command - no new backend needed

### Frontend - Configuration Import UI

- [ ] T060 [US3] Add active config path display to `src/routes/logs/+page.svelte` service control section
- [ ] T061 [US3] Add "Import Config" button next to reload button in service control toolbar
- [ ] T062 [US3] Wire up "Import Config" button to existing `importConfig()` in `src/services/tauri.ts`
- [ ] T063 [US3] Update active config path display after successful import
- [ ] T064 [US3] Add visual feedback for import success/failure (toast notification or inline message)
- [ ] T065 [US3] Style config path display with monospace font and subtle background
- [ ] T066 [US3] Add icon to "Import Config" button (folder or document icon)

**Checkpoint**: All three user stories now functional - log viewing, service reload, and config import before reload

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect all user stories and final validation

- [ ] T075 [P] Add comprehensive error messages for all error scenarios in `src-tauri/src/services/service_manager.rs` and `log_tailer.rs`
- [ ] T076 [P] Add debouncing for rapid log bursts in `src-tauri/src/services/log_tailer.rs` (100ms batch)
- [ ] T077 [P] Add dark mode support verification in `LogViewer.svelte` CSS (test with macOS system dark mode)
- [ ] T078 [P] Add accessibility attributes to `LogViewer.svelte`, `ServiceControl.svelte`, `ConfigSelector.svelte` (ARIA labels, keyboard navigation)
- [ ] T079 Run all unit tests: `cd src-tauri && cargo test`
- [ ] T080 Run all integration tests: `cd src-tauri && cargo test --test service_control_test`
- [ ] T081 Run frontend tests: `bun run test`
- [ ] T082 Run type checking: `bun run typecheck`
- [ ] T083 Run Rust linting: `cd src-tauri && cargo clippy`
- [ ] T084 Manual testing against quickstart.md checklist (all 10 manual test cases)
- [ ] T085 Performance validation: Test with >10MB log file, verify virtual scrolling prevents UI freeze
- [ ] T086 Edge case validation: Test all edge cases from spec.md (skhd not running, large logs, concurrent reload, etc.)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User Story 1 can proceed independently after Phase 2
  - User Story 2 can proceed independently after Phase 2 (uses US1 log viewer for verification but doesn't block)
  - User Story 3 can proceed independently after Phase 2 (uses existing import_config command)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1 (though benefits from log viewer for validation)
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Uses existing `import_config` command, extends US2 reload UI

### Within Each User Story

- Tests MUST be written and FAIL before implementation (T011-T013, T035-T037)
- Models before services (Phase 2 completes all models)
- Services before commands (service_manager/log_tailer before Tauri commands)
- Backend before frontend (commands registered before frontend wrappers)
- Core components before page integration
- User Story 3 is frontend-only using existing `import_config` command

### Parallel Opportunities

**Phase 1 (Setup)**: All tasks can run in parallel (T001-T003)

**Phase 2 (Foundational)**: All tasks can run in parallel (T004-T010)

**User Story 1**:
- Tests: T011, T012, T013 in parallel
- Backend: T020, T021, T022 in parallel (after T014-T019)
- Frontend: T027 can start as soon as T024-T026 complete

**User Story 2**:
- Tests: T035, T036, T037 in parallel
- Backend: T046, T047 in parallel (after T038-T045)
- Frontend: T053 can start as soon as T049-T052 complete

**User Story 3**:
- Frontend: All tasks (T060-T066) are frontend-only and can run in parallel once UI structure is in place

**Phase 6 (Polish)**: T075-T078 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Write all tests for User Story 1 together:
Task T011: "Unit test for log parsing in src-tauri/tests/unit/log_parser_test.rs"
Task T012: "Test fixtures in src-tauri/tests/fixtures/sample_logs.txt"
Task T013: "Component test in src/__tests__/components/LogViewer.test.ts"

# Develop backend commands in parallel:
Task T020: "start_log_stream command in src-tauri/src/commands/logs.rs"
Task T021: "stop_log_stream command in src-tauri/src/commands/logs.rs"
Task T022: "get_recent_logs command in src-tauri/src/commands/logs.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T010) - CRITICAL
3. Complete Phase 3: User Story 1 (T011-T034)
4. **STOP and VALIDATE**: Manual test log viewer against acceptance criteria
5. Run tests: T079-T081
6. Demo/Deploy MVP: Real-time log viewer

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP - log viewing!)
3. Add User Story 2 → Test independently → Deploy/Demo (service control added)
4. Add User Story 3 → Test independently → Deploy/Demo (full feature complete)
5. Polish → Final validation → Production ready

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T010)
2. Once Foundational is done:
   - Developer A: User Story 1 (T011-T034) - Backend specialist
   - Developer B: User Story 2 (T035-T059) - System integration specialist
   - Developer C: User Story 3 (T060-T074) - Frontend specialist
3. Stories complete and integrate independently
4. Team collaborates on Polish (T075-T086)

---

## Task Summary

**Total Tasks**: 80
- Phase 1 (Setup): 3 tasks
- Phase 2 (Foundational): 7 tasks (CRITICAL PATH)
- Phase 3 (User Story 1 - P1): 25 tasks (3 tests, 22 implementation)
- Phase 4 (User Story 2 - P2): 25 tasks (3 tests, 22 implementation)
- Phase 5 (User Story 3 - P3): 7 tasks (frontend-only, uses existing backend)
- Phase 6 (Polish): 13 tasks

**Parallel Opportunities**: 27 tasks marked [P] across all phases
**Test Tasks**: 6 (per constitution >80% coverage requirement)
**MVP Scope**: Phase 1 + Phase 2 + Phase 3 = 35 tasks
**Full Feature Scope**: All 80 tasks

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Constitution compliance: >80% test coverage for parsing and file operations (T011, T035, T036)
