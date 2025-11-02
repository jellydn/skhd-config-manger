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

## Adding New Scripts

When adding new scripts to this directory:

1. Make them executable: `chmod +x scripts/your-script.sh`
2. Add a shebang: `#!/bin/bash`
3. Include usage documentation in comments
4. Document them in this README
5. Test before committing
