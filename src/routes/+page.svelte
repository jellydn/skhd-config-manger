<script lang="ts">
  import { onMount } from 'svelte';
  import {
    detectActiveConfig,
    loadConfig,
    saveConfig,
    importConfig,
    exportConfig,
    createShortcut as createShortcutAPI,
    updateShortcut as updateShortcutAPI,
    deleteShortcut as deleteShortcutAPI,
    testShortcut as testShortcutAPI,
  } from '../services/tauri';
  import type {
    ConfigFile,
    Shortcut,
    CreateShortcutRequest,
    UpdateShortcutRequest,
    TestResult,
  } from '../types';
  import ShortcutList from '../components/ShortcutList.svelte';
  import ShortcutForm from '../components/ShortcutForm.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import ErrorDisplay from '../components/ErrorDisplay.svelte';
  import Modal from '../components/Modal.svelte';
  import TestResultDisplay from '../components/TestResultDisplay.svelte';
  import ConfirmDialog from '../components/ConfirmDialog.svelte';

  let config = $state<ConfigFile | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let showForm = $state(false);
  let editingShortcut = $state<Shortcut | null>(null);
  let testResult = $state<TestResult | null>(null);
  let showTestResult = $state(false);
  let showReloadConfirm = $state(false);
  let showDeleteConfirm = $state(false);
  let deletingShortcutId = $state<string | null>(null);

  // Don't auto-load - let user choose which file to open
  onMount(() => {
    loading = false;
  });

  async function handleDetectAndLoad() {
    try {
      loading = true;
      error = null;
      const activePath = await detectActiveConfig();
      config = await loadConfig(activePath);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to detect/load active config:', err);
    } finally {
      loading = false;
    }
  }

  async function handleCreateNew() {
    try {
      loading = true;
      error = null;
      // Load/create empty config at default location
      config = await loadConfig();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to create new config:', err);
    } finally {
      loading = false;
    }
  }

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

  async function handleImport() {
    try {
      loading = true;
      error = null;
      config = await importConfig();
    } catch (err) {
      // User cancellation is not an error to display
      if (err instanceof Error && err.message === 'Import cancelled') {
        console.log('Import cancelled by user');
      } else {
        error = err instanceof Error ? err.message : String(err);
        console.error('Failed to import config:', err);
      }
    } finally {
      loading = false;
    }
  }

  async function handleExport() {
    if (!config) return;

    try {
      const exportPath = await exportConfig();
      console.log('Configuration exported to:', exportPath);
      // Optionally show success notification to user
    } catch (err) {
      // User cancellation is not an error to display
      if (err instanceof Error && err.message === 'Export cancelled') {
        console.log('Export cancelled by user');
      } else {
        error = err instanceof Error ? err.message : String(err);
        console.error('Failed to export config:', err);
      }
    }
  }

  async function saveConfiguration() {
    if (!config) return;

    try {
      await saveConfig(config);
      // Update local state - create new config object to trigger reactivity
      config = {
        ...config,
        is_modified: false
      };
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

  function handleDelete(id: string) {
    console.log('handleDelete called with id:', id);
    deletingShortcutId = id;
    showDeleteConfirm = true;
  }

  async function confirmDelete() {
    showDeleteConfirm = false;
    const id = deletingShortcutId;
    deletingShortcutId = null;

    if (!id) return;

    try {
      console.log('Calling deleteShortcutAPI...');
      await deleteShortcutAPI(id);
      console.log('Delete API call successful');

      // Update local state - create new config object to trigger reactivity
      if (config) {
        const beforeCount = config.shortcuts.length;
        config = {
          ...config,
          shortcuts: config.shortcuts.filter(s => s.id !== id),
          is_modified: true
        };
        const afterCount = config.shortcuts.length;
        console.log(`Shortcuts count: ${beforeCount} -> ${afterCount}`);
      }
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to delete shortcut:', err);
    }
  }

  function cancelDelete() {
    showDeleteConfirm = false;
    deletingShortcutId = null;
    console.log('Delete cancelled by user');
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
        const updated = await updateShortcutAPI(updateData);

        // Update local state - create new config object to trigger reactivity
        if (config) {
          const index = config.shortcuts.findIndex(s => s.id === data.id);
          if (index !== -1) {
            const newShortcuts = [...config.shortcuts];
            newShortcuts[index] = updated;
            config = {
              ...config,
              shortcuts: newShortcuts,
              is_modified: true
            };
          }
        }
      } else {
        // Create new
        const newShortcut = await createShortcutAPI(data);

        // Update local state - create new config object to trigger reactivity
        if (config) {
          config = {
            ...config,
            shortcuts: [...config.shortcuts, newShortcut],
            is_modified: true
          };
        }
      }

      showForm = false;
      editingShortcut = null;
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

  function handleHomeClick() {
    // Check if there are unsaved changes
    if (config && config.is_modified) {
      showReloadConfirm = true;
    } else {
      goToWelcome();
    }
  }

  function goToWelcome() {
    showReloadConfirm = false;
    config = null;
    error = null;
  }

  function cancelHome() {
    showReloadConfirm = false;
  }
</script>

<svelte:head>
  <title>skhd Configuration Manager</title>
</svelte:head>

<main class="app-container">
  <header class="app-header">
    <h1>skhd Configuration Manager</h1>
    <div class="header-actions">
      <button class="btn-home" onclick={handleHomeClick} disabled={loading}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
          <polyline points="9 22 9 12 15 12 15 22" />
        </svg>
        Home
      </button>
      <button class="btn-import" onclick={handleImport} disabled={loading}> Import... </button>
      {#if config}
        <button class="btn-export" onclick={handleExport}> Export... </button>
      {/if}
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
        <h2>Configuration File Not Found</h2>
        <p class="error-message">{error}</p>

        <div class="error-help">
          <h3>skhd looks for configuration in these locations (in order):</h3>
          <ul>
            <li><code>$XDG_CONFIG_HOME/skhd/skhdrc</code></li>
            <li><code>~/.config/skhd/skhdrc</code></li>
            <li><code>~/.skhdrc</code></li>
          </ul>
        </div>

        <div class="error-actions">
          <button class="btn-create-new" onclick={handleCreateNew}>
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 4v16m8-8H4" />
            </svg>
            Create New Config
          </button>
          <button class="btn-import" onclick={handleImport}>
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            Import Existing File
          </button>
          <button class="btn-retry" onclick={handleDetectAndLoad}>
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Try Again
          </button>
        </div>
      </div>
    {:else if !config}
      <div class="welcome-state">
        <h2>Welcome to skhd Configuration Manager</h2>
        <p>Choose how you'd like to get started:</p>

        <div class="welcome-actions">
          <button class="welcome-btn welcome-btn-primary" onclick={handleDetectAndLoad}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <div>
              <h3>Detect Active Config</h3>
              <p>Open the config file that skhd is currently using</p>
            </div>
          </button>

          <button class="welcome-btn" onclick={handleImport}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <div>
              <h3>Import Existing File</h3>
              <p>Browse and open any skhd configuration file</p>
            </div>
          </button>

          <button class="welcome-btn" onclick={loadConfiguration}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 4v16m8-8H4" />
            </svg>
            <div>
              <h3>Create New Config</h3>
              <p>Start with an empty configuration file</p>
            </div>
          </button>
        </div>
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
          onCreate={handleCreate}
          onSave={saveConfiguration}
          isModified={config.is_modified}
        />
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

  <ConfirmDialog
    open={showReloadConfirm}
    title="Discard Unsaved Changes?"
    message="You have unsaved changes. Going home will discard all your changes. Are you sure you want to continue?"
    confirmLabel="Discard and Go Home"
    cancelLabel="Cancel"
    onConfirm={goToWelcome}
    onCancel={cancelHome}
  />

  <ConfirmDialog
    open={showDeleteConfirm}
    title="Delete Shortcut?"
    message="Are you sure you want to delete this shortcut? This action cannot be undone."
    confirmLabel="Delete"
    cancelLabel="Cancel"
    onConfirm={confirmDelete}
    onCancel={cancelDelete}
  />
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family:
      -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell',
      'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
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

  .btn-home {
    background: #f5f5f7;
    color: #1d1d1f;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .btn-home:hover:not(:disabled) {
    background: #e8e8ed;
  }

  .btn-home:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-home svg {
    flex-shrink: 0;
  }

  .btn-import {
    background: #5856d6;
    color: white;
    border-color: #5856d6;
  }

  .btn-import:hover:not(:disabled) {
    background: #4240a8;
    border-color: #4240a8;
  }

  .btn-import:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-export {
    background: #ff9500;
    color: white;
    border-color: #ff9500;
  }

  .btn-export:hover {
    background: #d17e00;
    border-color: #d17e00;
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

  .btn-save:hover:not(:disabled) {
    background: #28a745;
    border-color: #28a745;
  }

  .btn-save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .loading-state p {
    color: #666;
    margin: 0;
  }

  .error-state {
    max-width: 800px;
    margin: 0 auto;
    padding: 3rem 2rem;
  }

  .error-state h2 {
    color: #ff3b30;
    margin-bottom: 1rem;
    text-align: center;
  }

  .error-message {
    color: #666;
    margin-bottom: 2rem;
    text-align: center;
    font-size: 0.95rem;
  }

  .error-help {
    background: #f8f8f8;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
    text-align: left;
  }

  .error-help h3 {
    font-size: 1rem;
    margin: 0 0 1rem 0;
    color: #1d1d1f;
  }

  .error-help ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .error-help li {
    padding: 0.5rem 0;
    color: #666;
  }

  .error-help code {
    background: #e8e8e8;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
    color: #2c3e50;
  }

  .error-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .btn-create-new {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: #34c759;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-create-new:hover {
    background: #28a745;
  }

  .btn-create-new svg {
    flex-shrink: 0;
  }

  .btn-retry {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: #f5f5f7;
    color: #1d1d1f;
    border: 1px solid #d2d2d7;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-retry:hover {
    background: #e8e8ed;
  }

  .btn-retry svg {
    flex-shrink: 0;
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

    .btn-home {
      background: #3a3a3a;
      border-color: #4a4a4a;
      color: #f5f5f7;
    }

    .btn-home:hover:not(:disabled) {
      background: #4a4a4a;
    }

    .loading-state p,
    .error-message {
      color: #999;
    }

    .spinner {
      border-color: #3a3a3a;
      border-top-color: #007aff;
    }

    .error-help {
      background: #2a2a2a;
      border-color: #4a4a4a;
    }

    .error-help h3 {
      color: #f5f5f7;
    }

    .error-help li {
      color: #aaa;
    }

    .error-help code {
      background: #3a3a3a;
      color: #e0e0e0;
    }

    .btn-retry {
      background: #3a3a3a;
      border-color: #4a4a4a;
      color: #f5f5f7;
    }

    .btn-retry:hover {
      background: #4a4a4a;
    }
  }

  .welcome-state {
    text-align: center;
    padding: 3rem 2rem;
    max-width: 900px;
    margin: 0 auto;
  }

  .welcome-state h2 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
    color: #1d1d1f;
  }

  .welcome-state > p {
    color: #666;
    margin-bottom: 3rem;
    font-size: 1.1rem;
  }

  .welcome-actions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 600px;
    margin: 0 auto;
  }

  .welcome-btn {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 1.5rem;
    background: white;
    border: 2px solid #e0e0e0;
    border-radius: 12px;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .welcome-btn:hover {
    border-color: #007aff;
    box-shadow: 0 4px 12px rgba(0, 122, 255, 0.15);
    transform: translateY(-2px);
  }

  .welcome-btn-primary {
    border-color: #007aff;
    background: #f0f7ff;
  }

  .welcome-btn-primary:hover {
    background: #e0f0ff;
    box-shadow: 0 6px 16px rgba(0, 122, 255, 0.25);
  }

  .welcome-btn svg {
    flex-shrink: 0;
    color: #007aff;
  }

  .welcome-btn div {
    flex: 1;
  }

  .welcome-btn h3 {
    margin: 0 0 0.25rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #1d1d1f;
  }

  .welcome-btn p {
    margin: 0;
    font-size: 0.9rem;
    color: #666;
  }

  @media (prefers-color-scheme: dark) {
    .welcome-state h2 {
      color: #f5f5f7;
    }

    .welcome-state > p {
      color: #999;
    }

    .welcome-btn {
      background: #2a2a2a;
      border-color: #4a4a4a;
    }

    .welcome-btn:hover {
      border-color: #007aff;
      background: #333;
    }

    .welcome-btn-primary {
      background: #1a2a3a;
      border-color: #007aff;
    }

    .welcome-btn-primary:hover {
      background: #1f3545;
    }

    .welcome-btn h3 {
      color: #f5f5f7;
    }

    .welcome-btn p {
      color: #999;
    }
  }
</style>
