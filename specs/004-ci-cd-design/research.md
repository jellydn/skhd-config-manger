# Research: CI/CD Pipeline and Design Assets

**Feature**: 004-ci-cd-design
**Date**: 2025-11-02
**Purpose**: Resolve all "NEEDS CLARIFICATION" items from Technical Context

## Research Questions Addressed

This document resolves the following unknowns from the implementation plan:

### Design Asset Requirements
1. Icon format and tooling for .icns generation
2. DMG customization capabilities in Tauri
3. Asset source recommendations
4. Tauri configuration structure for icon integration

### CI/CD Implementation
5. GitHub Actions workflow trigger syntax
6. tauri-apps/tauri-action configuration
7. GitHub release automation
8. Pre-release tag detection

---

## 1. Icon Format and Tooling

### Decision: Use Tauri CLI `tauri icon` command with PNG source

**Research Findings**:
- **Source Format**: Single 1024x1024 PNG file with transparency
- **Generation Tool**: Tauri CLI command `tauri icon`
- **Output**: Automatically generates all required formats:
  - `.icns` file (macOS icon bundle with all sizes)
  - Multiple PNG sizes (32x32, 128x128, 128x128@2x, etc.)
  - Cross-platform icon formats (`.ico` for Windows if needed)

**Command**:
```bash
# Generate all icon variants from source
bun tauri icon ./assets/icons/app-icon.png

# Custom output directory (optional)
bun tauri icon ./assets/icons/app-icon.png --output src-tauri/icons
```

**Rationale**:
- Built-in Tauri functionality, no external tools needed
- Automatically generates correct sizes and formats
- Ensures consistency across all platforms
- Supports Retina (@2x) variants automatically

**Alternatives Considered**:
- Manual .icns creation (complex, error-prone)
- Third-party icon generators (unnecessary dependency)

**References**:
- Tauri Icon Command Documentation (Context7: /llmstxt/tauri_app_llms-full_txt)
- Apple HIG Icon Requirements

---

## 2. DMG Customization Capabilities

### Decision: Use Tauri's built-in DMG configuration in tauri.conf.json

**Research Findings**:
Tauri v2 supports comprehensive DMG customization through `tauri.conf.json`:

**Available Customizations**:
```json
{
  "bundle": {
    "macOS": {
      "dmg": {
        "background": "./assets/dmg/background.png",
        "windowSize": {
          "width": 660,
          "height": 400
        },
        "windowPosition": {
          "x": 400,
          "y": 400
        },
        "appPosition": {
          "x": 180,
          "y": 170
        },
        "applicationFolderPosition": {
          "x": 480,
          "y": 170
        }
      }
    }
  }
}
```

**Capabilities**:
- **Background Image**: PNG/JPG/GIF support, custom installer background
- **Window Size**: Customizable DMG window dimensions
- **Window Position**: Initial screen position of DMG window
- **Icon Layout**: Precise positioning of app icon and Applications folder

**Limitations**:
- Icon position settings may not apply on CI/CD platforms (documented behavior)
- Background image must be compatible with window size

**Rationale**:
- Native Tauri support, no additional tooling
- Sufficient for professional macOS installer experience
- Aligns with Apple HIG for DMG installers

**Alternatives Considered**:
- External DMG creation tools (unnecessary complexity)
- Post-processing scripts (would break Tauri bundling)

**References**:
- Tauri DMG Config Documentation (Context7: /llmstxt/tauri_app_llms-full_txt)
- Apple HIG for DMG installers

---

## 3. Asset Source Recommendations

### Decision: Create custom 1024x1024 PNG icon

**Options Evaluated**:

| Option | Cost | Quality | Effort | Recommendation |
|--------|------|---------|--------|----------------|
| AI Generation (DALL-E, Midjourney) | $0-20 | Medium-High | Low | ✅ **Recommended** |
| Purchase (IconStore, Creative Market) | $10-50 | High | Very Low | ✅ Good fallback |
| Custom Design (Figma/Sketch) | $0 | Varies | High | If design skills available |
| Icon Font/Library (free) | $0 | Low-Medium | Very Low | ❌ Generic appearance |

**Recommended Approach**:
1. **AI Generation** for unique, custom icon (DALL-E/Midjourney)
   - Prompt: "macOS app icon for keyboard shortcut manager, minimal, flat design, rounded square, professional"
   - Export at 1024x1024 PNG with transparency
   - Cost: ~$0-20 for multiple iterations

2. **Fallback**: Purchase from icon marketplace
   - Sites: IconStore, Creative Market, Icons8
   - Filter: macOS style, 1024x1024, commercial license
   - Cost: $10-30 one-time

**DMG Background**:
- Create simple gradient or solid color background (660x400 default)
- Use macOS-native colors for professional appearance
- Optional: Add subtle branding or product name

**Rationale**:
- AI generation provides unique, professional results quickly
- Purchase option ensures high quality if AI results unsatisfactory
- Both approaches deliver professional-grade assets for reasonable cost

**Apple HIG Compliance**:
- Rounded square shape (Tauri handles corner radius automatically)
- Clear at small sizes (16x16 minimum)
- No text or detailed elements
- Single focus/metaphor (keyboard or shortcuts)

---

## 4. Tauri Icon Integration

### Decision: Configure icon array in tauri.conf.json

**Configuration Structure**:
```json
{
  "bundle": {
    "identifier": "com.skhd.gui",
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

**Implementation Steps**:
1. Place source icon: `assets/icons/app-icon.png` (1024x1024)
2. Run: `bun tauri icon assets/icons/app-icon.png`
3. Tauri CLI generates icons in `src-tauri/icons/`
4. Update `tauri.conf.json` with icon paths (relative to `src-tauri/`)
5. Build process automatically bundles correct icons

**Directory Structure**:
```text
assets/
└── icons/
    └── app-icon.png           # Source (1024x1024)

src-tauri/
├── icons/                     # Generated by tauri icon
│   ├── 32x32.png
│   ├── 128x128.png
│   ├── 128x128@2x.png
│   ├── icon.icns              # macOS bundle
│   └── icon.ico               # Windows (if needed)
└── tauri.conf.json            # References icons/
```

**Automatic Handling**:
- Tauri bundler automatically selects correct icon for platform
- .icns used for macOS app bundle and DMG
- Dock, Finder, About dialog all use same icon source
- Retina displays automatically use @2x variants

**Rationale**:
- Standard Tauri workflow, well-documented
- Single source of truth (1024x1024 PNG)
- Automated variant generation eliminates manual work
- Platform-specific icon selection handled by bundler

**References**:
- Tauri Bundle Config Documentation (Context7)

---

## 5. GitHub Actions Workflow Triggers

### Decision: Use `on: push` with branch/tag filters

**CI Workflow (Build + Test)**:
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Bun
        uses: oven-sh/setup-bun@v2
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install dependencies
        run: bun install
      - name: Run tests
        run: |
          cargo test
          cargo clippy -- -D warnings
          bun run typecheck
```

**Release Workflow (Build + Publish)**:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'  # Matches v1.0.0, v1.0.0-alpha.1, etc.

jobs:
  release:
    runs-on: macos-latest
    # ... (build and release steps)
```

**Trigger Patterns**:
- **CI**: Runs on every push to `main` and all PRs
- **Release**: Runs only when tags matching `v*` are pushed
- **Tag Format**: `v1.0.0`, `v1.0.0-alpha.1`, `v1.0.0-beta.2`, `v1.0.0-rc.1`

**Rationale**:
- Standard GitHub Actions patterns
- Separate CI (frequent) from releases (infrequent)
- Tag-based releases enable manual control over release timing
- Glob pattern `v*` captures all semantic versions and pre-releases

**References**:
- GitHub Actions Workflow Syntax (Context7: /websites/github_en_actions)

---

## 6. tauri-apps/tauri-action Configuration

### Decision: Use official tauri-apps/tauri-action@v0

**Action Configuration**:
```yaml
- name: Build Tauri App
  uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    tagName: ${{ github.ref_name }}
    releaseName: 'skhd-gui v__VERSION__'
    releaseBody: 'See CHANGELOG.md for details'
    releaseDraft: false
    prerelease: false  # GitHub auto-detects from tag format
    args: --target universal-apple-darwin
```

**Key Features**:
- **Automatic Release Creation**: Creates GitHub release from tag
- **Asset Upload**: Automatically attaches built DMG/app to release
- **Universal Binary**: `--target universal-apple-darwin` for Intel + Apple Silicon
- **Version Substitution**: `__VERSION__` replaced with actual version
- **Pre-release Detection**: GitHub automatically marks `v1.0.0-alpha.1` as pre-release

**Build Targets**:
- `universal-apple-darwin`: Universal binary (Intel + Apple Silicon) - **Recommended**
- `x86_64-apple-darwin`: Intel only
- `aarch64-apple-darwin`: Apple Silicon only

**Environment Variables**:
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions for release API access

**Rationale**:
- Official Tauri action, maintained by Tauri team
- Handles complex build + release workflow automatically
- Universal binary ensures compatibility across all modern Macs
- No manual scripting for release creation or asset upload

**Alternatives Considered**:
- Manual Tauri CLI + GitHub CLI (more complex, more points of failure)
- Third-party release actions (less maintained, not Tauri-specific)

**References**:
- tauri-apps/tauri-action documentation
- Tauri bundling documentation (Context7)

---

## 7. GitHub Release Automation

### Decision: Let tauri-action create releases automatically

**Release Workflow**:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write  # Required for creating releases

jobs:
  publish:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v2

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin, x86_64-apple-darwin

      - name: Install dependencies
        run: bun install

      - name: Build and Release
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'skhd-gui ${{ github.ref_name }}'
          releaseBody: |
            ## What's Changed
            See [CHANGELOG.md](https://github.com/${{ github.repository }}/blob/${{ github.ref_name }}/CHANGELOG.md)

            ## Installation
            Download the DMG below and drag to Applications folder.
          releaseDraft: false
          prerelease: false
          args: --target universal-apple-darwin --bundles dmg
```

**Release Process**:
1. Developer creates and pushes tag: `git tag v1.0.0 && git push origin v1.0.0`
2. GitHub Actions detects tag push
3. Workflow runs tests (optional, can add)
4. tauri-action builds universal DMG
5. tauri-action creates GitHub release with tag name
6. DMG automatically attached to release as downloadable asset

**Release Metadata**:
- **Tag**: Version identifier (e.g., `v1.0.0`)
- **Title**: Formatted release name (e.g., "skhd-gui v1.0.0")
- **Body**: Installation instructions and changelog reference
- **Assets**: Universal DMG file (`skhd-gui_${version}_universal.dmg`)

**Rationale**:
- Fully automated, no manual GitHub UI interaction
- Consistent release format across all versions
- DMG availability immediate upon tag push (~10-15 minutes)
- Release metadata includes installation instructions

**Alternatives Considered**:
- Manual release creation (slow, error-prone, inconsistent)
- Separate release action (unnecessary complexity, tauri-action handles it)

---

## 8. Pre-release Tag Detection

### Decision: Use semantic versioning suffixes, GitHub auto-detects

**Tag Format**:
- **Stable**: `v1.0.0`, `v1.2.3`, `v2.0.0`
- **Pre-release**: `v1.0.0-alpha.1`, `v1.0.0-beta.2`, `v1.0.0-rc.1`

**GitHub Behavior**:
- Tags with `-alpha`, `-beta`, `-rc` suffixes automatically marked as "Pre-release"
- Pre-releases shown separately on Releases page
- Users can opt-in to pre-release downloads
- Latest stable release shown prominently

**Implementation**:
```yaml
# No special configuration needed - GitHub detects automatically
- uses: tauri-apps/tauri-action@v0
  with:
    tagName: ${{ github.ref_name }}  # v1.0.0-alpha.1 → auto pre-release
    prerelease: false  # Let GitHub auto-detect (can override if needed)
```

**Tagging Strategy**:
```bash
# Alpha releases (early testing)
git tag v1.0.0-alpha.1
git push origin v1.0.0-alpha.1

# Beta releases (feature complete, testing)
git tag v1.0.0-beta.1
git push origin v1.0.0-beta.1

# Release candidates (near-final)
git tag v1.0.0-rc.1
git push origin v1.0.0-rc.1

# Stable release
git tag v1.0.0
git push origin v1.0.0
```

**Versioning Rules** (from FR-013):
- Alpha: `v{MAJOR}.{MINOR}.{PATCH}-alpha.{N}`
- Beta: `v{MAJOR}.{MINOR}.{PATCH}-beta.{N}`
- RC: `v{MAJOR}.{MINOR}.{PATCH}-rc.{N}`
- Stable: `v{MAJOR}.{MINOR}.{PATCH}`

**Rationale**:
- Industry-standard semantic versioning
- No custom workflow logic needed
- GitHub UI clearly distinguishes pre-releases
- Well-supported by tooling (npm, cargo, etc.)

**Alternatives Considered**:
- Custom `prerelease: true` flag (less flexible, requires workflow changes)
- Branch-based pre-releases (complex, harder to track)
- Separate pre-release workflow (unnecessary duplication)

**References**:
- Semantic Versioning 2.0.0 specification
- GitHub Releases documentation (Context7: /websites/github_en_actions)

---

## Research Summary

### All Clarifications Resolved ✅

| Question | Resolution | Decision |
|----------|-----------|----------|
| Icon format/tooling | ✅ RESOLVED | Tauri CLI `tauri icon` with 1024x1024 PNG source |
| DMG customization | ✅ RESOLVED | tauri.conf.json DMG config (background, layout) |
| Asset source | ✅ RESOLVED | AI generation or purchase (~$0-30) |
| Tauri icon integration | ✅ RESOLVED | Bundle icon array in tauri.conf.json |
| Workflow triggers | ✅ RESOLVED | `on: push` with branches/tags filters |
| tauri-action config | ✅ RESOLVED | Official tauri-apps/tauri-action@v0 |
| Release automation | ✅ RESOLVED | tauri-action creates releases automatically |
| Pre-release detection | ✅ RESOLVED | Semantic versioning, GitHub auto-detects |

### Technology Stack Confirmed

- **CI Platform**: GitHub Actions (free for public repos)
- **Build Action**: tauri-apps/tauri-action@v0
- **Icon Tool**: Tauri CLI (`tauri icon` command)
- **Bundle Format**: Universal DMG (Intel + Apple Silicon)
- **Versioning**: Semantic versioning with suffixes

### Next Steps

1. **Phase 1**: Generate data-model.md (workflow and asset schemas)
2. **Phase 1**: Generate contracts/ (GitHub Actions YAML schemas)
3. **Phase 1**: Generate quickstart.md (how to trigger builds/releases)
4. **Phase 1**: Update agent context with new technologies
5. **Phase 2**: Generate tasks.md with `/speckit.tasks`

### Open Questions: None

All technical unknowns have been resolved through official documentation research.
