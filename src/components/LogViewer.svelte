<script lang="ts">
  /**
   * LogViewer component - Real-time log display with virtual scrolling
   *
   * Features:
   * - Virtual scrolling for performance with large datasets (1000+ logs)
   * - Color-coded log levels (ERROR=red, WARN=yellow, INFO=blue, DEBUG=gray)
   * - Auto-scroll to bottom on new logs
   * - Timestamp formatting
   * - Empty state handling
   * - Sort order toggle (ascending/descending)
   * - Level filtering
   */

  import { onMount, onDestroy } from 'svelte';
  import type { LogEntry } from '../types';
  import {
    formatLogTimestamp,
    getLogLevelClass,
    onLogEntry,
    startLogStream,
    stopLogStream,
  } from '../services/logService';
  import type { UnlistenFn } from '@tauri-apps/api/event';

  // Props
  export let maxLogs = 1000; // Maximum number of logs to keep in memory
  export let autoScroll = true; // Auto-scroll to bottom on new logs
  export let showControls = true; // Show internal controls header (false when controls are external)

  // State
  let logs: LogEntry[] = [];
  let isStreaming = false;
  let error: string | null = null;
  let unlisten: UnlistenFn | null = null;
  let scrollContainer: HTMLDivElement;
  let sortDescending = true; // Default: newest first

  // Expose state and methods for external control
  export function getIsStreaming() { return isStreaming; }
  export function getLogsCount() { return logs.length; }
  export function getSortDescending() { return sortDescending; }
  export { handleStart as startStream };
  export { handleStop as stopStream };
  export { toggleSortOrder };
  export { clearLogs };

  // Virtual scrolling state
  const itemHeight = 24; // Height of each log entry in pixels
  const viewportHeight = 600; // Height of visible area
  let scrollTop = 0;
  let visibleStart = 0;
  let visibleEnd = 0;

  // Computed values
  $: visibleLogs = logs.slice(visibleStart, visibleEnd);
  $: totalHeight = logs.length * itemHeight;
  $: offsetY = visibleStart * itemHeight;

  // Calculate visible range based on scroll position
  function updateVisibleRange() {
    const start = Math.floor(scrollTop / itemHeight);
    const viewportItemCount = Math.ceil(viewportHeight / itemHeight);
    visibleStart = Math.max(0, start - 10); // Buffer above
    visibleEnd = Math.min(logs.length, start + viewportItemCount + 10); // Buffer below
  }

  // Handle scroll events
  function handleScroll(event: Event) {
    const target = event.target as HTMLDivElement;
    scrollTop = target.scrollTop;
    updateVisibleRange();

    // Check if user scrolled away from bottom (disable auto-scroll)
    const isAtBottom = scrollTop + viewportHeight >= totalHeight - 10;
    if (!isAtBottom && autoScroll) {
      autoScroll = false;
    }
  }

  // Scroll to bottom
  function scrollToBottom() {
    if (scrollContainer) {
      scrollContainer.scrollTop = totalHeight;
    }
  }

  // Start log streaming
  async function handleStart() {
    try {
      error = null;
      await startLogStream();
      isStreaming = true;

      // Set up event listener
      unlisten = await onLogEntry((entry) => {
        // Enforce max logs limit (FIFO: keep newest, drop oldest)
        if (logs.length >= maxLogs) {
          logs = [entry, ...logs.slice(0, maxLogs - 1)];
        } else {
          logs = [...logs, entry];
        }

        updateVisibleRange();

        // Auto-scroll to bottom
        if (autoScroll) {
          setTimeout(scrollToBottom, 50);
        }
      });
    } catch (err) {
      error = String(err);
      isStreaming = false;
    }
  }

  // Stop log streaming
  async function handleStop() {
    try {
      error = null;
      await stopLogStream();
      isStreaming = false;

      // Remove event listener
      if (unlisten) {
        unlisten();
        unlisten = null;
      }
    } catch (err) {
      error = String(err);
    }
  }

  // Toggle sort order
  function toggleSortOrder() {
    sortDescending = !sortDescending;
    logs = [...logs].reverse();
    localStorage.setItem('logSortOrder', sortDescending ? 'desc' : 'asc');
    updateVisibleRange();
  }

  // Clear all logs
  function clearLogs() {
    logs = [];
    updateVisibleRange();
  }

  // Initialize
  onMount(() => {
    // Restore sort order from localStorage
    const savedOrder = localStorage.getItem('logSortOrder');
    if (savedOrder) {
      sortDescending = savedOrder === 'desc';
    }

    updateVisibleRange();
  });

  // Cleanup
  onDestroy(async () => {
    if (isStreaming) {
      await handleStop();
    }
  });
</script>

<div class="log-viewer">
  <!-- Header with controls (optional) -->
  {#if showControls}
  <div class="log-viewer__header">
    <div class="log-viewer__controls">
      <!-- Start/Stop button -->
      {#if isStreaming}
        <button
          class="btn btn--secondary"
          on:click={handleStop}
          aria-label="Stop log stream"
        >
          Stop Stream
        </button>
      {:else}
        <button
          class="btn btn--primary"
          on:click={handleStart}
          aria-label="Start log stream"
        >
          Start Stream
        </button>
      {/if}

      <!-- Sort order toggle -->
      <button
        class="btn btn--secondary"
        on:click={toggleSortOrder}
        aria-label="Toggle sort order"
      >
        {sortDescending ? '↓ Newest First' : '↑ Oldest First'}
      </button>

      <!-- Clear logs button -->
      <button
        class="btn btn--secondary"
        on:click={clearLogs}
        disabled={logs.length === 0}
        aria-label="Clear all logs"
      >
        Clear Logs
      </button>

      <!-- Auto-scroll toggle -->
      <label class="log-viewer__checkbox">
        <input type="checkbox" bind:checked={autoScroll} />
        Auto-scroll
      </label>
    </div>
  </div>
  {/if}

  <!-- Error message -->
  {#if error}
    <div class="log-viewer__error" role="alert">
      <strong>Error:</strong>
      {error}
    </div>
  {/if}

  <!-- Log display area with virtual scrolling -->
  <div
    bind:this={scrollContainer}
    class="log-viewer__container"
    style="height: {viewportHeight}px; overflow-y: auto;"
    on:scroll={handleScroll}
  >
    <div style="height: {totalHeight}px; position: relative;">
      <div style="transform: translateY({offsetY}px);">
        {#if logs.length === 0}
          <div class="log-viewer__empty">
            {#if isStreaming}
              <p>Waiting for logs...</p>
            {:else}
              <p>No logs available. Start the stream to begin viewing logs.</p>
            {/if}
          </div>
        {:else}
          {#each visibleLogs as log (log.id)}
            <div class="log-entry" style="height: {itemHeight}px;">
              <span class="log-entry__timestamp">
                {formatLogTimestamp(log.timestamp)}
              </span>
              <span class="log-entry__level {getLogLevelClass(log.level)}">
                [{log.level}]
              </span>
              <span class="log-entry__message">
                {log.message}
              </span>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>

  <!-- Footer with stats -->
  <div class="log-viewer__footer">
    <span>Total logs: {logs.length}</span>
    {#if isStreaming}
      <span class="log-viewer__status log-viewer__status--active">
        ● Streaming
      </span>
    {:else}
      <span class="log-viewer__status">○ Stopped</span>
    {/if}
  </div>
</div>

<style>
  .log-viewer {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 0;
    background: transparent;
    border-radius: 0;
  }

  .log-viewer__header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .log-viewer__controls {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .log-viewer__checkbox {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    cursor: pointer;
  }

  .log-viewer__error {
    padding: 0.75rem;
    background: #3a1a1a;
    border: 1px solid #5a2a2a;
    border-radius: 0.25rem;
    color: #ff6b6b;
  }

  .log-viewer__container {
    background: #1e1e1e;
    border: 1px solid var(--border-color, #ddd);
    border-radius: 0.25rem;
    font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
    font-size: 0.875rem;
    color: #d4d4d4;
  }

  .log-viewer__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #888;
  }

  .log-entry {
    display: flex;
    gap: 0.75rem;
    padding: 0.25rem 0.75rem;
    border-bottom: 1px solid #2d2d2d;
    white-space: nowrap;
    overflow: hidden;
  }

  .log-entry:hover {
    background: #252525;
  }

  .log-entry__timestamp {
    color: #858585;
    min-width: 5rem;
  }

  .log-entry__level {
    font-weight: 600;
    min-width: 4rem;
  }

  /* Log level colors optimized for dark background */
  .log-entry__level :global(.log-level-error) {
    color: #f48771;
  }

  .log-entry__level :global(.log-level-warn) {
    color: #dcdcaa;
  }

  .log-entry__level :global(.log-level-info) {
    color: #4fc1ff;
  }

  .log-entry__level :global(.log-level-debug) {
    color: #b5b5b5;
  }

  .log-entry__level :global(.log-level-default) {
    color: #d4d4d4;
  }

  .log-entry__message {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    color: #d4d4d4;
  }

  .log-viewer__footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.875rem;
    color: var(--text-muted, #666);
  }

  .log-viewer__status {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .log-viewer__status--active {
    color: var(--success-color, #0a0);
  }

  .btn {
    padding: 0.5rem 1rem;
    border: 1px solid var(--border-color, #ddd);
    border-radius: 0.25rem;
    background: var(--bg-primary, #fff);
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.2s;
  }

  .btn:hover:not(:disabled) {
    background: var(--bg-hover, #f5f5f5);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn--primary {
    background: var(--primary-color, #007bff);
    color: white;
    border-color: var(--primary-color, #007bff);
  }

  .btn--primary:hover:not(:disabled) {
    background: var(--primary-hover, #0056b3);
  }

  .btn--secondary {
    background: var(--bg-secondary, #f9f9f9);
  }
</style>
