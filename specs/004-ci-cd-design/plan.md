# Implementation Plan: CI/CD Pipeline and Design Assets

**Branch**: `004-ci-cd-design` | **Date**: 2025-11-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-ci-cd-design/spec.md`

## Summary

Implement automated CI/CD pipeline using GitHub Actions to build, test, and release the macOS application. Create professional design assets (application icon, DMG background) following Apple Human Interface Guidelines. Enable tag-triggered releases with distributable DMG packages attached to GitHub Releases.

**Primary Requirements**: Automated builds on push/PR (FR-001, FR-002), GitHub releases on tag (FR-004, FR-005), custom macOS icon (FR-008), DMG installer customization (FR-010), pre-release versioning support (FR-013).

**Technical Approach**: GitHub Actions workflows for CI/CD, Tauri bundling for DMG creation, .icns icon generation, asset integration via Tauri configuration.

## Technical Context

**Language/Version**: YAML (GitHub Actions workflows), Rust 1.75+, TypeScript 5+, macOS 11+
**Primary Dependencies**: GitHub Actions (tauri-apps/tauri-action), Tauri CLI v2, cargo, bun, clippy, existing test suite
**Storage**: N/A (CI/CD infrastructure, assets stored in git)
**Testing**: cargo test, cargo clippy, bun run typecheck (existing test commands from constitution)
**Target Platform**: macOS 11+ (Big Sur) minimum, Universal binary (Intel + Apple Silicon)
**Project Type**: DevOps/Infrastructure (GitHub Actions workflows in .github/workflows/)
**Performance Goals**: <10 minutes build time (SC-001), <15 minutes release time (SC-006), <100ms test execution overhead
**Constraints**: Free GitHub Actions tier, macOS runners availability, no code signing initially (out of scope)
**Scale/Scope**: Single Tauri application, macOS-only builds initially, 3-5 workflows (CI, release, potentially PR checks)

**Design Asset Requirements**:
- Icon format: NEEDS CLARIFICATION (tooling for .icns generation)
- Icon sizes: 16x16 to 1024x1024 with @2x Retina variants (FR-008)
- DMG customization: NEEDS CLARIFICATION (Tauri DMG background capabilities)
- Asset source: NEEDS CLARIFICATION (purchase vs custom design vs generation tools)
- Integration approach: NEEDS CLARIFICATION (Tauri config structure for icons)

**CI/CD Implementation Details**:
- Workflow triggers: NEEDS CLARIFICATION (exact GitHub Actions trigger syntax)
- Tauri bundling: NEEDS CLARIFICATION (tauri-apps/tauri-action configuration)
- Release automation: NEEDS CLARIFICATION (GitHub release creation from tags)
- Pre-release detection: NEEDS CLARIFICATION (GitHub tag parsing for alpha/beta/rc)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### ✅ I. Native macOS Experience
- **Application**: Icon must follow Apple HIG (rounded square, clear at small sizes) ✅
- **DMG Experience**: Professional installer with custom background ✅
- **Status**: PASS - Design requirements align with native macOS expectations

### ⚠️ II. Configuration Safety (NON-NEGOTIABLE)
- **Application**: CI/CD does not directly modify skhd configuration
- **Risk**: Failed builds could break existing functionality
- **Mitigation**: Tests must pass before merge (FR-003), no direct config changes
- **Status**: PASS - Tests enforce configuration safety before merging

### ✅ III. Test Coverage
- **Application**: CI runs all existing tests (cargo test, clippy, typecheck) per FR-002
- **Requirement**: >80% parser/file operation coverage already exists (from 001-skhd-config-gui)
- **CI Enforcement**: PR merge blocked on test failures (FR-003)
- **Status**: PASS - Leverages existing test suite, adds enforcement

### ✅ IV. Performance Standards
- **Build Time**: <10 minutes target (SC-001) aligns with constitution responsiveness
- **Release Time**: <15 minutes (SC-006) enables fast iteration
- **Bundle Size**: <20MB target from constitution (need to verify in CI)
- **Status**: PASS - Performance targets meet constitution standards

### ✅ V. Simple Architecture
- **Approach**: GitHub Actions (free, integrated), Tauri bundling (built-in)
- **Complexity**: No custom infrastructure, no external services
- **Justification**: Standard CI/CD, no premature abstraction
- **Status**: PASS - Uses simplest approach for automated builds

### 🎯 Gate Summary
**Status**: ✅ PASS - All constitution principles satisfied
**Violations**: None
**Justification Required**: None
**Proceed to Phase 0**: YES

## Project Structure

### Documentation (this feature)

```text
specs/004-ci-cd-design/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output: GitHub Actions, Tauri bundling, icon tooling
├── data-model.md        # Phase 1 output: Workflow configuration structure, asset schema
├── quickstart.md        # Phase 1 output: How to trigger builds, create releases
├── contracts/           # Phase 1 output: GitHub Actions workflow YAML schemas
│   ├── ci.yml           # CI workflow contract (build + test on push/PR)
│   ├── release.yml      # Release workflow contract (build + publish on tag)
│   └── asset-schema.md  # Design asset requirements and directory structure
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created yet)
```

### Source Code (repository root)

```text
# GitHub Actions Workflows (new)
.github/
└── workflows/
    ├── ci.yml           # Build and test on push/PR
    ├── release.yml      # Build and release on tag (v*)
    └── pr-checks.yml    # Optional: PR-specific validations

# Design Assets (new)
assets/
├── icons/
│   ├── app-icon.png     # Source icon (1024x1024)
│   ├── icon.icns        # macOS icon bundle (generated)
│   └── icon-sources/    # Design source files (Sketch/Figma/AI)
└── dmg/
    ├── background.png   # DMG installer background (Retina @2x)
    └── background@2x.png

# Tauri Configuration (modified)
src-tauri/
├── Cargo.toml           # May need version field updates
├── tauri.conf.json      # Icon and bundle configuration
└── icons/               # Generated icon variants (Tauri CLI output)

# Existing structure (unchanged)
src/                     # Frontend code
tests/                   # Test suite
```

**Structure Decision**: Use `.github/workflows/` for CI/CD (GitHub Actions standard), `assets/` for design source files, integrate with existing `src-tauri/` for Tauri bundling configuration. Follows constitution's Simple Architecture principle (standard locations, no custom infrastructure).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations detected. All complexity is justified by explicit requirements:
- GitHub Actions: Industry standard, free tier, no simpler alternative for automated CI/CD
- Tauri bundling: Built-in feature, no additional complexity
- Asset creation: One-time design work, stored in git, no ongoing complexity

## Post-Design Constitution Re-Evaluation

_Re-checked after Phase 0 (Research) and Phase 1 (Design) completion._

### ✅ I. Native macOS Experience
**Status**: ✅ PASS - Design Enhanced

**Design Decisions**:
- Icon generation uses Apple HIG-compliant workflow (1024x1024 → .icns)
- DMG customization follows macOS installer conventions (drag-to-Applications)
- Universal binary support ensures native performance (Intel + Apple Silicon)
- Design assets integrate via Tauri's macOS-native bundling

**Validation**: All design choices enhance native macOS experience. No compromises made.

---

### ✅ II. Configuration Safety (NON-NEGOTIABLE)
**Status**: ✅ PASS - No Impact

**Design Decisions**:
- CI/CD does not touch skhd configuration files directly
- Test suite (from 001-skhd-config-gui) validates configuration safety
- CI enforces test passage before merge (FR-003)

**Validation**: No design decisions impact configuration safety. Tests enforce safety.

---

### ✅ III. Test Coverage
**Status**: ✅ PASS - Enhanced Enforcement

**Design Decisions**:
- CI workflow runs all existing tests automatically (cargo test, clippy, typecheck)
- PR merge blocked on test failures (GitHub branch protection + FR-003)
- No new test requirements (existing >80% parser coverage from 001-skhd-config-gui)

**Validation**: Design enhances test enforcement through automation. Coverage maintained.

---

### ✅ IV. Performance Standards
**Status**: ✅ PASS - Targets Met

**Design Decisions**:
- Build time target: <10 minutes (SC-001) - achievable with macOS runners
- Release time target: <15 minutes (SC-006) - validated in research
- Bundle size: Universal DMG target <20MB - will validate in implementation
- Asset sizes: Icon <500KB, DMG background <2MB - reasonable for constitution target

**Validation**: All performance targets align with constitution standards. Research confirms feasibility.

---

### ✅ V. Simple Architecture
**Status**: ✅ PASS - Simplicity Maintained

**Design Decisions**:
- Uses GitHub Actions (built-in, no external services)
- Uses Tauri CLI for icon generation (built-in tool)
- Uses tauri-apps/tauri-action (official, no custom scripting)
- Assets stored in git (no external asset management)

**Validation**: Zero additional complexity introduced. All tooling is standard and built-in.

---

### 🎯 Post-Design Gate Summary

**Status**: ✅✅ PASS ALL CHECKS
**Violations**: 0
**New Complexity**: None
**Constitution Alignment**: 100%

**Key Findings**:
- All design decisions enhance or maintain constitution compliance
- No compromises required for any constitution principle
- Implementation will use standard, well-supported tooling
- Performance targets validated through research

**Proceed to Phase 2 (Tasks)**: ✅ YES

---

## Phase Summary

### Phase 0: Research (Completed)
- ✅ Resolved all "NEEDS CLARIFICATION" items
- ✅ Validated technical approaches (Tauri CLI, GitHub Actions, DMG bundling)
- ✅ Documented decisions in `research.md`

### Phase 1: Design (Completed)
- ✅ Defined data models in `data-model.md` (workflow + asset schemas)
- ✅ Created implementation contracts in `contracts/` (ci.yml, release.yml, asset-schema.md)
- ✅ Wrote developer quickstart guide in `quickstart.md`
- ✅ Updated agent context (CLAUDE.md) with new technologies

### Phase 2: Tasks (Next Step)
- Run `/speckit.tasks` to generate implementation task breakdown
- Tasks will reference contracts and data models from Phase 1

---

**Planning Phase Complete**: Ready for `/speckit.tasks`
