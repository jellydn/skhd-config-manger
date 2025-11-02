# Tasks: Command Execution Test

**Input**: Design documents from `/specs/005-command-test/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are included per constitution requirement (Principle III: >80% coverage for critical paths)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

Project uses Tauri single-project architecture:
- **Backend**: `src-tauri/src/` for Rust code
- **Frontend**: `src/` for Svelte/TypeScript code
- **Tests**: `src-tauri/tests/` for Rust tests

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add dependencies and configure project for command execution functionality

- [x] T001 Add regex and lazy_static dependencies to src-tauri/Cargo.toml
- [x] T002 [P] Add tokio process feature to src-tauri/Cargo.toml if not already present
- [x] T003 [P] Verify chrono dependency in src-tauri/Cargo.toml for timestamp generation
- [x] T004 Run cargo build to verify all dependencies resolve correctly

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data model extensions that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Extend TestResult model with execution fields in src-tauri/src/models/test_result.rs
- [x] T006 [P] Add ExecutionState struct to src-tauri/src/commands/testing.rs
- [x] T007 [P] Implement output truncation utility function in src-tauri/src/commands/testing.rs
- [x] T008 Register ExecutionState in Tauri state management in src-tauri/src/lib.rs
- [x] T009 [P] Update TypeScript TestResult interface in src/types.ts
- [x] T010 [P] Add ExecutionStatus type definition in src/types.ts
- [x] T011 Run cargo test to verify TestResult serialization works with new fields

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Execute Command to Verify Behavior (Priority: P1) 🎯 MVP

**Goal**: Users can execute keyboard shortcut commands and see real execution results (stdout, stderr, exit code, duration)

**Independent Test**: Click test button on any shortcut, verify command executes and displays output in UI

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T012 [P] [US1] Create unit test for output truncation in src-tauri/tests/command_execution.rs
- [ ] T013 [P] [US1] Create integration test for simple command execution in src-tauri/tests/command_execution.rs
- [ ] T014 [P] [US1] Create integration test for failed command execution in src-tauri/tests/command_execution.rs
- [ ] T015 [P] [US1] Create integration test for command timeout in src-tauri/tests/command_execution.rs

### Implementation for User Story 1

- [x] T016 [US1] Implement execute_shortcut_command async function in src-tauri/src/commands/testing.rs
- [x] T017 [US1] Add command spawning with tokio::process::Command in execute_shortcut_command
- [x] T018 [US1] Add timeout handling with tokio::time::timeout (30 seconds) in execute_shortcut_command
- [x] T019 [US1] Implement stdout/stderr capture and truncation in execute_shortcut_command
- [x] T020 [US1] Add execution duration tracking in execute_shortcut_command
- [x] T021 [US1] Build TestResult with execution data in execute_shortcut_command
- [x] T022 [US1] Register execute_shortcut_command in Tauri invoke_handler in src-tauri/src/lib.rs
- [x] T023 [US1] Export execute_shortcut_command from commands module in src-tauri/src/commands/mod.rs
- [x] T024 [US1] Add execution state management in ShortcutItem component in src/components/ShortcutItem.svelte
- [x] T025 [US1] Implement handleExecuteCommand function in src/components/ShortcutItem.svelte
- [x] T026 [US1] Add test button click handler to call execute_shortcut_command in src/components/ShortcutItem.svelte
- [x] T027 [US1] Update UI to show executing state with spinner in src/components/ShortcutItem.svelte
- [x] T028 [US1] Run cargo test to verify all US1 tests pass
- [ ] T029 [US1] Manual test: Execute "echo 'Hello World'" command and verify output displays

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - View Execution Results (Priority: P2)

**Goal**: Users see detailed execution results including stdout, stderr, exit code, and execution time in a dedicated display panel

**Independent Test**: Execute various commands (successful, failed, with/without output) and verify all result details display correctly

### Tests for User Story 2

- [ ] T030 [P] [US2] Create unit test for TestResultDisplay component rendering in src/components/__tests__/TestResultDisplay.test.ts
- [ ] T031 [P] [US2] Create integration test for success result display in src/components/__tests__/TestResultDisplay.test.ts
- [ ] T032 [P] [US2] Create integration test for error result display in src/components/__tests__/TestResultDisplay.test.ts

### Implementation for User Story 2

- [ ] T033 [P] [US2] Add result display logic for executed=true in src/components/TestResultDisplay.svelte
- [ ] T034 [P] [US2] Add result header with status icon and exit code in src/components/TestResultDisplay.svelte
- [ ] T035 [P] [US2] Add stdout output section with pre/code formatting in src/components/TestResultDisplay.svelte
- [ ] T036 [P] [US2] Add stderr output section with error styling in src/components/TestResultDisplay.svelte
- [ ] T037 [P] [US2] Add execution duration display in src/components/TestResultDisplay.svelte
- [ ] T038 [P] [US2] Add truncation notice when output_truncated is true in src/components/TestResultDisplay.svelte
- [ ] T039 [P] [US2] Add styling for success/error states with macOS color scheme in src/components/TestResultDisplay.svelte
- [ ] T040 [US2] Connect TestResultDisplay to ShortcutItem test result state in src/components/ShortcutItem.svelte
- [ ] T041 [US2] Run bun run test to verify US2 component tests pass
- [ ] T042 [US2] Manual test: Execute commands with various outputs and verify display correctness

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Safety Confirmation for Destructive Commands (Priority: P2)

**Goal**: Users receive warning confirmation before executing potentially destructive commands (rm, sudo, kill, etc.)

**Independent Test**: Create shortcuts with destructive commands, verify confirmation dialog appears; create safe commands, verify no dialog appears

### Tests for User Story 3

- [ ] T043 [P] [US3] Create unit test for destructive pattern detection in src-tauri/tests/destructive_detection.rs
- [ ] T044 [P] [US3] Test pattern matching for sudo commands in src-tauri/tests/destructive_detection.rs
- [ ] T045 [P] [US3] Test pattern matching for rm -rf commands in src-tauri/tests/destructive_detection.rs
- [ ] T046 [P] [US3] Test pattern matching for shutdown/reboot commands in src-tauri/tests/destructive_detection.rs
- [ ] T047 [P] [US3] Test safe command returns is_destructive=false in src-tauri/tests/destructive_detection.rs

### Implementation for User Story 3

- [ ] T048 [P] [US3] Create DestructiveCheck struct in src-tauri/src/commands/testing.rs
- [ ] T049 [P] [US3] Define DESTRUCTIVE_PATTERNS with regex list in src-tauri/src/commands/testing.rs
- [ ] T050 [US3] Implement check_destructive_command function in src-tauri/src/commands/testing.rs
- [ ] T051 [US3] Register check_destructive_command in Tauri invoke_handler in src-tauri/src/lib.rs
- [ ] T052 [US3] Export check_destructive_command from commands module in src-tauri/src/commands/mod.rs
- [ ] T053 [P] [US3] Add DestructiveCheck type definition in src/types.ts
- [ ] T054 [US3] Add confirmation state to ShortcutItem component in src/components/ShortcutItem.svelte
- [ ] T055 [US3] Call check_destructive_command before execution in src/components/ShortcutItem.svelte
- [ ] T056 [US3] Show ConfirmDialog when is_destructive=true in src/components/ShortcutItem.svelte
- [ ] T057 [US3] Handle confirmation accept/cancel actions in src/components/ShortcutItem.svelte
- [ ] T058 [US3] Run cargo test to verify US3 pattern tests pass
- [ ] T059 [US3] Manual test: Test destructive command shows confirmation, safe command executes immediately

**Checkpoint**: User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Stop Long-Running Commands (Priority: P3)

**Goal**: Users can cancel long-running or stuck commands during execution to regain control

**Independent Test**: Execute command with intentional delay (sleep 30), verify cancel button appears and successfully terminates process

### Tests for User Story 4

- [ ] T060 [P] [US4] Create integration test for command cancellation in src-tauri/tests/command_execution.rs
- [ ] T061 [P] [US4] Test process is removed from ExecutionState after cancel in src-tauri/tests/command_execution.rs
- [ ] T062 [P] [US4] Test cancel returns appropriate error if no process running in src-tauri/tests/command_execution.rs

### Implementation for User Story 4

- [ ] T063 [US4] Store Child process in ExecutionState during execution in src-tauri/src/commands/testing.rs
- [ ] T064 [US4] Implement cancel_shortcut_execution function in src-tauri/src/commands/testing.rs
- [ ] T065 [US4] Add SIGTERM/SIGKILL process termination in cancel_shortcut_execution
- [ ] T066 [US4] Remove process from ExecutionState on cancel in cancel_shortcut_execution
- [ ] T067 [US4] Register cancel_shortcut_execution in Tauri invoke_handler in src-tauri/src/lib.rs
- [ ] T068 [US4] Export cancel_shortcut_execution from commands module in src-tauri/src/commands/mod.rs
- [ ] T069 [US4] Add cancel button UI when executionState='executing' in src/components/ShortcutItem.svelte
- [ ] T070 [US4] Implement handleCancelClick function in src/components/ShortcutItem.svelte
- [ ] T071 [US4] Update UI to show cancelled state after cancellation in src/components/ShortcutItem.svelte
- [ ] T072 [US4] Add timeout display when timed_out=true in src/components/TestResultDisplay.svelte
- [ ] T073 [US4] Run cargo test to verify US4 cancellation tests pass
- [ ] T074 [US4] Manual test: Execute "sleep 60" command, cancel mid-execution, verify process killed

**Checkpoint**: All user stories should now be independently functional

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and final quality assurance

- [ ] T075 [P] Add comprehensive error handling for all execution edge cases in src-tauri/src/commands/testing.rs
- [ ] T076 [P] Add debug logging for command execution lifecycle in src-tauri/src/commands/testing.rs
- [ ] T077 [P] Verify dark mode styling for all new UI components in src/components/
- [ ] T078 [P] Add accessibility attributes (aria-label, role) to test and cancel buttons in src/components/ShortcutItem.svelte
- [ ] T079 [P] Test keyboard navigation for confirmation dialogs
- [ ] T080 Run cargo clippy and fix any warnings in src-tauri/
- [ ] T081 Run bun run typecheck and fix any TypeScript errors in src/
- [ ] T082 [P] Profile memory usage during command execution to verify <100MB constraint
- [ ] T083 [P] Test rapid successive command execution on different shortcuts
- [ ] T084 [P] Test command with special characters (pipes, redirects, quotes)
- [ ] T085 [P] Test command with very large output (>10,000 chars) to verify truncation
- [ ] T086 Run full test suite: cargo test && bun run test
- [ ] T087 Manual testing per quickstart.md validation scenarios
- [ ] T088 Update CLAUDE.md if needed (should already be updated by update-agent-context.sh)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P2 → P3)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Builds on US1 but independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Independent of US1/US2
- **User Story 4 (P3)**: Can start after Foundational (Phase 2) - Enhances US1 but independently testable

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Backend models/commands before frontend UI
- Backend registration before frontend invocation
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- **Setup Phase**: T001, T002, T003 can run in parallel (different dependency entries)
- **Foundational Phase**: T006, T007, T009, T010 can run in parallel (different files)
- **US1 Tests**: T012, T013, T014, T015 can run in parallel (different test cases)
- **US2 Implementation**: T033-T039 can run in parallel (different sections of same file with careful coordination)
- **US3 Tests**: T043-T047 can run in parallel (different test cases)
- **US3 Setup**: T048, T049, T053 can run in parallel (different files)
- **US4 Tests**: T060, T061, T062 can run in parallel (different test cases)
- **Polish**: T075-T085 can run in parallel (different files/concerns)
- **Different user stories** can be worked on in parallel by different team members after Foundational phase

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Create unit test for output truncation in src-tauri/tests/command_execution.rs"
Task: "Create integration test for simple command execution in src-tauri/tests/command_execution.rs"
Task: "Create integration test for failed command execution in src-tauri/tests/command_execution.rs"
Task: "Create integration test for command timeout in src-tauri/tests/command_execution.rs"

# After tests are written and failing, implement backend (sequential):
T016 → T017 → T018 → T019 → T020 → T021 → T022 → T023

# Then update frontend (sequential within component):
T024 → T025 → T026 → T027
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (~30 minutes)
2. Complete Phase 2: Foundational (~1 hour)
3. Complete Phase 3: User Story 1 (~2-3 hours)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

**MVP Deliverable**: Users can execute commands and see real stdout/stderr/exit code results

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (enhanced result display)
4. Add User Story 3 → Test independently → Deploy/Demo (safety confirmations)
5. Add User Story 4 → Test independently → Deploy/Demo (cancellation support)
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 + tests
   - Developer B: User Story 3 + tests (can start in parallel with US1)
   - Developer C: User Story 4 + tests (can start in parallel with US1)
3. User Story 2 builds on US1, so starts after US1 core implementation
4. Stories complete and integrate independently

---

## Task Completion Summary

**Total Tasks**: 88

**Tasks by User Story**:
- Setup: 4 tasks
- Foundational: 7 tasks
- User Story 1 (P1): 18 tasks
- User Story 2 (P2): 13 tasks
- User Story 3 (P2): 17 tasks
- User Story 4 (P3): 15 tasks
- Polish: 14 tasks

**Parallel Opportunities**: 35 tasks marked [P] can run in parallel

**Independent Test Criteria**:
- **US1**: Execute any command, verify output displays
- **US2**: Execute various commands, verify all result details display
- **US3**: Test destructive command shows confirmation, safe command doesn't
- **US4**: Execute long command, cancel it, verify process terminates

**Suggested MVP Scope**: Phase 1 + Phase 2 + Phase 3 (User Story 1 only) = ~4-5 hours

---

## Notes

- [P] tasks = different files or independent sections, no sequential dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests must fail before implementing features (TDD approach per constitution)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Constitution requires >80% test coverage for critical paths (command execution, timeout, cancellation)
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All file paths are absolute and reference src-tauri/ or src/ directories
