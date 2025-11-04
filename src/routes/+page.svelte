<script lang="ts">
  import { onMount } from 'svelte';
  import {
    detectActiveConfig,
    loadConfig,
    saveConfig,
    saveAsConfig,
    importConfig,
    exportConfig,
    createShortcut as createShortcutAPI,
    updateShortcut as updateShortcutAPI,
    deleteShortcut as deleteShortcutAPI,
    testShortcut as testShortcutAPI,
    executeShortcutCommand,
    cancelShortcutExecution,
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
      // Check if this is a blank config without a file path
      if (!config.file_path || config.file_path.trim() === '') {
        // Use "Save As" dialog to let user choose location
        const updatedConfig = await saveAsConfig(config);
        // Update local state with the new file path and saved status
        config = updatedConfig;
      } else {
        // Normal save to existing file path
        await saveConfig(config);
        // Update local state - create new config object to trigger reactivity
        config = {
          ...config,
          is_modified: false
        };
      }
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

  async function handleCancelExecution(id: string) {
    try {
      await cancelShortcutExecution(id);
      executingShortcutId = null;
      console.log('Execution cancelled successfully');
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to cancel execution:', err);
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
  <title>Shortcuts - Keybinder</title>
</svelte:head>

<div class="page-container">
  <!-- Toolbar -->
  <header class="toolbar">
    <div class="toolbar-title">
      <h1>Shortcuts</h1>
      {#if config}
        <span class="toolbar-subtitle">{config.shortcuts.length} shortcuts</span>
      {/if}
    </div>
    <div class="toolbar-actions">
      {#if config}
        <button class="toolbar-btn toolbar-btn-save" onclick={saveConfiguration} disabled={!config.is_modified} aria-label="Save changes to configuration">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
          Save Changes
        </button>
        <button class="toolbar-btn toolbar-btn-primary" onclick={handleCreate} aria-label="Create new shortcut">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
          New Shortcut
        </button>
        <button class="toolbar-btn" onclick={handleExport} aria-label="Export configuration">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M12 15V3" />
          </svg>
          Export
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
        <div class="error-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
        </div>

        <h2>No Configuration Found</h2>
        <p class="error-subtitle">Choose an option below to get started</p>

        <div class="config-locations">
          <div class="locations-header">skhd looks for configuration in these locations:</div>
          <div class="location-item"><code>$XDG_CONFIG_HOME/skhd/skhdrc</code></div>
          <div class="location-item"><code>~/.config/skhd/skhdrc</code></div>
          <div class="location-item"><code>~/.skhdrc</code></div>
        </div>

        <div class="error-actions">
          <button class="action-btn action-btn-primary" onclick={handleCreateBlank}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 4v16m8-8H4" />
            </svg>
            Create New Config
          </button>
          <button class="action-btn action-btn-secondary" onclick={handleImport}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
          executingShortcutId={executingShortcutId}
          onCancelExecution={handleCancelExecution}
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
    title="?? DANGER: Potentially Destructive Command!"
    message={pendingDestructiveCommand ? `Command: ${pendingDestructiveCommand.command}\n\nThis command may:\n? Delete important files or directories\n? Modify system files\n? Terminate critical processes\n? Cause data loss or system instability\n\nAre you ABSOLUTELY SURE you want to execute this command?` : ''}
    confirmLabel="Execute Anyway"
    cancelLabel="Cancel"
    variant="danger"
    onConfirm={confirmDestructiveExecution}
    onCancel={cancelDestructiveExecution}
  />
</div>

<style>
  .page-container {
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

  .toolbar-title {
    display: flex;
    align-items: baseline;
    gap: 12px;
  }

  .toolbar-title h1 {
    font-size: 15px;
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
  }

  .toolbar-subtitle {
    font-size: 12px;
    color: var(--color-text-secondary);
    font-weight: 400;
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
    opacity: 0.8;
  }

  .toolbar-btn:hover svg {
    opacity: 1;
  }

  .toolbar-btn-save {
    background: var(--color-button-success-bg);
    border-color: var(--color-button-success-border);
    color: var(--color-button-success-text);
  }

  .toolbar-btn-save:hover:not(:disabled) {
    background: var(--color-button-success-hover-bg);
    border-color: var(--color-button-success-hover-border);
  }

  .toolbar-btn-save:active:not(:disabled) {
    background: var(--color-button-success-hover-bg);
    opacity: 0.9;
  }

  .toolbar-btn-save:focus-visible {
    outline: 2px solid var(--color-button-success-border);
    outline-offset: 2px;
  }

  .toolbar-btn-save:disabled {
    background: var(--color-button-disabled-bg);
    color: var(--color-button-disabled-text);
    border-color: var(--color-button-disabled-border);
    cursor: not-allowed;
    opacity: 0.6;
  }

  .toolbar-btn-primary {
    background: var(--color-button-primary-bg);
    border-color: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
  }

  .toolbar-btn-primary:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
    color: var(--color-button-primary-text);
  }

  .toolbar-btn-primary:active:not(:disabled) {
    background: var(--color-button-primary-active);
    border-color: var(--color-button-primary-active);
    color: var(--color-button-primary-text);
  }

  .toolbar-btn-primary:focus-visible {
    outline: 2px solid var(--color-button-primary-focus);
    outline-offset: 2px;
  }

  .app-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 20px;
    background: var(--color-background);
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 300px;
    gap: 16px;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--color-border);
    border-top: 3px solid var(--color-button-primary-bg);
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
    color: var(--color-text-secondary);
    margin: 0;
    font-size: 13px;
  }

  .error-state {
    max-width: 600px;
    margin: 0 auto;
    padding: 4rem 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .error-icon {
    margin-bottom: 1.5rem;
    color: var(--color-text-tertiary);
  }

  .error-state h2 {
    color: var(--color-text);
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
    font-weight: 600;
  }

  .error-subtitle {
    color: var(--color-text-secondary);
    margin: 0 0 2rem 0;
    font-size: 0.9rem;
  }

  .config-locations {
    width: 100%;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 1.25rem;
    margin-bottom: 2rem;
    text-align: left;
  }

  .locations-header {
    font-size: 0.85rem;
    color: var(--color-text-secondary);
    margin-bottom: 1rem;
    font-weight: 500;
  }

  .location-item {
    padding: 0.5rem 0;
  }

  .location-item code {
    background: var(--color-surface);
    padding: 0.375rem 0.625rem;
    border-radius: 4px;
    font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
    font-size: 0.8rem;
    color: var(--color-text);
    border: 1px solid var(--color-border);
  }

  .error-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    border: none;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn svg {
    flex-shrink: 0;
  }

  .action-btn-primary {
    background: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
    border: 1px solid var(--color-button-primary-bg);
  }

  .action-btn-primary:hover {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .action-btn-secondary {
    background: var(--color-button-secondary-bg);
    color: var(--color-button-secondary-text);
    border: 1px solid var(--color-button-secondary-border);
  }

  .action-btn-secondary:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
  }

  .welcome-state {
    padding: 60px 40px;
    max-width: 600px;
    margin: 0 auto;
  }

  .welcome-state h2 {
    font-size: 22px;
    margin-bottom: 8px;
    color: var(--color-text);
    font-weight: 600;
  }

  .welcome-state > p {
    color: var(--color-text-secondary);
    margin-bottom: 32px;
    font-size: 14px;
  }

  .welcome-actions {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .welcome-btn {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 10px;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .welcome-btn:hover {
    background: var(--color-surface-secondary);
    border-color: var(--color-border-hover);
    transform: translateY(-1px);
  }

  .welcome-btn-primary {
    border-color: var(--color-border-hover);
    background: var(--color-surface-secondary);
  }

  .welcome-btn-primary:hover {
    background: var(--color-surface-secondary);
    border-color: var(--color-border-hover);
  }

  .welcome-btn svg {
    flex-shrink: 0;
    color: var(--color-border-hover);
    align-self: flex-start;
  }

  .welcome-btn div {
    flex: 1;
  }

  .welcome-btn h3 {
    margin: 0 0 4px 0;
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text);
  }

  .welcome-btn p {
    margin: 0;
    font-size: 12px;
    color: var(--color-text-secondary);
  }
</style>
