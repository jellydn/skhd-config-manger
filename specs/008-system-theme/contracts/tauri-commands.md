# Tauri Commands - System Theme Support

**Feature**: 008-system-theme
**Protocol**: Tauri IPC (Invoke/Emit pattern)
**Date**: 2025-11-02

## Command Definitions

### Theme Detection Commands

#### `get_system_theme`

Detects the current macOS system theme preference (light or dark mode).

**Request**:
```typescript
invoke('get_system_theme'): Promise<ThemeMode>
```

**Response**:
```typescript
type ThemeMode = 'light' | 'dark';
```

**Behavior**:
- Reads `AppleInterfaceStyle` from macOS NSUserDefaults
- Returns `"dark"` if value is `"Dark"`, `"light"` otherwise
- Falls back to `"dark"` if detection fails (maintains current app default)
- Completes within 100ms target (non-blocking)

**Errors**:
- No explicit errors thrown - always returns valid ThemeMode
- On detection failure, defaults to `"dark"` mode
- Errors logged to console for debugging but not exposed to frontend

**Example**:
```typescript
try {
  const theme = await invoke<ThemeMode>('get_system_theme');
  // theme is 'light' or 'dark'
  applyTheme(theme);
} catch (error) {
  // Should not occur, but handle gracefully
  console.error('Theme detection failed:', error);
  applyTheme('dark'); // Fallback
}
```

---

#### `start_theme_monitor`

Starts monitoring macOS system theme changes and emits events when theme changes.

**Request**:
```typescript
invoke('start_theme_monitor'): Promise<void>
```

**Response**:
- Success: `void` (theme change events begin emitting via `theme-changed` events)
- Error: `string` (error message if monitoring cannot be started)

**Behavior**:
- Subscribes to `AppleInterfaceThemeChangedNotification` via NSDistributedNotificationCenter
- Emits `theme-changed` Tauri event when system theme changes
- Continues monitoring until `stop_theme_monitor` is called or app closes
- Only one monitor allowed at a time (subsequent calls ignored if already monitoring)
- Falls back to polling every 1-2 seconds if notification API unavailable

**Errors**:
- `"Monitoring already active"` - Theme monitor already running
- `"API unavailable"` - System theme monitoring APIs not available (fallback to polling)

**Example**:
```typescript
try {
  await invoke('start_theme_monitor');
  // Theme changes now trigger 'theme-changed' events
} catch (error) {
  console.error('Failed to start theme monitor:', error);
  // Fallback: Poll theme every 2 seconds manually
}
```

---

#### `stop_theme_monitor`

Stops monitoring macOS system theme changes.

**Request**:
```typescript
invoke('stop_theme_monitor'): Promise<void>
```

**Response**:
- Success: `void`
- Error: N/A (idempotent - safe to call even if no monitor active)

**Behavior**:
- Unsubscribes from `AppleInterfaceThemeChangedNotification`
- Cleans up monitoring resources
- Stops emitting `theme-changed` events

**Example**:
```typescript
await invoke('stop_theme_monitor');
```

---

## Event Definitions

### `theme-changed`

Emitted when macOS system theme changes during application runtime.

**Event Name**: `theme-changed`

**Payload**:
```typescript
interface ThemeChangedEvent {
  theme: ThemeMode; // 'light' | 'dark'
  timestamp: string; // ISO 8601 datetime
}
```

**When Emitted**:
- When `AppleInterfaceThemeChangedNotification` is received from macOS
- Only if `start_theme_monitor` has been called
- Immediately after system theme change is detected

**Frontend Subscription**:
```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<ThemeChangedEvent>('theme-changed', (event) => {
  const { theme } = event.payload;
  applyTheme(theme);
});

// Later, to stop listening:
unlisten();
```

**Example Flow**:
1. User changes macOS system theme from Light to Dark in System Settings
2. macOS posts `AppleInterfaceThemeChangedNotification`
3. Backend receives notification and emits `theme-changed` event with `theme: 'dark'`
4. Frontend receives event and calls `applyTheme('dark')`
5. CSS variables update, UI transitions to dark theme within 2 seconds

---

## Rust Implementation Signatures

### Commands

```rust
#[tauri::command]
pub async fn get_system_theme() -> Result<String, String> {
    // Returns "light" or "dark"
    // On error, returns Ok("dark") to maintain current default
}

#[tauri::command]
pub async fn start_theme_monitor(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Subscribes to NSDistributedNotificationCenter
    // Emits 'theme-changed' events on theme change
}

#[tauri::command]
pub async fn stop_theme_monitor() -> Result<(), String> {
    // Unsubscribes from notifications
    // Cleans up resources
}
```

### Event Emission

```rust
use tauri::Manager;

app_handle.emit("theme-changed", ThemeChangedPayload {
    theme: "dark".to_string(),
    timestamp: chrono::Utc::now().to_rfc3339(),
})?;
```

---

## Error Handling Strategy

### Detection Errors

- **NSUserDefaults access failure**: Default to `"dark"`, log error
- **Invalid theme value**: Default to `"dark"`, log warning
- **API unavailable**: Default to `"dark"`, log info

### Monitoring Errors

- **Notification subscription failure**: Fallback to polling, log warning
- **Event emission failure**: Log error, continue monitoring
- **Multiple monitor attempts**: Ignore subsequent calls, log info

### Frontend Errors

- **Command invocation failure**: Use CSS media query fallback
- **Event subscription failure**: Fallback to polling theme every 2 seconds
- **Theme application failure**: Log error, maintain current theme

---

## Performance Requirements

- **`get_system_theme`**: Must complete within 100ms (non-blocking)
- **`start_theme_monitor`**: Must complete within 200ms (non-blocking setup)
- **`theme-changed` event**: Must be emitted within 500ms of system theme change
- **Frontend theme application**: Must complete within 2 seconds (spec requirement)

---

## Testing Contracts

### Unit Tests

- `get_system_theme` returns correct theme for light mode
- `get_system_theme` returns correct theme for dark mode
- `get_system_theme` defaults to dark on detection failure
- `start_theme_monitor` prevents duplicate monitors
- `stop_theme_monitor` is idempotent

### Integration Tests

- `get_system_theme` detects actual macOS system theme
- `theme-changed` event emitted when system theme changes
- Frontend receives and processes `theme-changed` events
- Theme monitoring survives app lifecycle events

### Manual Tests

- Change macOS theme while app running, verify UI updates
- Launch app in light mode, verify light theme applied
- Launch app in dark mode, verify dark theme applied
- Rapidly switch themes, verify all changes are captured
