# 2025-11-01-skhd-gui Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-11-01

## Active Technologies
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
- 002-config-import-export: Added Rust 1.75+ (backend), TypeScript 5+ (frontend), Svelte 5 (UI framework) + Tauri v2, rfd (Rust file dialogs), existing skhd parser

- 001-skhd-config-gui: Added Rust 1.75+ (backend), Svelte 4 + TypeScript 5 (frontend) + Tauri v2, pest v2.7+ (parser), tempfile v3.8+ (atomic writes), Vite v5 (build), Vitest v1+ (testing)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
