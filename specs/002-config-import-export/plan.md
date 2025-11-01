# Implementation Plan: Configuration Import/Export

**Branch**: `002-config-import-export` | **Date**: 2025-11-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-config-import-export/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature adds configuration import/export capabilities to the skhd GUI manager. Users will be able to load configurations from custom file locations, export current configurations to new files, and reload from disk with unsaved change warnings. This enables testing multiple configurations, backing up customizations, and sharing configurations across machines. The implementation uses Tauri's native file dialog APIs for macOS-native file picking and maintains configuration safety through atomic operations and validation.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework)
**Primary Dependencies**: Tauri v2, rfd (Rust file dialogs), existing skhd parser
**Storage**: File-based (skhd configuration files), no database required
**Testing**: cargo test (Rust unit/integration), vitest (TypeScript), manual E2E
**Target Platform**: macOS 11+ (Big Sur), Universal binary (Intel + Apple Silicon)
**Project Type**: Desktop application (Tauri hybrid - Rust backend + Svelte frontend)
**Performance Goals**: File operations <100ms, UI updates <16ms frame time
**Constraints**: No network access, file system permissions required, atomic writes mandatory
**Scale/Scope**: Single user, typical configs <1000 lines, 3 new Tauri commands, 2 new UI components

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### ✅ Native macOS Experience

- **Compliance**: Uses native macOS file dialogs via rfd crate (Tauri standard)
- **Evidence**: File picker will use system dialog, respects macOS accessibility
- **UI Impact**: Import/Export/Reload buttons follow existing native button patterns

### ✅ Configuration Safety (NON-NEGOTIABLE)

- **Compliance**: PASS - Existing atomic write infrastructure will be reused
- **Evidence**:
  - Import: Validates before displaying (existing parser)
  - Export: Uses tempfile + atomic rename pattern (existing save_config logic)
  - Reload: Warns before discarding unsaved changes (new confirmation dialog)
- **Risk Mitigation**: All file operations go through existing validated ConfigState

### ✅ Test Coverage

- **Compliance**: PASS - Will add tests for new commands and file dialog paths
- **Test Plan**:
  - Unit: Test import/export command logic with temp files
  - Integration: Test full round-trip (import → edit → export → verify)
  - Manual: File dialog behavior, error messages, permission handling

### ✅ Performance Standards

- **Compliance**: PASS - File dialog and I/O operations are async, non-blocking
- **Evidence**:
  - File dialogs run async (rfd AsyncFileDialog)
  - Config parsing already meets <100ms target
  - UI updates use Svelte reactivity (non-blocking)

### ✅ Simple Architecture

- **Compliance**: PASS - Extends existing config command pattern
- **Justification**: No new abstractions, reuses ConfigState pattern
- **Design**:
  - 3 new Tauri commands: `import_config`, `export_config`, `reload_config`
  - Reuses existing `load_config` and `save_config` internals
  - File paths tracked in ConfigState (simple field addition)

**GATE RESULT**: ✅ PASS - All constitutional principles satisfied

## Project Structure

### Documentation (this feature)

```text
specs/002-config-import-export/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── commands.yaml    # Tauri command signatures
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── commands/
│   │   ├── config.rs          # EXISTING: Extend with import/export/reload
│   │   └── mod.rs             # EXISTING: Register new commands
│   ├── models/
│   │   └── config_file.rs     # EXISTING: Add current_path field
│   └── lib.rs                 # EXISTING: Add new commands to invoke_handler
├── tests/
│   ├── integration/
│   │   └── config_import_export.rs  # NEW: Integration tests
│   └── fixtures/
│       └── test-configs/       # NEW: Test skhd files

src/
├── services/
│   └── tauri.ts               # EXISTING: Add import/export/reload bindings
├── components/
│   └── ConfirmDialog.svelte   # NEW: Reusable confirmation dialog
└── routes/
    └── +page.svelte           # EXISTING: Add Import/Export/Reload buttons

tests/
└── unit/
    └── tauri-service.test.ts  # EXISTING: Add tests for new service methods
```

**Structure Decision**: Single Tauri project structure (Option 1). Backend commands in `src-tauri/src/commands/config.rs`, frontend UI in `src/routes/+page.svelte`. This follows existing patterns established in Feature 001 and maintains constitutional simplicity principle.

## Complexity Tracking

> **No violations to justify** - This feature fully complies with all constitutional principles without exceptions.
