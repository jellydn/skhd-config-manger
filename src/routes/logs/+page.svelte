<script lang="ts">
  /**
   * Logs page - View skhd service logs in real-time
   *
   * This page provides the main interface for viewing and managing
   * logs from the skhd service.
   */

  import { onMount, onDestroy } from 'svelte';
  import LogViewer from '../../components/LogViewer.svelte';
  import type { ServiceStatus, ConfigFile } from '../../types';
  import { getServiceStatus, reloadService } from '../../services/service';
  import { detectActiveConfig, importConfig, saveConfig } from '../../services/tauri';

  // Service status state
  let status = $state<ServiceStatus | null>(null);
  let isReloading = $state(false);
  let statusPollInterval: number | null = null;

  // Configuration state
  let activeConfigPath = $state<string>('');
  let loadedConfigPath = $state<string>(''); // Config loaded in app (may differ from service config)
  let loadedConfig = $state<ConfigFile | null>(null); // The actual imported config
  let isImporting = $state(false);
  let importFeedback = $state<{ type: 'success' | 'error'; message: string } | null>(null);

  // Log viewer state that we'll pass down
  let logViewerRef: any = null;
  let logsCount = $state(0);
  let sortDescending = $state(true);
  let logLevelFilter = $state<'error' | 'info'>('error');

  // Pagination state
  let currentPage = $state(1);
  let logsPerPage = 50;

  // Lifecycle
  onMount(async () => {
    await Promise.all([loadStatus(), loadActiveConfig()]);
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

  async function loadActiveConfig() {
    try {
      activeConfigPath = await detectActiveConfig();
    } catch (err) {
      console.error('Failed to detect active config:', err);
      activeConfigPath = 'Unable to detect config path';
    }
  }

  async function handleImportConfig() {
    if (isImporting) return;

    try {
      isImporting = true;
      importFeedback = null;

      const config = await importConfig();

      // Store the imported config
      loadedConfig = config;
      loadedConfigPath = config.file_path;

      importFeedback = {
        type: 'success',
        message: `Imported: ${config.file_path}. Click "Reload Service" to apply.`
      };

      // Clear feedback after 5 seconds
      setTimeout(() => {
        importFeedback = null;
      }, 5000);
    } catch (err) {
      console.error('Failed to import config:', err);
      importFeedback = {
        type: 'error',
        message: `Failed to import: ${err}`
      };

      // Clear feedback after 10 seconds for errors
      setTimeout(() => {
        importFeedback = null;
      }, 10000);
    } finally {
      isImporting = false;
    }
  }

  async function handleReload() {
    if (isReloading) return;
    try {
      isReloading = true;

      // If there's a loaded config (from import), save it to the active config path first
      // This ensures skhd reads the imported config when it reloads
      if (loadedConfig && activeConfigPath) {
        // Update the config's file_path to the active skhd config location
        // so it gets saved where skhd will actually read it from
        const configToSave = { ...loadedConfig, file_path: activeConfigPath };
        await saveConfig(configToSave);
      }

      await reloadService();

      // After reload, update what skhd service is actually using
      setTimeout(async () => {
        await loadStatus();
        await loadActiveConfig();
        // Clear loaded config since it's now active
        loadedConfigPath = '';
        loadedConfig = null;
      }, 1000);
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

  // Pagination controls
  function handlePrevPage() {
    if (currentPage > 1) {
      currentPage--;
      if (logViewerRef) {
        logViewerRef.scrollToTop();
      }
    }
  }

  function handleNextPage() {
    const totalPages = Math.ceil(logsCount / logsPerPage);
    if (currentPage < totalPages) {
      currentPage++;
      if (logViewerRef) {
        logViewerRef.scrollToTop();
      }
    }
  }

  // Load more historical logs
  let isLoadingMore = $state(false);
  let hasMoreLogs = $state(true);
  let previousLogsCount = 0;

  async function handleLoadMore() {
    if (logViewerRef && !isLoadingMore && hasMoreLogs) {
      try {
        isLoadingMore = true;
        previousLogsCount = logsCount;
        await logViewerRef.loadMoreLogs(500); // Load 500 more logs
        updateLogState();

        // Check if we got new logs
        if (logsCount === previousLogsCount) {
          hasMoreLogs = false; // No more logs available
        }
      } catch (err) {
        console.error('Failed to load more logs:', err);
      } finally {
        isLoadingMore = false;
      }
    }
  }

  // Computed pagination values
  let totalPages = $derived(Math.ceil(logsCount / logsPerPage));
  let startIndex = $derived((currentPage - 1) * logsPerPage);
  let endIndex = $derived(startIndex + logsPerPage);

  function updateLogState() {
    if (logViewerRef) {
      logsCount = logViewerRef.getLogsCount();
    }
  }

  // Poll log state periodically
  $effect(() => {
    const interval = setInterval(updateLogState, 500);
    return () => clearInterval(interval);
  });

  // Reset load more state and pagination when filter changes
  $effect(() => {
    // Watch for filter changes
    logLevelFilter;
    // Reset hasMoreLogs and go back to page 1 when filter changes
    hasMoreLogs = true;
    currentPage = 1;
  });
</script>

<svelte:head>
  <title>Service Manager - Keybinder</title>
</svelte:head>

<div class="logs-page">
  <!-- Toolbar -->
  <header class="toolbar">
    <div class="toolbar-left">
      <h1>Service Manager</h1>
      {#if status}
        <div class="service-status" role="status" aria-label="Service status: {status.state}">
          <div class="status-indicator {getStatusClass(status.state)}" aria-hidden="true"></div>
          <span class="status-text">{status.state}</span>
          {#if status.pid}
            <span class="status-pid">PID: {status.pid}</span>
          {/if}
        </div>
      {/if}
    </div>
    <div class="toolbar-actions">
      <!-- Configuration Import -->
      <button
        class="toolbar-btn"
        onclick={handleImportConfig}
        disabled={isImporting}
        aria-label="Import configuration"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
          <line x1="12" y1="18" x2="12" y2="12"></line>
          <line x1="9" y1="15" x2="12" y2="12"></line>
          <line x1="15" y1="15" x2="12" y2="12"></line>
        </svg>
        Import Config
      </button>

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
    </div>
  </header>

  <main class="logs-page__content">
    <!-- Log Viewer Controls Panel -->
    <div class="log-controls-panel">
      <div class="log-controls-left">
        <div class="log-header-group">
          <h2>Service Logs</h2>
          <!-- Active Config inline -->
          {#if activeConfigPath}
            <span class="config-path-inline">
              <span class="config-label">Config:</span>
              <code class="config-value">{activeConfigPath}</code>
            </span>
          {/if}
        </div>
        <!-- Import Feedback inline -->
        {#if importFeedback}
          <div class="import-feedback-inline import-feedback-{importFeedback.type}">
            {#if importFeedback.type === 'success'}
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
            {:else}
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" y1="8" x2="12" y2="12"></line>
                <line x1="12" y1="16" x2="12.01" y2="16"></line>
              </svg>
            {/if}
            <span>{importFeedback.message}</span>
          </div>
        {/if}
      </div>
      <div class="log-controls-actions">
        <!-- Log Level Filter (Compact Toggle) -->
        <div class="filter-group">
          <button
            class="filter-btn {logLevelFilter === 'error' ? 'filter-btn-active' : ''}"
            onclick={() => logLevelFilter = 'error'}
            aria-label="Show error logs only"
            aria-pressed={logLevelFilter === 'error'}
          >
            Error
          </button>
          <button
            class="filter-btn {logLevelFilter === 'info' ? 'filter-btn-active' : ''}"
            onclick={() => logLevelFilter = 'info'}
            aria-label="Show info logs only"
            aria-pressed={logLevelFilter === 'info'}
          >
            Info
          </button>
        </div>

        <!-- Sort Order Indicator (descending by default) -->
        <div class="sort-indicator" title="Newest First">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 2L14 8H10V14H6V8H2L8 2Z" fill="currentColor" opacity="0.8"/>
          </svg>
        </div>

        <!-- Pagination and Count Group -->
        <div class="pagination-group">
          <span class="log-count">
            {logsCount} {logLevelFilter === 'error' ? 'error' : 'info'} logs
          </span>
          <div class="pagination-controls">
            <button
              class="pagination-btn"
              onclick={handlePrevPage}
              disabled={currentPage === 1}
              aria-label="Previous page"
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="15 18 9 12 15 6"></polyline>
              </svg>
            </button>
            <span class="pagination-info">
              Page {currentPage} of {totalPages || 1}
            </span>
            <button
              class="pagination-btn"
              onclick={handleNextPage}
              disabled={currentPage >= totalPages || totalPages === 0}
              aria-label="Next page"
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <LogViewer
      bind:this={logViewerRef}
      maxLogs={1000}
      levelFilter={logLevelFilter}
      {startIndex}
      {endIndex}
      showControls={false}
    />

    <!-- Load More at bottom (only show on last page) -->
    {#if currentPage === totalPages && hasMoreLogs && totalPages > 0}
      <div class="load-more-bottom">
        <button
          class="load-more-bottom-btn"
          onclick={handleLoadMore}
          disabled={isLoadingMore}
          aria-label="Load more historical logs"
        >
          {#if isLoadingMore}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="spinner">
              <circle cx="12" cy="12" r="10"></circle>
            </svg>
            Loading more logs...
          {:else}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 12h18M3 12l6-6M3 12l6 6"></path>
              <path d="M21 12l-6-6M21 12l-6 6"></path>
            </svg>
            Load More (500 logs)
          {/if}
        </button>
      </div>
    {/if}
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
    background: var(--color-surface-secondary);
    border-bottom: 1px solid var(--color-border);
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
    color: var(--color-text);
    margin: 0;
  }

  .service-status {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--color-text);
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-running {
    background: var(--color-status-success);
    box-shadow: 0 0 6px var(--color-status-success-bg);
  }

  .status-stopped {
    background: var(--color-status-stopped);
  }

  .status-transitioning {
    background: var(--color-status-warning);
    animation: pulse 1.5s ease-in-out infinite;
  }

  .status-error {
    background: var(--color-status-error);
    box-shadow: 0 0 6px var(--color-status-error-bg);
  }

  .status-unknown {
    background: var(--color-status-unknown);
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
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 6px;
    color: var(--color-button-secondary-text);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    min-height: 28px;
  }

  .toolbar-btn:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .toolbar-btn:active:not(:disabled) {
    background: var(--color-button-secondary-active);
  }

  .toolbar-btn:focus-visible {
    outline: 2px solid var(--color-button-secondary-focus);
    outline-offset: 2px;
  }

  .toolbar-btn:disabled {
    background: var(--color-button-disabled-bg);
    color: var(--color-button-disabled-text);
    border-color: var(--color-button-disabled-border);
    cursor: not-allowed;
    opacity: 0.6;
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
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 6px;
    color: var(--color-button-secondary-text);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toolbar-checkbox:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .toolbar-checkbox svg {
    flex-shrink: 0;
    color: var(--color-border-hover);
  }

  .toolbar-btn-primary {
    background: var(--color-button-primary-bg);
    border-color: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
  }

  .toolbar-btn-primary:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .toolbar-btn-primary:active:not(:disabled) {
    background: var(--color-button-primary-active);
    border-color: var(--color-button-primary-active);
  }

  .toolbar-btn-primary:focus-visible {
    outline: 2px solid var(--color-button-primary-focus);
    outline-offset: 2px;
  }

  .toolbar-btn-primary:disabled {
    background: var(--color-button-disabled-bg);
    color: var(--color-button-disabled-text);
    border-color: var(--color-button-disabled-border);
    cursor: not-allowed;
    opacity: 0.6;
  }

  .logs-page__content {
    flex: 1;
    overflow: hidden;
    padding: 20px;
    background: var(--color-background);
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-height: 0;
  }

  /* Active Configuration Display */
  .config-path-display {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 16px;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 12px;
  }

  .config-path-label {
    color: var(--color-text-secondary);
    font-weight: 500;
  }

  .config-path-value {
    flex: 1;
    padding: 4px 10px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-text);
    font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
    font-size: 11px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .config-path-pending {
    border-color: #ff9500;
    background: rgba(255, 149, 0, 0.1);
  }

  .config-path-hint {
    color: rgba(255, 149, 0, 0.8);
    font-size: 11px;
    font-style: italic;
  }

  /* Import Feedback */
  .import-feedback {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 16px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    animation: slideIn 0.3s ease-out;
  }

  .import-feedback svg {
    flex-shrink: 0;
  }

  .import-feedback-success {
    background: var(--color-status-success-bg);
    border: 1px solid var(--color-status-success-border);
    color: var(--color-status-success);
  }

  .import-feedback-error {
    background: var(--color-status-error-bg);
    border: 1px solid var(--color-status-error-border);
    color: var(--color-status-error);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Log Controls Panel */
  .log-controls-panel {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    margin-bottom: 12px;
  }

  .log-controls-left {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .log-header-group {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .log-controls-left h2 {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
  }

  .config-path-inline {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
  }

  .config-label {
    color: var(--color-text-secondary);
    font-weight: 500;
  }

  .config-value {
    padding: 2px 8px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-text);
    font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
    font-size: 10px;
  }

  .import-feedback-inline {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 500;
  }

  .import-feedback-inline.import-feedback-success {
    background: var(--color-status-success-bg);
    border: 1px solid var(--color-status-success-border);
    color: var(--color-status-success);
  }

  .import-feedback-inline.import-feedback-error {
    background: var(--color-status-error-bg);
    border: 1px solid var(--color-status-error-border);
    color: var(--color-status-error);
  }

  .import-feedback-inline svg {
    flex-shrink: 0;
  }

  .log-controls-actions {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .log-control-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 5px;
    color: var(--color-button-secondary-text);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .log-control-btn:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .log-control-btn:active:not(:disabled) {
    background: var(--color-surface-secondary);
  }

  .log-control-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .log-control-btn svg {
    flex-shrink: 0;
    opacity: 0.8;
  }

  .log-control-btn:hover:not(:disabled) svg {
    opacity: 1;
  }

  .log-control-btn-primary {
    background: var(--color-button-primary-bg);
    border-color: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
  }

  .log-control-btn-primary:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .log-control-btn-stop {
    background: var(--color-button-secondary-bg);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .log-control-btn-stop:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
  }

  .log-control-checkbox {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 5px;
    color: var(--color-button-secondary-text);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .log-control-checkbox:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .log-control-checkbox svg {
    flex-shrink: 0;
    color: var(--color-border-hover);
  }

  /* Log Level Filter */
  .filter-group {
    display: flex;
    gap: 0;
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 5px;
    overflow: hidden;
  }

  .filter-btn {
    padding: 5px 10px;
    background: transparent;
    border: none;
    border-right: 1px solid var(--color-button-secondary-border);
    color: var(--color-button-secondary-text);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .filter-btn:last-child {
    border-right: none;
  }

  .filter-btn:hover:not(.filter-btn-active) {
    background: var(--color-button-secondary-hover);
    color: var(--color-text);
  }

  .filter-btn-active {
    background: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
  }

  /* Pagination Controls */
  .pagination-controls {
    display: flex;
    gap: 6px;
    align-items: center;
    padding: 0 4px;
  }

  .pagination-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 5px 6px;
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 5px;
    color: var(--color-button-secondary-text);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .pagination-btn:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .pagination-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .pagination-btn svg {
    flex-shrink: 0;
  }

  .pagination-info {
    font-size: 11px;
    color: var(--color-text-secondary);
    font-weight: 500;
    min-width: 40px;
    text-align: center;
  }

  .pagination-group {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .log-count {
    font-size: 11px;
    color: var(--color-text-secondary);
    font-weight: 500;
    white-space: nowrap;
  }

  /* Load More at Bottom */
  .load-more-bottom {
    display: flex;
    justify-content: center;
    padding: 20px;
    margin-top: 16px;
  }

  .load-more-bottom-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 6px;
    color: var(--color-button-secondary-text);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .load-more-bottom-btn:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
    color: var(--color-text);
  }

  .load-more-bottom-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .load-more-bottom-btn svg {
    flex-shrink: 0;
  }
</style>
