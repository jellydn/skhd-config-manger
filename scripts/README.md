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

## Adding New Scripts

When adding new scripts to this directory:

1. Make them executable: `chmod +x scripts/your-script.sh`
2. Add a shebang: `#!/bin/bash`
3. Include usage documentation in comments
4. Document them in this README
5. Test before committing
