# Developer Quickstart - Service Log Viewer and Reload

**Feature**: 006-service-log-reload
**Prerequisites**: Rust 1.75+, Node.js 18+, skhd installed on macOS
**Estimated Setup Time**: 10 minutes

## Quick Start

### 1. Understand the Feature

This feature adds:
- **Real-time log viewer** for skhd service logs
- **Service control panel** to reload skhd with selected configurations
- **Configuration selector** to choose which config to activate

**Architecture**:
- Rust backend: launchctl integration, log parsing, service control
- Svelte frontend: reactive log display, service status UI
- Tauri IPC: commands (invoke) and events (emit) for communication

### 2. Development Environment Setup

**Verify skhd is installed**:
```bash
launchctl list | grep skhd
# Should show: PID  status  com.koekeishiya.skhd
```

**Install dependencies** (if not already):
```bash
cd /Users/huynhdung/src/tries/2025-11-01-skhd-gui
bun install
```

**Run development server**:
```bash
bun run tauri dev
```

### 3. Key File Locations

**Backend (Rust)**:
```
src-tauri/src/
├── commands/
│   ├── service.rs        # Service control commands (NEW)
│   └── logs.rs           # Log access commands (NEW)
├── services/
│   ├── service_manager.rs  # launchctl integration (NEW)
│   └── log_tailer.rs       # Real-time log streaming (NEW)
└── models/
    ├── log_entry.rs      # LogEntry data structure (NEW)
    └── service_status.rs # ServiceStatus data structure (NEW)
```

**Frontend (Svelte)**:
```
src/
├── components/
│   ├── LogViewer.svelte       # Log display component (NEW)
│   ├── ServiceControl.svelte  # Reload/status controls (NEW)
│   └── ConfigSelector.svelte  # Configuration picker (NEW)
├── services/
│   └── service.ts             # Tauri command wrappers (NEW)
└── routes/logs/
    └── +page.svelte           # Log viewer page (NEW)
```

### 4. Implementation Workflow

**Phase 1: Backend - Service Manager** ⏱️ ~2 hours
1. Create `src-tauri/src/services/service_manager.rs`
2. Implement `ServiceManager` struct with methods:
   - `get_status()` - Query launchctl for skhd status
   - `reload_service(config_path)` - Stop, switch config, start
   - `stop_service()` - Stop skhd via launchctl
   - `start_service()` - Start skhd via launchctl
3. Write unit tests for service control logic

**Phase 2: Backend - Log Tailer** ⏱️ ~2 hours
1. Create `src-tauri/src/services/log_tailer.rs`
2. Implement `LogTailer` struct with methods:
   - `start_stream(app_handle)` - Spawn log stream process, emit events
   - `stop_stream()` - Terminate log stream process
   - `parse_log_line(line)` - Extract timestamp, level, message
3. Write tests for log parsing edge cases

**Phase 3: Backend - Tauri Commands** ⏱️ ~1 hour
1. Create `src-tauri/src/commands/service.rs` and `logs.rs`
2. Implement command handlers:
   - `start_log_stream()`, `stop_log_stream()`
   - `get_service_status()`
   - `reload_service(config_id)`
   - `get_recent_logs(limit)`
   - `get_available_configs()`
3. Register commands in `src-tauri/src/lib.rs`

**Phase 4: Frontend - Service Client** ⏱️ ~1 hour
1. Create `src/services/service.ts`
2. Wrap Tauri commands in typed TypeScript functions
3. Export reactive stores for service status and logs

**Phase 5: Frontend - UI Components** ⏱️ ~3 hours
1. Create `LogViewer.svelte` - display log entries with color coding
2. Create `ServiceControl.svelte` - reload button, status indicator
3. Create `ConfigSelector.svelte` - dropdown/list for config selection
4. Implement virtual scrolling for log performance
5. Wire up event listeners for real-time updates

**Phase 6: Integration & Testing** ⏱️ ~2 hours
1. Create log viewer page at `src/routes/logs/+page.svelte`
2. Manual testing: start app, view logs, reload service, test error cases
3. Write component tests for LogViewer and ServiceControl
4. Write integration tests for service control flow

**Total Estimated Time**: ~11 hours

### 5. Testing Strategy

**Unit Tests (Rust)**:
```bash
cd src-tauri
cargo test
```

Test coverage:
- `service_manager_test.rs` - Service control logic
- `log_parser_test.rs` - Log line parsing

**Component Tests (Frontend)**:
```bash
bun run test
```

Test coverage:
- `LogViewer.test.ts` - Log rendering, color coding
- `ServiceControl.test.ts` - Button states, error display

**Integration Tests**:
```bash
cargo test --test service_control_test
```

End-to-end scenarios:
- Reload service with valid configuration
- Reload failure with rollback
- Log streaming start/stop

**Manual Testing Checklist**:
- [ ] Log viewer displays existing logs on open
- [ ] New logs appear in real-time (<1s latency)
- [ ] Log levels have correct colors (red, yellow, white, gray)
- [ ] Service status indicator updates correctly
- [ ] Reload button triggers service restart
- [ ] Error messages are clear and actionable
- [ ] Configuration selector shows available configs
- [ ] Active configuration is highlighted
- [ ] Concurrent reload attempts are blocked
- [ ] Large log files (>10MB) don't freeze UI

### 6. Common Development Tasks

**Add new log level**:
1. Update `LogLevel` enum in `src-tauri/src/models/log_entry.rs`
2. Update regex in `log_tailer.rs` parsing logic
3. Add color mapping in `LogViewer.svelte`

**Change service reload timeout**:
1. Modify timeout in `service_manager.rs::reload_service()`
2. Update success criteria in `spec.md` if necessary

**Add new Tauri command**:
1. Define function in `src-tauri/src/commands/*.rs`
2. Add `#[tauri::command]` attribute
3. Register in `src-tauri/src/lib.rs` builder
4. Create TypeScript wrapper in `src/services/service.ts`

**Debug log parsing issues**:
1. Enable debug logging: `RUST_LOG=debug bun run tauri dev`
2. Check raw log lines in `LogEntry.raw` field
3. Test regex against sample logs in unit tests

### 7. Troubleshooting

**"Permission denied" when accessing logs**:
- Grant Full Disk Access to app in System Preferences → Security & Privacy
- Or run: `sudo log stream --predicate 'process == "skhd"'` once to trigger permission prompt

**skhd service won't reload**:
- Check skhd is installed: `which skhd`
- Verify plist exists: `ls ~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
- Try manual reload: `launchctl kickstart -k gui/$(id -u)/com.koekeishiya.skhd`

**Logs not streaming in real-time**:
- Verify skhd is running: `launchctl list | grep skhd`
- Check log stream command manually: `log stream --predicate 'process == "skhd"'`
- Ensure log stream process is spawned: add debug logging to `log_tailer.rs`

**UI freezes with large logs**:
- Verify virtual scrolling is enabled in `LogViewer.svelte`
- Check log entry limit (should be max 10,000 in memory)
- Profile with browser dev tools to identify bottleneck

### 8. Code Examples

**Rust: Service Status Check**
```rust
use std::process::Command;

pub fn get_service_status() -> Result<ServiceStatus, String> {
    let output = Command::new("launchctl")
        .args(&["list"])
        .output()
        .map_err(|e| format!("Failed to execute launchctl: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(line) = stdout.lines().find(|l| l.contains("skhd")) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let pid = parts[0].parse::<u32>().ok();

        Ok(ServiceStatus {
            state: if pid.is_some() { ServiceState::Running } else { ServiceState::Stopped },
            pid,
            last_updated: Utc::now(),
            config_path: None,
            error_message: None,
        })
    } else {
        Ok(ServiceStatus {
            state: ServiceState::Unknown,
            pid: None,
            last_updated: Utc::now(),
            config_path: None,
            error_message: Some("skhd not found in launchctl list".to_string()),
        })
    }
}
```

**TypeScript: Event Listener**
```typescript
import { listen } from '@tauri-apps/api/event';

interface LogEntry {
  id: string;
  timestamp: string;
  level: 'ERROR' | 'WARN' | 'INFO' | 'DEBUG';
  message: string;
  raw: string;
}

export async function subscribeToLogs(callback: (log: LogEntry) => void) {
  const unlisten = await listen<LogEntry>('log-entry', (event) => {
    callback(event.payload);
  });

  return unlisten; // Call this to cleanup subscription
}
```

**Svelte: Log Viewer Component**
```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { subscribeToLogs } from '$lib/services/service';

  let logs: LogEntry[] = [];
  let unlisten: (() => void) | null = null;

  onMount(async () => {
    // Load recent logs
    logs = await invoke<LogEntry[]>('get_recent_logs', { limit: 100 });

    // Start streaming
    await invoke('start_log_stream');
    unlisten = await subscribeToLogs((log) => {
      logs = [log, ...logs].slice(0, 10000); // Keep max 10k entries
    });
  });

  onDestroy(async () => {
    await invoke('stop_log_stream');
    unlisten?.();
  });
</script>

<div class="log-viewer">
  {#each logs as log}
    <div class="log-entry log-{log.level.toLowerCase()}">
      <span class="timestamp">{log.timestamp}</span>
      <span class="level">{log.level}</span>
      <span class="message">{log.message}</span>
    </div>
  {/each}
</div>

<style>
  .log-viewer {
    font-family: 'SF Mono', 'Monaco', monospace;
    font-size: 12px;
    background: #1e1e1e;
    color: #d4d4d4;
    padding: 8px;
    overflow-y: auto;
    height: 600px;
  }

  .log-entry {
    padding: 2px 0;
  }

  .log-error { color: #f44747; }
  .log-warn { color: #dda712; }
  .log-info { color: #d4d4d4; }
  .log-debug { color: #888888; }

  .timestamp { margin-right: 8px; color: #666; }
  .level { margin-right: 8px; font-weight: bold; }
</style>
```

### 9. Debugging Tips

**Enable Rust debug logging**:
```bash
RUST_LOG=debug bun run tauri dev
```

**Inspect Tauri IPC messages**:
Open browser dev tools in Tauri window, check console for Tauri command calls.

**Test launchctl commands manually**:
```bash
launchctl list | grep skhd
launchctl kickstart -k gui/$(id -u)/com.koekeishiya.skhd
log stream --predicate 'process == "skhd"' --last 1m
```

**Profile frontend performance**:
Use browser dev tools Performance tab, record while logs are streaming, look for:
- Long tasks (>50ms)
- Excessive re-renders
- Memory leaks (growing heap size)

### 10. Ready to Implement

You now have:
- ✅ Technical context and architecture
- ✅ Data models and API contracts
- ✅ File locations and structure
- ✅ Implementation workflow and timeline
- ✅ Testing strategy and examples
- ✅ Code examples and debugging guides

**Next step**: Run `/speckit.tasks` to generate detailed implementation tasks.
