# Phase 0: Research - Service Log Viewer and Reload

**Feature**: 006-service-log-reload
**Date**: 2025-11-02
**Status**: Complete

## Research Areas

### 1. skhd Service Management on macOS

**Question**: How is skhd installed and managed as a service on macOS? What's the correct way to reload/restart it?

**Decision**: Use macOS `launchctl` for service control

**Rationale**:
- skhd is typically installed as a launchd service via Homebrew or manual plist installation
- Standard service location: `~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
- launchctl is the official macOS tool for service lifecycle management
- Commands:
  - Status check: `launchctl list | grep skhd`
  - Stop: `launchctl unload ~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
  - Start: `launchctl load ~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
  - Restart: `launchctl kickstart -k gui/$(id -u)/com.koekeishiya.skhd`

**Alternatives Considered**:
- Direct process kill/restart: Too fragile, doesn't respect macOS service lifecycle
- Homebrew services: Adds dependency on Homebrew, unnecessary wrapper
- AppleScript/osascript: Over-complicated for direct launchctl access

**Implementation Notes**:
- Use `tokio::process::Command` for async launchctl execution
- Parse launchctl output to determine service status (running/stopped/error)
- Handle case where skhd is not installed or plist is missing
- Provide clear error messages for permission issues

### 2. skhd Log Location and Format

**Question**: Where does skhd write logs and what format does it use?

**Decision**: Read from skhd's stdout/stderr via macOS unified logging system

**Rationale**:
- skhd outputs logs to stdout/stderr, captured by launchd
- macOS unified logging: `log stream --predicate 'process == "skhd"'`
- Alternative: Read from launchd log files in `~/Library/Logs/`
- Log format: Plain text with timestamps (ISO 8601), log level prefix, message
- Example: `2025-11-02 10:15:30 [INFO] skhd: configuration loaded successfully`

**Alternatives Considered**:
- Custom log file: skhd doesn't write to custom location by default
- System.log parsing: Deprecated in modern macOS, unified logging is standard
- Console.app integration: Over-complicated, direct log access is simpler

**Implementation Notes**:
- Use `log stream` command for real-time log tailing
- Parse log lines for timestamp, level, message components
- Support fallback to `~/Library/Logs/skhd/` if custom logging is configured
- Handle log rotation and large file scenarios (>10MB)
- Implement log level detection: ERROR, WARN, INFO, DEBUG

### 3. Real-Time Log Tailing in Rust

**Question**: What's the best approach for real-time log monitoring in a Tauri app?

**Decision**: Use tokio async process + Tauri event system for streaming

**Rationale**:
- Spawn `log stream` as async process using `tokio::process::Command`
- Stream stdout line-by-line, avoiding memory issues with large logs
- Emit Tauri events to frontend for each new log entry
- Frontend subscribes to events and updates UI reactively
- Non-blocking: log tailing runs in background, doesn't block UI

**Alternatives Considered**:
- `notify` crate file watching: Doesn't work for streaming command output
- Periodic polling: Higher latency, inefficient
- WebSocket connection: Over-engineered for local desktop app

**Implementation Notes**:
- Tauri event: `log-entry` with payload `{timestamp, level, message}`
- Use `BufReader` with `lines()` for efficient line-by-line reading
- Handle process termination gracefully (user closes log viewer)
- Debounce rapid log bursts to prevent UI flooding

### 4. Configuration Selection and Switching

**Question**: How should users select configurations and ensure safe switching?

**Decision**: Reuse existing configuration management from feature 002

**Rationale**:
- Feature 002 already implements import/export and configuration storage
- Leverage existing atomic write patterns and validation
- Add UI dropdown/list to select from available configurations
- Store "active configuration" reference in app state
- Reload operation: validate selected config → write to skhd location → reload service

**Alternatives Considered**:
- Separate configuration storage: Duplicates feature 002, violates DRY
- Direct file selection each time: Poor UX, bypasses validation

**Implementation Notes**:
- Query available configurations from existing storage
- Display configuration list with identifying metadata (name, last modified)
- Highlight currently active configuration
- Prevent reload if selected config has validation errors

### 5. Service Reload Safety and Error Handling

**Question**: How to handle service reload failures and protect against broken states?

**Decision**: Pre-validate, attempt reload, verify startup, rollback on failure

**Rationale**:
- Configuration Safety principle is non-negotiable
- Multi-step reload process:
  1. Validate selected configuration syntax (using existing parser)
  2. Backup current configuration path
  3. Stop skhd service
  4. Write new configuration to skhd location
  5. Start skhd service
  6. Check service status after 2-second grace period
  7. If failed: restore backup, restart with original config
- Provide detailed error messages at each step

**Alternatives Considered**:
- Fire-and-forget reload: Dangerous, could leave service broken
- Manual rollback: Poor UX, violates safety principle

**Implementation Notes**:
- Mutex/lock to prevent concurrent reloads (FR-011)
- Timeout handling: if service doesn't start within 5s, mark as failed
- Error categorization: validation errors, permission errors, service startup failures
- Status feedback: "Reloading...", "Success", "Failed - rolled back"

### 6. UI Design Patterns for Log Viewer

**Question**: How to design a native-feeling macOS log viewer?

**Decision**: Monospaced text, syntax highlighting, auto-scroll, native colors

**Rationale**:
- Follow macOS Terminal.app and Console.app conventions
- Monospaced font: SF Mono or system monospace
- Color coding: Red (error), yellow (warning), white (info), gray (debug)
- Auto-scroll to bottom for new entries (with pause on user scroll)
- Virtual scrolling for performance with large log histories
- Dark mode support following system preferences

**Alternatives Considered**:
- Rich text editor: Over-complicated, performance issues
- Table view: Doesn't match macOS log viewer conventions
- HTML rendering: Security concerns, performance overhead

**Implementation Notes**:
- Svelte component with reactive log entry array
- CSS: monospace font, color classes by log level
- Virtual scrolling: render only visible entries (use svelte-virtual-list or similar)
- Auto-scroll: `scrollIntoView()` on new entry, disable on user scroll
- Export: consider future "copy logs" button

## Technology Stack Summary

**Backend (Rust)**:
- `tokio::process::Command` - launchctl and log stream execution
- `tokio::sync::Mutex` - prevent concurrent reloads
- Existing skhd parser - configuration validation
- Tauri event system - stream logs to frontend

**Frontend (Svelte)**:
- Tauri event listener - receive log updates
- Reactive stores - manage log state and service status
- Virtual scrolling - performance for large logs
- Native CSS - macOS-style log viewer

**macOS Integration**:
- `launchctl` - service lifecycle management
- `log stream` - real-time log access
- Unified logging system - skhd log capture

## Performance Considerations

**Log Volume**: Assume <1MB/day typical, support up to 10MB
**Update Frequency**: Process log entries in batches (100ms debounce)
**Memory Usage**: Keep max 10,000 log entries in memory, trim older entries
**UI Responsiveness**: Virtual scrolling prevents DOM bloat
**Async Operations**: All service control and log access is non-blocking

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Service fails to start after reload | Automatic rollback to previous configuration |
| Concurrent reload attempts | Mutex lock prevents simultaneous operations |
| Large log files crash UI | Virtual scrolling and entry limit (10k entries) |
| Permission denied for launchctl | Clear error message, link to setup instructions |
| skhd not installed | Graceful handling, guide user to installation |
| Log format changes | Flexible regex parsing, fallback to raw display |

## Open Questions (Resolved)

All technical unknowns have been researched and resolved. Ready to proceed to Phase 1 (Design & Contracts).
