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
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="modal-backdrop"
  onclick={handleBackdropClick}
  role="dialog"
  aria-modal="true"
  aria-label="Application Picker"
>
  <div class="modal-dialog" onclick={(e) => e.stopPropagation()}>
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
        autofocus
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
                    {#if filteredApps.filter((a) => a.display_name === app.display_name).length > 1}
                      <span class="app-path" title={app.app_path}>
                        {app.app_path.split('/').slice(-2, -1)[0]}
                      </span>
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
    background: rgba(0, 0, 0, 0.5);
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
    background: white;
    border-radius: 12px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.2s ease-out;
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
    border-bottom: 1px solid #e5e7eb;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #111827;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: #6b7280;
    cursor: pointer;
    padding: 0.25rem;
    line-height: 1;
    transition: color 0.15s;
  }

  .close-btn:hover {
    color: #111827;
  }

  .modal-body {
    padding: 1.5rem;
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .search-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    font-size: 0.875rem;
    transition: border-color 0.15s, box-shadow 0.15s;
  }

  .search-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
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
    color: #6b7280;
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #e5e7eb;
    border-top-color: #3b82f6;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error {
    color: #dc2626;
  }

  .hint {
    font-size: 0.875rem;
    color: #9ca3af;
  }

  .app-list {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
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
    padding: 0.75rem;
    background: none;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.15s;
    text-align: left;
  }

  .app-button:hover {
    background: #f3f4f6;
  }

  .app-button:active {
    background: #e5e7eb;
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
    color: #111827;
    font-size: 0.875rem;
  }

  .app-meta {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .app-version {
    color: #6b7280;
    font-size: 0.75rem;
  }

  .app-path {
    color: #9ca3af;
    font-size: 0.7rem;
    font-style: italic;
  }

  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid #e5e7eb;
    display: flex;
    justify-content: flex-end;
  }

  .cancel-btn {
    padding: 0.5rem 1rem;
    background: #f3f4f6;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    cursor: pointer;
    transition: background-color 0.15s, border-color 0.15s;
  }

  .cancel-btn:hover {
    background: #e5e7eb;
    border-color: #9ca3af;
  }

  @media (prefers-color-scheme: dark) {
    .modal-backdrop {
      background: rgba(0, 0, 0, 0.7);
    }

    .modal-dialog {
      background: #1f2937;
    }

    .modal-header {
      border-bottom-color: #374151;
    }

    .modal-header h2 {
      color: #f9fafb;
    }

    .close-btn {
      color: #9ca3af;
    }

    .close-btn:hover {
      color: #f9fafb;
    }

    .search-input {
      background: #111827;
      border-color: #374151;
      color: #f9fafb;
    }

    .search-input:focus {
      border-color: #3b82f6;
      box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
    }

    .spinner {
      border-color: #374151;
      border-top-color: #3b82f6;
    }

    .app-button:hover {
      background: #374151;
    }

    .app-button:active {
      background: #4b5563;
    }

    .app-name {
      color: #f9fafb;
    }

    .app-version {
      color: #9ca3af;
    }

    .app-path {
      color: #6b7280;
    }

    .modal-footer {
      border-top-color: #374151;
    }

    .cancel-btn {
      background: #374151;
      border-color: #4b5563;
      color: #f9fafb;
    }

    .cancel-btn:hover {
      background: #4b5563;
      border-color: #6b7280;
    }
  }
</style>
