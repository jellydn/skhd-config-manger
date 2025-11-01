# skhd Configuration Manager

A modern, user-friendly GUI application for managing [skhd](https://github.com/koekeishiya/skhd) keyboard shortcuts on macOS.

![skhd GUI Manager](https://img.shields.io/badge/platform-macOS-blue)
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

### Backup & Safety

- **Atomic Writes**: Safe file operations prevent corruption
- **Backup System**: Automatic backups before major changes
- **Unsaved Changes Protection**: Confirmation dialogs prevent accidental data loss

### User Experience

- **Welcome Screen**: Guided onboarding with clear action paths
- **Error Handling**: Helpful error messages with actionable solutions
- **Dark Mode**: Full dark mode support following system preferences
- **Keyboard Shortcuts Count**: Real-time shortcut count display

## Prerequisites

- macOS 10.15 or later
- [skhd](https://github.com/koekeishiya/skhd) installed (optional for config detection)
- [Rust](https://rustup.rs/) 1.75+ (for building from source)
- [Bun](https://bun.sh/) or [Node.js](https://nodejs.org/) 18+ (for frontend development)

## Installation

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

### Keyboard Shortcut Format

Shortcuts follow the skhd format:

```
[modifiers] - key : command
```

Examples:

- `cmd - return : open -a Terminal`
- `cmd + shift - f : open ~`
- `ctrl + alt - b : open -a "Brave Browser"`

### Importing/Exporting

- **Import**: Click **Import...** to browse for an existing skhd config file

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
