#!/bin/bash

#########################################
# macOS Icon Cache Refresh Script
# Clears macOS icon caches to show updated app icons
#
# Usage: ./scripts/refresh-mac-icons.sh
#########################################

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   macOS Icon Cache Refresh           ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""

# 1. Kill and restart Dock (refreshes Dock icon cache)
echo -e "${BLUE}[1/5]${NC} Restarting Dock..."
killall Dock
echo -e "${GREEN}✓${NC} Dock restarted"

# 2. Clear Icon Services cache
echo ""
echo -e "${BLUE}[2/5]${NC} Clearing Icon Services cache..."
sudo rm -rfv /Library/Caches/com.apple.iconservices.store 2>/dev/null || true
rm -rfv ~/Library/Caches/com.apple.iconservices.store 2>/dev/null || true
echo -e "${GREEN}✓${NC} Icon Services cache cleared"

# 3. Touch icon files to update timestamps
echo ""
echo -e "${BLUE}[3/5]${NC} Updating icon file timestamps..."
touch src-tauri/icons/*
echo -e "${GREEN}✓${NC} Icon timestamps updated"

# 4. Clean Tauri build cache
echo ""
echo -e "${BLUE}[4/5]${NC} Cleaning Tauri build cache..."
if [ -d "src-tauri/target" ]; then
    rm -rf src-tauri/target/release/bundle 2>/dev/null || true
    echo -e "${GREEN}✓${NC} Build cache cleaned"
else
    echo -e "${YELLOW}⊘${NC} No build cache to clean"
fi

# 5. Rebuild app with new icons
echo ""
echo -e "${BLUE}[5/5]${NC} Rebuilding app..."
echo -e "${YELLOW}Running: bun run tauri build${NC}"
echo ""

cd "$(dirname "$0")/.."
bun run tauri build

echo ""
echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Icon Cache Refresh Complete! 🎉    ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Next steps:${NC}"
echo "  1. The new app bundle is in: src-tauri/target/release/bundle/"
echo "  2. Install the new .app or .dmg"
echo "  3. If icon still doesn't update, log out and log back in"
echo ""
echo -e "${YELLOW}Note:${NC} For development mode (bun run tauri dev):"
echo "  Icon changes may not show until you rebuild the app bundle"
echo ""
