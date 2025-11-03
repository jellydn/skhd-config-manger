# Implementation Plan: Command and Application Picker for Hotkey Setup

**Branch**: `007-hotkey-command-picker` | **Date**: 2025-11-03 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/007-hotkey-command-picker/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement visual pickers and browser interfaces to replace text-only command entry in the hotkey editor. Users will be able to:
1. Browse and select installed macOS applications with icons and search
2. Discover and select common shell commands from categorized templates
3. Navigate file system to select custom scripts/executables
4. Use pre-configured command templates with customizable parameters

This dramatically improves usability by eliminating the need to manually type application paths or remember command syntax.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework)
**Primary Dependencies**: Tauri v2, rfd (Rust file dialogs), existing skhd parser
**Storage**: File-based (command templates as embedded JSON/YAML), no database required
**Testing**: cargo test (Rust backend), Vitest (frontend)
**Target Platform**: macOS 11+ (Tauri v2 requirement), Universal binary (Intel + Apple Silicon)
**Project Type**: Desktop application (Tauri web + Rust backend)
**Performance Goals**:
  - Application list load: <500ms for 100 apps, <2s for 500+ apps
  - Search filtering: <200ms response time (SC-004)
  - UI responsiveness: <16ms frame time for 60fps
**Constraints**:
  - Must work with existing skhd configuration format
  - Must integrate with existing hotkey editing UI
  - File validation must prevent invalid configurations
**Scale/Scope**:
  - Support 500+ installed applications
  - 20-30 initial command templates (expandable)
  - 5-10 new Svelte components
  - 8-10 new Tauri backend commands

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### I. Native macOS Experience ✅

**Status**: COMPLIANT

- Application picker will use native file dialogs via `rfd` crate
- UI follows existing Svelte component patterns with macOS styling
- Icons displayed using macOS application bundles
- Search and filtering follows macOS conventions

### II. Configuration Safety ✅

**Status**: COMPLIANT

- Command insertion uses existing validated configuration writing
- Path escaping ensures no syntax errors (FR-014)
- All commands validated before insertion (SC-006: zero syntax errors)
- User can manually edit after picker insertion (FR-015)
- No direct skhd config modifications - only command text generation

### III. Test Coverage ✅

**Status**: COMPLIANT - TEST COVERAGE REQUIRED

- Unit tests required for:
  - Application discovery logic (macOS bundle parsing)
  - Command template parameter substitution
  - Path escaping and command syntax generation
  - Search/filter algorithms
- Integration tests required for:
  - Tauri command invocations (app list, file picker)
  - Command insertion into hotkey editor
- Manual testing required for:
  - UI interactions in all pickers
  - Search performance with large application lists
  - Keyboard navigation and accessibility

**Target Coverage**: >80% for Rust backend logic (application discovery, template processing, path validation)

### IV. Performance Standards ✅

**Status**: COMPLIANT WITH CLARIFICATION

- Application list loading: Clarified to load all at once (may lag with 500+ apps) - acceptable tradeoff per user decision
- Search response: <200ms target (SC-004)
- No blocking on main thread: async Tauri commands for I/O operations
- Memory footprint: Minimal increase (<10MB for app list cache)

**Clarification Note**: User explicitly chose Option A (load all apps at once) accepting potential lag with 500+ applications. This is documented in clarifications and meets the "simple architecture" principle.

### V. Simple Architecture ✅

**Status**: COMPLIANT

- Command templates stored as embedded JSON (no external database)
- Direct macOS API calls for application discovery
- No complex state management - pickers are ephemeral dialogs
- Straightforward Tauri command pattern: frontend requests → backend provides data → frontend displays

### Gate Summary

**Overall Status**: ✅ PASS - All principles compliant

- No violations requiring justification
- Performance tradeoff (load all apps) explicitly accepted by user in clarification
- Architecture remains simple with file-based templates and direct macOS APIs

## Project Structure

### Documentation (this feature)

```text
specs/007-hotkey-command-picker/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── tauri-commands.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/src/
├── commands/
│   ├── applications.rs      # NEW: macOS application discovery
│   ├── templates.rs          # NEW: command template management
│   └── file_picker.rs        # NEW: enhanced file selection
├── models/
│   ├── application.rs        # NEW: Application entity
│   ├── command_template.rs   # NEW: CommandTemplate, CommandParameter entities
│   └── command_category.rs   # NEW: CommandCategory entity
├── services/
│   ├── app_discovery.rs      # NEW: macOS bundle scanning
│   ├── template_loader.rs    # NEW: load/filter templates
│   └── path_validator.rs     # NEW: path escaping and validation
└── data/
    └── command_templates.json # NEW: embedded template definitions

src/
├── components/
│   ├── pickers/
│   │   ├── ApplicationPicker.svelte   # NEW: application browser
│   │   ├── CommandPicker.svelte       # NEW: command template browser
│   │   ├── FilePicker.svelte          # NEW: file system browser
│   │   └── TemplateParameterForm.svelte # NEW: parameter input form
│   └── ShortcutForm.svelte            # MODIFIED: add picker buttons
└── services/
    ├── applicationService.ts   # NEW: frontend service for app picker
    ├── commandService.ts       # NEW: frontend service for command picker
    └── templateService.ts      # NEW: frontend service for templates

tests/
├── unit/
│   ├── app_discovery_test.rs     # NEW: test macOS app scanning
│   ├── template_processing_test.rs # NEW: test parameter substitution
│   └── path_validation_test.rs    # NEW: test escaping logic
└── integration/
    └── picker_commands_test.rs    # NEW: test Tauri command integration
```

**Structure Decision**: Tauri desktop application structure (existing). This feature adds:
- 3 new Rust modules (commands/applications, commands/templates, commands/file_picker)
- 4 new Rust services (app_discovery, template_loader, path_validator)
- 3 new data models (Application, CommandTemplate, CommandCategory)
- 4 new Svelte components (pickers directory)
- 1 embedded data file (command_templates.json)

The existing project structure (src/ for frontend, src-tauri/src/ for backend) is maintained. No architectural changes needed.

## Complexity Tracking

No violations - this section is not needed for this feature.

---

## Phase 0: Research & Technical Decisions

See [research.md](./research.md) for detailed investigation of:

1. **macOS Application Discovery**
   - How to enumerate installed applications from /Applications, ~/Applications, /System/Applications
   - Parsing .app bundles to extract names, icons, bundle identifiers, executable paths
   - Handling non-standard application structures
   - Performance considerations for scanning 500+ applications

2. **Command Template Design**
   - JSON/YAML schema for template definitions
   - Parameter substitution patterns (e.g., {volume_level}, {brightness})
   - Category organization (System, Media, Window Management, etc.)
   - Template validation and preview generation

3. **Path Validation & Escaping**
   - Shell escaping for paths with spaces and special characters
   - Validation for executable files and permissions
   - Interpreter detection for script files (.sh, .py, .rb, etc.)

4. **Svelte 5 Dialog Patterns**
   - Modal dialog component patterns
   - Search/filter implementation with real-time updates
   - Keyboard navigation and accessibility
   - Integration with existing ShortcutForm component

5. **File Picker Enhancement**
   - Using rfd crate for native file dialogs vs custom UI
   - Filtering for executable files
   - Permission checking before selection

---

## Phase 1: Design Artifacts

### Data Model

See [data-model.md](./data-model.md) for complete entity definitions including:

- **Application**: display_name, icon_path, bundle_id, executable_path, app_path
- **CommandTemplate**: id, name, description, category, command_pattern, parameters[]
- **CommandParameter**: name, description, data_type, default_value, validation_regex
- **CommandCategory**: id, name, description, icon

### API Contracts

See [contracts/tauri-commands.md](./contracts/tauri-commands.md) for Tauri command definitions:

**Backend Commands (Rust → Frontend)**:
- `get_installed_applications() -> Vec<Application>`
- `get_command_templates(category?: string) -> Vec<CommandTemplate>`
- `get_command_categories() -> Vec<CommandCategory>`
- `validate_file_executable(path: string) -> Result<bool, string>`
- `open_file_picker(filter: string) -> Result<Option<string>, string>`
- `generate_command_from_template(template_id: string, params: Map<string, string>) -> Result<string, string>`
- `escape_shell_path(path: string) -> string`

**Frontend Services (TypeScript)**:
- `applicationService.listApplications(): Promise<Application[]>`
- `applicationService.searchApplications(query: string): Application[]`
- `commandService.listTemplates(category?: string): Promise<CommandTemplate[]>`
- `commandService.generateCommand(template: CommandTemplate, params: Record<string, string>): Promise<string>`
- `templateService.getCategories(): Promise<CommandCategory[]>`

### Quickstart Guide

See [quickstart.md](./quickstart.md) for developer setup:
- How to add new command templates
- How to test application discovery locally
- How to run picker component tests
- How to verify path escaping edge cases

---

## Phase 2: Task Breakdown

**Note**: Task breakdown is generated by `/speckit.tasks` command (not part of `/speckit.plan`).

See [tasks.md](./tasks.md) when available for dependency-ordered implementation tasks.

---

## Implementation Notes

### Integration Points

1. **ShortcutForm.svelte** (existing component)
   - Add 3 buttons: "Browse Applications", "Browse Commands", "Use Template"
   - Wire buttons to open respective picker components
   - Handle command text insertion from pickers

2. **skhd Parser** (existing)
   - No changes needed - pickers generate valid command strings
   - Existing validation logic applies to generated commands

3. **Configuration Writer** (existing)
   - No changes needed - command text inserted into existing flow
   - Existing safety mechanisms (backup, validation) apply

### Risk Mitigation

**Risk**: Application scanning performance on systems with 500+ apps
**Mitigation**:
  - User accepted this tradeoff in clarifications
  - Implement async scanning with progress indicator
  - Cache results for session duration
  - Future optimization: virtualized list rendering (deferred)

**Risk**: Command template parameters may not cover all use cases
**Mitigation**:
  - User can manually edit after template insertion (FR-015)
  - Start with 20-30 templates, gather feedback, expand iteratively
  - Template JSON is easy to extend without code changes

**Risk**: Path escaping edge cases on different shells
**Mitigation**:
  - Comprehensive test suite for escaping logic (>80% coverage target)
  - Test with spaces, quotes, backslashes, special chars
  - Document any limitations in template descriptions

### Future Enhancements (Out of Scope)

- Virtualized list rendering for 500+ applications
- User-defined custom templates
- Cloud template marketplace
- Automatic command suggestions based on usage
- Application version detection for duplicate disambiguation
