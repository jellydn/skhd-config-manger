# Tauri Commands - Service Log Viewer and Reload

**Feature**: 006-service-log-reload
**Protocol**: Tauri IPC (Invoke/Emit pattern)
**Date**: 2025-11-02

## Command Definitions

### Service Control Commands

#### `start_log_stream`

Starts streaming skhd service logs to the frontend via Tauri events.

**Request**:
```typescript
invoke('start_log_stream'): Promise<void>
```

**Response**:
- Success: `void` (logs begin streaming via `log-entry` events)
- Error: `string` (error message if log stream cannot be started)

**Behavior**:
- Spawns `log stream --predicate 'process == "skhd"'` as background process
- Parses each log line and emits `log-entry` event
- Continues streaming until `stop_log_stream` is called or app closes
- Only one stream allowed at a time (subsequent calls ignored if already streaming)

**Errors**:
- `"Permission denied"` - Insufficient permissions for log access
- `"skhd not running"` - skhd service not active, no logs to stream
- `"Stream already active"` - Log stream already running

**Example**:
```typescript
try {
  await invoke('start_log_stream');
  // Logs now streaming via 'log-entry' events
} catch (error) {
  console.error('Failed to start log stream:', error);
}
```

---

#### `stop_log_stream`

Stops the active log stream.

**Request**:
```typescript
invoke('stop_log_stream'): Promise<void>
```

**Response**:
- Success: `void`
- Error: N/A (idempotent - safe to call even if no stream active)

**Behavior**:
- Terminates background `log stream` process
- Cleans up resources
- Stops emitting `log-entry` events

**Example**:
```typescript
await invoke('stop_log_stream');
```

---

#### `get_service_status`

Retrieves current skhd service status.

**Request**:
```typescript
invoke('get_service_status'): Promise<ServiceStatus>
```

**Response**:
```typescript
interface ServiceStatus {
  state: 'Stopped' | 'Starting' | 'Running' | 'Stopping' | 'Reloading' | 'Error' | 'Unknown';
  pid: number | null;
  last_updated: string; // ISO 8601 datetime
  config_path: string | null;
  error_message: string | null;
}
```

**Behavior**:
- Executes `launchctl list | grep skhd`
- Parses output to determine service state and PID
- Returns current status snapshot

**Errors**:
- `"Permission denied"` - Cannot access launchctl
- `"Command failed"` - launchctl execution error

**Example**:
```typescript
const status = await invoke<ServiceStatus>('get_service_status');
console.log(`skhd is ${status.state}, PID: ${status.pid}`);
```

---

#### `reload_service`

Reloads skhd service with the specified configuration.

**Request**:
```typescript
interface ReloadRequest {
  config_id: string; // Configuration ID from feature 002 storage
}

invoke('reload_service', { config_id: string }): Promise<void>
```

**Response**:
- Success: `void`
- Error: `string` (detailed error message)

**Behavior**:
1. Acquire reload lock (prevent concurrent reloads)
2. Retrieve configuration by `config_id`
3. Validate configuration syntax using skhd parser
4. Backup current skhd configuration path
5. Stop skhd service (`launchctl kickstart -k`)
6. Write new configuration to skhd location
7. Start skhd service
8. Wait 2 seconds for startup
9. Check service status
10. If running: emit `service-reload-success` event, release lock
11. If failed: restore backup config, restart, emit `service-reload-error` event, release lock

**Errors**:
- `"Reload already in progress"` - Concurrent reload attempt blocked
- `"Configuration not found"` - Invalid config_id
- `"Validation failed: {details}"` - Configuration syntax errors
- `"Permission denied"` - Cannot control service or write config
- `"Service failed to start"` - Service didn't start after reload (rollback performed)

**Example**:
```typescript
try {
  await invoke('reload_service', { config_id: 'abc-123' });
  // Service reloaded successfully
} catch (error) {
  console.error('Reload failed:', error);
}
```

---

### Log Access Commands

#### `get_recent_logs`

Retrieves recent log entries (fallback for historical logs before stream starts).

**Request**:
```typescript
interface LogsRequest {
  limit?: number; // Default: 100, max: 1000
}

invoke('get_recent_logs', { limit?: number }): Promise<LogEntry[]>
```

**Response**:
```typescript
interface LogEntry {
  id: string; // UUID v4
  timestamp: string; // ISO 8601 datetime
  level: 'ERROR' | 'WARN' | 'INFO' | 'DEBUG';
  message: string;
  raw: string;
}
```

**Behavior**:
- Reads last N lines from skhd logs (via `log show --predicate 'process == "skhd"' --last 1h`)
- Parses log entries
- Returns array ordered by timestamp (newest first)

**Errors**:
- `"Permission denied"` - Cannot access logs
- `"No logs found"` - skhd hasn't generated logs or log retention expired

**Example**:
```typescript
const logs = await invoke<LogEntry[]>('get_recent_logs', { limit: 50 });
console.log(`Retrieved ${logs.length} log entries`);
```

---

### Configuration Commands

#### `get_available_configs`

Retrieves list of available configurations for selection.

**Request**:
```typescript
invoke('get_available_configs'): Promise<ConfigurationReference[]>
```

**Response**:
```typescript
interface ConfigurationReference {
  id: string;
  name: string;
  path: string;
  is_active: boolean;
  last_modified: string; // ISO 8601 datetime
  valid: boolean;
}
```

**Behavior**:
- Queries configuration storage (feature 002)
- Validates each configuration syntax
- Marks active configuration (currently loaded in skhd)
- Returns sorted by last_modified (newest first)

**Errors**:
- `"Storage not accessible"` - Cannot read configuration storage
- `"No configurations found"` - No saved configurations

**Example**:
```typescript
const configs = await invoke<ConfigurationReference[]>('get_available_configs');
const activeConfig = configs.find(c => c.is_active);
```

## Event Definitions

### Frontend Subscriptions

#### `log-entry`

Emitted for each new log line from skhd service.

**Payload**:
```typescript
interface LogEntry {
  id: string; // UUID v4
  timestamp: string; // ISO 8601 datetime
  level: 'ERROR' | 'WARN' | 'INFO' | 'DEBUG';
  message: string;
  raw: string;
}
```

**Frequency**: Real-time (as logs are generated by skhd)

**Example Subscription**:
```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<LogEntry>('log-entry', (event) => {
  console.log('New log:', event.payload.message);
  // Update UI with new log entry
});

// Cleanup
unlisten();
```

---

#### `service-status-changed`

Emitted when skhd service status changes.

**Payload**:
```typescript
interface ServiceStatus {
  state: 'Stopped' | 'Starting' | 'Running' | 'Stopping' | 'Reloading' | 'Error' | 'Unknown';
  pid: number | null;
  last_updated: string;
  config_path: string | null;
  error_message: string | null;
}
```

**Trigger Conditions**:
- Service state changes (Stopped → Running, Running → Error, etc.)
- Reload operations
- Service crashes or restarts

**Example Subscription**:
```typescript
const unlisten = await listen<ServiceStatus>('service-status-changed', (event) => {
  console.log('Service status:', event.payload.state);
  // Update UI status indicator
});
```

---

#### `service-reload-success`

Emitted when service reload completes successfully.

**Payload**:
```typescript
interface ReloadSuccess {
  config_id: string;
  config_name: string;
  reload_duration_ms: number;
}
```

**Example Subscription**:
```typescript
const unlisten = await listen<ReloadSuccess>('service-reload-success', (event) => {
  showNotification(`Service reloaded with ${event.payload.config_name}`);
});
```

---

#### `service-reload-error`

Emitted when service reload fails.

**Payload**:
```typescript
interface ReloadError {
  config_id: string;
  error_message: string;
  rollback_performed: boolean;
}
```

**Example Subscription**:
```typescript
const unlisten = await listen<ReloadError>('service-reload-error', (event) => {
  showError(`Reload failed: ${event.payload.error_message}`);
  if (event.payload.rollback_performed) {
    showInfo('Service restored to previous configuration');
  }
});
```

## Error Handling Patterns

### Command Errors

All commands follow consistent error handling:
- Return `Result<T, String>` in Rust
- Frontend receives rejected Promise with error string
- Error messages are user-friendly and actionable

### Event Errors

Events are fire-and-forget, no error responses:
- If frontend unsubscribes, events are silently dropped
- No backpressure or queueing (real-time streaming)

## Security Considerations

**Input Validation**:
- All `config_id` parameters validated against stored configurations
- No path traversal allowed in configuration paths
- launchctl commands use fixed arguments, no shell injection

**Permission Requirements**:
- Log access: User must have permission to read unified logs
- Service control: User must have permission to control user-level launchd services
- File access: User must have read/write access to skhd configuration location

**Rate Limiting**:
- Reload operations: Maximum 1 concurrent (enforced by mutex)
- Log stream: Maximum 1 active stream per app instance
- Status checks: No rate limiting (low cost operation)
