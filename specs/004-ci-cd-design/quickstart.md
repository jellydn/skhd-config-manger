# Quickstart: CI/CD Pipeline and Design Assets

**Feature**: 004-ci-cd-design
**Date**: 2025-11-02
**Purpose**: Quick reference for triggering builds and creating releases

---

## Table of Contents

1. [CI/CD Overview](#cicd-overview)
2. [Triggering CI Builds](#triggering-ci-builds)
3. [Creating Releases](#creating-releases)
4. [Managing Design Assets](#managing-design-assets)
5. [Troubleshooting](#troubleshooting)

---

## CI/CD Overview

### What Runs Automatically

| Event | Workflow | Actions | Time |
|-------|----------|---------|------|
| Push to `main` | CI | Tests (cargo test, clippy, typecheck) | ~5-10 min |
| Pull Request to `main` | CI | Tests (cargo test, clippy, typecheck) | ~5-10 min |
| Push tag `v*` | Release | Build universal DMG + Create GitHub release | ~10-15 min |

### Workflow Files

- **CI**: `.github/workflows/ci.yml`
- **Release**: `.github/workflows/release.yml`

---

## Triggering CI Builds

### Automatic CI Triggers

**1. Push to main branch**:
```bash
git checkout main
git pull origin main
# Make changes
git add .
git commit -m "feat: add new feature"
git push origin main  # ← CI runs automatically
```

**2. Create/update pull request**:
```bash
git checkout -b feature/my-feature
# Make changes
git add .
git commit -m "feat: implement my feature"
git push origin feature/my-feature  # ← CI runs on PR
```

### What CI Tests

CI workflow runs all quality checks:
```bash
cargo test             # Rust unit tests
cargo clippy -- -D warnings  # Rust linting (warnings = errors)
bun run typecheck      # TypeScript type checking
```

### Viewing CI Results

**GitHub UI**:
1. Go to repository on GitHub
2. Click "Actions" tab
3. Select "CI" workflow
4. View latest run status

**PR Status**:
- ✅ Green checkmark: All tests passed, can merge
- ❌ Red X: Tests failed, cannot merge (unless you bypass)
- 🟡 Yellow circle: Tests running

### Local Testing (Before Push)

Run the same tests locally to catch issues early:

```bash
# Run all tests that CI will run
cargo test && cargo clippy -- -D warnings && bun run typecheck

# Or run them individually
cargo test        # Rust tests
cargo clippy      # Rust linting
bun run typecheck # TypeScript checks
```

---

## Creating Releases

### Release Process Overview

```
Tag Version → Push Tag → GitHub Actions → Build DMG → Create Release → Publish
                          (10-15 minutes)
```

### Step-by-Step Release Instructions

#### 1. Prepare for Release

```bash
# Ensure you're on main branch with latest changes
git checkout main
git pull origin main

# Run tests locally (optional but recommended)
cargo test && cargo clippy && bun run typecheck

# Update CHANGELOG.md (if using one)
# Add release notes for this version
```

#### 2. Create Version Tag

**Stable Release**:
```bash
# Format: v{MAJOR}.{MINOR}.{PATCH}
git tag v1.0.0
git tag v1.2.3
git tag v2.0.0
```

**Pre-Release (Alpha/Beta/RC)**:
```bash
# Alpha: Early testing
git tag v1.0.0-alpha.1
git tag v1.0.0-alpha.2

# Beta: Feature complete, testing
git tag v1.0.0-beta.1
git tag v1.0.0-beta.2

# Release Candidate: Near-final
git tag v1.0.0-rc.1
git tag v1.0.0-rc.2
```

#### 3. Push Tag to GitHub

```bash
# Push the tag (triggers release workflow)
git push origin v1.0.0

# Or push specific tag
git push origin <tag-name>
```

#### 4. Monitor Release Build

**GitHub UI**:
1. Go to "Actions" tab
2. Select "Release" workflow
3. Watch build progress (~10-15 minutes)

**Command Line** (using GitHub CLI):
```bash
# View recent workflow runs
gh run list

# Watch specific run
gh run watch
```

#### 5. Verify Release

Once workflow completes:

1. Go to "Releases" page on GitHub
2. Find your new release (e.g., "skhd-gui v1.0.0")
3. Verify:
   - DMG file is attached (e.g., `skhd-gui_1.0.0_universal.dmg`)
   - Release notes are present
   - Pre-release flag correct (if alpha/beta/rc)

### Release Versioning Strategy

| Release Type | Tag Format | Example | GitHub Marks As |
|--------------|------------|---------|-----------------|
| Major | `v{X}.0.0` | `v2.0.0` | Stable |
| Minor | `v{X}.{Y}.0` | `v1.5.0` | Stable |
| Patch | `v{X}.{Y}.{Z}` | `v1.2.3` | Stable |
| Alpha | `v{X}.{Y}.{Z}-alpha.{N}` | `v1.0.0-alpha.1` | Pre-release |
| Beta | `v{X}.{Y}.{Z}-beta.{N}` | `v1.0.0-beta.1` | Pre-release |
| RC | `v{X}.{Y}.{Z}-rc.{N}` | `v1.0.0-rc.1` | Pre-release |

**GitHub Auto-Detection**: Tags with `-alpha`, `-beta`, or `-rc` are automatically marked as pre-releases.

### Deleting/Fixing Tags

**If you tagged incorrectly**:

```bash
# Delete local tag
git tag -d v1.0.0

# Delete remote tag (if already pushed)
git push origin :refs/tags/v1.0.0

# Create correct tag
git tag v1.0.1
git push origin v1.0.1
```

**Note**: Deleting a tag after release workflow starts won't stop it. GitHub Actions uses the tag at trigger time.

---

## Managing Design Assets

### Icon Asset Workflow

#### Creating/Updating Application Icon

```bash
# 1. Place source icon (1024x1024 PNG with transparency)
cp your-icon.png assets/icons/app-icon.png

# 2. Generate all icon variants using Tauri CLI
bun tauri icon assets/icons/app-icon.png

# 3. Verify generated icons
ls -lh src-tauri/icons/
# Should see: 32x32.png, 128x128.png, 128x128@2x.png, icon.icns, icon.ico

# 4. Test build locally
bun run tauri build --bundles dmg

# 5. Verify icon appears in DMG, Dock, Finder

# 6. Commit changes
git add assets/icons/app-icon.png src-tauri/icons/
git commit -m "Update application icon"
git push origin main
```

#### Icon Requirements Checklist

Before running `tauri icon`:
- [ ] Exactly 1024x1024 pixels
- [ ] PNG format with transparency (alpha channel)
- [ ] Clear at small sizes (test at 16x16 preview)
- [ ] Follows Apple HIG (rounded square, no text)
- [ ] File size <500KB

### DMG Background Workflow

#### Creating/Updating DMG Background

```bash
# 1. Place background images
cp background.png assets/dmg/background.png       # 660x400 (standard)
cp background@2x.png assets/dmg/background@2x.png # 1320x800 (Retina)

# 2. Update Tauri config (if needed)
# Edit src-tauri/tauri.conf.json
# Ensure bundle.macOS.dmg.background points to "../../assets/dmg/background.png"

# 3. Test DMG build
bun run tauri build --bundles dmg

# 4. Open DMG and verify background appears
open target/release/bundle/dmg/*.dmg

# 5. Commit changes
git add assets/dmg/ src-tauri/tauri.conf.json
git commit -m "Update DMG background"
git push origin main
```

#### DMG Background Requirements

- [ ] Dimensions match windowSize (default: 660x400)
- [ ] Retina @2x variant (1320x800)
- [ ] PNG format recommended
- [ ] File size <2MB
- [ ] Professional, minimal design
- [ ] Works in both light and dark macOS themes

### Verifying Assets in Build

```bash
# Build DMG locally
bun run tauri build --bundles dmg

# Check DMG location
ls -lh target/release/bundle/dmg/

# Open DMG to verify
open target/release/bundle/dmg/*.dmg

# Manual verification:
# - Custom background appears
# - Application icon is custom (not default Tauri icon)
# - Icon positions are correct (app icon left, Applications folder right)
```

---

## Troubleshooting

### CI Workflow Issues

#### Tests Failing Locally But Pass in CI (or vice versa)

```bash
# Ensure dependencies are up-to-date
bun install
cargo build

# Run exact CI commands locally
cargo test
cargo clippy -- -D warnings
bun run typecheck
```

#### CI Taking Too Long (>10 minutes)

Check GitHub Actions logs:
1. Go to "Actions" tab
2. Select failing CI run
3. Expand "Run tests" step
4. Look for slow tests or hanging operations

**Common fixes**:
- Optimize slow tests
- Check for network timeouts
- Ensure bun cache is working

#### PR Blocked Even Though Tests Pass

Ensure GitHub branch protection rules are configured:
1. Repository Settings → Branches
2. Edit "main" branch protection
3. Require status checks: Enable "test" job from CI workflow

### Release Workflow Issues

#### Release Workflow Doesn't Trigger

**Check tag format**:
```bash
# Tags MUST start with 'v'
git tag v1.0.0  # ✅ Works
git tag 1.0.0   # ❌ Won't trigger release workflow
```

**Verify tag was pushed**:
```bash
git ls-remote --tags origin
# Should see your tag listed
```

#### Release Build Fails

**Common causes**:
1. Tests failing (add optional test step to release workflow)
2. Icon files missing (run `tauri icon` first)
3. Tauri config invalid (verify `src-tauri/tauri.conf.json`)

**Debug locally**:
```bash
# Try building DMG locally
bun run tauri build --bundles dmg --target universal-apple-darwin

# Check for errors in build output
```

#### DMG Not Attached to Release

**Verify tauri-action configuration**:
- Check `.github/workflows/release.yml`
- Ensure `args: --bundles dmg` is present
- Verify `GITHUB_TOKEN` environment variable set

**Check GitHub Actions logs**:
1. Go to failed release run
2. Expand "Build and Release" step
3. Look for DMG creation and upload messages

#### Pre-Release Not Marked Correctly

**Tag format issue**:
```bash
# GitHub auto-detects pre-release from tag format
v1.0.0-alpha.1  # ✅ Marked as pre-release
v1.0.0-Alpha.1  # ❌ Case-sensitive, won't detect
v1.0.0.alpha.1  # ❌ Must use hyphen, not dot
```

### Design Asset Issues

#### Icon Doesn't Appear in App

**Check generation**:
```bash
# Verify icon files exist
ls -lh src-tauri/icons/icon.icns

# Verify Tauri config references icons
cat src-tauri/tauri.conf.json | grep -A10 '"icon"'
```

**Rebuild app**:
```bash
# Clean build
rm -rf target/
bun run tauri build --bundles dmg
```

#### DMG Background Doesn't Show

**Check path**:
```bash
# Path in tauri.conf.json must be relative to src-tauri/
# Correct: "../../assets/dmg/background.png"
# Wrong: "/assets/dmg/background.png"
```

**Check file exists**:
```bash
ls -lh assets/dmg/background.png
```

**Known limitation**: Icon positions may not work on CI/CD. Background image should always work.

#### Icon Looks Blurry on Retina Displays

**Cause**: Source icon too small or @2x variants not generated

**Fix**:
```bash
# Ensure source is 1024x1024
file assets/icons/app-icon.png

# Regenerate with Tauri CLI (creates @2x variants)
bun tauri icon assets/icons/app-icon.png
```

---

## Quick Reference Commands

### CI/CD

```bash
# Run tests locally (same as CI)
cargo test && cargo clippy -- -D warnings && bun run typecheck

# Create and push stable release
git tag v1.0.0
git push origin v1.0.0

# Create and push pre-release
git tag v1.0.0-alpha.1
git push origin v1.0.0-alpha.1

# View workflow runs (GitHub CLI)
gh run list
gh run watch
```

### Design Assets

```bash
# Generate icon variants
bun tauri icon assets/icons/app-icon.png

# Build DMG locally
bun run tauri build --bundles dmg

# Build universal binary (Intel + Apple Silicon)
bun run tauri build --bundles dmg --target universal-apple-darwin

# Open built DMG
open target/release/bundle/dmg/*.dmg
```

### Git Tag Management

```bash
# List all tags
git tag -l

# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin :refs/tags/v1.0.0

# Push all tags
git push origin --tags
```

---

## Next Steps

After implementing this feature:

1. **First Release**: Create `v1.0.0` tag and verify complete workflow
2. **Documentation**: Update README with installation instructions
3. **Branch Protection**: Enable required status checks for CI
4. **Monitoring**: Watch first few releases for any issues

---

## Additional Resources

- **GitHub Actions Documentation**: https://docs.github.com/en/actions
- **Tauri Build Documentation**: https://tauri.app/v1/guides/building/
- **Apple HIG**: https://developer.apple.com/design/human-interface-guidelines/macos
- **Semantic Versioning**: https://semver.org/

---

**Questions?** See `specs/004-ci-cd-design/` for detailed planning documentation.
