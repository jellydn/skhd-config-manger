<script lang="ts">
  import { onMount } from 'svelte';
  import type { ServiceState, ServiceStatus } from '../types';
  import {
    getServiceStatus,
    openAccessibilitySettings,
    openInputMonitoringSettings,
    restartService,
    startService,
  } from '../services/service';

  let {
    isImporting,
    onImport,
    onReload,
  }: {
    isImporting: boolean;
    onImport: () => Promise<void>;
    onReload: () => Promise<void>;
  } = $props();

  let status = $state<ServiceStatus | null>(null);
  let isChangingService = $state(false);
  let isReloading = $state(false);
  let feedback = $state<{ type: 'success' | 'error'; message: string } | null>(null);
  let statusPollInterval: number | null = null;
  let displayedFeedback = $derived(
    status?.error_message ? { type: 'error' as const, message: status.error_message } : feedback
  );

  onMount(() => {
    let destroyed = false;
    void loadStatus().then(() => {
      if (!destroyed) statusPollInterval = window.setInterval(loadStatus, 5000);
    });

    return () => {
      destroyed = true;
      if (statusPollInterval) window.clearInterval(statusPollInterval);
    };
  });

  async function loadStatus() {
    try {
      status = await getServiceStatus();
    } catch (error) {
      console.error('Failed to get service status:', error);
    }
  }

  async function handleServiceAction(action: 'start' | 'restart') {
    if (isChangingService) return;

    isChangingService = true;
    feedback = null;
    try {
      await (action === 'start' ? startService() : restartService());
      await loadStatus();
      feedback = {
        type: 'success',
        message: action === 'start' ? 'Service started.' : 'Service restarted.',
      };
    } catch (error) {
      feedback = { type: 'error', message: String(error) };
      await loadStatus();
    } finally {
      isChangingService = false;
    }
  }

  async function handleReload() {
    if (isReloading) return;

    isReloading = true;
    feedback = null;
    try {
      await onReload();
      feedback = { type: 'success', message: 'Service configuration reloaded.' };
    } catch (error) {
      feedback = { type: 'error', message: String(error) };
    } finally {
      isReloading = false;
    }
  }

  async function handleOpenAccessibilitySettings() {
    try {
      await openAccessibilitySettings();
    } catch (error) {
      feedback = {
        type: 'error',
        message: `Could not open System Settings: ${error}`,
      };
    }
  }

  async function handleOpenInputMonitoringSettings() {
    try {
      await openInputMonitoringSettings();
    } catch (error) {
      feedback = {
        type: 'error',
        message: `Could not open System Settings: ${error}`,
      };
    }
  }

  function getStatusClass(state: ServiceState): string {
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
</script>

<header class="toolbar">
  <div class="toolbar-left">
    <h1>Service Manager</h1>
    {#if status}
      <div class="service-status" role="status">
        <div class="status-indicator {getStatusClass(status.state)}" aria-hidden="true"></div>
        <span class="status-text">{status.state}</span>
        <span class="variant-badge">{status.variant === 'zig' ? 'skhd.zig' : 'skhd'}</span>
        {#if status.variant === 'zig'}
          <span class="status-detail">Input: {status.input_monitoring_permission}</span>
        {/if}
        {#if status.pid}
          <span class="status-pid">PID: {status.pid}</span>
        {/if}
      </div>
    {/if}
  </div>
  <div class="toolbar-actions">
    {#if status && (status.state === 'Stopped' || status.state === 'Unknown')}
      <button
        class="toolbar-btn toolbar-btn-primary"
        onclick={() => handleServiceAction('start')}
        disabled={isChangingService || isReloading}
      >
        {isChangingService ? 'Starting…' : 'Start Service'}
      </button>
    {:else if status?.state === 'Running' || status?.state === 'Error'}
      <button
        class="toolbar-btn"
        onclick={() => handleServiceAction('restart')}
        disabled={isChangingService || isReloading}
      >
        {isChangingService ? 'Restarting…' : 'Restart Service'}
      </button>
    {/if}
    <button
      class="toolbar-btn"
      onclick={onImport}
      disabled={isImporting}
      aria-label="Import configuration"
    >
      <svg
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
        <polyline points="14 2 14 8 20 8"></polyline>
        <line x1="12" y1="18" x2="12" y2="12"></line>
        <line x1="9" y1="15" x2="12" y2="12"></line>
        <line x1="15" y1="15" x2="12" y2="12"></line>
      </svg>
      Import Config
    </button>
    <button
      class="toolbar-btn"
      onclick={handleReload}
      disabled={isReloading || isChangingService || status?.state !== 'Running'}
      aria-label="Reload skhd service"
    >
      {#if isReloading}
        <svg
          class="spinner"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
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
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <polyline points="23 4 23 10 17 10"></polyline>
          <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
        </svg>
      {/if}
      Reload Service
    </button>
  </div>
</header>

{#if status && (status.accessibility_permission !== 'Granted' || status.input_monitoring_permission === 'Denied')}
  <section
    class:permission-denied={status.accessibility_permission === 'Denied' ||
      status.input_monitoring_permission === 'Denied'}
    class="permission-panel"
    aria-live="polite"
  >
    <div>
      <h2>
        {status.input_monitoring_permission === 'Denied'
          ? 'Input Monitoring permission denied'
          : `Accessibility ${status.accessibility_permission === 'Denied' ? 'permission denied' : 'permission not verified'}`}
      </h2>
      <p>{status.accessibility_guidance}</p>
      {#if status.accessibility_permission === 'Unknown'}
        <p class="permission-note">
          Keybinder verifies permission from the skhd daemon after it starts; it does not request
          permission for itself.
        </p>
      {/if}
    </div>
    <div class="permission-actions">
      <button class="toolbar-btn" onclick={handleOpenAccessibilitySettings}
        >Open Accessibility Settings</button
      >
      {#if status.variant === 'zig'}
        <button class="toolbar-btn" onclick={handleOpenInputMonitoringSettings}
          >Open Input Monitoring</button
        >
      {/if}
    </div>
  </section>
{/if}

{#if displayedFeedback}
  <div
    class:error-feedback={displayedFeedback.type === 'error'}
    class="service-feedback"
    role="alert"
  >
    {displayedFeedback.message}
  </div>
{/if}

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
    min-height: 52px;
    padding: 20px 20px 12px;
    background: var(--color-surface-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .toolbar-left,
  .service-status,
  .toolbar-actions,
  .toolbar-btn {
    display: flex;
    align-items: center;
  }

  .toolbar-left {
    gap: 16px;
  }

  .toolbar-left h1 {
    margin: 0;
    color: var(--color-text);
    font-size: 15px;
    font-weight: 600;
  }

  .service-status {
    gap: 8px;
    color: var(--color-text);
    font-size: 12px;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    flex-shrink: 0;
    border-radius: 50%;
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

  .status-text {
    font-weight: 500;
  }

  .status-pid {
    color: rgba(255, 255, 255, 0.5);
  }

  .status-detail {
    color: var(--color-text-secondary);
    font-size: 10px;
  }

  .variant-badge {
    padding: 2px 6px;
    color: var(--color-text-secondary);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 999px;
    font-size: 10px;
  }

  .toolbar-actions {
    gap: 8px;
  }

  .toolbar-btn {
    min-height: 28px;
    gap: 6px;
    padding: 6px 12px;
    color: var(--color-button-secondary-text);
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .toolbar-btn:hover:not(:disabled) {
    color: var(--color-text);
    background: var(--color-button-secondary-hover);
  }

  .toolbar-btn:active:not(:disabled) {
    background: var(--color-button-secondary-active);
  }

  .toolbar-btn:focus-visible {
    outline: 2px solid var(--color-button-secondary-focus);
    outline-offset: 2px;
  }

  .toolbar-btn:disabled {
    color: var(--color-button-disabled-text);
    background: var(--color-button-disabled-bg);
    border-color: var(--color-button-disabled-border);
    cursor: not-allowed;
    opacity: 0.6;
  }

  .toolbar-btn svg {
    flex-shrink: 0;
    opacity: 0.8;
  }

  .toolbar-btn-primary {
    color: var(--color-button-primary-text);
    background: var(--color-button-primary-bg);
    border-color: var(--color-button-primary-bg);
  }

  .toolbar-btn-primary:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  .permission-panel {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    margin: 12px 20px 0;
    padding: 12px 16px;
    color: var(--color-status-warning);
    background: var(--color-status-warning-bg);
    border: 1px solid var(--color-status-warning-border);
    border-radius: 6px;
  }

  .permission-panel.permission-denied {
    color: var(--color-status-error);
    background: var(--color-status-error-bg);
    border-color: var(--color-status-error-border);
  }

  .permission-panel h2 {
    margin: 0 0 4px;
    font-size: 13px;
  }

  .permission-panel p {
    margin: 0;
    font-size: 11px;
    line-height: 1.5;
  }

  .permission-note {
    margin-top: 4px !important;
    color: var(--color-text-secondary);
  }

  .permission-actions {
    display: flex;
    flex-shrink: 0;
    gap: 8px;
  }

  .service-feedback {
    margin: 12px 20px 0;
    padding: 10px 16px;
    color: var(--color-status-success);
    background: var(--color-status-success-bg);
    border: 1px solid var(--color-status-success-border);
    border-radius: 6px;
    font-size: 12px;
    white-space: pre-line;
  }

  .service-feedback.error-feedback {
    color: var(--color-status-error);
    background: var(--color-status-error-bg);
    border-color: var(--color-status-error-border);
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
