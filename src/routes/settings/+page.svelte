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
  import {
    getEffectiveVariant,
    getSkhdVariantSetting,
    setSkhdVariantSetting,
  } from '../../services/settingsService';
  import { installService } from '../../services/service';
  import type { EffectiveVariantResponse, SkhdVariantSetting } from '../../types';

  let currentVersion = $state('Loading…');
  let automaticCheck = $state(true);
  let automaticDownload = $state(false);
  let variantSetting = $state<SkhdVariantSetting>('auto');
  let effectiveVariant = $state<EffectiveVariantResponse | null>(null);
  let variantFeedback = $state<string | null>(null);
  let changingVariant = $state(false);

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
    await loadVariant();
  });

  async function loadVariant() {
    try {
      variantSetting = await getSkhdVariantSetting();
      effectiveVariant = await getEffectiveVariant();
    } catch (error) {
      variantFeedback = `Could not inspect skhd: ${error}`;
    }
  }

  async function changeVariant(value: SkhdVariantSetting) {
    changingVariant = true;
    variantFeedback = null;
    try {
      await setSkhdVariantSetting(value);
      variantSetting = value;
      effectiveVariant = await getEffectiveVariant();
    } catch (error) {
      variantFeedback = `Could not save the skhd selection: ${error}`;
    } finally {
      changingVariant = false;
    }
  }

  async function handleInstallService() {
    changingVariant = true;
    variantFeedback = null;
    try {
      await installService();
      variantFeedback = 'skhd.zig service registration completed.';
      effectiveVariant = await getEffectiveVariant();
    } catch (error) {
      variantFeedback = String(error);
    } finally {
      changingVariant = false;
    }
  }

  function effectiveVariantName(): string {
    if (!effectiveVariant) return 'Detecting…';
    return effectiveVariant.variant === 'zig' ? 'skhd.zig' : 'skhd';
  }

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
          <h2>skhd implementation</h2>
          <p>
            Active: {effectiveVariantName()}
            {effectiveVariant?.is_auto_detected ? ' (detected automatically)' : ''}
          </p>
        </div>
        <select
          aria-label="skhd implementation"
          value={variantSetting}
          disabled={changingVariant}
          onchange={(event) => changeVariant(event.currentTarget.value as SkhdVariantSetting)}
        >
          <option value="auto">Auto-detect</option>
          <option value="original">skhd (original)</option>
          <option value="zig">skhd.zig</option>
        </select>
      </div>

      <div class="variant-details">
        {#if effectiveVariant?.warning}
          <strong>{effectiveVariant.warning}</strong>
        {/if}
        {#if effectiveVariant?.variant === 'zig'}
          <p><code>brew install --cask jackielii/tap/skhd-zig</code></p>
          <p>
            skhd.zig requires macOS 13 or later. It uses <code>/Applications/skhd.app</code>,
            SMAppService label
            <code>com.jackielii.skhd</code>, and <code>~/Library/Logs/skhd.log</code>. It does not
            use
            <code>brew services</code>.
          </p>
          <button onclick={handleInstallService} disabled={changingVariant}
            >Register or Repair Service</button
          >
          <small>
            Configurations with advanced <code>.remap</code> rules must run
            <code>skhd --install-service</code> in Terminal to review privileged helper setup.
          </small>
        {:else}
          <p><code>brew install koekeishiya/formulae/skhd</code></p>
          <p>Classic skhd continues to use its per-user LaunchAgent and Homebrew service flow.</p>
        {/if}
        {#if variantFeedback}
          <p class="variant-feedback" role="status">{variantFeedback}</p>
        {/if}
      </div>
    </section>

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
            $updateState.phase === 'ready' ||
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

  .settings-card + .settings-card {
    margin-top: 20px;
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

  select {
    min-width: 160px;
    padding: 7px 9px;
    color: var(--color-text);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 6px;
  }

  .variant-details {
    padding: 16px;
    color: var(--color-text-secondary);
    font-size: 12px;
    line-height: 1.5;
  }

  .variant-details p + p,
  .variant-details button,
  .variant-details small {
    margin-top: 10px;
  }

  .variant-details small {
    display: block;
  }

  .variant-feedback {
    color: var(--color-status-warning);
    white-space: pre-line;
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
