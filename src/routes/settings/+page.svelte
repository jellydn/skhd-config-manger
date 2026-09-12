<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';
  import {
    checkForUpdates,
    downloadUpdate,
    getUpdatePreferences,
    installUpdate,
    setAutomaticCheck,
    setAutomaticDownload,
    updateState,
  } from '../../services/updateService';

  let currentVersion = $state('Loading…');
  let automaticCheck = $state(true);
  let automaticDownload = $state(false);

  let progress = $derived(
    $updateState.totalBytes
      ? Math.min(100, Math.round(($updateState.downloadedBytes / $updateState.totalBytes) * 100))
      : null
  );

  onMount(async () => {
    ({ automaticCheck, automaticDownload } = getUpdatePreferences());
    try {
      currentVersion = await getVersion();
    } catch {
      currentVersion = 'Development build';
    }
  });

  function changeAutomaticCheck(enabled: boolean) {
    automaticCheck = enabled;
    setAutomaticCheck(enabled);
  }

  function changeAutomaticDownload(enabled: boolean) {
    automaticDownload = enabled;
    setAutomaticDownload(enabled);
  }
</script>

<svelte:head>
  <title>Settings - Keybinder</title>
</svelte:head>

<div class="settings-page">
  <header class="toolbar">
    <h1>Settings</h1>
  </header>

  <main>
    <section class="settings-card">
      <div class="section-heading">
        <div>
          <h2>Updates</h2>
          <p>Installed version: {currentVersion}</p>
        </div>
        <button
          class="primary-button"
          onclick={() => checkForUpdates(true)}
          disabled={$updateState.phase === 'checking' ||
            $updateState.phase === 'downloading' ||
            $updateState.phase === 'installing'}
        >
          {$updateState.phase === 'checking' ? 'Checking…' : 'Check for Updates'}
        </button>
      </div>

      <label class="setting-row">
        <span>
          <strong>Automatically check for updates</strong>
          <small>Check the latest stable GitHub release at most once each day.</small>
        </span>
        <input
          type="checkbox"
          checked={automaticCheck}
          onchange={(event) => changeAutomaticCheck(event.currentTarget.checked)}
        />
      </label>

      <label class="setting-row" class:disabled={!automaticCheck}>
        <span>
          <strong>Automatically download updates</strong>
          <small
            >Download signed updates in the background. Installation still needs confirmation.</small
          >
        </span>
        <input
          type="checkbox"
          checked={automaticDownload}
          disabled={!automaticCheck}
          onchange={(event) => changeAutomaticDownload(event.currentTarget.checked)}
        />
      </label>

      <div
        class:error={$updateState.phase === 'error'}
        class="update-status"
        role="status"
        aria-live="polite"
      >
        <div>
          <strong>{$updateState.message}</strong>
          {#if $updateState.notes}
            <p>{$updateState.notes}</p>
          {/if}
          {#if $updateState.phase === 'downloading'}
            <progress
              value={progress ?? undefined}
              max="100"
              aria-label="Keybinder update download progress"
            ></progress>
            <small>{progress === null ? 'Downloading…' : `${progress}% downloaded`}</small>
          {/if}
        </div>
        {#if $updateState.phase === 'available'}
          <button onclick={downloadUpdate}>Download Update</button>
        {:else if $updateState.phase === 'ready'}
          <button class="primary-button" onclick={installUpdate}>Install and Relaunch</button>
        {/if}
      </div>

      <p class="trust-note">
        Keybinder installs only update bundles that pass its embedded Tauri signature check. macOS
        Developer ID signing and notarization are separate release requirements.
      </p>
    </section>
  </main>
</div>

<style>
  .settings-page {
    height: 100%;
    overflow: auto;
    background: var(--color-background);
  }

  .toolbar {
    padding: 20px 24px;
    background: var(--color-surface-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  h1,
  h2,
  p {
    margin: 0;
  }

  h1 {
    color: var(--color-text);
    font-size: 15px;
  }

  main {
    max-width: 760px;
    padding: 24px;
  }

  .settings-card {
    overflow: hidden;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
  }

  .section-heading,
  .setting-row,
  .update-status {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    padding: 16px;
    border-bottom: 1px solid var(--color-border);
  }

  .section-heading h2 {
    color: var(--color-text);
    font-size: 15px;
  }

  .section-heading p,
  small,
  .trust-note {
    color: var(--color-text-secondary);
    font-size: 11px;
  }

  .setting-row strong,
  .update-status strong {
    display: block;
    margin-bottom: 4px;
    color: var(--color-text);
    font-size: 12px;
  }

  .setting-row.disabled {
    opacity: 0.55;
  }

  button {
    padding: 7px 12px;
    color: var(--color-button-secondary-text);
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 6px;
    cursor: pointer;
    white-space: nowrap;
  }

  button:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
  }

  button:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }

  .primary-button {
    color: var(--color-button-primary-text);
    background: var(--color-button-primary-bg);
    border-color: var(--color-button-primary-bg);
  }

  .update-status {
    align-items: flex-start;
    color: var(--color-text-secondary);
    background: var(--color-surface);
  }

  .update-status.error strong {
    color: var(--color-status-error);
  }

  .update-status p {
    max-width: 560px;
    margin-top: 8px;
    color: var(--color-text-secondary);
    font-size: 11px;
    line-height: 1.5;
    white-space: pre-line;
  }

  progress {
    display: block;
    width: 280px;
    margin: 10px 0 4px;
  }

  .trust-note {
    padding: 12px 16px;
    line-height: 1.5;
  }
</style>
