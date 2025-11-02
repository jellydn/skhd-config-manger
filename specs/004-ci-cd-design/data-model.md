# Data Model: CI/CD Pipeline and Design Assets

**Feature**: 004-ci-cd-design
**Date**: 2025-11-02
**Purpose**: Define workflow configuration structures and asset schemas

## Overview

This document defines the "data models" for CI/CD workflows and design assets. Since this is infrastructure (not application code), the models represent configuration schemas rather than traditional database entities.

---

## 1. GitHub Actions Workflow Schema

### 1.1 CI Workflow Configuration

**Entity**: CI Workflow
**Purpose**: Build and test on every push/PR to ensure code quality
**File**: `.github/workflows/ci.yml`

#### Workflow Properties

| Property | Type | Required | Description | Validation Rules |
|----------|------|----------|-------------|------------------|
| name | string | Yes | Workflow display name | "CI" |
| on | object | Yes | Trigger configuration | See Trigger Schema |
| jobs | object | Yes | Job definitions | See Job Schema |
| permissions | object | No | Workflow permissions | Default: read-only |

#### Trigger Schema (CI)

```typescript
interface CITrigger {
  push: {
    branches: string[];  // ["main"]
  };
  pull_request: {
    branches: string[];  // ["main"]
  };
}
```

**Validation**:
- Must trigger on `main` branch pushes (FR-001)
- Must trigger on pull requests to `main` (FR-002)
- No tag triggers (releases handled separately)

#### Job Schema (CI)

```typescript
interface CIJob {
  test: {
    'runs-on': 'macos-latest';  // macOS runner required
    steps: Step[];
  };
}

interface Step {
  name?: string;
  uses?: string;  // Action reference
  run?: string;   // Shell command
  env?: Record<string, string>;
}
```

**Required Steps**:
1. Checkout code (`actions/checkout@v4`)
2. Setup Bun (`oven-sh/setup-bun@v2`)
3. Setup Rust (`dtolnay/rust-toolchain@stable`)
4. Install dependencies (`bun install`)
5. Run tests:
   - `cargo test` (Rust unit tests)
   - `cargo clippy -- -D warnings` (Rust linting)
   - `bun run typecheck` (TypeScript type checking)

**Validation**:
- All three test commands must pass (FR-002)
- Clippy warnings treated as errors (constitution: Quality Gates)
- Runner must be `macos-latest` (macOS-specific project)

---

### 1.2 Release Workflow Configuration

**Entity**: Release Workflow
**Purpose**: Build DMG and create GitHub release on version tags
**File**: `.github/workflows/release.yml`

#### Workflow Properties

| Property | Type | Required | Description | Validation Rules |
|----------|------|----------|-------------|------------------|
| name | string | Yes | Workflow display name | "Release" |
| on | object | Yes | Trigger configuration | See Release Trigger Schema |
| permissions | object | Yes | Workflow permissions | `contents: write` required |
| jobs | object | Yes | Job definitions | See Release Job Schema |

#### Release Trigger Schema

```typescript
interface ReleaseTrigger {
  push: {
    tags: string[];  // ["v*"]
  };
}
```

**Validation**:
- Must match all tags starting with `v` (FR-004)
- Captures: `v1.0.0`, `v1.0.0-alpha.1`, `v2.1.3-beta.2`, etc.
- Pre-release detection automatic (GitHub behavior for `-alpha`, `-beta`, `-rc`)

#### Release Job Schema

```typescript
interface ReleaseJob {
  publish: {
    'runs-on': 'macos-latest';
    steps: Step[];
  };
}
```

**Required Steps**:
1. Checkout code
2. Setup Bun
3. Setup Rust with targets:
   - `aarch64-apple-darwin` (Apple Silicon)
   - `x86_64-apple-darwin` (Intel)
4. Install dependencies
5. Build and release using `tauri-apps/tauri-action@v0`

**tauri-action Configuration**:
```typescript
interface TauriActionConfig {
  tagName: string;           // ${{ github.ref_name }}
  releaseName: string;       // "skhd-gui ${{ github.ref_name }}"
  releaseBody: string;       // Markdown with changelog and install instructions
  releaseDraft: boolean;     // false (publish immediately)
  prerelease: boolean;       // false (let GitHub auto-detect)
  args: string;              // "--target universal-apple-darwin --bundles dmg"
}
```

**Validation**:
- Must use `universal-apple-darwin` target (FR-005: Intel + Apple Silicon)
- Must bundle as DMG (FR-005)
- Must create GitHub release (FR-004)
- Must attach DMG to release (FR-006)

---

### 1.3 Workflow State Transitions

```text
Tag Push (v*)
    ↓
Release Workflow Triggered
    ↓
Build Universal DMG
    ↓
Create GitHub Release
    ↓
Attach DMG Asset
    ↓
Release Published ✅
```

**Success Criteria Mapping**:
- Build completes in <10 minutes (SC-001)
- Release process <15 minutes from tag to published (SC-006)
- 100% of releases include DMG (SC-002)

---

## 2. Design Asset Schema

### 2.1 Application Icon

**Entity**: App Icon
**Purpose**: Visual identifier for application across macOS interfaces
**Location**: `assets/icons/app-icon.png` (source), `src-tauri/icons/` (generated)

#### Icon Properties

| Property | Type | Required | Description | Validation Rules |
|----------|------|----------|-------------|------------------|
| source_path | string | Yes | Path to source PNG | Must be 1024x1024, PNG with transparency |
| format | string | Yes | File format | "PNG" |
| dimensions | object | Yes | Width and height | `{width: 1024, height: 1024}` |
| color_space | string | Yes | Color profile | sRGB or Display P3 |
| transparency | boolean | Yes | Alpha channel | Must be `true` |
| content_guidelines | string[] | Yes | Design constraints | See Apple HIG compliance |

#### Generated Variants

Generated by `tauri icon` command:

```typescript
interface GeneratedIcons {
  png_sizes: number[];      // [32, 128, 256, 512, 1024]
  retina_variants: boolean; // @2x versions for Retina displays
  icns_bundle: string;      // macOS icon bundle (all sizes)
  ico_bundle: string;       // Windows icon (optional)
}
```

**Output Files**:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS bundle)
- `icon.ico` (Windows, optional)

**Validation Rules (FR-008)**:
- Source must be exactly 1024x1024 pixels
- PNG format with transparency (alpha channel)
- Clear and recognizable at 16x16 (smallest size)
- Follows Apple HIG:
  - Rounded square shape (Tauri auto-applies corner radius)
  - No text or detailed elements
  - Single visual metaphor
  - Professional appearance

#### Apple HIG Compliance

```typescript
interface IconDesignGuidelines {
  shape: 'rounded-square';       // Tauri applies automatically
  minimum_clarity: '16x16';      // Must be clear at smallest size
  background: 'filled' | 'transparent'; // Preference for filled
  text: 'none';                  // No text in icon
  detail_level: 'simple';        // Avoid fine details
  focus: 'single-metaphor';      // One clear concept
  style: 'flat' | 'minimal';     // Modern macOS aesthetic
}
```

---

### 2.2 DMG Background

**Entity**: DMG Background
**Purpose**: Custom visual for DMG installer window
**Location**: `assets/dmg/background.png`, `assets/dmg/background@2x.png`

#### DMG Background Properties

| Property | Type | Required | Description | Validation Rules |
|----------|------|----------|-------------|------------------|
| path | string | Yes | Path to background image | PNG/JPG/GIF |
| dimensions | object | Yes | Width and height | Default: `{width: 660, height: 400}` |
| retina_variant | string | No | @2x version for Retina | `{width: 1320, height: 800}` |
| format | string | Yes | File format | "PNG", "JPG", or "GIF" |

**Validation Rules (FR-010)**:
- Dimensions must match `windowSize` in tauri.conf.json
- Retina variant should be 2x resolution
- File size <2MB for reasonable DMG size
- Professional appearance (gradient or solid color preferred)

#### DMG Layout Configuration

```typescript
interface DMGLayoutConfig {
  windowSize: {
    width: number;   // Default: 660
    height: number;  // Default: 400
  };
  windowPosition?: {
    x: number;       // Screen position (optional)
    y: number;
  };
  appPosition: {
    x: number;       // Icon X coordinate (default: 180)
    y: number;       // Icon Y coordinate (default: 170)
  };
  applicationFolderPosition: {
    x: number;       // Applications folder X (default: 480)
    y: number;       // Applications folder Y (default: 170)
  };
  background?: string; // Path to background image
}
```

**Standard Layout** (FR-010):
- Window: 660x400 (default, customizable)
- App icon: (180, 170) - left side
- Applications folder: (480, 170) - right side
- Visual: Drag app to Applications folder

**Note**: Icon positions may not apply on CI/CD platforms (known Tauri limitation)

---

### 2.3 Tauri Bundle Configuration

**Entity**: Tauri Bundle Config
**Purpose**: Define how application is packaged for distribution
**File**: `src-tauri/tauri.conf.json`

#### Bundle Configuration Schema

```typescript
interface TauriConfig {
  bundle: {
    identifier: string;          // "com.skhd.gui"
    icon: string[];              // Paths to icon files
    macOS: {
      dmg: DMGLayoutConfig;
      minimumSystemVersion: string;  // "10.13" (constitution)
      hardenedRuntime: boolean;      // true (constitution)
    };
  };
}
```

**Icon Array** (FR-009):
```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

**Validation**:
- All icon paths must exist relative to `src-tauri/`
- `.icns` file required for macOS
- DMG config must be valid for macOS build

---

## 3. Release Artifact Schema

### 3.1 GitHub Release

**Entity**: GitHub Release
**Purpose**: Distributable package with metadata and downloadable assets
**Created By**: tauri-apps/tauri-action

#### Release Properties

| Property | Type | Required | Description | Validation Rules |
|----------|------|----------|-------------|------------------|
| tag_name | string | Yes | Version tag | Must match `v*` pattern |
| name | string | Yes | Release title | "skhd-gui v{version}" |
| body | string | Yes | Release notes | Markdown with changelog reference |
| draft | boolean | Yes | Draft status | `false` (publish immediately) |
| prerelease | boolean | Yes | Pre-release flag | Auto-detected by GitHub |
| assets | Asset[] | Yes | Downloadable files | Must include DMG |

#### Release Asset Schema

```typescript
interface ReleaseAsset {
  name: string;           // "skhd-gui_{version}_universal.dmg"
  content_type: string;   // "application/x-apple-diskimage"
  size: number;           // File size in bytes
  download_count: number; // Downloads (tracked by GitHub)
  browser_download_url: string; // Direct download link
}
```

**Asset Naming Convention**:
- Pattern: `skhd-gui_{version}_{arch}.dmg`
- Example: `skhd-gui_1.0.0_universal.dmg`
- Universal: Intel + Apple Silicon in single DMG

**Validation Rules**:
- DMG must be present (FR-006, SC-002)
- Size should be <20MB (constitution: bundle size target)
- Download URL must be publicly accessible (SC-004)

---

### 3.2 Pre-Release Detection

**Entity**: Version Tag
**Purpose**: Identify release stability level
**Pattern**: Semantic Versioning with suffixes

#### Version Tag Schema

```typescript
type VersionTag = StableTag | PreReleaseTag;

interface StableTag {
  pattern: /^v\d+\.\d+\.\d+$/;
  examples: ['v1.0.0', 'v2.1.3', 'v10.0.0'];
  github_prerelease: false;
}

interface PreReleaseTag {
  pattern: /^v\d+\.\d+\.\d+-(alpha|beta|rc)\.\d+$/;
  examples: ['v1.0.0-alpha.1', 'v1.0.0-beta.2', 'v1.0.0-rc.1'];
  github_prerelease: true;  // Auto-detected by GitHub
}
```

**State Transitions** (FR-013):
```text
v1.0.0-alpha.1 → v1.0.0-alpha.2 → v1.0.0-beta.1 → v1.0.0-rc.1 → v1.0.0
```

**GitHub Behavior**:
- Tags with `-alpha`, `-beta`, `-rc` automatically marked as pre-release
- Latest stable release shown prominently
- Pre-releases shown separately on Releases page
- Users must opt-in to download pre-releases

---

## 4. Configuration Validation

### 4.1 Validation Rules Summary

#### CI Workflow Validation
- ✅ Triggers on `main` push and PRs
- ✅ Runs all three test commands
- ✅ Fails workflow if any test fails
- ✅ Uses macOS runner

#### Release Workflow Validation
- ✅ Triggers on `v*` tags only
- ✅ Builds universal binary (Intel + Apple Silicon)
- ✅ Creates DMG bundle
- ✅ Attaches DMG to GitHub release
- ✅ Publishes release immediately (not draft)

#### Icon Asset Validation
- ✅ Source is 1024x1024 PNG with transparency
- ✅ Clear at 16x16 minimum size
- ✅ Follows Apple HIG guidelines
- ✅ Generated variants include .icns bundle

#### DMG Asset Validation
- ✅ Background dimensions match window size
- ✅ Retina variant is 2x resolution (if provided)
- ✅ File size <2MB
- ✅ Professional appearance

#### Bundle Configuration Validation
- ✅ Icon array includes all required formats
- ✅ DMG config has valid layout
- ✅ Minimum system version matches constitution (10.13+)
- ✅ Hardened runtime enabled

---

## 5. Data Flow

### 5.1 Asset Generation Flow

```text
Source Icon (1024x1024 PNG)
    ↓
tauri icon command
    ↓
Generated Icons (32px → 1024px, .icns)
    ↓
tauri.conf.json (icon array)
    ↓
Tauri Build
    ↓
macOS App Bundle (icon embedded)
    ↓
DMG with custom background
```

### 5.2 Release Flow

```text
Git Tag (v1.0.0)
    ↓
GitHub Push
    ↓
Release Workflow Triggered
    ↓
Checkout Code
    ↓
Setup Environment (Bun, Rust)
    ↓
Install Dependencies
    ↓
tauri-action Build
    ↓
Universal DMG Created
    ↓
GitHub Release Created
    ↓
DMG Attached as Asset
    ↓
Release Published ✅
```

### 5.3 CI Flow

```text
Code Push / PR
    ↓
CI Workflow Triggered
    ↓
Checkout Code
    ↓
Setup Environment
    ↓
Install Dependencies
    ↓
Run Tests (cargo test, clippy, typecheck)
    ↓
All Pass? ✅ Merge allowed / ❌ Merge blocked
```

---

## 6. Schema References

### 6.1 External Schema Dependencies

- **GitHub Actions Workflow Syntax**: [GitHub Docs](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions)
- **Tauri Config Schema**: Tauri v2 Configuration Reference
- **Apple HIG**: Human Interface Guidelines for macOS icons
- **Semantic Versioning**: [semver.org](https://semver.org/)

### 6.2 Internal Schema Files

- `.github/workflows/ci.yml`: CI workflow implementation
- `.github/workflows/release.yml`: Release workflow implementation
- `src-tauri/tauri.conf.json`: Tauri bundle configuration
- `specs/004-ci-cd-design/contracts/`: Detailed YAML schemas (next artifact)

---

## Summary

This data model defines:
1. **Workflow Schemas**: CI and Release workflow configurations with validation rules
2. **Asset Schemas**: Icon and DMG background specifications with Apple HIG compliance
3. **Release Schemas**: GitHub release structure and asset metadata
4. **Validation Rules**: Ensure all components meet requirements and constitution standards

All schemas map directly to functional requirements (FR-001 through FR-013) and success criteria (SC-001 through SC-006).
