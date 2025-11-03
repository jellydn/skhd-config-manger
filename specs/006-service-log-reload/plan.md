# Implementation Plan: Service Log Viewer and Reload

**Branch**: `006-service-log-reload` | **Date**: 2025-11-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-service-log-reload/spec.md`

## Summary

This feature adds a real-time log viewer for the skhd service and provides GUI controls to reload the service and import configurations. Users can monitor skhd service activity through a live log display, import different configuration files, and restart the service without using terminal commands. The implementation focuses on seamless macOS service integration using launchctl, real-time log tailing, and leveraging existing configuration import functionality.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework)
**Primary Dependencies**: Tauri v2, tokio (async runtime), chrono (timestamps), existing skhd parser
**Storage**: File-based (skhd logs from system, no database required)
**Testing**: cargo test (Rust backend), Vitest (frontend), integration tests for service control
**Target Platform**: macOS 11+ (Big Sur minimum for Tauri v2)
**Project Type**: Tauri desktop application (Rust backend + Svelte frontend)
**Performance Goals**: Log updates <1s latency, service reload <5s, UI responsive <16ms frame time
**Constraints**: Must not block UI thread, handle large log files (>10MB), prevent concurrent reloads
**Scale/Scope**: Single-user desktop app, handle typical skhd log volumes (<1MB/day)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### ✅ Native macOS Experience
- **Compliance**: Using native macOS service control (launchctl), follows HIG for log viewer UI
- **Actions Required**: Design log viewer with native macOS patterns (monospaced font, color coding)

### ✅ Configuration Safety (NON-NEGOTIABLE)
- **Compliance**: Service reload uses existing import_config command with built-in validation
- **Actions Required**: Leverage existing configuration validation, provide clear error messages if service fails to start

### ✅ Test Coverage
- **Compliance**: Will test service control logic, log parsing, error handling
- **Actions Required**: Unit tests for log parsing, integration tests for launchctl interactions, manual UI testing

### ✅ Performance Standards
- **Compliance**: Non-blocking log tailing, async service control, efficient log rendering
- **Actions Required**: Implement virtual scrolling for large logs, debounce log updates

### ✅ Simple Architecture
- **Compliance**: Direct log file reading, straightforward launchctl integration
- **Actions Required**: No unnecessary abstractions, follow Tauri command/event patterns

**Constitution Check Result**: ✅ PASS - All principles aligned, no violations

### Post-Design Constitution Re-Check (Phase 1 Complete)

**Re-evaluation Date**: 2025-11-02

✅ **Native macOS Experience** - CONFIRMED
- Log viewer uses SF Mono monospace font (macOS standard)
- Service control follows macOS service patterns (launchctl)
- Color scheme respects dark mode
- No constitution violations introduced

✅ **Configuration Safety** - CONFIRMED
- Validation before import enforced by existing import_config command
- Configuration parsing errors shown to user before reload
- Uses battle-tested import_config from existing codebase
- No constitution violations introduced

✅ **Test Coverage** - CONFIRMED
- Unit tests: service_manager, log_parser
- Integration tests: service_control flow
- Component tests: LogViewer, ServiceControl
- Coverage targets maintained

✅ **Performance Standards** - CONFIRMED
- Log streaming: <1s latency via async tokio
- Service reload: <5s target
- Virtual scrolling prevents DOM bloat
- No blocking operations on main thread

✅ **Simple Architecture** - CONFIRMED
- Direct launchctl integration (no wrappers)
- Straightforward log parsing (regex)
- Tauri command/event patterns followed
- No unnecessary abstractions added

**Final Constitution Check**: ✅ PASS - Design maintains full compliance

## Project Structure

### Documentation (this feature)

```text
specs/006-service-log-reload/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (service control patterns, log formats)
├── data-model.md        # Phase 1 output (log entry structure, service state)
├── quickstart.md        # Phase 1 output (developer guide)
├── contracts/           # Phase 1 output (Tauri commands, events)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created yet)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── commands/
│   │   ├── service.rs           # NEW: Service control commands (reload, status)
│   │   └── logs.rs               # NEW: Log access commands
│   ├── services/
│   │   ├── service_manager.rs    # NEW: macOS launchctl integration
│   │   └── log_tailer.rs         # NEW: Real-time log monitoring
│   ├── models/
│   │   ├── log_entry.rs          # NEW: Log entry data structure
│   │   └── service_status.rs     # NEW: Service state representation
│   └── lib.rs                    # UPDATE: Register new commands
└── tests/
    ├── unit/
    │   ├── service_manager_test.rs  # NEW: Service control tests
    │   └── log_parser_test.rs       # NEW: Log parsing tests
    └── integration/
        └── service_control_test.rs  # NEW: End-to-end service tests

src/
├── components/
│   ├── LogViewer.svelte          # NEW: Log display component
│   └── (ServiceControl integrated in logs page)
├── services/
│   ├── service.ts                # NEW: Frontend service client
│   └── tauri.ts                  # UPDATE: Uses existing import_config
└── routes/
    └── logs/
        └── +page.svelte          # NEW: Log viewer page with integrated controls

tests/
└── __tests__/
    └── components/
        ├── LogViewer.test.ts     # NEW: Component tests
        └── ServiceControl.test.ts # NEW: Control tests
```

**Structure Decision**: Tauri monorepo structure with Rust backend (src-tauri/) and Svelte frontend (src/). Backend handles system interactions (launchctl, log files), frontend provides reactive UI. Follows existing project conventions established in features 001-005.

## Complexity Tracking

_No constitution violations - this section left empty._

