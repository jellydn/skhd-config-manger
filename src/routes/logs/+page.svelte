<script lang="ts">
  /**
   * Logs page - View skhd service logs in real-time
   *
   * This page provides the main interface for viewing and managing
   * logs from the skhd service.
   */

  import { onMount, onDestroy } from 'svelte';
  import LogViewer from '../../components/LogViewer.svelte';
  import type { ServiceStatus } from '../../types';
  import { getServiceStatus, reloadService } from '../../services/service';

  // Service status state
  let status = $state<ServiceStatus | null>(null);
  let isReloading = $state(false);
  let statusPollInterval: number | null = null;

  // Log viewer state that we'll pass down
  let logViewerRef: any = null;
  let isStreaming = $state(false);
  let logsCount = $state(0);
  let sortDescending = $state(true);
  let autoScroll = $state(true);

  // Lifecycle
  onMount(async () => {
    await loadStatus();
    statusPollInterval = window.setInterval(loadStatus, 5000);
  });

  onDestroy(() => {
    if (statusPollInterval) {
      window.clearInterval(statusPollInterval);
    }
  });

  async function loadStatus() {
    try {
      status = await getServiceStatus();
    } catch (err) {
      console.error('Failed to get service status:', err);
    }
  }

  async function handleReload() {
    if (isReloading) return;
    try {
      isReloading = true;
      await reloadService();
      setTimeout(loadStatus, 1000);
    } catch (err) {
      console.error('Failed to reload service:', err);
    } finally {
      isReloading = false;
    }
  }

  function getStatusClass(state: string): string {
    switch (state) {
      case 'Running': return 'status-running';
      case 'Stopped': return 'status-stopped';
      case 'Starting':
      case 'Stopping':
      case 'Reloading': return 'status-transitioning';
      case 'Error': return 'status-error';
      default: return 'status-unknown';
    }
  }

  // Log viewer control functions
  function handleStartStream() {
    if (logViewerRef) {
      logViewerRef.startStream();
      updateLogState();
    }
  }

  function handleStopStream() {
    if (logViewerRef) {
      logViewerRef.stopStream();
      updateLogState();
    }
  }

  function handleToggleSort() {
    if (logViewerRef) {
      logViewerRef.toggleSortOrder();
      updateLogState();
    }
  }

  function handleClearLogs() {
    if (logViewerRef) {
      logViewerRef.clearLogs();
      updateLogState();
    }
  }

  function updateLogState() {
    if (logViewerRef) {
      isStreaming = logViewerRef.getIsStreaming();
      logsCount = logViewerRef.getLogsCount();
      sortDescending = logViewerRef.getSortDescending();
    }
  }

  // Poll log state periodically
  $effect(() => {
    const interval = setInterval(updateLogState, 500);
    return () => clearInterval(interval);
  });
</script>

<svelte:head>
  <title>Service Logs - Keybinder</title>
</svelte:head>

<div class="logs-page">
  <!-- Toolbar -->
  <header class="toolbar">
    <div class="toolbar-left">
      <h1>Service Logs</h1>
      {#if status}
        <div class="service-status">
          <div class="status-indicator {getStatusClass(status.state)}"></div>
          <span class="status-text">{status.state}</span>
          {#if status.pid}
            <span class="status-pid">PID: {status.pid}</span>
          {/if}
        </div>
      {/if}
    </div>
    <div class="toolbar-actions">
      <!-- Service Control -->
      <button
        class="toolbar-btn"
        onclick={handleReload}
        disabled={isReloading || !status || status?.state === 'Error'}
        aria-label="Reload skhd service"
      >
        {#if isReloading}
          <svg class="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="12" y1="2" x2="12" y2="6"></line>
            <line x1="12" y1="18" x2="12" y2="22"></line>
            <line x1="4.93" y1="4.93" x2="7.76" y2="7.76"></line>
            <line x1="16.24" y1="16.24" x2="19.07" y2="19.07"></line>
            <line x1="2" y1="12" x2="6" y2="12"></line>
            <line x1="18" y1="12" x2="22" y2="12"></line>
            <line x1="4.93" y1="19.07" x2="7.76" y2="16.24"></line>
            <line x1="16.24" y1="7.76" x2="19.07" y2="4.93"></line>
          </svg>
        {:else}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="23 4 23 10 17 10"></polyline>
            <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
          </svg>
        {/if}
        Reload Service
      </button>

      <!-- Log Viewer Controls -->
      {#if isStreaming}
        <button class="toolbar-btn" onclick={handleStopStream} aria-label="Stop log stream">
          Stop Stream
        </button>
      {:else}
        <button class="toolbar-btn toolbar-btn-primary" onclick={handleStartStream} aria-label="Start log stream">
          Start Stream
        </button>
      {/if}

      <button class="toolbar-btn" onclick={handleToggleSort} aria-label="Toggle sort order">
        {sortDescending ? '↑ Oldest First' : '↓ Newest First'}
      </button>

      <button class="toolbar-btn" onclick={handleClearLogs} disabled={logsCount === 0} aria-label="Clear all logs">
        Clear Logs
      </button>

      <label class="toolbar-checkbox">
        <input type="checkbox" bind:checked={autoScroll} />
        <svg width="14" height="14" viewBox="0 0 20 20" fill="currentColor">
          {#if autoScroll}
            <path d="M0 11l2-2 5 5L18 3l2 2L7 18z"/>
          {:else}
            <rect x="3" y="3" width="14" height="14" rx="2" fill="none" stroke="currentColor" stroke-width="2"/>
          {/if}
        </svg>
        Auto-scroll
      </label>
    </div>
  </header>

  <main class="logs-page__content">
    <LogViewer bind:this={logViewerRef} maxLogs={1000} bind:autoScroll showControls={false} />
  </main>
</div>

<style>
  .logs-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  /* Toolbar - Native macOS style */
  .toolbar {
    background: #1c1c1c;
    border-bottom: 1px solid #2d2d2d;
    padding: 20px 20px 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-shrink: 0;
    min-height: 52px;
  }

  .toolbar-left {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .toolbar-left h1 {
    font-size: 15px;
    font-weight: 600;
    color: #ffffff;
    margin: 0;
  }

  .service-status {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.85);
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-running {
    background: #30d158;
    box-shadow: 0 0 6px rgba(48, 209, 88, 0.4);
  }

  .status-stopped {
    background: #8e8e93;
  }

  .status-transitioning {
    background: #ff9500;
    animation: pulse 1.5s ease-in-out infinite;
  }

  .status-error {
    background: #ff3b30;
    box-shadow: 0 0 6px rgba(255, 59, 48, 0.4);
  }

  .status-unknown {
    background: #636366;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .status-text {
    font-weight: 500;
  }

  .status-pid {
    color: rgba(255, 255, 255, 0.5);
  }

  .toolbar-actions {
    display: flex;
    gap: 8px;
  }

  .toolbar-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.85);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toolbar-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.12);
    border-color: rgba(255, 255, 255, 0.15);
    color: #ffffff;
  }

  .toolbar-btn:active:not(:disabled) {
    background: rgba(255, 255, 255, 0.06);
  }

  .toolbar-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .toolbar-btn svg {
    flex-shrink: 0;
    opacity: 0.8;
  }

  .toolbar-btn:hover:not(:disabled) svg {
    opacity: 1;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .toolbar-checkbox {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.85);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toolbar-checkbox:hover {
    background: rgba(255, 255, 255, 0.12);
    border-color: rgba(255, 255, 255, 0.15);
    color: #ffffff;
  }

  .toolbar-checkbox input[type="checkbox"] {
    appearance: none;
    width: 0;
    height: 0;
    position: absolute;
  }

  .toolbar-checkbox svg {
    flex-shrink: 0;
    color: #0a84ff;
  }

  .toolbar-btn-primary {
    background: #0a84ff;
    border-color: #0a84ff;
    color: #ffffff;
  }

  .toolbar-btn-primary:hover:not(:disabled) {
    background: #0071e3;
    border-color: #0071e3;
  }

  .logs-page__content {
    flex: 1;
    overflow: hidden;
    padding: 20px;
    background: #1e1e1e;
  }
</style>
