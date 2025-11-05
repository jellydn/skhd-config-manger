# Phase 1: Data Model - Service Log Viewer and Reload

**Feature**: 006-service-log-reload
**Date**: 2025-11-02

## Entities

### LogEntry

Represents a single log line from the skhd service.

**Fields**:
- `id`: String (UUID v4) - Unique identifier for deduplication and tracking
- `timestamp`: DateTime (ISO 8601) - When the log entry was generated
- `level`: LogLevel (enum) - Severity level of the log entry
- `message`: String - The actual log message content
- `raw`: String - Original unparsed log line (fallback for display)

**Validation Rules**:
- `timestamp` must be valid ISO 8601 datetime
- `message` must not be empty
- `level` must be one of: ERROR, WARN, INFO, DEBUG
- `raw` is always preserved for fallback display

**State Transitions**: N/A (immutable once created)

**Relationships**:
- Ordered by timestamp in descending order (newest first)
- No persistence - logs are read-only from skhd service

### LogLevel (Enum)

Categorizes log entry severity for visual distinction.

**Values**:
- `ERROR` - Critical errors requiring immediate attention
- `WARN` - Warning conditions that may require investigation
- `INFO` - Normal informational messages
- `DEBUG` - Detailed debugging information

**Display Mapping**:
- ERROR → Red color, "ERROR" badge
- WARN → Yellow color, "WARN" badge
- INFO → White color, "INFO" badge
- DEBUG → Gray color, "DEBUG" badge

### ServiceStatus

Represents the current state of the skhd service.

**Fields**:
- `state`: ServiceState (enum) - Current lifecycle state
- `pid`: Option<u32> - Process ID if running, None if stopped
- `last_updated`: DateTime - When status was last checked
- `config_path`: Option<String> - Path to active configuration file
- `error_message`: Option<String> - Error details if state is Error

**Validation Rules**:
- `pid` must be present if `state` is Running
- `error_message` must be present if `state` is Error
- `last_updated` must be valid datetime

**State Transitions**:
```
Stopped → Starting → Running
Running → Stopping → Stopped
Running → Error (if service crashes)
Error → Starting (if user attempts recovery)
Starting → Error (if startup fails)
```

**Relationships**:
- One active ServiceStatus instance per application session
- Updated via periodic status checks and service control operations

### ServiceState (Enum)

Represents skhd service lifecycle states.

**Values**:
- `Stopped` - Service is not running
- `Starting` - Service start initiated, awaiting confirmation
- `Running` - Service is active and operational
- `Stopping` - Service stop initiated, awaiting confirmation
- `Reloading` - Service restart in progress (stop → start sequence)
- `Error` - Service encountered a failure (check error_message)
- `Unknown` - Cannot determine service state (skhd not installed, permission issues)

**UI Indicators**:
- Stopped → Gray dot, "Stopped" text
- Starting → Yellow pulsing dot, "Starting..." text
- Running → Green dot, "Running" text
- Stopping → Yellow pulsing dot, "Stopping..." text
- Reloading → Blue pulsing dot, "Reloading..." text
- Error → Red dot, "Error" text + error message
- Unknown → Gray question mark, "Unknown" text

### ConfigurationReference

Reference to a stored configuration file that can be loaded into skhd service.

**Fields**:
- `id`: String - Unique identifier (from feature 002 configuration storage)
- `name`: String - Human-readable configuration name
- `path`: String - Absolute path to configuration file
- `is_active`: Boolean - Whether this config is currently loaded in skhd
- `last_modified`: DateTime - File modification timestamp
- `valid`: Boolean - Whether configuration passes syntax validation

**Validation Rules**:
- `path` must point to an existing file
- `name` must not be empty
- Only one configuration can have `is_active = true` at a time
- Configuration must be parseable by skhd parser to set `valid = true`

**Relationships**:
- List of available configurations retrieved from feature 002 storage
- Active configuration determines which config is loaded during service reload

## Data Flow

### Log Viewing Flow

```
skhd service → launchd logs → log stream command → Rust log parser
→ LogEntry creation → Tauri event emission → Frontend event listener
→ Svelte reactive store update → LogViewer component render
```

### Service Reload Flow

```
User selects configuration → ConfigurationReference retrieved
→ Validate configuration syntax → Acquire reload lock (prevent concurrent)
→ Stop skhd service → ServiceStatus = Stopping
→ Write configuration to skhd location → Start skhd service
→ ServiceStatus = Starting → Check service status (2s grace period)
→ If started: ServiceStatus = Running, emit success event
→ If failed: ServiceStatus = Error, rollback to previous config, emit error event
→ Release reload lock
```

### Service Status Monitoring Flow

```
Frontend requests status → Tauri command invoked
→ Execute `launchctl list | grep skhd` → Parse output
→ Update ServiceStatus (state, pid) → Return to frontend
→ Frontend updates UI indicators
(Repeat periodically every 5 seconds while log viewer is open)
```

## Storage

**No persistent storage required** - all data is ephemeral and read-only from system sources:
- Logs are streamed from macOS unified logging system
- Service status is queried from launchctl
- Configuration references are managed by feature 002 storage

**In-memory storage**:
- Recent log entries (max 10,000) stored in Svelte reactive store
- Current ServiceStatus cached for UI display
- Active configuration reference cached for reload operations

## Performance Characteristics

**Log Entry Creation**: O(1) per entry, regex parsing overhead ~10μs/entry
**Log Display**: O(visible entries) due to virtual scrolling, typically <100 entries
**Service Status Check**: O(1), launchctl command overhead ~50-100ms
**Configuration Validation**: O(n) where n = configuration file size, typically <10ms
**Reload Operation**: O(1) operations, total time ~2-5s (waiting for service startup)

## Error States

### Log Parsing Errors
- **Cause**: Log format doesn't match expected pattern
- **Handling**: Store as LogEntry with level=WARN, display raw line
- **User Impact**: Log shown but without color coding

### Service Control Errors
- **Permission Denied**: Clear message, link to setup instructions
- **Service Not Found**: Guide user to install skhd
- **Reload Failed**: Automatic rollback, display error details
- **Timeout**: Mark as Error state, suggest manual intervention

### Configuration Errors
- **Syntax Invalid**: Prevent reload, show validation errors
- **File Not Found**: Error message, suggest reimporting configuration
- **Write Permission Denied**: Error message, suggest permission fix
