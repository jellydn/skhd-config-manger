<script lang="ts">
  import { applicationService } from '../../services/applicationService';
  import type { Application } from '../../types';

  interface Props {
    onSelect: (command: string) => void;
    onCancel: () => void;
  }

  let { onSelect, onCancel }: Props = $props();

  let applications = $state<Application[]>([]);
  let searchQuery = $state('');
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    applicationService
      .listApplications()
      .then((apps) => {
        applications = apps;
        loading = false;
      })
      .catch((err) => {
        console.error('Failed to load applications:', err);
        error = err.toString();
        loading = false;
      });
  });

  const filteredApps = $derived(
    applicationService.searchApplications(applications, searchQuery)
  );

  function handleSelect(app: Application) {
    const command = applicationService.generateLaunchCommand(app);
    onSelect(command);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onCancel();
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onCancel();
    }
  }

  function handleDialogClick(event: MouseEvent) {
    event.stopPropagation();
  }

  function handleDialogKeydown(event: KeyboardEvent) {
    event.stopPropagation();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="modal-backdrop"
  onclick={handleBackdropClick}
  onkeydown={(e) => e.key === 'Enter' && handleBackdropClick(e as unknown as MouseEvent)}
  role="dialog"
  aria-modal="true"
  aria-label="Application Picker"
  tabindex="-1"
>
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <div class="modal-dialog" onclick={handleDialogClick} onkeydown={handleDialogKeydown} role="document">
    <div class="modal-header">
      <h2>Select Application</h2>
      <button class="close-btn" onclick={onCancel} aria-label="Close">✕</button>
    </div>

    <div class="modal-body">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search applications..."
        class="search-input"
      />

      {#if loading}
        <div class="loading">
          <div class="spinner"></div>
          <p>Loading applications...</p>
        </div>
      {:else if error}
        <div class="error">
          <p>Failed to load applications: {error}</p>
        </div>
      {:else if filteredApps.length === 0}
        <div class="empty">
          <p>No applications found</p>
          {#if searchQuery}
            <p class="hint">Try a different search term</p>
          {/if}
        </div>
      {:else}
        <ul class="app-list">
          {#each filteredApps as app (app.bundle_id + app.app_path)}
            <li class="app-item">
              <button class="app-button" onclick={() => handleSelect(app)}>
                <div class="app-icon-placeholder">📱</div>
                <div class="app-info">
                  <span class="app-name">{app.display_name}</span>
                  <div class="app-meta">
                    {#if app.version}
                      <span class="app-version">v{app.version}</span>
                    {/if}
                  </div>
                </div>
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="modal-footer">
      <button class="cancel-btn" onclick={onCancel}>Cancel</button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--color-modal-backdrop);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
    backdrop-filter: blur(4px);
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .modal-dialog {
    background: var(--color-modal-bg);
    border-radius: 12px;
    box-shadow: 0 20px 60px var(--color-form-shadow);
    width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.2s ease-out;
    border: 1px solid var(--color-modal-border);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
    border-bottom: 1px solid var(--color-border);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: 0.25rem;
    line-height: 1;
    transition: color 0.15s;
  }

  .close-btn:hover {
    color: var(--color-text);
  }

  .modal-body {
    padding: 1.5rem;
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .search-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--color-input-border);
    border-radius: 8px;
    font-size: 0.875rem;
    transition: border-color 0.15s, box-shadow 0.15s;
    margin-bottom: 1rem;
    box-sizing: border-box;
    background: var(--color-input-bg);
    color: var(--color-text);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--color-input-focus-border);
    box-shadow: 0 0 0 3px var(--color-input-focus-shadow);
    background: var(--color-input-focus-bg);
  }

  .loading,
  .error,
  .empty {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem 1rem;
    color: var(--color-text-secondary);
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-input-focus-border);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error {
    color: var(--color-text);
  }

  .hint {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
  }

  .app-list {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .app-list::-webkit-scrollbar {
    width: 12px;
  }

  .app-list::-webkit-scrollbar-track {
    background: var(--color-scrollbar-track);
    border-radius: 0;
  }

  .app-list::-webkit-scrollbar-thumb {
    background: var(--color-scrollbar-thumb);
    border-radius: 6px;
    border: 2px solid var(--color-scrollbar-track);
  }

  .app-list::-webkit-scrollbar-thumb:hover {
    background: var(--color-scrollbar-thumb-hover);
  }

  .app-item {
    margin: 0;
    padding: 0;
  }

  .app-button {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
  }

  .app-button:hover {
    background: var(--color-surface-secondary);
    border-color: var(--color-input-focus-border);
  }

  .app-button:active {
    background: var(--color-input-focus-bg);
  }

  .app-icon-placeholder {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    flex-shrink: 0;
  }

  .app-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .app-name {
    font-weight: 500;
    color: var(--color-text);
    font-size: 0.875rem;
  }

  .app-meta {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .app-version {
    color: var(--color-text-secondary);
    font-size: 0.75rem;
  }


  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--color-border);
    display: flex;
    justify-content: flex-end;
  }

  .cancel-btn {
    padding: 0.5rem 1rem;
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-button-secondary-text);
    cursor: pointer;
    transition: background-color 0.15s, border-color 0.15s;
  }

  .cancel-btn:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
  }
</style>
