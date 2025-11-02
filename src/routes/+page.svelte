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
    executeShortcutCommand,
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
  let editingShortcut = $state<Shortcut | undefined>(undefined);
  let formMode = $state<'create' | 'edit' | 'duplicate'>('create');
  let testResult = $state<TestResult | null>(null);
  let showTestResult = $state(false);
  let showReloadConfirm = $state(false);
  let showDeleteConfirm = $state(false);
  let deletingShortcutId = $state<string | null>(null);
  let executingShortcutId = $state<string | null>(null);
  let showDestructiveWarning = $state(false);
  let pendingDestructiveCommand = $state<{ shortcutId: string; command: string } | null>(null);

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

  async function handleCreateBlank() {
    try {
      loading = true;
      error = null;
      // Create a truly blank configuration without auto-detection
      // This always creates an empty config regardless of existing files
      config = {
        file_path: '', // No path yet - user will choose on first save
        shortcuts: [],
        global_comments: [],
        parse_errors: [],
        last_modified: new Date().toISOString(),
        is_modified: true, // Mark as modified so user can save with location choice
        current_file_path: '' // No current file path yet - will be set on first save
      };
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to create blank config:', err);
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

  function getNextLineNumber(): number {
    if (!config || config.shortcuts.length === 0) return 1;
    return Math.max(...config.shortcuts.map(s => s.line_number)) + 1;
  }

  function handleCreate() {
    editingShortcut = undefined;
    formMode = 'create';
    showForm = true;
  }

  function handleEdit(shortcut: Shortcut) {
    editingShortcut = shortcut;
    formMode = 'edit';
    showForm = true;
  }

  function handleDuplicate(shortcut: Shortcut) {
    editingShortcut = {
      ...shortcut,
      id: crypto.randomUUID(),
      line_number: getNextLineNumber()
    };
    formMode = 'duplicate';
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
      editingShortcut = undefined;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : String(err));
    }
  }

  function handleCancelForm() {
    showForm = false;
    editingShortcut = undefined;
  }

  // Check if command is potentially destructive
  function isDestructiveCommand(command: string): boolean {
    const destructivePatterns = [
      // Recursive delete
      /rm\s+-rf/i,
      /rm\s+-fr/i,
      /rm\s+.*-r/i,

      // rm with critical paths
      /\brm\b.*\s+\/\s*$/,           // rm / (root) - catches "rm /" at end of string
      /\brm\b.*\s+\/\s+/,            // rm / with more args
      /\brm\b.*\/\*/,                // rm with wildcards in root
      /\brm\b.*~\//,                 // rm in home directory
      /\brm\b.*\/usr/i,
      /\brm\b.*\/etc/i,
      /\brm\b.*\/var/i,
      /\brm\b.*\/bin/i,
      /\brm\b.*\/sbin/i,
      /\brm\b.*\/System/i,           // macOS system folder
      /\brm\b.*\/Library/i,          // macOS library folder

      // Privileged commands
      /\bsudo\b/i,

      // Process killing
      /\bkillall\b/i,
      /\bpkill\b/i,
      /\bkill\b.*-9/,

      // Disk operations
      />\s*\/dev\//i,
      /mkfs/i,
      /dd\s+if=/i,
      /dd\s+of=/i,
      /format\s+/i,
      /diskutil.*erase/i,

      // Dangerous redirects
      />\s*\/etc\//i,
      />\s*\/usr\//i,
      />\s*\/var\//i,

      // Fork bomb and similar
      /:\(\)\{.*:\|:/,
      /\bwhile\s+true\b/i,

      // Chmod/chown on critical paths
      /chmod.*\/\s*$/,
      /chown.*\/\s*$/,
    ];

    return destructivePatterns.some(pattern => pattern.test(command));
  }

  async function handleTest(id: string) {
    // Find the shortcut to check if it's destructive
    const shortcut = config?.shortcuts.find(s => s.id === id);
    if (!shortcut) return;

    if (isDestructiveCommand(shortcut.command)) {
      // Show warning dialog instead of executing immediately
      pendingDestructiveCommand = { shortcutId: id, command: shortcut.command };
      showDestructiveWarning = true;
      return;
    }

    // Execute non-destructive commands immediately
    await executeCommand(id);
  }

  async function executeCommand(id: string) {
    try {
      executingShortcutId = id;
      const result = await executeShortcutCommand(id);
      testResult = result;
      showTestResult = true;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to execute shortcut:', err);
    } finally {
      executingShortcutId = null;
    }
  }

  function confirmDestructiveExecution() {
    if (pendingDestructiveCommand) {
      executeCommand(pendingDestructiveCommand.shortcutId);
    }
    showDestructiveWarning = false;
    pendingDestructiveCommand = null;
  }

  function cancelDestructiveExecution() {
    showDestructiveWarning = false;
    pendingDestructiveCommand = null;
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
  <title>Keybinder</title>
</svelte:head>

<main class="app-container">
  <header class="app-header">
    <h1>Keybinder</h1>
    <div class="header-actions">
      {#if config}
        <button class="btn-home" onclick={handleHomeClick} disabled={loading} aria-label="Return to home screen">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
            <polyline points="9 22 9 12 15 12 15 22" />
          </svg>
          Home
        </button>
        <button class="btn-export" onclick={handleExport} aria-label="Export configuration to file">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            <path d="M12 3v6m0 0l3-3m-3 3l-3-3" />
          </svg>
          Export...
        </button>
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
          <button class="btn-create-new" onclick={handleCreateBlank}>
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
        </div>
      </div>
    {:else if !config}
      <div class="welcome-state">
        <h2>Welcome to Keybinder</h2>
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

          <button class="welcome-btn" onclick={handleCreateBlank}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 4v16m8-8H4" />
            </svg>
            <div>
              <h3>Create Blank Config</h3>
              <p>Start with an empty configuration (never auto-detects existing configs)</p>
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
          onDuplicate={handleDuplicate}
          onCreate={handleCreate}
          onSave={saveConfiguration}
          isModified={config.is_modified}
          executingShortcutId={executingShortcutId}
        />
      {/if}
    {/if}
  </div>

  <Modal open={showForm} onClose={handleCancelForm}>
    <ShortcutForm
      shortcut={editingShortcut}
      mode={formMode}
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

  <ConfirmDialog
    open={showDestructiveWarning}
    title="⚠️ DANGER: Potentially Destructive Command!"
    message={pendingDestructiveCommand ? `Command: ${pendingDestructiveCommand.command}\n\nThis command may:\n• Delete important files or directories\n• Modify system files\n• Terminate critical processes\n• Cause data loss or system instability\n\nAre you ABSOLUTELY SURE you want to execute this command?` : ''}
    confirmLabel="Execute Anyway"
    cancelLabel="Cancel"
    variant="danger"
    onConfirm={confirmDestructiveExecution}
    onCancel={cancelDestructiveExecution}
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
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .btn-export:hover {
    background: #d17e00;
    border-color: #d17e00;
  }

  .btn-export svg {
    flex-shrink: 0;
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
    align-self: flex-start;
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
