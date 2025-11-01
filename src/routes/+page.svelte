<script lang="ts">
  import { onMount } from 'svelte';
  import {
    loadConfig,
    saveConfig,
    createShortcut as createShortcutAPI,
    updateShortcut as updateShortcutAPI,
    deleteShortcut as deleteShortcutAPI,
    testShortcut as testShortcutAPI,
  } from '../services/tauri';
  import type { ConfigFile, Shortcut, CreateShortcutRequest, UpdateShortcutRequest, TestResult } from '../types';
  import ShortcutList from '../components/ShortcutList.svelte';
  import ShortcutForm from '../components/ShortcutForm.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import ErrorDisplay from '../components/ErrorDisplay.svelte';
  import Modal from '../components/Modal.svelte';
  import TestResultDisplay from '../components/TestResultDisplay.svelte';

  let config = $state<ConfigFile | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let showForm = $state(false);
  let editingShortcut = $state<Shortcut | null>(null);
  let testResult = $state<TestResult | null>(null);
  let showTestResult = $state(false);

  onMount(async () => {
    await loadConfiguration();
  });

  async function loadConfiguration() {
    try {
      loading = true;
      error = null;
      config = await loadConfig();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to load config:', err);
    } finally {
      loading = false;
    }
  }

  async function saveConfiguration() {
    if (!config) return;

    try {
      await saveConfig(config);
      config.is_modified = false;
      config = config; // Trigger reactivity
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to save config:', err);
    }
  }

  function handleCreate() {
    editingShortcut = null;
    showForm = true;
  }

  function handleEdit(shortcut: Shortcut) {
    editingShortcut = shortcut;
    showForm = true;
  }

  async function handleDelete(id: string) {
    if (!confirm('Are you sure you want to delete this shortcut?')) {
      return;
    }

    try {
      await deleteShortcutAPI(id);
      await loadConfiguration(); // Reload to get fresh state
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to delete shortcut:', err);
    }
  }

  async function handleSaveShortcut(data: CreateShortcutRequest & { id?: string }) {
    try {
      if (data.id) {
        // Update existing
        const updateData: UpdateShortcutRequest = {
          id: data.id,
          modifiers: data.modifiers,
          key: data.key,
          command: data.command,
          mode: data.mode,
          comment: data.comment,
        };
        await updateShortcutAPI(updateData);
      } else {
        // Create new
        await createShortcutAPI(data);
      }

      showForm = false;
      editingShortcut = null;
      await loadConfiguration(); // Reload to get fresh state
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : String(err));
    }
  }

  function handleCancelForm() {
    showForm = false;
    editingShortcut = null;
  }

  async function handleTest(id: string) {
    try {
      const result = await testShortcutAPI(id);
      testResult = result;
      showTestResult = true;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to test shortcut:', err);
    }
  }

  function handleCloseTestResult() {
    showTestResult = false;
    testResult = null;
  }
</script>

<svelte:head>
  <title>skhd Configuration Manager</title>
</svelte:head>

<main class="app-container">
  <header class="app-header">
    <h1>skhd Configuration Manager</h1>
    <div class="header-actions">
      {#if config && config.shortcuts.length > 0}
        <button class="btn-create" onclick={handleCreate}>
          + New Shortcut
        </button>
      {/if}
      {#if config && config.is_modified}
        <button class="btn-save" onclick={saveConfiguration}>
          Save Changes
        </button>
      {/if}
      <button class="btn-reload" onclick={loadConfiguration} disabled={loading}>
        {loading ? 'Loading...' : 'Reload'}
      </button>
    </div>
  </header>

  <div class="app-content">
    {#if loading}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading configuration...</p>
      </div>
    {:else if error}
      <div class="error-state">
        <h2>Error Loading Configuration</h2>
        <p>{error}</p>
        <button class="btn-retry" onclick={loadConfiguration}>
          Try Again
        </button>
      </div>
    {:else if config}
      {#if config.parse_errors.length > 0}
        <ErrorDisplay errors={config.parse_errors} />
      {/if}

      {#if config.shortcuts.length === 0}
        <EmptyState onCreateFirst={handleCreate} />
      {:else}
        <ShortcutList
          shortcuts={config.shortcuts}
          onEdit={handleEdit}
          onDelete={handleDelete}
          onTest={handleTest}
        />
      {/if}

      {#if config.is_modified}
        <div class="modified-indicator">
          Configuration has unsaved changes
        </div>
      {/if}
    {/if}
  </div>

  <Modal open={showForm} onClose={handleCancelForm}>
    <ShortcutForm
      shortcut={editingShortcut}
      onSave={handleSaveShortcut}
      onCancel={handleCancelForm}
    />
  </Modal>

  <Modal open={showTestResult} onClose={handleCloseTestResult}>
    {#if testResult}
      <TestResultDisplay result={testResult} onClose={handleCloseTestResult} />
    {/if}
  </Modal>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
      'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
      sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    background: #f5f5f7;
    color: #1d1d1f;
  }

  .app-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .app-header {
    background: white;
    border-bottom: 1px solid #e0e0e0;
    padding: 1.5rem 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h1 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 600;
    color: #1d1d1f;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  button {
    padding: 0.5rem 1.25rem;
    border: 1px solid #d2d2d7;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-reload {
    background: #f5f5f7;
    color: #1d1d1f;
  }

  .btn-reload:hover:not(:disabled) {
    background: #e8e8ed;
  }

  .btn-reload:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-create {
    background: #007aff;
    color: white;
    border-color: #007aff;
  }

  .btn-create:hover {
    background: #0051d5;
    border-color: #0051d5;
  }

  .btn-save {
    background: #34c759;
    color: white;
    border-color: #34c759;
  }

  .btn-save:hover {
    background: #28a745;
    border-color: #28a745;
  }

  .app-content {
    flex: 1;
    padding: 2rem;
    max-width: 1400px;
    width: 100%;
    margin: 0 auto;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    gap: 1rem;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #f3f3f3;
    border-top: 3px solid #007aff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .loading-state p {
    color: #666;
    margin: 0;
  }

  .error-state {
    text-align: center;
    padding: 3rem 2rem;
  }

  .error-state h2 {
    color: #ff3b30;
    margin-bottom: 1rem;
  }

  .error-state p {
    color: #666;
    margin-bottom: 2rem;
  }

  .btn-retry {
    padding: 0.75rem 2rem;
    background: #007aff;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-retry:hover {
    background: #0051d5;
  }

  .modified-indicator {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    background: #ff9500;
    color: white;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: #1d1d1f;
      color: #f5f5f7;
    }

    .app-header {
      background: #2a2a2a;
      border-bottom-color: #3a3a3a;
    }

    h1 {
      color: #f5f5f7;
    }

    .btn-reload {
      background: #3a3a3a;
      border-color: #4a4a4a;
      color: #f5f5f7;
    }

    .btn-reload:hover:not(:disabled) {
      background: #4a4a4a;
    }

    .loading-state p,
    .error-state p {
      color: #999;
    }

    .spinner {
      border-color: #3a3a3a;
      border-top-color: #007aff;
    }
  }
</style>
