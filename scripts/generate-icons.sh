#!/bin/bash

#########################################
# Icon Generator Script
# Generates all required app icons from a single source image
#
# Usage: ./scripts/generate-icons.sh <source-image>
# Example: ./scripts/generate-icons.sh ~/Downloads/keybinder_icon_1024.png
#
# Requirements:
# - macOS (sips, iconutil)
# - ImageMagick (for .ico generation)
#########################################

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check arguments
if [ "$#" -ne 1 ]; then
    echo -e "${RED}Error: Missing source image${NC}"
    echo "Usage: $0 <source-image>"
    echo "Example: $0 ~/Downloads/keybinder_icon_1024.png"
    exit 1
fi

SOURCE_IMAGE="$1"
ICONS_DIR="src-tauri/icons"

# Validate source image exists
if [ ! -f "$SOURCE_IMAGE" ]; then
    echo -e "${RED}Error: Source image not found: $SOURCE_IMAGE${NC}"
    exit 1
fi

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null && ! command -v magick &> /dev/null; then
    echo -e "${YELLOW}Warning: ImageMagick not found. .ico file will not be generated.${NC}"
    echo "Install with: brew install imagemagick"
    SKIP_ICO=true
else
    SKIP_ICO=false
fi

echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Keybinder Icon Generator           ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Source image:${NC} $SOURCE_IMAGE"
echo -e "${GREEN}Output directory:${NC} $ICONS_DIR"
echo ""

# Create icons directory if it doesn't exist
mkdir -p "$ICONS_DIR"

#########################################
# Step 1: Copy and ensure RGBA format for master icon
#########################################
echo -e "${BLUE}[1/5]${NC} Preparing master icon (1024x1024)..."

# Use ImageMagick to ensure RGBA format if available, otherwise use sips
if command -v magick &> /dev/null || command -v convert &> /dev/null; then
    if command -v magick &> /dev/null; then
        CONVERT_CMD="magick"
    else
        CONVERT_CMD="convert"
    fi
    # Convert to RGBA PNG format
    $CONVERT_CMD "$SOURCE_IMAGE" -resize 1024x1024 -background none -gravity center -extent 1024x1024 "$ICONS_DIR/icon.png"
    echo -e "${GREEN}✓${NC} Created icon.png (RGBA with ImageMagick)"
else
    # Fallback to sips
    cp "$SOURCE_IMAGE" "$ICONS_DIR/icon.png"
    sips -z 1024 1024 "$ICONS_DIR/icon.png" &> /dev/null
    echo -e "${GREEN}✓${NC} Created icon.png (with sips)"
    echo -e "${YELLOW}⚠${NC}  ImageMagick not found - alpha channel may not be preserved"
fi

#########################################
# Step 2: Generate macOS PNG icons
#########################################
echo ""
echo -e "${BLUE}[2/5]${NC} Generating macOS PNG icons..."

if command -v magick &> /dev/null || command -v convert &> /dev/null; then
    # Use ImageMagick for proper RGBA handling
    $CONVERT_CMD "$ICONS_DIR/icon.png" -resize 32x32 "$ICONS_DIR/32x32.png"
    echo -e "${GREEN}✓${NC} Created 32x32.png (RGBA)"

    $CONVERT_CMD "$ICONS_DIR/icon.png" -resize 128x128 "$ICONS_DIR/128x128.png"
    echo -e "${GREEN}✓${NC} Created 128x128.png (RGBA)"

    $CONVERT_CMD "$ICONS_DIR/icon.png" -resize 256x256 "$ICONS_DIR/128x128@2x.png"
    echo -e "${GREEN}✓${NC} Created 128x128@2x.png (RGBA)"
else
    # Fallback to sips
    sips -z 32 32 "$ICONS_DIR/icon.png" --out "$ICONS_DIR/32x32.png" &> /dev/null
    echo -e "${GREEN}✓${NC} Created 32x32.png"

    sips -z 128 128 "$ICONS_DIR/icon.png" --out "$ICONS_DIR/128x128.png" &> /dev/null
    echo -e "${GREEN}✓${NC} Created 128x128.png"

    sips -z 256 256 "$ICONS_DIR/icon.png" --out "$ICONS_DIR/128x128@2x.png" &> /dev/null
    echo -e "${GREEN}✓${NC} Created 128x128@2x.png"
fi

#########################################
# Step 3: Generate macOS .icns
#########################################
echo ""
echo -e "${BLUE}[3/5]${NC} Generating macOS .icns file..."

# Create iconset directory
ICONSET_DIR="$ICONS_DIR/icon.iconset"
mkdir -p "$ICONSET_DIR"

# Generate all required sizes for .icns
sips -z 16 16 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_16x16.png" &> /dev/null
sips -z 32 32 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_16x16@2x.png" &> /dev/null
sips -z 32 32 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_32x32.png" &> /dev/null
sips -z 64 64 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_32x32@2x.png" &> /dev/null
sips -z 128 128 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_128x128.png" &> /dev/null
sips -z 256 256 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_128x128@2x.png" &> /dev/null
sips -z 256 256 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_256x256.png" &> /dev/null
sips -z 512 512 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_256x256@2x.png" &> /dev/null
sips -z 512 512 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_512x512.png" &> /dev/null
sips -z 1024 1024 "$ICONS_DIR/icon.png" --out "$ICONSET_DIR/icon_512x512@2x.png" &> /dev/null

# Convert to .icns
iconutil -c icns "$ICONSET_DIR" -o "$ICONS_DIR/icon.icns"
rm -rf "$ICONSET_DIR"

echo -e "${GREEN}✓${NC} Created icon.icns"

#########################################
# Step 4: Generate Windows .ico
#########################################
echo ""
echo -e "${BLUE}[4/5]${NC} Generating Windows .ico file..."

if [ "$SKIP_ICO" = false ]; then
    # Use magick if available (IMv7), otherwise use convert
    if command -v magick &> /dev/null; then
        CONVERT_CMD="magick"
    else
        CONVERT_CMD="convert"
    fi

    $CONVERT_CMD "$ICONS_DIR/icon.png" \
      \( -clone 0 -resize 16x16 \) \
      \( -clone 0 -resize 32x32 \) \
      \( -clone 0 -resize 48x48 \) \
      \( -clone 0 -resize 64x64 \) \
      \( -clone 0 -resize 128x128 \) \
      \( -clone 0 -resize 256x256 \) \
      -delete 0 -colors 256 "$ICONS_DIR/icon.ico" 2> /dev/null

    echo -e "${GREEN}✓${NC} Created icon.ico (6 embedded sizes)"
else
    echo -e "${YELLOW}⊘${NC} Skipped icon.ico (ImageMagick not installed)"
fi

#########################################
# Step 5: Generate Windows Store logos
#########################################
echo ""
echo -e "${BLUE}[5/5]${NC} Generating Windows Store logos..."

# Windows Store tile sizes
declare -a STORE_SIZES=(
    "30:Square30x30Logo.png"
    "44:Square44x44Logo.png"
    "71:Square71x71Logo.png"
    "89:Square89x89Logo.png"
    "107:Square107x107Logo.png"
    "142:Square142x142Logo.png"
    "150:Square150x150Logo.png"
    "284:Square284x284Logo.png"
    "310:Square310x310Logo.png"
)

for size_file in "${STORE_SIZES[@]}"; do
    SIZE="${size_file%%:*}"
    FILE="${size_file##*:}"
    sips -z "$SIZE" "$SIZE" "$ICONS_DIR/icon.png" --out "$ICONS_DIR/$FILE" &> /dev/null
    echo -e "${GREEN}✓${NC} Created $FILE"
done

# Store logo
sips -z 50 50 "$ICONS_DIR/icon.png" --out "$ICONS_DIR/StoreLogo.png" &> /dev/null
echo -e "${GREEN}✓${NC} Created StoreLogo.png"

#########################################
# Summary
#########################################
echo ""
echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Icon Generation Complete! 🎉       ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Generated files:${NC}"
echo "  • icon.png (1024x1024 master)"
echo "  • macOS: 32x32, 128x128, 128x128@2x, icon.icns"
if [ "$SKIP_ICO" = false ]; then
    echo "  • Windows: icon.ico"
else
    echo "  • Windows: icon.ico (skipped)"
fi
echo "  • Windows Store: 9 Square logos + StoreLogo"
echo ""
echo -e "${YELLOW}Total:${NC} 16 icon files"
echo ""
echo -e "${GREEN}Next steps:${NC}"
echo "  1. Review icons: ls -lh $ICONS_DIR/"
echo "  2. Test in app: bun run tauri dev"
echo "  3. Commit changes: git add $ICONS_DIR && git commit -m 'chore: Update app icons'"
echo ""
