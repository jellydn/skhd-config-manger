# Implementation Plan: System Theme Support

**Branch**: `008-system-theme` | **Date**: 2025-11-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/008-system-theme/spec.md`

## Summary

This feature enables automatic detection and application of macOS system theme preferences (light/dark mode) in the application. The app will detect the current system theme on launch, apply it immediately, and dynamically update when users change their macOS system appearance settings. The implementation integrates macOS system APIs via Tauri to monitor theme changes and updates all UI elements consistently without manual configuration.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework)  
**Primary Dependencies**: Tauri v2, objc crate (macOS Objective-C runtime bindings for theme detection)  
**Storage**: N/A (theme state is runtime-only, no persistence required)  
**Testing**: cargo test (Rust backend), Vitest (frontend), manual testing for theme transitions  
**Target Platform**: macOS 11+ (Big Sur minimum for Tauri v2, system theme APIs available)  
**Project Type**: Tauri desktop application (Rust backend + Svelte frontend)  
**Performance Goals**: Theme detection <100ms on launch, theme updates <2s as per spec, UI responsive <16ms frame time  
**Constraints**: Must not block UI thread, handle theme detection failures gracefully, support older macOS versions with fallback  
**Scale/Scope**: Single-user desktop app, all existing UI components (~15+ components to update)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### ✅ Native macOS Experience
- **Compliance**: System theme integration is a core macOS native feature requirement
- **Actions Required**: Use native macOS system APIs for theme detection, follow Apple HIG for theme transitions, ensure theme matches system appearance settings automatically

### ✅ Configuration Safety (NON-NEGOTIABLE)
- **Compliance**: Theme changes do not affect skhd configuration files or data
- **Actions Required**: Theme is visual-only, no impact on configuration parsing or storage

### ✅ Test Coverage
- **Compliance**: Will test theme detection, theme application, and transition handling
- **Actions Required**: Unit tests for theme detection logic, integration tests for theme change events, manual UI testing for visual consistency

### ✅ Performance Standards
- **Compliance**: Theme detection must be fast (<100ms), updates non-blocking, <2s transition time per spec
- **Actions Required**: Efficient CSS variable updates, avoid blocking operations, optimize theme transition rendering

### ✅ Simple Architecture
- **Compliance**: Direct system API integration, straightforward CSS variable updates
- **Actions Required**: Use Tauri commands for system theme detection, follow existing Tauri command/event patterns, avoid unnecessary abstraction layers

**Constitution Check Result**: ✅ PASS - All principles aligned, no violations

### Post-Design Constitution Re-Check (Phase 1 Complete)

**Re-evaluation Date**: 2025-11-02

✅ **Native macOS Experience** - CONFIRMED
- Using native macOS NSUserDefaults and NSDistributedNotificationCenter APIs
- Theme detection follows macOS system preferences automatically
- Theme transitions follow Apple HIG patterns (smooth, non-jarring)
- No cross-platform abstractions introduced
- No constitution violations introduced

✅ **Configuration Safety** - CONFIRMED
- Theme changes are visual-only, no impact on skhd configuration files
- No modifications to configuration parsing or storage logic
- Theme state is runtime-only, no persistence that could interfere with config
- No constitution violations introduced

✅ **Test Coverage** - CONFIRMED
- Unit tests: theme detection logic, NSUserDefaults access
- Integration tests: theme change event handling, Tauri event system
- Manual UI testing: visual consistency, theme transitions
- Coverage targets maintained for new code
- No constitution violations introduced

✅ **Performance Standards** - CONFIRMED
- Theme detection: <100ms target via efficient NSUserDefaults access
- Theme updates: <2s target per spec, non-blocking CSS variable updates
- No blocking operations on main thread
- CSS transitions optimized for smooth rendering
- No constitution violations introduced

✅ **Simple Architecture** - CONFIRMED
- Direct NSUserDefaults access (no wrappers)
- Straightforward CSS variable updates (no complex state management)
- Tauri command/event patterns followed (consistent with existing codebase)
- No unnecessary abstractions added
- No constitution violations introduced

**Final Constitution Check**: ✅ PASS - Design maintains full compliance

## Project Structure

### Documentation (this feature)

```text
specs/008-system-theme/
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
│   ├── commands/
│   │   └── theme.rs          # New: Tauri commands for theme detection
│   ├── services/
│   │   └── theme_monitor.rs   # New: Service to monitor system theme changes
│   └── lib.rs                 # Register theme commands

src/
├── routes/
│   └── +layout.svelte         # Update: Theme initialization and CSS variables
├── components/                # Update: All components to use CSS variables (no hardcoded colors)
└── services/
    └── themeService.ts        # New: Frontend service for theme state management
```

**Structure Decision**: Tauri desktop application structure maintained. New theme detection commands in Rust backend, theme state management in frontend. Existing CSS variable infrastructure will be enhanced for dynamic theme switching.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations - complexity tracking not required.
