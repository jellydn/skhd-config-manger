# Scripts

Utility scripts for the Keybinder project.

## Icon Generation

### `generate-icons.sh`

Automatically generates all required app icons from a single source image.

#### Requirements

- **macOS** (uses native `sips` and `iconutil`)
- **ImageMagick** (for Windows `.ico` generation)
  ```bash
  brew install imagemagick
  ```

#### Usage

```bash
./scripts/generate-icons.sh <source-image>
```

**Example:**
```bash
./scripts/generate-icons.sh ~/Downloads/keybinder_icon_1024.png
```

#### What It Generates

**Total: 16 icon files**

| Platform | Files | Description |
|----------|-------|-------------|
| **Master** | `icon.png` | 1024x1024 source image |
| **macOS** | `32x32.png`<br>`128x128.png`<br>`128x128@2x.png`<br>`icon.icns` | Menu bar, standard, retina, app bundle |
| **Windows** | `icon.ico` | 6 embedded sizes (16, 32, 48, 64, 128, 256) |
| **Windows Store** | `Square30x30Logo.png`<br>`Square44x44Logo.png`<br>`Square71x71Logo.png`<br>`Square89x89Logo.png`<br>`Square107x107Logo.png`<br>`Square142x142Logo.png`<br>`Square150x150Logo.png`<br>`Square284x284Logo.png`<br>`Square310x310Logo.png`<br>`StoreLogo.png` | UWP/Microsoft Store tiles |

#### Source Image Requirements

- **Format:** PNG (recommended), JPG, or any format supported by `sips`
- **Size:** 1024x1024 pixels minimum (square)
- **Transparency:** Supported (recommended for app icons)
- **Color depth:** RGB or RGBA

#### Example Workflow

```bash
# 1. Design your icon in Figma/Sketch/etc and export as 1024x1024 PNG
# 2. Run the script
./scripts/generate-icons.sh ~/Downloads/my-new-icon.png

# 3. Review generated icons
ls -lh src-tauri/icons/

# 4. Test in app
bun run tauri dev

# 5. Commit changes
git add src-tauri/icons/
git commit -m "chore: Update app icons to new design"
```

#### Troubleshooting

**ImageMagick not found:**
```bash
brew install imagemagick
```

**Permission denied:**
```bash
chmod +x scripts/generate-icons.sh
```

**Source image not square:**
The script will still work, but icons may appear stretched. Use a square source image for best results.

**Icons don't update after regeneration (macOS):**
macOS aggressively caches app icons. After generating new icons, use the refresh script:
```bash
./scripts/refresh-mac-icons.sh
```
See the "Icon Cache Refresh" section below for details.

#### Output Structure

```
src-tauri/icons/
├── icon.png              # 1024x1024 master
├── 32x32.png            # macOS menu bar
├── 128x128.png          # macOS standard
├── 128x128@2x.png       # macOS retina (256x256)
├── icon.icns            # macOS app bundle
├── icon.ico             # Windows executable
├── Square30x30Logo.png  # Windows Store tile
├── Square44x44Logo.png  # Windows Store tile
├── Square71x71Logo.png  # Windows Store tile
├── Square89x89Logo.png  # Windows Store tile
├── Square107x107Logo.png # Windows Store tile
├── Square142x142Logo.png # Windows Store tile
├── Square150x150Logo.png # Windows Store tile
├── Square284x284Logo.png # Windows Store tile
├── Square310x310Logo.png # Windows Store tile
└── StoreLogo.png        # Windows Store logo (50x50)
```

## Icon Cache Refresh

### `refresh-mac-icons.sh`

Clears macOS icon caches to show updated app icons. macOS caches icons in multiple locations (Dock, Icon Services, app bundles), which prevents new icons from appearing immediately.

#### When to Use

Run this script when:
- Icons don't update after running `generate-icons.sh`
- Alt-Tab still shows old app icon
- Dock shows old icon even after rebuild
- You want to ensure new icons are visible system-wide

#### Requirements

- **macOS only** (uses macOS-specific cache locations)
- **sudo access** (required to clear system-level caches)

#### Usage

```bash
./scripts/refresh-mac-icons.sh
```

#### What It Does

1. **Restarts Dock** - Clears Dock icon cache
2. **Clears Icon Services** - Removes system and user icon caches
3. **Updates timestamps** - Touches icon files to force refresh
4. **Cleans build cache** - Removes old app bundles
5. **Rebuilds app** - Creates new bundle with updated icons

#### Complete Icon Update Workflow

```bash
# 1. Generate new icons from source
./scripts/generate-icons.sh ~/Downloads/new-icon.png

# 2. Clear caches and rebuild
./scripts/refresh-mac-icons.sh

# 3. Install the new app bundle
open src-tauri/target/release/bundle/macos/

# 4. If icon still doesn't show, log out and back in
```

#### Why Icon Caching Happens

macOS caches icons for performance:
- **Development mode** (`bun run tauri dev`) uses temporary bundles that macOS caches
- **Icon Services** caches all app icons system-wide
- **Dock** maintains its own icon cache
- **Launch Services** database caches app metadata

The refresh script clears all these caches to ensure new icons are visible.

#### Alternative Manual Steps

If you prefer manual cache clearing:

```bash
# Kill Dock
killall Dock

# Clear Icon Services cache (requires sudo)
sudo rm -rf /Library/Caches/com.apple.iconservices.store
rm -rf ~/Library/Caches/com.apple.iconservices.store

# Rebuild app
bun run tauri build

# If needed, clear Launch Services database
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -kill -r -domain local -domain system -domain user
```

## Version Management

### `bump-version.sh`

Updates version numbers across all project configuration files to ensure consistency between the git tag and generated artifacts.

#### Requirements

- **macOS or Linux** (uses `sed` for file updates)
- **Git repository** (for version tracking)

#### Usage

```bash
./scripts/bump-version.sh <new_version>
```

**Example:**
```bash
./scripts/bump-version.sh 0.2.0
```

#### What It Updates

The script automatically updates version numbers in:

| File | Location | Purpose |
|------|----------|---------|
| `package.json` | Line 3 | JavaScript package version |
| `src-tauri/Cargo.toml` | Line 3 | Rust package version |
| `src-tauri/tauri.conf.json` | Line 4 | Tauri app version (used for DMG naming) |
| `src-tauri/Cargo.lock` | Auto | Rust dependency lock file |
| `Makefile` | Info section | Documentation version display |

#### Version Format

Supports semantic versioning (semver):
- **Standard:** `MAJOR.MINOR.PATCH` (e.g., `1.0.0`, `0.2.1`)
- **Pre-release:** `MAJOR.MINOR.PATCH-PRERELEASE` (e.g., `1.0.0-alpha.1`, `2.0.0-beta.2`)

#### Makefile Integration

Instead of running the script directly, use the Makefile commands:

```bash
# Show current version
make version

# Bump to specific version
make bump VERSION=0.2.0

# Automatic version bumps
make bump-patch   # 0.1.0 → 0.1.1
make bump-minor   # 0.1.0 → 0.2.0
make bump-major   # 0.1.0 → 1.0.0

# Complete release workflow (bump + commit + tag + push)
make release VERSION=0.2.0
```

#### Complete Release Workflow

**Option 1: Manual (step by step)**
```bash
# 1. Bump version
make bump VERSION=0.2.0

# 2. Review changes
git diff

# 3. Commit
git add -A
git commit -m "chore: bump version to 0.2.0"

# 4. Create tag
git tag v0.2.0

# 5. Push
git push origin main --tags
```

**Option 2: Automated (recommended)**
```bash
# Single command does everything
make release VERSION=0.2.0
```

The automated release command:
1. Checks git status is clean
2. Bumps version in all files
3. Commits the changes
4. Creates a git tag
5. Pushes to remote
6. Triggers GitHub Actions to build and publish

#### Why Version Consistency Matters

**Problem:** Git tag v0.2.0 but DMG named `keybinder_0.1.0_universal_darwin.dmg`

This happens when version in `tauri.conf.json` doesn't match the git tag. The Tauri build process reads `tauri.conf.json` to name the DMG file.

**Solution:** Always bump version in config files before creating git tag.

#### Troubleshooting

**Script fails with "Invalid version format":**
```bash
# Wrong format
./scripts/bump-version.sh v0.2.0  # Don't include 'v' prefix
./scripts/bump-version.sh 0.2     # Incomplete version

# Correct format
./scripts/bump-version.sh 0.2.0
```

**Git push fails during release:**
```bash
# Ensure you're on main branch and it's up to date
git checkout main
git pull origin main

# Then retry
make release VERSION=0.2.0
```

**Working directory not clean:**
```bash
# Commit or stash changes first
git status
git add -A
git commit -m "Your changes"

# Then retry release
make release VERSION=0.2.0
```

## Adding New Scripts

When adding new scripts to this directory:

1. Make them executable: `chmod +x scripts/your-script.sh`
2. Add a shebang: `#!/bin/bash`
3. Include usage documentation in comments
4. Document them in this README
5. Test before committing
