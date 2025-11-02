<script lang="ts">
  /**
   * ServiceControl component - Controls for skhd service management
   *
   * Features:
   * - Service status display with colored indicator
   * - Reload service button
   * - Loading state during reload
   * - Error handling with toast notifications
   * - Periodic status polling
   */

  import { onMount, onDestroy } from 'svelte';
  import type { ServiceStatus } from '../types';
  import { getServiceStatus, reloadService } from '../services/service';

  // State
  let status = $state<ServiceStatus | null>(null);
  let isReloading = $state(false);
  let error = $state<string | null>(null);
  let success = $state<string | null>(null);
  let statusPollInterval: number | null = null;
  let failureCount = 0;
  const MAX_FAILURES = 5;
  const BASE_POLL_INTERVAL = 5000; // 5 seconds

  // Lifecycle
  onMount(async () => {
    // Initial status load
    await loadStatus();

    // Start polling with initial interval
    startPolling();
  });

  onDestroy(() => {
    stopPolling();
  });

  // Start status polling with exponential backoff
  function startPolling() {
    stopPolling(); // Clear any existing interval
    const interval = Math.min(
      BASE_POLL_INTERVAL * Math.pow(2, failureCount),
      30000 // Max 30 seconds
    );
    statusPollInterval = window.setInterval(loadStatus, interval);
  }

  // Stop status polling
  function stopPolling() {
    if (statusPollInterval) {
      window.clearInterval(statusPollInterval);
      statusPollInterval = null;
    }
  }

  // Load service status with circuit breaker
  async function loadStatus() {
    // Circuit breaker: stop polling after too many failures
    if (failureCount >= MAX_FAILURES) {
      console.error(`Status polling stopped after ${MAX_FAILURES} consecutive failures`);
      error = 'Service monitoring paused due to repeated failures. Please check service status manually.';
      stopPolling();
      return;
    }

    try {
      status = await getServiceStatus();

      // Reset failure count on success
      if (failureCount > 0) {
        failureCount = 0;
        startPolling(); // Reset to normal interval
      }

      // Clear error on successful load
      if (error && error.includes('monitoring paused')) {
        error = null;
      }
    } catch (err) {
      failureCount++;
      const errorMsg = String(err);
      console.error(`Failed to get service status (attempt ${failureCount}/${MAX_FAILURES}):`, err);

      // Only show error if we haven't hit max failures yet
      if (failureCount < MAX_FAILURES) {
        // Don't overwrite existing errors immediately
        if (!error || !error.includes('monitoring paused')) {
          error = errorMsg;
        }
        // Restart polling with exponential backoff
        startPolling();
      }
    }
  }

  // Handle reload button click
  async function handleReload() {
    if (isReloading) return;

    try {
      isReloading = true;
      error = null;
      success = null;

      await reloadService();

      success = 'Service reloaded successfully';

      // Reset failure count and resume polling on successful reload
      failureCount = 0;
      startPolling();

      // Reload status after a brief delay
      setTimeout(loadStatus, 1000);

      // Clear success message after 5 seconds
      setTimeout(() => {
        success = null;
      }, 5000);
    } catch (err) {
      error = String(err);
      console.error('Failed to reload service:', err);
    } finally {
      isReloading = false;
    }
  }

  // Get status indicator class based on state
  function getStatusClass(state: string): string {
    switch (state) {
      case 'Running':
        return 'status-running';
      case 'Stopped':
        return 'status-stopped';
      case 'Starting':
      case 'Stopping':
      case 'Reloading':
        return 'status-transitioning';
      case 'Error':
        return 'status-error';
      default:
        return 'status-unknown';
    }
  }

  // Get status label
  function getStatusLabel(state: string): string {
    switch (state) {
      case 'Running':
        return 'Running';
      case 'Stopped':
        return 'Stopped';
      case 'Starting':
        return 'Starting';
      case 'Stopping':
        return 'Stopping';
      case 'Reloading':
        return 'Reloading';
      case 'Error':
        return 'Error';
      default:
        return 'Unknown';
    }
  }
</script>

<div class="service-control">
  <!-- Service Status -->
  <div class="service-status">
    {#if status}
      <div class="status-indicator {getStatusClass(status.state)}"></div>
      <span class="status-text">{getStatusLabel(status.state)}</span>
      {#if status.pid}
        <span class="status-pid">PID: {status.pid}</span>
      {/if}
    {:else}
      <div class="status-indicator status-unknown"></div>
      <span class="status-text">Loading...</span>
    {/if}
  </div>

  <!-- Reload Button -->
  <button
    class="btn-reload"
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
      Reloading...
    {:else}
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="23 4 23 10 17 10"></polyline>
        <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
      </svg>
      Reload Service
    {/if}
  </button>

  <!-- Notifications -->
  {#if success}
    <div class="notification notification-success" role="alert">
      {success}
    </div>
  {/if}

  {#if error}
    <div class="notification notification-error" role="alert">
      <strong>Error:</strong> {error}
    </div>
  {/if}
</div>

<style>
  .service-control {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    background: #252525;
    border: 1px solid #2d2d2d;
    border-radius: 8px;
    margin-bottom: 1rem;
  }

  .service-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
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
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .status-text {
    color: rgba(255, 255, 255, 0.85);
    font-weight: 500;
  }

  .status-pid {
    color: rgba(255, 255, 255, 0.5);
    font-size: 0.75rem;
    margin-left: auto;
  }

  .btn-reload {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: rgba(10, 132, 255, 0.15);
    border: 1px solid rgba(10, 132, 255, 0.3);
    border-radius: 6px;
    color: #0a84ff;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-reload:hover:not(:disabled) {
    background: rgba(10, 132, 255, 0.25);
    border-color: rgba(10, 132, 255, 0.5);
  }

  .btn-reload:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-reload svg {
    flex-shrink: 0;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .notification {
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.875rem;
  }

  .notification-success {
    background: rgba(48, 209, 88, 0.15);
    border: 1px solid rgba(48, 209, 88, 0.3);
    color: #30d158;
  }

  .notification-error {
    background: rgba(255, 59, 48, 0.15);
    border: 1px solid rgba(255, 59, 48, 0.3);
    color: #ff6b6b;
  }

  .notification-error strong {
    font-weight: 600;
  }
</style>
