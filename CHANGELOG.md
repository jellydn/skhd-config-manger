# Changelog

All notable changes to skhd GUI will be documented in this file.

**Note**: Starting from version 0.1.0, release notes are automatically generated from git commit messages when creating new releases. See individual [GitHub Releases](https://github.com/jellydn/skhd-config-manger/releases) for detailed changelogs.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Automated CI/CD pipeline with GitHub Actions
- Professional DMG installer with custom background
- Universal binary support (Intel + Apple Silicon)
- Automated quality checks (cargo test, clippy, typecheck)
- GitHub releases automation on version tags

## [0.1.0] - 2025-11-02

### Added
- Initial release of skhd GUI
- Visual editor for skhd keyboard shortcuts
- Import/export skhd configuration files
- Shortcut duplication functionality
- Basic configuration management interface

### Technical
- Built with Tauri v2
- Svelte 5 frontend
- Rust backend with pest parser for skhd configs
- macOS 11+ support
