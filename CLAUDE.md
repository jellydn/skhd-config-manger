# Keybinder Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-11-02

## Active Technologies
- Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework) + Tauri v2 (app framework), existing ShortcutForm and ShortcutItem components (003-shortcut-duplicate)
- YAML (GitHub Actions workflows), Rust 1.75+, TypeScript 5+, macOS 11+ + GitHub Actions (tauri-apps/tauri-action), Tauri CLI v2, cargo, bun, clippy, existing test suite (004-ci-cd-design)
- N/A (CI/CD infrastructure, assets stored in git) (004-ci-cd-design)
- Rust 1.75+ (backend), TypeScript 5+ (frontend) + Tauri v2, tokio (async runtime), chrono (timestamps), Svelte 5 (UI framework) (005-command-test)
- Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework) + Tauri v2, tokio (async runtime), notify (file watching), chrono (timestamps), existing skhd parser (006-service-log-reload)
- File-based (skhd logs from system, no database required) (006-service-log-reload)

- Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework) + Tauri v2, rfd (Rust file dialogs), existing skhd parser (002-config-import-export)
- File-based (skhd configuration files), no database required (002-config-import-export)

- Rust 1.75+ (backend), Svelte 4 + TypeScript 5 (frontend) + Tauri v2, pest v2.7+ (parser), tempfile v3.8+ (atomic writes), Vite v5 (build), Vitest v1+ (testing) (001-skhd-config-gui)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.75+ (backend), Svelte 4 + TypeScript 5 (frontend): Follow standard conventions

## Recent Changes
- 006-service-log-reload: Added Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework) + Tauri v2, tokio (async runtime), notify (file watching), chrono (timestamps), existing skhd parser
- 005-command-test: Added Rust 1.75+ (backend), TypeScript 5+ (frontend) + Tauri v2, tokio (async runtime), chrono (timestamps), Svelte 5 (UI framework)
- 004-ci-cd-design: Added YAML (GitHub Actions workflows), Rust 1.75+, TypeScript 5+, macOS 11+ + GitHub Actions (tauri-apps/tauri-action), Tauri CLI v2, cargo, bun, clippy, existing test suite



<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
