<!--
═══════════════════════════════════════════════════════════════════════════
SYNC IMPACT REPORT
═══════════════════════════════════════════════════════════════════════════
Version Change: 0.0.0 → 1.0.0
Type: MAJOR - Initial constitution establishment

Modified Principles:
- Initial establishment of all principles (not a modification)

Added Sections:
- Core Principles (Native macOS Experience, Configuration Safety, Test Coverage, Performance Standards, Simple Architecture)
- Platform Requirements
- Development Workflow
- Governance

Removed Sections: None

Templates Requiring Updates:
✅ .specify/templates/plan-template.md - Constitution Check section will reference these principles
✅ .specify/templates/spec-template.md - Requirements will align with Native macOS Experience and Configuration Safety
✅ .specify/templates/tasks-template.md - Task categorization will reflect Tauri/Swift/macOS testing requirements

Follow-up TODOs: None - all placeholders filled

Rationale: Initial constitution for skhd GUI Tauri app on macOS. Focuses on native experience, configuration safety, and macOS platform requirements.
═══════════════════════════════════════════════════════════════════════════
-->

# skhd-gui Constitution

## Core Principles

### I. Native macOS Experience
The application MUST feel native to macOS and integrate seamlessly with system conventions.

- Follow Apple Human Interface Guidelines for all UI/UX decisions
- Use native macOS controls and patterns (not generic cross-platform widgets)
- Support macOS-specific features: menu bar integration, keyboard shortcuts, dark mode
- Respect macOS accessibility standards (VoiceOver, keyboard navigation, reduced motion)
- Application should be indistinguishable from native Swift apps in look and behavior

**Rationale**: Users expect macOS apps to behave consistently with system conventions. Generic cross-platform UIs feel foreign and reduce user trust and adoption.

### II. Configuration Safety (NON-NEGOTIABLE)
The application MUST never corrupt or lose user skhd configuration data.

- Always backup existing configuration before any modification
- Validate all configuration changes before writing to disk
- Atomic file operations: write to temporary file, verify, then move
- Provide undo/redo capability for all configuration changes
- Never write partial or invalid skhd configuration syntax
- Clear error messages when configuration cannot be parsed or applied
- Read-only mode if configuration file permissions are insufficient

**Rationale**: skhd is a critical system tool. Configuration corruption can break user workflows or lock users out of their keyboard shortcuts. Safety is paramount.

### III. Test Coverage
Testing is required for configuration parsing, file operations, and critical user paths.

- Unit tests MUST cover: skhd config parser, file operations, validation logic
- Integration tests MUST cover: configuration read/write cycles, backup/restore
- Manual testing required for: UI interactions, accessibility features, keyboard shortcuts
- Tests run automatically before commits (pre-commit hook)
- Test coverage target: >80% for parsing and file operation code

**Rationale**: Configuration safety principle requires robust testing. Parser bugs or file operation errors have high impact. UI testing can be manual given Tauri/native complexity.

### IV. Performance Standards
The application MUST be responsive and lightweight as a macOS utility.

- Application launch: <2 seconds cold start, <500ms warm start
- Configuration parsing: <100ms for typical skhd files (<1000 lines)
- UI responsiveness: No blocking operations on main thread, <16ms frame time
- Memory footprint: <50MB idle, <100MB active editing
- CPU usage: <5% idle, <15% during configuration edits
- Bundle size: <20MB for distributable .app

**Rationale**: As a system utility, the app must be fast and lightweight. Users expect instant responsiveness when managing keyboard shortcuts.

### V. Simple Architecture
Prefer straightforward implementations that can evolve over premature abstraction.

- Start with direct skhd config file parsing (text-based)
- No database unless configuration history/versioning feature explicitly required
- Tauri backend handles file I/O and validation, frontend handles UI
- Follow Tauri conventions: commands for backend operations, events for updates
- Avoid unnecessary abstraction layers (no "repository pattern" for simple file reads)

**Rationale**: This is a configuration editor, not an enterprise system. Simple file operations and validation are sufficient. Complexity should be justified by concrete requirements.

## Platform Requirements

### macOS Targets
- **Minimum Version**: macOS 11 (Big Sur) - Tauri v2 requirement
- **Recommended**: macOS 12+ for optimal experience
- **Architecture**: Universal binary (Intel + Apple Silicon)

### Technology Stack
- **Framework**: Tauri v2 (Rust backend + web frontend)
- **Frontend**: HTML/CSS/JavaScript (or React/Vue/Svelte if complexity warrants)
- **Backend**: Rust for file operations, skhd parsing, validation
- **Testing**: Rust: cargo test + integration tests | Frontend: framework test tools
- **Distribution**: .dmg installer with code signing (when ready for release)

### Security & Permissions
- Request file system access explicitly (macOS sandboxing)
- Code signing required for distribution outside development
- No network access required (local file operations only)
- Document required permissions in README and during installation

## Development Workflow

### Code Organization
- **Tauri Structure**: Follow Tauri conventions (src-tauri/ for Rust, src/ for frontend)
- **Configuration Logic**: Centralize skhd parsing and validation in Rust backend
- **UI Components**: Modular components for config editor, syntax highlighting, preview
- **Tests**: `src-tauri/tests/` for Rust, `src/__tests__/` for frontend

### Quality Gates
- All Rust code must pass: `cargo clippy`, `cargo fmt --check`, `cargo test`
- All frontend code must pass framework linting and type checking
- Pre-commit hook runs linters and tests automatically
- No warnings in production builds (clippy warnings = errors)

### Version Control
- Commit atomic changes with descriptive messages
- Feature branches for new capabilities (e.g., `feature/syntax-highlighting`)
- Tag releases with semantic versioning (v1.0.0, v1.1.0, etc.)

### Release Process
- Test on both Intel and Apple Silicon hardware
- Verify code signing and notarization before public distribution
- Include release notes with user-facing changes and bug fixes

## Governance

This constitution supersedes all other development practices and guidelines. Any decision conflicting with these principles must be explicitly justified in writing.

### Amendment Process
1. Propose amendment with clear rationale and impacted sections
2. Update version following semantic versioning:
   - **MAJOR**: Backward incompatible principle removals or redefinitions
   - **MINOR**: New principles added or significant guidance expansions
   - **PATCH**: Clarifications, wording improvements, non-semantic fixes
3. Update all dependent templates (.specify/templates/) to maintain consistency
4. Document amendment in Sync Impact Report (at top of this file)

### Compliance Review
- All feature specifications must reference relevant principles
- All pull requests must verify compliance with constitution
- Implementation plans must justify any principle exceptions
- Retrospectives should identify constitution gaps or needed amendments

### Runtime Guidance
This constitution defines **what** principles govern the project. For **how** to implement day-to-day development tasks, refer to:
- `.claude/CLAUDE.md` for AI assistant development practices
- Tauri documentation for framework-specific patterns
- Apple Human Interface Guidelines for macOS UI/UX standards

**Version**: 1.0.0 | **Ratified**: 2025-11-01 | **Last Amended**: 2025-11-01
