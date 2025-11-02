#!/usr/bin/env bash
# Version bump script for Keybinder
# Updates version in Cargo.toml, package.json, and tauri.conf.json
# Usage: ./scripts/bump-version.sh <new_version>

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Get new version from argument
NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
  echo -e "${RED}Error: Version argument required${NC}"
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

# Validate version format (basic semver)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
  echo -e "${RED}Error: Invalid version format${NC}"
  echo "Version must be in semver format: MAJOR.MINOR.PATCH[-PRERELEASE]"
  echo "Examples: 1.0.0, 1.2.3, 2.0.0-alpha.1"
  exit 1
fi

# Get current version from package.json
CURRENT_VERSION=$(grep '"version":' package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/')

echo -e "${CYAN}Current version: ${CURRENT_VERSION}${NC}"
echo -e "${CYAN}New version: ${NEW_VERSION}${NC}"
echo ""

# Update package.json
echo -e "${YELLOW}Updating package.json...${NC}"
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  sed -i '' "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" package.json
else
  # Linux
  sed -i "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" package.json
fi

# Update Cargo.toml
echo -e "${YELLOW}Updating Cargo.toml...${NC}"
if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i '' "s/^version = \"${CURRENT_VERSION}\"/version = \"${NEW_VERSION}\"/" src-tauri/Cargo.toml
else
  sed -i "s/^version = \"${CURRENT_VERSION}\"/version = \"${NEW_VERSION}\"/" src-tauri/Cargo.toml
fi

# Update tauri.conf.json
echo -e "${YELLOW}Updating tauri.conf.json...${NC}"
if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i '' "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" src-tauri/tauri.conf.json
else
  sed -i "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" src-tauri/tauri.conf.json
fi

# Update Makefile info section
echo -e "${YELLOW}Updating Makefile...${NC}"
if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i '' "s/Version: ${CURRENT_VERSION}/Version: ${NEW_VERSION}/" Makefile
else
  sed -i "s/Version: ${CURRENT_VERSION}/Version: ${NEW_VERSION}/" Makefile
fi

# Update Cargo.lock
echo -e "${YELLOW}Updating Cargo.lock...${NC}"
cd src-tauri && cargo update --workspace --offline 2>/dev/null || cargo update --workspace
cd ..

echo ""
echo -e "${GREEN}✓ Version updated successfully!${NC}"
echo -e "${GREEN}  ${CURRENT_VERSION} → ${NEW_VERSION}${NC}"
echo ""
echo -e "${CYAN}Files updated:${NC}"
echo "  - package.json"
echo "  - src-tauri/Cargo.toml"
echo "  - src-tauri/tauri.conf.json"
echo "  - src-tauri/Cargo.lock"
echo "  - Makefile"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Review the changes: git diff"
echo "  2. Commit the version bump: git add -A && git commit -m 'chore: bump version to ${NEW_VERSION}'"
echo "  3. Create a tag: git tag v${NEW_VERSION}"
echo "  4. Push: git push origin main --tags"
echo ""
echo -e "${CYAN}Or use 'make release VERSION=${NEW_VERSION}' to automate all steps${NC}"
