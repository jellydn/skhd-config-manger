# Implementation Plan: skhd Configuration GUI

**Branch**: `001-skhd-config-gui` | **Date**: 2025-11-01 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-skhd-config-gui/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a native macOS GUI application for managing skhd keyboard shortcut configurations. The application enables users to view, edit, add, and delete keyboard shortcuts through a visual interface instead of manually editing text files. Core requirements include safe configuration file handling with automatic backups, real-time validation, and a native macOS experience following Apple HIG. The application will use Tauri v2 with a Rust backend for file operations and parsing, and a web-based frontend for the UI.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), Svelte 4 + TypeScript 5 (frontend)
**Primary Dependencies**: Tauri v2, pest v2.7+ (parser), tempfile v3.8+ (atomic writes), Vite v5 (build), Vitest v1+ (testing)
**Storage**: File-based (skhd config at ~/.config/skhd/skhdrc), backup files with timestamps
**Testing**: cargo test + cargo tarpaulin (Rust backend), Vitest + @testing-library/svelte (frontend)
**Target Platform**: macOS 11+ (Big Sur and later), Universal binary (Intel + Apple Silicon)
**Project Type**: Desktop application (Tauri-based with Rust backend and Svelte frontend)
**Performance Goals**: <2s cold start, <500ms warm start, <100ms config parsing for 1000 lines, <16ms frame time (60fps UI)
**Constraints**: <50MB idle memory, <100MB active memory, <20MB bundle size, offline-only (no network), file system permissions required
**Scale/Scope**: Single-user desktop app, typical config files <1000 lines, ~10-100 keyboard shortcuts per config

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### I. Native macOS Experience ✅

- **Requirement**: Application must feel native to macOS, follow Apple HIG, use native controls
- **Status**: PASS - Plan specifies Tauri v2 on macOS 11+, Universal binary, native UI patterns
- **Evidence**: Feature spec requires macOS-specific behavior, dark mode support, accessibility standards

### II. Configuration Safety (NON-NEGOTIABLE) ✅

- **Requirement**: Never corrupt or lose skhd configuration data. Backup before modification, validate before writing, atomic file operations, undo/redo capability
- **Status**: PASS - Requirements FR-006, FR-007 mandate validation and automatic backups
- **Evidence**: FR-007 (automatic backup), FR-006 (validation), FR-009 (undo/redo), atomic file operations planned

### III. Test Coverage ✅

- **Requirement**: Unit tests for parser/file ops, integration tests for read/write cycles, >80% coverage for parsing and file operations
- **Status**: PASS - Testing framework specified (cargo test + frontend tests)
- **Evidence**: Plan includes test structure, constitution allows manual UI testing

### IV. Performance Standards ✅

- **Requirement**: <2s cold start, <500ms warm start, <100ms parsing, <16ms frame time, <50MB idle, <100MB active, <20MB bundle
- **Status**: PASS - Performance goals directly match constitution requirements
- **Evidence**: Technical Context specifies all required performance targets

### V. Simple Architecture ✅

- **Requirement**: Direct file parsing, no database unless required, Tauri conventions, avoid unnecessary abstraction
- **Status**: PASS - File-based storage, straightforward Tauri architecture planned
- **Evidence**: Storage specified as file-based, no database, follows Tauri backend/frontend separation

**Overall Status**: ✅ PASS - All constitutional requirements met. No violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── main.rs              # Tauri application entry point
│   ├── commands/            # Tauri commands (backend API)
│   │   ├── mod.rs
│   │   ├── config.rs        # Config file operations (load, save, backup)
│   │   └── parser.rs        # skhd syntax parsing and validation
│   ├── models/              # Data structures
│   │   ├── mod.rs
│   │   ├── shortcut.rs      # Keyboard shortcut model
│   │   └── config.rs        # Configuration file model
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   ├── file_io.rs       # File operations with atomic writes
│   │   ├── validation.rs    # Configuration validation logic
│   │   └── backup.rs        # Backup management
│   └── utils/               # Utility functions
│       ├── mod.rs
│       └── path.rs          # Path resolution (~/.config/skhd/skhdrc)
├── tests/
│   ├── integration/         # Integration tests for file ops
│   │   ├── config_lifecycle.rs
│   │   └── backup_restore.rs
│   └── unit/                # Unit tests for parser and validation
│       ├── parser_tests.rs
│       └── validation_tests.rs
├── Cargo.toml
└── tauri.conf.json

src/
├── main.{js,jsx,ts,tsx}     # Frontend entry point (extension TBD)
├── App.{js,jsx,ts,tsx}      # Root component
├── components/              # UI components
│   ├── ShortcutList.{ext}   # List view of shortcuts
│   ├── ShortcutEditor.{ext} # Add/edit shortcut form
│   ├── ShortcutItem.{ext}   # Individual shortcut display
│   ├── SearchBar.{ext}      # Search/filter shortcuts
│   └── EmptyState.{ext}     # Empty state when no config
├── hooks/                   # Custom hooks (if React/Vue)
│   ├── useConfig.{ext}      # Config state management
│   └── useShortcuts.{ext}   # Shortcut CRUD operations
├── services/                # Frontend services
│   └── tauri.{ext}          # Tauri command invocations
├── styles/                  # CSS/styling
│   └── main.css
└── __tests__/               # Frontend tests (framework-dependent)
    └── components/
```

**Structure Decision**: Tauri-based desktop application structure. Rust backend in `src-tauri/` handles all file I/O, parsing, and validation to ensure configuration safety. Web-based frontend in `src/` provides the UI layer. This separation aligns with Constitution V (Simple Architecture) by keeping concerns clearly separated: Rust for critical file operations, web tech for flexible UI development.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**No violations to report** - All constitutional requirements met in design phase.

---

## Post-Design Constitution Re-Check

_Re-evaluation after Phase 1 design completion_

### I. Native macOS Experience ✅

- **Status**: PASS - Svelte chosen for performance, pest parser for accuracy
- **Evidence**: Svelte delivers smallest bundle (~5-10MB vs React ~15-20MB), meets <20MB target

### II. Configuration Safety (NON-NEGOTIABLE) ✅

- **Status**: PASS - Detailed atomic write implementation, backup strategy defined
- **Evidence**: tempfile + atomic rename pattern documented in research.md, all safety requirements addressed in contracts

### III. Test Coverage ✅

- **Status**: PASS - Comprehensive testing strategy defined
- **Evidence**: cargo test + Vitest specified, >80% coverage target for parser/file ops, manual UI testing acceptable

### IV. Performance Standards ✅

- **Status**: PASS - All performance targets achievable with chosen stack
- **Evidence**: Svelte performance characteristics, pest parser benchmarks support <100ms parsing target

### V. Simple Architecture ✅

- **Status**: PASS - No unnecessary abstraction layers introduced
- **Evidence**: Direct file operations, no database, straightforward Tauri command pattern

**Final Status**: ✅ ALL CHECKS PASS - Ready for implementation phase
