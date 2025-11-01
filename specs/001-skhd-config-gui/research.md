# Research: skhd Configuration GUI

**Feature**: 001-skhd-config-gui
**Created**: 2025-11-01
**Purpose**: Resolve technical unknowns and establish implementation approach

## Research Tasks

### 1. Frontend Framework Selection

**Question**: Which frontend framework (HTML/CSS/JS, React, Vue, Svelte) should be used for the Tauri application?

**Research Findings**:

**Option A: Vanilla HTML/CSS/JavaScript**

- Pros: Minimal bundle size, fastest performance, no build complexity
- Cons: More manual DOM manipulation, less structured state management
- Bundle impact: ~5-10MB
- Performance: Fastest (no framework overhead)

**Option B: React**

- Pros: Large ecosystem, familiar to many developers, excellent tooling, strong TypeScript support
- Cons: Larger bundle size, can be overkill for simple UIs
- Bundle impact: ~15-20MB
- Performance: Good with proper optimization

**Option C: Vue 3**

- Pros: Lightweight, intuitive, good performance, excellent documentation
- Cons: Smaller ecosystem than React
- Bundle impact: ~10-15MB
- Performance: Excellent

**Option D: Svelte**

- Pros: Smallest bundle size, compiles to vanilla JS, excellent performance, simple syntax
- Cons: Smaller ecosystem, less mature tooling
- Bundle impact: ~5-10MB
- Performance: Best (compile-time framework)

**Decision**: **Svelte**

**Rationale**:

- Aligns with Constitution IV (Performance Standards): Svelte produces the smallest bundles and fastest runtime performance due to compile-time optimization
- Aligns with Constitution V (Simple Architecture): Svelte's straightforward reactive syntax reduces complexity compared to React's hooks or Vue's composition API
- Bundle size target (<20MB total): Svelte contributes minimal overhead (~5-10MB vs React's ~15-20MB)
- Native-feeling UI: Svelte's performance characteristics help achieve <16ms frame times
- Simple state management: Svelte stores are sufficient for this application's scope

**Alternatives Considered**:

- React: Rejected due to larger bundle size and unnecessary complexity for this use case
- Vue: Close second choice, but Svelte's performance edge and simpler learning curve won out
- Vanilla JS: Considered but rejected - state management complexity outweighs bundle savings

---

### 2. skhd Configuration Parser

**Question**: Should we use an existing skhd parser library or build our own?

**Research Findings**:

**Available Libraries**:

- **None found**: No mature Rust crates specifically for parsing skhd configuration files
- **sxhkd parsers**: Similar tool (sxhkd) has some parsers, but syntax differs from skhd
- **Custom parser required**: skhd uses a specific syntax that requires purpose-built parsing

**skhd Syntax Structure** (from skhd documentation):

```
# Comments start with #
<modifiers> - <key> : <command>

# Examples:
cmd - return : open -a Terminal
cmd + shift - f : open ~
alt - h : yabai -m window --focus west
```

**Syntax Components**:

- **Modifiers**: cmd, alt, shift, ctrl, fn (can be combined with +)
- **Keys**: Any keyboard key (letters, numbers, symbols, function keys)
- **Command**: Shell command to execute
- **Comments**: Lines starting with # are ignored
- **Modes**: skhd supports modal configurations (:: prefix) - should support in parser
- **Passthrough**: Lines starting with . passthrough to other tools

**Decision**: **Build custom parser in Rust**

**Rationale**:

- No existing library available that handles skhd syntax correctly
- Custom parser ensures 100% compatibility with skhd syntax (Success Criterion SC-003)
- Allows fine-grained error reporting (FR-010 requirement)
- Can preserve comments and formatting (FR-012 requirement)
- Full control over validation logic (FR-006 requirement)

**Implementation Approach**:

- Use `pest` crate for PEG (Parsing Expression Grammar) parsing - battle-tested, excellent error messages
- Define skhd grammar in pest format
- Parse to AST (Abstract Syntax Tree) representing shortcuts, comments, modes
- Implement validation rules on AST before serialization

**Alternatives Considered**:

- **Regex-based parsing**: Rejected - too fragile for complex syntax, poor error messages
- **nom crate**: Considered but pest provides better error reporting out of the box
- **Manual hand-written parser**: Rejected - pest grammar is clearer and more maintainable

---

### 3. Frontend Testing Framework

**Question**: What testing framework should be used for the Svelte frontend?

**Research Findings**:

**Svelte Testing Options**:

- **Vitest + @testing-library/svelte**: Modern, fast, excellent DX
- **Jest + @testing-library/svelte**: Mature but slower than Vitest
- **Playwright**: E2E testing for full integration tests

**Decision**: **Vitest + @testing-library/svelte for unit/integration, manual testing for E2E**

**Rationale**:

- Vitest is fast and designed for Vite-based projects (Svelte's default)
- @testing-library/svelte follows testing best practices (test user behavior, not implementation)
- Aligns with Constitution III: Manual testing acceptable for UI interactions
- Vitest integrates well with Svelte and provides excellent debugging experience

**Test Coverage Strategy**:

- **Unit tests (Vitest)**: Component rendering, state management, Tauri command mocking
- **Integration tests (Vitest)**: Full user flows (add shortcut, edit, delete, save)
- **Manual tests**: Accessibility (VoiceOver), keyboard navigation, visual appearance

**Alternatives Considered**:

- Full Playwright E2E suite: Rejected as over-engineering for this scope (Constitution V)
- Jest: Rejected due to slower performance compared to Vitest
- No frontend tests: Rejected - need confidence in component behavior even if UI testing is manual

---

### 4. macOS File System Permissions

**Question**: How should the app request and handle macOS file system permissions?

**Research Findings**:

**Tauri File System Permissions**:

- Tauri v2 uses scoped permissions model
- Can request specific directory access via `tauri.conf.json`
- Must handle both sandboxed and non-sandboxed scenarios

**skhd Config Location**: `~/.config/skhd/skhdrc`

- Requires home directory access
- May need to create `~/.config/skhd/` directory if it doesn't exist
- Backup files should go to same directory (e.g., `skhdrc.backup.2025-11-01-143022`)

**Decision**: **Request home directory read/write access, implement graceful degradation for permission denial**

**Rationale**:

- FR-016: Application must request necessary file system permissions on first launch
- FR-017: Must handle read-only mode gracefully when permissions unavailable
- Tauri provides built-in permission APIs for macOS sandboxing

**Implementation Plan**:

1. Configure `tauri.conf.json` to request home directory access
2. On first launch, check if config file is readable/writable
3. If permissions denied:
   - Show clear message explaining why permissions are needed
   - Provide "Open Settings" button to system preferences
   - Offer read-only mode for viewing configuration
4. If permissions granted:
   - Proceed with full functionality
   - Create `~/.config/skhd/` directory if needed

**Alternatives Considered**:

- Ask user to manually specify config file location: Rejected - violates macOS conventions and adds complexity
- Store config in app-local directory: Rejected - skhd needs config at standard location
- No permission handling: Rejected - would fail silently and confuse users

---

### 5. Atomic File Operations

**Question**: What is the best practice for implementing atomic file writes in Rust to prevent configuration corruption?

**Research Findings**:

**Atomic Write Pattern**:

```rust
1. Write content to temporary file (.tmp extension)
2. Validate the written content by reading it back
3. If valid, atomically rename temp file to target file
4. On any error, clean up temp file and preserve original
```

**Rust Crates**:

- **tempfile**: Secure temporary file creation
- **std::fs::rename**: Atomic rename operation on same filesystem
- **std::fs**: Standard file operations

**Decision**: **Use tempfile crate + atomic rename pattern**

**Rationale**:

- Aligns with Constitution II (Configuration Safety): Atomic operations ensure never writing partial/invalid config
- `tempfile` crate handles cleanup automatically if operation fails
- `fs::rename` is atomic on macOS when source and dest are on same filesystem
- Simple pattern that's easy to test and verify

**Implementation**:

```rust
use tempfile::NamedTempFile;
use std::fs;

fn save_config_atomic(path: &Path, content: &str) -> Result<()> {
    // 1. Create temp file in same directory (ensures same filesystem)
    let temp_file = NamedTempFile::new_in(path.parent().unwrap())?;

    // 2. Write content
    temp_file.write_all(content.as_bytes())?;

    // 3. Validate by parsing (ensure no corruption)
    let parsed = parse_config(content)?;

    // 4. Atomic rename (temp file auto-deletes on drop if this fails)
    temp_file.persist(path)?;

    Ok(())
}
```

**Alternatives Considered**:

- Write-in-place: Rejected - not atomic, can corrupt on crash
- Copy-on-write filesystems: Rejected - not all macOS systems use APFS
- Lock files: Rejected - unnecessary complexity, atomic rename is sufficient

---

## Summary of Decisions

| Area               | Decision                                     | Key Justification                                                    |
| ------------------ | -------------------------------------------- | -------------------------------------------------------------------- |
| Frontend Framework | Svelte                                       | Best performance, smallest bundle, aligns with performance standards |
| Parser Library     | Custom (pest crate)                          | No existing library, full control, better error reporting            |
| Frontend Testing   | Vitest + @testing-library/svelte             | Fast, modern, good DX, adequate for scope                            |
| File Permissions   | Home directory access + graceful degradation | Required for config location, good UX on denial                      |
| Atomic Writes      | tempfile + rename                            | Proven pattern, prevents corruption, simple implementation           |

## Technology Stack (Final)

**Backend (Rust)**:

- Tauri v2 framework
- pest v2.7+ for parsing
- tempfile v3.8+ for atomic operations
- serde v1.0+ for JSON serialization
- cargo test for testing

**Frontend (Svelte)**:

- Svelte v4 (latest stable)
- Vite v5 (build tool)
- TypeScript v5 (type safety)
- Vitest v1+ (@testing-library/svelte) for testing
- CSS (no framework, custom styling for native feel)

**Development Tools**:

- cargo clippy (Rust linting)
- cargo fmt (Rust formatting)
- ESLint + Prettier (JavaScript/TypeScript)
- rust-analyzer (LSP)

---

## Next Steps

All NEEDS CLARIFICATION items from Technical Context have been resolved:

- ✅ Frontend framework choice: Svelte
- ✅ skhd config parser library: Custom parser using pest
- ✅ Frontend testing framework: Vitest + @testing-library/svelte

Ready to proceed to Phase 1: Data Model and Contracts design.
