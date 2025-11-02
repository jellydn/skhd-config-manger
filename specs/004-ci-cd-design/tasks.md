# Implementation Tasks: CI/CD Pipeline and Design Assets

**Feature**: `004-ci-cd-design`
**Branch**: `004-ci-cd-design`
**Date**: 2025-11-02
**Plan**: [plan.md](./plan.md)

---

## Task Organization

Tasks are organized by user story priority to enable independent implementation and testing.

**User Stories**:
- **US1 (P1)**: Automated Build and Release - CI workflows for quality enforcement
- **US2 (P2)**: Professional Brand Identity - Application icon and DMG assets
- **US3 (P3)**: Release Asset Distribution - GitHub release automation

**Legend**:
- `[P]` = Parallelizable (can run concurrently with other [P] tasks in same phase)
- `[US1]`, `[US2]`, `[US3]` = User story association
- File paths are relative to repository root unless specified

---

## Phase 1: Setup & Prerequisites

**Goal**: Initialize GitHub Actions infrastructure and asset directories

### Tasks

- [ ] T001 Create `.github/workflows/` directory structure
- [ ] T002 [P] Create `assets/icons/` directory for source icon files
- [ ] T003 [P] Create `assets/dmg/` directory for DMG background images
- [ ] T004 [P] Create `.github/workflows/.gitkeep` to track empty workflow directory

**Completion Criteria**: Directory structure exists and committed to git

---

## Phase 2: User Story 1 - Automated Build and Release (P1)

**Story Goal**: Developers can push code to `main` and PRs, triggering automated builds and tests. Failed tests block merges, ensuring code quality.

**Independent Test Criteria**:
1. Push code to main → CI workflow runs → All tests pass → Build succeeds
2. Create PR → CI workflow runs → Tests fail → PR merge blocked
3. Locally passing tests that fail in CI → Merge blocked (CI is source of truth)

### CI Workflow Implementation

- [ ] T005 [US1] Create `.github/workflows/ci.yml` with basic workflow structure (name, triggers)
- [ ] T006 [US1] Configure CI triggers in `ci.yml`: `on: push` for `main` branch
- [ ] T007 [US1] Add `on: pull_request` trigger for `main` branch in `ci.yml`
- [ ] T008 [US1] Define `test` job in `ci.yml` with `runs-on: macos-latest`
- [ ] T009 [US1] Add checkout step in `ci.yml`: `actions/checkout@v4`
- [ ] T010 [US1] Add Bun setup step in `ci.yml`: `oven-sh/setup-bun@v2`
- [ ] T011 [US1] Add Rust toolchain step in `ci.yml`: `dtolnay/rust-toolchain@stable`
- [ ] T012 [US1] Add dependency install step in `ci.yml`: `run: bun install`
- [ ] T013 [US1] Add test execution step in `ci.yml`: `cargo test && cargo clippy -- -D warnings && bun run typecheck`

**Reference Contract**: `specs/004-ci-cd-design/contracts/ci.yml`

### Validation & Testing

- [ ] T014 [US1] Commit `ci.yml` and push to trigger first CI run
- [ ] T015 [US1] Verify CI workflow appears in GitHub Actions tab
- [ ] T016 [US1] Verify all tests run successfully in CI (cargo test, clippy, typecheck)
- [ ] T017 [US1] Create test PR to verify CI runs on pull requests
- [ ] T018 [US1] Verify PR shows CI status (green checkmark or red X)

**Acceptance Validation**:
- ✅ AS1.1: Code pushed to main triggers CI → All tests pass
- ✅ AS1.2: Pull request created → CI status reported, merge blocked on failure
- ✅ AS1.4: Test failures show clear error messages in GitHub Actions logs

**Completion Criteria**: CI workflow runs on every push/PR, executes all tests, blocks merges on failure

---

## Phase 3: User Story 2 - Professional Brand Identity (P2)

**Story Goal**: Application displays custom, professional icon in all macOS interfaces (Dock, Finder, DMG) instead of generic defaults.

**Independent Test Criteria**:
1. Build app → Install → Custom icon appears in Dock (not default Tauri icon)
2. View app in Finder → Custom icon shows at all sizes (16px to 1024px, Retina)
3. Open DMG → Custom background and icon layout appear
4. Open About dialog → Custom icon and branding visible

### Icon Asset Creation

- [ ] T019 [P] [US2] Design or acquire source icon (1024x1024 PNG with transparency) and save to `assets/icons/app-icon.png`
- [ ] T020 [US2] Validate source icon meets requirements: exactly 1024x1024, PNG format, transparency, clear at 16x16
- [ ] T021 [US2] Generate icon variants using Tauri CLI: `bun tauri icon assets/icons/app-icon.png`
- [ ] T022 [US2] Verify generated icons exist in `src-tauri/icons/`: `32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.icns`

**Reference**: `specs/004-ci-cd-design/contracts/asset-schema.md` (Icon Requirements)

### DMG Background Creation

- [ ] T023 [P] [US2] Design DMG background image (660x400 PNG) and save to `assets/dmg/background.png`
- [ ] T024 [P] [US2] Create Retina variant (1320x800 PNG) and save to `assets/dmg/background@2x.png`
- [ ] T025 [US2] Validate DMG backgrounds: dimensions correct, file size <2MB, professional appearance

**Reference**: `specs/004-ci-cd-design/contracts/asset-schema.md` (DMG Background)

### Tauri Configuration Integration

- [ ] T026 [US2] Update `src-tauri/tauri.conf.json`: Add icon paths to `bundle.icon` array
- [ ] T027 [US2] Update `src-tauri/tauri.conf.json`: Configure `bundle.macOS.dmg.background` path
- [ ] T028 [US2] Update `src-tauri/tauri.conf.json`: Configure `bundle.macOS.dmg.windowSize` (660x400)
- [ ] T029 [US2] Update `src-tauri/tauri.conf.json`: Configure `bundle.macOS.dmg.appPosition` (x:180, y:170)
- [ ] T030 [US2] Update `src-tauri/tauri.conf.json`: Configure `bundle.macOS.dmg.applicationFolderPosition` (x:480, y:170)

**Reference**: `specs/004-ci-cd-design/data-model.md` (Tauri Bundle Configuration Schema)

### Build & Validation

- [ ] T031 [US2] Build DMG locally: `bun run tauri build --bundles dmg`
- [ ] T032 [US2] Open built DMG and verify custom background appears
- [ ] T033 [US2] Verify icon positions in DMG (app icon left, Applications folder right)
- [ ] T034 [US2] Install app from DMG and verify custom icon appears in Dock
- [ ] T035 [US2] Verify custom icon in Finder for app bundle (all sizes, Retina)
- [ ] T036 [US2] Open app and verify icon in About dialog (if About dialog exists)

**Acceptance Validation**:
- ✅ AS2.1: Custom icon displays in Dock (not generic icon)
- ✅ AS2.2: Custom icon in Finder with Retina support
- ✅ AS2.3: Branded assets in title bar and About dialog
- ✅ AS2.4: DMG shows custom background and icon layout

**Completion Criteria**: All design assets integrated, app displays custom icon everywhere, DMG has professional appearance

---

## Phase 4: User Story 3 - Release Asset Distribution (P3)

**Story Goal**: When developers create version tags, GitHub automatically builds universal DMG and creates release with download links.

**Independent Test Criteria**:
1. Create tag `v1.0.0` → Push tag → GitHub release created with DMG attached
2. Visit Releases page → DMG downloadable with version in filename
3. Release notes include changelog reference and installation instructions
4. Pre-release tags (v1.0.0-alpha.1) automatically marked as pre-release by GitHub

**Dependencies**: Requires US1 (CI workflow knowledge) and US2 (design assets exist)

### Release Workflow Implementation

- [ ] T037 [US3] Create `.github/workflows/release.yml` with basic structure (name, triggers)
- [ ] T038 [US3] Configure release trigger in `release.yml`: `on: push: tags: ['v*']`
- [ ] T039 [US3] Add `permissions: contents: write` to `release.yml` (required for creating releases)
- [ ] T040 [US3] Define `publish` job in `release.yml` with `runs-on: macos-latest`
- [ ] T041 [US3] Add checkout step in `release.yml`: `actions/checkout@v4`
- [ ] T042 [US3] Add Bun setup step in `release.yml`: `oven-sh/setup-bun@v2`
- [ ] T043 [US3] Add Rust setup with universal targets in `release.yml`: `targets: aarch64-apple-darwin, x86_64-apple-darwin`
- [ ] T044 [US3] Add dependency install step in `release.yml`: `run: bun install`

**Reference Contract**: `specs/004-ci-cd-design/contracts/release.yml`

### Tauri Action Configuration

- [ ] T045 [US3] Add `tauri-apps/tauri-action@v0` step in `release.yml`
- [ ] T046 [US3] Configure `tagName: ${{ github.ref_name }}` in tauri-action
- [ ] T047 [US3] Configure `releaseName: 'skhd-gui ${{ github.ref_name }}'` in tauri-action
- [ ] T048 [US3] Configure `releaseBody` in tauri-action with changelog reference and installation instructions
- [ ] T049 [US3] Set `releaseDraft: false` and `prerelease: false` in tauri-action (GitHub auto-detects pre-releases)
- [ ] T050 [US3] Set `args: --target universal-apple-darwin --bundles dmg` in tauri-action
- [ ] T051 [US3] Add `GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}` environment variable to tauri-action step

**Reference**: `specs/004-ci-cd-design/data-model.md` (tauri-action Configuration Schema)

### Release Validation

- [ ] T052 [US3] Create test tag locally: `git tag v0.0.1-test`
- [ ] T053 [US3] Push test tag: `git push origin v0.0.1-test`
- [ ] T054 [US3] Monitor GitHub Actions for release workflow execution
- [ ] T055 [US3] Verify release workflow builds universal DMG (Intel + Apple Silicon)
- [ ] T056 [US3] Verify GitHub release created with tag name
- [ ] T057 [US3] Verify DMG attached to release with version in filename (e.g., `skhd-gui_0.0.1-test_universal.dmg`)
- [ ] T058 [US3] Download DMG from release and verify it installs correctly
- [ ] T059 [US3] Verify release notes include changelog reference and installation instructions
- [ ] T060 [US3] Test pre-release tagging: Create `v0.0.2-alpha.1` → Verify marked as pre-release
- [ ] T061 [US3] Delete test tags and releases: `git push origin :refs/tags/v0.0.1-test` and delete from GitHub UI

**Acceptance Validation**:
- ✅ AS3.1: Tag pushed → DMG attached to release with version in filename
- ✅ AS3.2: Release notes visible with changelog and what changed
- ✅ AS3.3: DMG opens with drag-to-Applications installation flow

**Completion Criteria**: Release workflow functional, creates GitHub releases with DMG on tag push, pre-release detection works

---

## Phase 5: Polish & Cross-Cutting Concerns

**Goal**: Documentation, validation, and final quality checks

### Documentation

- [ ] T062 [P] Update README.md with CI/CD status badges (GitHub Actions workflow badges)
- [ ] T063 [P] Update README.md with installation instructions (download DMG from Releases)
- [ ] T064 [P] Update README.md with developer setup (how to trigger CI, create releases)
- [ ] T065 [P] Create or update CHANGELOG.md template for release notes

### Branch Protection & Settings

- [ ] T066 Configure GitHub branch protection for `main`: Require CI status checks to pass
- [ ] T067 Configure GitHub branch protection for `main`: Require PR reviews (optional, project preference)
- [ ] T068 Verify GitHub Actions has write permissions for releases (should be automatic with `GITHUB_TOKEN`)

### Final Validation

- [ ] T069 Create first real release: Tag `v1.0.0`, push, verify complete workflow end-to-end
- [ ] T070 Verify release time <15 minutes from tag push to published release (SC-006)
- [ ] T071 Verify build time <10 minutes for CI workflow (SC-001)
- [ ] T072 Verify DMG bundle size <20MB (constitution requirement)
- [ ] T073 Download and install from official v1.0.0 release to validate user experience
- [ ] T074 Commit all changes and update feature tracking documentation

**Completion Criteria**: All documentation updated, branch protection configured, first official release successful

---

## Implementation Strategy

### MVP Scope (Minimum Viable Product)

**Recommended MVP**: User Story 1 (P1) - Automated Build and Release

**Rationale**:
- Delivers immediate value: Automated testing and quality enforcement
- Blocks bad code from merging (constitution: Configuration Safety)
- Foundation for all future releases
- Can be tested independently without design assets

**MVP Tasks**: T001-T018 (Setup + US1)

**Post-MVP Increments**:
1. **Increment 2**: US2 (Design Assets) - Tasks T019-T036
2. **Increment 3**: US3 (Release Distribution) - Tasks T037-T061
3. **Polish**: Final documentation and validation - Tasks T062-T074

### Parallel Execution Opportunities

#### Phase 1 (Setup) - All Parallel
- T002, T003, T004 can all run in parallel (different directories)

#### Phase 2 (US1) - Sequential
- T005-T013: Sequential (building single `ci.yml` file)
- T014-T018: Sequential validation steps

#### Phase 3 (US2) - Mixed
**Parallel Group 1** (Asset Creation):
- T019 (icon design) || T023 (DMG background) || T024 (DMG Retina)

**Sequential**: T020-T022 (icon validation and generation)
**Sequential**: T025 (DMG validation)
**Sequential**: T026-T030 (Tauri config updates, single file)
**Sequential**: T031-T036 (build and validation)

#### Phase 4 (US3) - Sequential
- T037-T051: Sequential (building single `release.yml` file)
- T052-T061: Sequential validation steps

#### Phase 5 (Polish) - Mixed
**Parallel Group** (Documentation):
- T062 || T063 || T064 || T065 (different README sections and files)

**Sequential**: T066-T074 (GitHub settings and final validation)

### Execution Time Estimates

| Phase | Sequential Time | With Parallelization | Tasks |
|-------|----------------|---------------------|-------|
| Phase 1: Setup | 10 min | 5 min | 4 tasks |
| Phase 2: US1 (CI) | 60 min | 60 min | 14 tasks |
| Phase 3: US2 (Assets) | 120 min | 90 min | 18 tasks |
| Phase 4: US3 (Release) | 90 min | 90 min | 25 tasks |
| Phase 5: Polish | 45 min | 30 min | 13 tasks |
| **Total** | **~5.5 hours** | **~4.5 hours** | **74 tasks** |

**Note**: Times include testing and validation. Asset design time (T019, T023, T024) varies based on whether purchasing or creating custom designs.

---

## Dependencies & Story Completion Order

### Story Dependencies

```
Phase 1 (Setup)
    ↓
Phase 2 (US1: CI/CD) ← INDEPENDENT (MVP)
    ↓
Phase 3 (US2: Design Assets) ← INDEPENDENT (can be done anytime after Setup)
    ↓
Phase 4 (US3: Release Distribution) ← DEPENDS ON: US1 (workflow knowledge), US2 (assets exist)
    ↓
Phase 5 (Polish)
```

### Critical Path

1. **Setup** → Required for all phases
2. **US1 (CI)** → Enables quality enforcement, foundation for releases
3. **US2 (Assets)** → Required before meaningful releases (US3)
4. **US3 (Release)** → Requires US1 knowledge and US2 assets
5. **Polish** → Final touches after core functionality working

### Independent Testing Per Story

**US1 Test Criteria**:
- Push to main → CI runs → Tests execute → Status reported
- Create PR → CI blocks merge on test failure
- **Test without**: US2 or US3 (fully independent)

**US2 Test Criteria**:
- Build app locally → Install → Icon appears everywhere
- DMG shows custom background and layout
- **Test without**: US3 (build manually, no release automation needed)

**US3 Test Criteria**:
- Create tag → Push → GitHub release created with DMG
- Download from Releases → Install → Works correctly
- **Requires**: US1 (workflow patterns), US2 (assets for DMG)

---

## Task Validation Checklist

### Format Validation

- [x] All tasks follow checklist format: `- [ ] [TaskID] [Labels] Description`
- [x] All task IDs are sequential (T001-T074)
- [x] All user story phase tasks have story labels ([US1], [US2], [US3])
- [x] All parallelizable tasks marked with [P]
- [x] All tasks include specific file paths
- [x] Setup and Polish phase tasks have NO story labels

### Completeness Validation

- [x] Each user story has complete task coverage (tests → implementation → validation)
- [x] Each user story has independent test criteria documented
- [x] Each user story acceptance scenarios mapped to validation tasks
- [x] Dependencies between stories clearly documented
- [x] Parallel execution opportunities identified
- [x] MVP scope clearly defined (US1 only)

### Quality Validation

- [x] Tasks reference contract files from `specs/004-ci-cd-design/contracts/`
- [x] Tasks reference data models from `specs/004-ci-cd-design/data-model.md`
- [x] Tasks reference research decisions from `specs/004-ci-cd-design/research.md`
- [x] All functional requirements (FR-001 to FR-013) covered by tasks
- [x] All success criteria (SC-001 to SC-006) have validation tasks

---

## Summary

**Total Tasks**: 74
- **Setup**: 4 tasks
- **US1 (P1 - CI/CD)**: 14 tasks
- **US2 (P2 - Design Assets)**: 18 tasks
- **US3 (P3 - Release Distribution)**: 25 tasks
- **Polish**: 13 tasks

**Parallelization**: 11 tasks marked [P] (15% parallelizable)

**Suggested MVP**: US1 only (Tasks T001-T018, ~1 hour)

**Story Independence**:
- US1: ✅ Fully independent, MVP
- US2: ✅ Independent (can implement anytime)
- US3: ⚠️ Depends on US1 + US2

**Next Step**: Run `/speckit.implement` to execute tasks, or manually work through task list in order.

**Quick Start**:
```bash
# Start with MVP (US1):
# 1. Setup directories (T001-T004)
# 2. Create ci.yml workflow (T005-T013)
# 3. Test and validate (T014-T018)
# 4. Proceed to US2 or US3 as needed
```

---

**Ready for Implementation** ✅
