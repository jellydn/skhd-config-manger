.PHONY: help dev build test test-cov lint format check clean install deps tauri-dev tauri-build version bump bump-patch bump-minor bump-major release

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[0;36m
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m # No Color

## help: Show this help message
help:
	@echo "\033[0;36mskhd-gui Makefile Commands\033[0m"
	@echo ""
	@echo "\033[0;32mDevelopment:\033[0m"
	@echo "  make dev          - Start development server (Vite + Tauri watch mode)"
	@echo "  make watch        - Watch and rebuild on changes (frontend only)"
	@echo "  make preview      - Preview production build"
	@echo ""
	@echo "\033[0;32mTesting:\033[0m"
	@echo "  make test         - Run all tests"
	@echo "  make test-cov     - Run tests with coverage"
	@echo "  make test-rust    - Run only Rust tests"
	@echo "  make test-frontend - Run only frontend tests"
	@echo ""
	@echo "\033[0;32mQuality:\033[0m"
	@echo "  make lint         - Run linters (Rust + TypeScript/Svelte)"
	@echo "  make format       - Format code (Rust + TypeScript/Svelte)"
	@echo "  make check        - Run type checking and validation"
	@echo ""
	@echo "\033[0;32mBuild:\033[0m"
	@echo "  make build        - Build for production"
	@echo "  make tauri-build  - Build Tauri app"
	@echo "  make clean        - Clean build artifacts"
	@echo ""
	@echo "\033[0;32mMaintenance:\033[0m"
	@echo "  make deps         - Install dependencies (Rust + Node.js)"
	@echo "  make update       - Update dependencies"
	@echo "  make audit        - Security audit"
	@echo ""
	@echo "\033[0;32mCI/CD:\033[0m"
	@echo "  make ci           - Run CI checks (lint + test + build)"
	@echo ""
	@echo "\033[0;32mVersion Management:\033[0m"
	@echo "  make version      - Show current version"
	@echo "  make bump VERSION=x.y.z - Bump to specific version"
	@echo "  make bump-patch   - Bump patch version (0.1.0 → 0.1.1)"
	@echo "  make bump-minor   - Bump minor version (0.1.0 → 0.2.0)"
	@echo "  make bump-major   - Bump major version (0.1.0 → 1.0.0)"
	@echo "  make release VERSION=x.y.z - Bump, commit, tag, and push release"
	@echo ""
	@echo "\033[0;32mInfo:\033[0m"
	@echo "  make info         - Show project info"

## dev: Start development server (Vite + Tauri watch mode)
dev:
	@echo "$(CYAN)Starting development server...$(NC)"
	bun run tauri dev

## build: Build for production
build:
	@echo "$(CYAN)Building for production...$(NC)"
	bun run tauri build

## test: Run all tests
test:
	@echo "$(CYAN)Running Rust tests...$(NC)"
	cd src-tauri && cargo test
	@echo "$(CYAN)Running frontend tests...$(NC)"
	bun test

## test-cov: Run tests with coverage
test-cov:
	@echo "$(CYAN)Running tests with coverage...$(NC)"
	cd src-tauri && cargo test
	bun run test:coverage

## test-rust: Run only Rust tests
test-rust:
	@echo "$(CYAN)Running Rust tests...$(NC)"
	cd src-tauri && cargo test

## test-frontend: Run only frontend tests
test-frontend:
	@echo "$(CYAN)Running frontend tests...$(NC)"
	bun test

## lint: Run linters (Rust + TypeScript/Svelte)
lint:
	@echo "$(CYAN)Running Rust linter...$(NC)"
	cd src-tauri && cargo clippy -- -D warnings
	@echo "$(CYAN)Running frontend linter...$(NC)"
	bun run lint

## format: Format code (Rust + TypeScript/Svelte)
format:
	@echo "$(CYAN)Formatting Rust code...$(NC)"
	cd src-tauri && cargo fmt
	@echo "$(CYAN)Formatting frontend code...$(NC)"
	bun run format

## check: Run type checking and validation
check:
	@echo "$(CYAN)Checking Rust code...$(NC)"
	cd src-tauri && cargo check
	@echo "$(CYAN)Checking frontend code...$(NC)"
	bun run check

## clean: Clean build artifacts
clean:
	@echo "$(CYAN)Cleaning build artifacts...$(NC)"
	cd src-tauri && cargo clean
	rm -rf dist
	rm -rf .svelte-kit
	rm -rf node_modules/.vite

## install: Install all dependencies
install: deps

## deps: Install dependencies (Rust + Node.js)
deps:
	@echo "$(CYAN)Installing Rust dependencies...$(NC)"
	cd src-tauri && cargo fetch
	@echo "$(CYAN)Installing Node.js dependencies...$(NC)"
	bun install

## ci: Run CI checks (lint + test + build)
ci: lint test build
	@echo "$(GREEN)✓ All CI checks passed!$(NC)"

## tauri-dev: Start Tauri development mode only
tauri-dev:
	@echo "$(CYAN)Starting Tauri dev mode...$(NC)"
	bun run tauri dev

## tauri-build: Build Tauri app
tauri-build:
	@echo "$(CYAN)Building Tauri application...$(NC)"
	bun run tauri build

## watch: Watch and rebuild on changes (frontend only)
watch:
	@echo "$(CYAN)Starting Vite watch mode...$(NC)"
	bun run dev

## preview: Preview production build
preview:
	@echo "$(CYAN)Starting preview server...$(NC)"
	bun run preview

## update: Update dependencies
update:
	@echo "$(CYAN)Updating Rust dependencies...$(NC)"
	cd src-tauri && cargo update
	@echo "$(CYAN)Updating Node.js dependencies...$(NC)"
	bun update

## audit: Security audit
audit:
	@echo "$(CYAN)Running Rust security audit...$(NC)"
	cd src-tauri && cargo audit || echo "$(YELLOW)Install cargo-audit: cargo install cargo-audit$(NC)"
	@echo "$(CYAN)Running npm security audit...$(NC)"
	bun audit || true

## info: Show project info
info:
	@echo "$(CYAN)Project Information:$(NC)"
	@echo "  Name: skhd-gui"
	@echo "  Version: 0.1.0"
	@echo "  Description: Native macOS GUI for managing skhd keyboard shortcuts"
	@echo ""
	@echo "$(CYAN)Rust:$(NC)"
	@rustc --version
	@cargo --version
	@echo ""
	@echo "$(CYAN)Node.js:$(NC)"
	@node --version
	@bun --version
	@echo ""
	@echo "$(CYAN)Tauri:$(NC)"
	@bun run tauri --version || echo "Tauri CLI not installed"

## version: Show current version
version:
	@echo "$(CYAN)Current version:$(NC)"
	@grep '"version":' package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/'

## bump: Bump to a specific version
bump:
ifndef VERSION
	@echo "$(RED)Error: VERSION is required$(NC)"
	@echo "Usage: make bump VERSION=x.y.z"
	@echo "Example: make bump VERSION=0.2.0"
	@exit 1
endif
	@echo "$(CYAN)Bumping version to $(VERSION)...$(NC)"
	@./scripts/bump-version.sh $(VERSION)

## bump-patch: Bump patch version (0.1.0 → 0.1.1)
bump-patch:
	@echo "$(CYAN)Calculating new patch version...$(NC)"
	@CURRENT=$$(grep '"version":' package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/'); \
	MAJOR=$$(echo $$CURRENT | cut -d. -f1); \
	MINOR=$$(echo $$CURRENT | cut -d. -f2); \
	PATCH=$$(echo $$CURRENT | cut -d. -f3 | cut -d- -f1); \
	NEW_PATCH=$$((PATCH + 1)); \
	NEW_VERSION="$$MAJOR.$$MINOR.$$NEW_PATCH"; \
	$(MAKE) bump VERSION=$$NEW_VERSION

## bump-minor: Bump minor version (0.1.0 → 0.2.0)
bump-minor:
	@echo "$(CYAN)Calculating new minor version...$(NC)"
	@CURRENT=$$(grep '"version":' package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/'); \
	MAJOR=$$(echo $$CURRENT | cut -d. -f1); \
	MINOR=$$(echo $$CURRENT | cut -d. -f2); \
	NEW_MINOR=$$((MINOR + 1)); \
	NEW_VERSION="$$MAJOR.$$NEW_MINOR.0"; \
	$(MAKE) bump VERSION=$$NEW_VERSION

## bump-major: Bump major version (0.1.0 → 1.0.0)
bump-major:
	@echo "$(CYAN)Calculating new major version...$(NC)"
	@CURRENT=$$(grep '"version":' package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/'); \
	MAJOR=$$(echo $$CURRENT | cut -d. -f1); \
	NEW_MAJOR=$$((MAJOR + 1)); \
	NEW_VERSION="$$NEW_MAJOR.0.0"; \
	$(MAKE) bump VERSION=$$NEW_VERSION

## release: Bump version, commit, tag, and push release
release:
ifndef VERSION
	@echo "$(RED)Error: VERSION is required$(NC)"
	@echo "Usage: make release VERSION=x.y.z"
	@echo "Example: make release VERSION=0.2.0"
	@exit 1
endif
	@echo "$(CYAN)Creating release $(VERSION)...$(NC)"
	@echo ""
	@echo "$(YELLOW)Step 1/5: Checking git status...$(NC)"
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "$(RED)Error: Working directory is not clean$(NC)"; \
		echo "Please commit or stash your changes first"; \
		exit 1; \
	fi
	@echo "$(GREEN)✓ Working directory is clean$(NC)"
	@echo ""
	@echo "$(YELLOW)Step 2/5: Bumping version to $(VERSION)...$(NC)"
	@./scripts/bump-version.sh $(VERSION)
	@echo "$(GREEN)✓ Version bumped$(NC)"
	@echo ""
	@echo "$(YELLOW)Step 3/5: Committing changes...$(NC)"
	@git add -A
	@git commit -m "chore: bump version to $(VERSION)"
	@echo "$(GREEN)✓ Changes committed$(NC)"
	@echo ""
	@echo "$(YELLOW)Step 4/5: Creating tag v$(VERSION)...$(NC)"
	@git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@echo "$(GREEN)✓ Tag created$(NC)"
	@echo ""
	@echo "$(YELLOW)Step 5/5: Pushing to remote...$(NC)"
	@git push origin main
	@git push origin v$(VERSION)
	@echo "$(GREEN)✓ Pushed to remote$(NC)"
	@echo ""
	@echo "$(GREEN)========================================$(NC)"
	@echo "$(GREEN)✓ Release $(VERSION) created successfully!$(NC)"
	@echo "$(GREEN)========================================$(NC)"
	@echo ""
	@echo "$(CYAN)GitHub Actions will now:$(NC)"
	@echo "  1. Build the application"
	@echo "  2. Create a GitHub release"
	@echo "  3. Upload the DMG artifact"
	@echo ""
	@echo "$(CYAN)Track the release at:$(NC)"
	@echo "  https://github.com/jellydn/skhd-config-manger/actions"
