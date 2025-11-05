# Keybinder

A modern, intuitive macOS app for managing [skhd](https://github.com/koekeishiya/skhd) keyboard shortcuts.

![CI](https://github.com/jellydn/skhd-config-manger/workflows/CI/badge.svg)
![Release](https://github.com/jellydn/skhd-config-manger/workflows/Release/badge.svg)
![Keybinder](https://img.shields.io/badge/platform-macOS-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange)
![Svelte](https://img.shields.io/badge/Svelte-5-red)
![Rust](https://img.shields.io/badge/Rust-1.75+-brown)

## Features

### Configuration Management

- **Auto-Detection**: Automatically detects skhd configuration from standard locations:
  - `$XDG_CONFIG_HOME/skhd/skhdrc`
  - `~/.config/skhd/skhdrc`
  - `~/.skhdrc`
- **Import/Export**: Browse and open configs from any location
- **File Editor Pattern**: Changes stay in memory until explicitly saved
- **Real-time Validation**: Syntax checking with detailed error messages

### Shortcut Management

- **Visual Editor**: Clean, modern interface for editing keyboard shortcuts
- **CRUD Operations**: Create, Read, Update, Delete shortcuts with ease
- **Duplicate Detection**: Prevents conflicting key combinations
- **Modifier Support**: Full support for cmd, shift, ctrl, alt, fn modifiers

### Command Testing & Execution

- **Test Execution**: Execute commands directly from the GUI with real-time feedback
- **Safety Controls**: Automatic detection and confirmation for destructive commands (rm, sudo, etc.)
- **Output Capture**: View stdout, stderr, and exit codes for debugging
- **Cancellation**: Stop long-running commands instantly with the cancel button
- **Timeout Protection**: Automatic 30-second timeout prevents hanging commands
- **Execution Status**: Visual indicators (spinner, duration) during command execution

### Backup & Safety

- **Atomic Writes**: Safe file operations prevent corruption
- **Backup System**: Automatic backups before major changes
- **Unsaved Changes Protection**: Confirmation dialogs prevent accidental data loss

### Service Manager

- **Real-time Log Viewer**: Live streaming of skhd service logs with automatic updates using `tail -f`
- **Historical Logs**: Load up to 100 recent logs from before the app opened
- **Progressive Loading**: Infinite scroll support to load older logs on demand
- **Dual Log Sources**: Monitors both stdout (INFO) and stderr (ERROR) streams
- **Level Filtering**: Toggle between ERROR and INFO log views with pagination support
- **Virtual Scrolling**: Smooth performance with thousands of logs (1000+ entries)
- **Color-Coded Levels**: Visual distinction for ERROR (red) and INFO (blue) messages
- **Service Status**: Real-time monitoring of skhd service state and PID
- **Service Reload**: One-click service restart to apply configuration changes
- **Configuration Import**: Import external configs with visual pending state before reload
- **Auto-scroll Control**: Smart auto-scroll that pauses when scrolling manually
- **Descending Order**: Logs displayed in descending order (newest first) by default for optimal viewing
- **Clear Logs**: Remove all logs from viewer with one click
- **Accessibility**: Full ARIA support for screen readers and keyboard navigation
- **Circuit Breaker**: Automatic polling pause after repeated failures to prevent resource exhaustion
- **Memory Management**: Proper cleanup of timeouts and event listeners to prevent memory leaks

### User Experience

- **Welcome Screen**: Guided onboarding with clear action paths
- **Error Handling**: Helpful error messages with actionable solutions
- **System Theme Integration**: Automatic detection and application of macOS system theme (light/dark mode)
  - **Launch Detection**: App automatically detects and applies system theme on startup
  - **Runtime Updates**: Theme updates dynamically when system theme changes (within 2 seconds)
  - **Consistent Theming**: All interface elements (backgrounds, text, borders, buttons, modals, inputs) adapt to the active theme
  - **Smooth Transitions**: CSS transitions provide seamless theme switching without visual glitches
- **Keyboard Shortcuts Count**: Real-time shortcut count display

## Prerequisites

- macOS 10.15 or later
- [skhd](https://github.com/koekeishiya/skhd) installed (optional for config detection)
- [Rust](https://rustup.rs/) 1.75+ (for building from source)
- [Bun](https://bun.sh/) or [Node.js](https://nodejs.org/) 18+ (for frontend development)

## Installation

### From Release (Recommended)

1. Go to the [Releases page](https://github.com/jellydn/skhd-config-manger/releases)
2. Download the latest `.dmg` file (works on both Intel and Apple Silicon Macs)
3. Open the DMG and drag Keybinder to your Applications folder
4. Launch from Applications or Spotlight

**Note**: On first launch, you may need to right-click → Open to bypass macOS Gatekeeper.

### From Source

1. Clone the repository:

```bash
git clone https://github.com/jellydn/skhd-config-manger.git
cd skhd-config-manger
```

2. Install dependencies:

```bash
bun install
# or
npm install
```

3. Build and run:

```bash
bun run tauri dev
# or
npm run tauri dev
```

### Building Release

```bash
bun run tauri build
# or
npm run tauri build
```

The built app will be in `src-tauri/target/release/bundle/`.

## Usage

### Getting Started

1. **Launch the app** - You'll see a welcome screen with three options:
   - **Detect Active Config**: Automatically finds your skhd config
   - **Import Existing File**: Browse for a config file
   - **Create New Config**: Start with an empty configuration

2. **Edit Shortcuts**:
   - Click **+ New Shortcut** to add a shortcut
   - Click **Edit** on any shortcut to modify it
   - Click **Delete** to remove a shortcut

3. **Save Changes**:
   - The **Save Changes** button appears when you have unsaved changes
   - Click **Save** to write changes to disk
   - Click **Home** to return to the welcome screen (warns if unsaved changes)

4. **Test Commands**:
   - Click the **▶ Execute** button next to any shortcut to test it
   - View real-time output (stdout, stderr, exit code, duration)
   - **Safety confirmations** for destructive commands (rm, sudo, kill, etc.)
   - **Cancel button** appears during execution to stop long-running commands
   - Commands timeout automatically after 30 seconds

### Keyboard Shortcut Format

Shortcuts follow the skhd format:

```
[modifiers] - key : command
```

Examples:

- `cmd - return : open -a Terminal`
- `cmd + shift - f : open ~`
- `ctrl + alt - b : open -a "Brave Browser"`

### Command Testing Safety

Keybinder automatically detects potentially destructive commands and requires confirmation:

**Destructive patterns detected:**
- File operations: `rm`, `mv`, `cp` with system paths
- System commands: `sudo`, `shutdown`, `reboot`, `killall`
- Permission changes: `chmod`, `chown`
- Package management: `brew uninstall`, `npm uninstall`

**Example workflow:**
1. Create shortcut: `cmd + shift - t : rm -rf /tmp/test`
2. Click **Execute** button
3. ⚠️ **Warning modal appears**: "This command may be destructive"
4. Review command and click **Execute Anyway** to proceed
5. View results: exit code, output, duration

### Importing/Exporting

- **Import**: Click **Import...** to browse for an existing skhd config file

### Service Manager

The Service Manager provides comprehensive monitoring and control of the skhd service:

#### Viewing Logs

1. **Navigate to Service Manager** via the sidebar navigation
2. **Automatic Streaming** - Log stream starts automatically on page load
3. **Historical Logs** - Automatically loads the last 100 recent logs
4. **Progressive Loading** - Scroll to load more historical logs (500 at a time)
5. **Level Filtering**:
   - **ERROR Tab**: Shows only stderr messages (critical issues)
   - **INFO Tab**: Shows stdout messages (general information)
   - Each tab has independent pagination and filtering
6. **Color-Coded Levels**:
   - 🔴 ERROR: Red text for critical issues
   - 🔵 INFO: Blue text for general information

#### Log Controls

- **Filter Icon**: Click the filter icon (positioned after pagination) to toggle between ERROR and INFO log views
  - Shows warning triangle icon when filtering errors
  - Shows list icon when filtering info logs
- **Pagination**: Navigate through logs 50 entries at a time
- **Descending Order**: Logs displayed newest-first by default (no toggle needed)
- **Auto-scroll**: Enable/disable automatic scrolling to new logs
- **Load More**: Button to fetch older historical logs (500 more entries)
- **Keyboard Navigation**: Arrow keys, Page Up/Down, Home/End for scrolling

#### Service Management

1. **Monitor Status**:
   - View real-time service state (Running, Stopped, Starting, Stopping, Reloading, Error)
   - See current PID when service is running
   - Status indicator with color coding (green = running, red = error, orange = transitioning)
   - Automatic status polling every 5 seconds with exponential backoff on failures
   - Circuit breaker pauses polling after 5 consecutive failures

2. **Reload Service**:
   - Click **Reload Service** to restart skhd with current configuration
   - Shows "Reloading..." spinner during the operation
   - Service automatically reloads and displays new PID
   - Check logs for reload confirmation messages
   - Success/error notifications with auto-dismiss after 5 seconds

#### Configuration Import Workflow

1. **Import Config**: Click **Import Config** to select an external skhd configuration file
2. **Pending State**: Imported config shows in orange with "(Click 'Reload Service' to apply)" hint
3. **Apply Changes**: Click **Reload Service** to activate the imported configuration
4. **Active State**: Config path returns to normal styling, service uses new configuration

#### Accessibility Features

- **Keyboard Navigation**: Tab through all controls, Enter/Space to activate
- **Screen Reader Support**: Full ARIA labels and live region announcements
- **Log Viewer Focus**: Press Tab to focus log viewer, use arrow keys to scroll
- **Status Announcements**: Service state changes announced automatically

## Development Workflow

### Tech Stack

- **Frontend**: Svelte 5 (with Runes mode for reactive state management)
- **Backend**: Rust 1.75+ with Tauri v2 framework
- **Build Tool**: Vite 5 with SvelteKit for frontend bundling
- **Package Manager**: Bun (or npm/pnpm as alternatives)
- **Testing**: Vitest for unit tests, cargo test for Rust backend
- **Linting**: ESLint + TypeScript + cargo clippy

**Note on Svelte 5**: This project fully embraces Svelte 5's runes mode (`$state`, `$derived`, `$props`, `$effect`) for cleaner, more explicit reactivity. All components use modern syntax including `{@render children()}` instead of slots and `onclick={}` instead of `on:click={}`.

### CI/CD Pipeline

This project uses GitHub Actions for automated quality checks and releases:

#### Continuous Integration (CI)
- **Trigger**: Every push and pull request
- **Checks**:
  - Rust: `cargo test`, `cargo clippy`, `cargo check`
  - TypeScript: `bun run typecheck`
  - Runs on macOS latest
- **Badge**: ![CI](https://github.com/jellydn/skhd-config-manger/workflows/CI/badge.svg)

#### Automated Releases
- **Trigger**: Push version tags (e.g., `v1.0.0`, `v1.0.0-beta.1`)
- **Output**: Universal DMG (Intel + Apple Silicon)
- **Location**: [GitHub Releases](https://github.com/jellydn/skhd-config-manger/releases)
- **Badge**: ![Release](https://github.com/jellydn/skhd-config-manger/workflows/Release/badge.svg)

#### Creating a Release
```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions will automatically:
# 1. Build universal DMG
# 2. Create GitHub release
# 3. Upload DMG as downloadable asset
```

### Spec-Kit Development

This project uses [spec-kit](https://github.com/github/spec-kit) for specification-driven development. Each feature is documented in the `specs/` directory.

### Understanding Spec-Kit

**TL;DR**: Spec-kit is great for greenfield projects, less practical for ongoing development. Use it for initial feature planning, but expect to deviate during implementation.

**What it's good for:**
- ✅ Initial feature planning and requirements gathering
- ✅ Creating structured documentation for new features
- ✅ Establishing project constitution and coding standards
- ✅ Generating implementation tasks from specifications

**Limitations in real-world usage:**
- ❌ Assumes specs remain static during implementation (rarely true)
- ❌ No clear guidance for handling PR feedback or bug fixes
- ❌ Designed for research/academic contexts, not production workflows
- ❌ Spec-to-code sync breaks down quickly in iterative development

### How We Use Spec-Kit

We use a **pragmatic subset** of spec-kit for initial planning only:

#### 1. **Initial Feature Specification**
```bash
# Create a new feature spec (creates branch and spec directory)
/speckit.specify <description of what you want to build>

# Example: /speckit.specify Add configuration import/export functionality
# Creates: specs/002-config-import-export/spec.md
# Creates branch: 002-config-import-export
```

#### 2. **Implementation Planning**
```bash
# Generate technical plan from specification
/speckit.plan <tech stack and architecture decisions>

# Example: /speckit.plan Use Rust rfd crate for file dialogs, existing parser
# Creates: specs/002-config-import-export/plan.md
```

#### 3. **Task Breakdown**
```bash
# Generate implementation tasks
/speckit.tasks

# Creates: specs/002-config-import-export/tasks.md
```

#### 4. **After That: Normal Development**

Once specs and tasks are generated, we **abandon strict spec-kit workflow**:

- ✅ Implement features based on tasks.md
- ✅ Fix bugs as they're discovered (don't update specs retroactively)
- ✅ Handle PR feedback directly in code
- ✅ Make UX improvements based on testing
- ✅ Update CLAUDE.md with tech stack changes (not specs)

**Why?** Because real development is iterative, specs get stale quickly, and maintaining spec-to-code sync is overhead without value.

### Feature Development Process

Our actual workflow (spec-kit + pragmatic deviations):

```bash
# 1. Plan feature with spec-kit
/speckit.specify <feature description>
/speckit.plan <technical approach>
/speckit.tasks

# 2. Normal development
git checkout -b <feature-branch>
# Implement features from tasks.md
# Run tests: bun run typecheck, cargo test, cargo clippy
# Iterate based on feedback

# 3. PR and review
git commit -m "Implement <feature>: what, why, how"
gh pr create --title "Feature: <name>" --body "..."

# 4. Handle feedback PRAGMATICALLY
# - Fix bugs in code (don't update specs)
# - Address PR comments directly
# - Update docs/checklists with lessons learned
# - Specs are historical record, not living docs
```

### Directory Structure

```
specs/
  001-skhd-config-gui/       # Initial GUI feature
    spec.md                   # Requirements and user stories
    plan.md                   # Technical implementation plan
    tasks.md                  # Task breakdown
    data-model.md            # Data structures
    contracts/               # API contracts
  002-config-import-export/  # Import/export feature
    spec.md
    plan.md
    tasks.md
    ...
```

**Note**: Files in `specs/` are **reference documentation** for feature planning. Don't expect them to match current code exactly—they represent the initial plan, not the evolved implementation.

### Spec-Kit Decision Framework

When working with spec-kit, use this framework to decide whether to update specs:

#### ✅ Update Specs When:

- **Missing requirements discovered** (accessibility, security, performance)
  - Example: PR feedback reveals missing aria-labels → Add NFR to spec
- **Acceptance criteria incomplete/wrong**
  - Example: Edge case not covered → Update scenarios
- **New constraint for ALL future features**
  - Example: File size limits, browser compatibility
- **Production incident reveals spec gap**
  - Example: Crash on large files → Add performance NFR
- **API contract changes**
  - Example: New required parameter → Update contracts

#### ❌ Don't Update Specs (Just Fix Code):

- Typos or syntax errors
- Better variable naming
- Code refactoring (same behavior)
- Performance optimization (already meets spec)
- Bug in implementation logic (spec was correct)
- UI polish/styling tweaks

#### 🔄 Real Example from This Project:

**PR Feedback**: "Missing aria-labels on icon buttons"

**Spec-Kit Approach:**
1. Recognized as **spec gap** (accessibility requirement missing)
2. Updated `specs/003-shortcut-duplicate/spec.md` with NFR-A01 to NFR-A04
3. Fixed code (added aria-labels)
4. Updated checklist (marked accessibility complete)
5. **Result**: Future features won't miss this requirement

#### 📚 Full Methodology Guide

See [`claudedocs/spec-kit-methodology-summary.md`](claudedocs/spec-kit-methodology-summary.md) for:
- Complete decision frameworks
- Handling PR comments, bug fixes, production issues
- When spec-kit works well vs. when to adapt
- Team collaboration workflows
- Lessons learned from real implementation

### Contributing

When adding features:

1. Use spec-kit for initial planning (specify → plan → tasks)
2. **During PR review**: If feedback reveals missing requirements, update specs
3. Create clear, focused PRs with "what, why, how" descriptions
4. Run tests before committing: `bun run typecheck && cargo test && cargo clippy`
5. Use judgment: Update specs for missing requirements, just fix code for implementation bugs

## Author

👤 **Huynh Duc Dung**

- Website: https://productsway.com/
- Twitter: [@jellydn](https://twitter.com/jellydn)
- Github: [@jellydn](https://github.com/jellydn)

## Show your support

Give a ⭐️ if this project helped you!

[![kofi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/dunghd)
[![paypal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://paypal.me/dunghd)
[![buymeacoffee](https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://www.buymeacoffee.com/dunghd)
