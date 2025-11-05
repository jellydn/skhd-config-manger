<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  interface Props {
    onSelect: (command: string) => void;
    onCancel: () => void;
    startDirectory?: string;
  }

  let { onSelect, onCancel, startDirectory }: Props = $props();

  let loading = $state(false);
  let error = $state<string | null>(null);
  let selectedPath = $state<string | null>(null);
  let isExecutable = $state<boolean | null>(null);
  let interpreter = $state<string | null>(null);
  let escapedPath = $state<string>('');

  async function handleBrowse() {
    loading = true;
    error = null;
    selectedPath = null;
    isExecutable = null;
    interpreter = null;

    try {
      const filePath = await invoke<string | null>('open_file_picker', {
        startDirectory,
      });

      if (!filePath) {
        // User canceled
        onCancel();
        return;
      }

      selectedPath = filePath;

      // Check if file is executable
      const executableResult = await invoke<boolean>('check_file_executable', {
        filePath,
      });
      isExecutable = executableResult;

      // Detect interpreter for scripts
      const interpreterResult = await invoke<string | null>(
        'detect_script_interpreter',
        {
          filePath,
        }
      );
      interpreter = interpreterResult;

      // Escape path for shell
      const escaped = await invoke<string>('escape_path_for_shell', {
        filePath,
      });
      escapedPath = escaped;

      loading = false;
    } catch (err) {
      console.error('File picker error:', err);
      error = err instanceof Error ? err.message : String(err);
      loading = false;
    }
  }

  function handleUseFile() {
    if (!escapedPath) return;

    let command = escapedPath;

    // If script has interpreter, prepend it
    if (interpreter) {
      command = `${interpreter} ${escapedPath}`;
    }

    onSelect(command);
  }

  // Auto-open file picker when component mounts
  $effect(() => {
    handleBrowse();
  });

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

{#if !loading && (selectedPath || error)}
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === 'Enter' && handleBackdropClick(e as unknown as MouseEvent)}
    role="dialog"
    aria-modal="true"
    aria-label="File Picker Result"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="modal-dialog" onclick={handleDialogClick} onkeydown={handleDialogKeydown} role="document">
      <div class="modal-header">
        <h2>Selected File</h2>
        <button class="close-btn" onclick={onCancel} aria-label="Close">✕</button>
      </div>

      <div class="modal-body">
        {#if error}
          <div class="error">
            <p>{error}</p>
          </div>
        {:else}
          <div class="file-details">
            <div class="detail-section">
              <div class="detail-label">File Path</div>
              <code class="file-path">{selectedPath}</code>
            </div>

            <div class="detail-section">
              <div class="detail-label">Command</div>
              <code class="command-preview"
                >{interpreter ? `${interpreter} ` : ''}{escapedPath}</code
              >
            </div>

            <div class="status-badges">
              {#if isExecutable}
                <span class="badge badge-success">✓ Executable</span>
              {:else}
                <span class="badge badge-warning">⚠ Not Executable</span>
              {/if}

              {#if interpreter}
                <span class="badge badge-info">🔧 {interpreter}</span>
              {/if}
            </div>

            {#if !isExecutable}
              <div class="warning-message">
                <p>
                  <strong>Note:</strong> This file is not marked as executable. You
                  may need to run:
                </p>
                <code>chmod +x {escapedPath}</code>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" onclick={onCancel}>Cancel</button>
        <button class="btn-browse" onclick={handleBrowse}>Browse Again</button>
        <button class="btn-use" onclick={handleUseFile} disabled={!escapedPath}>
          Use This File
        </button>
      </div>
    </div>
  </div>
{/if}

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
    overflow-y: auto;
  }

  .error {
    color: var(--color-text);
    text-align: center;
    padding: 2rem;
  }

  .file-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .detail-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .detail-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
  }

  code {
    padding: 0.75rem 1rem;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
    color: var(--color-text);
    word-break: break-all;
  }

  .file-path {
    color: var(--color-text-secondary);
  }

  .command-preview {
    color: var(--color-text);
  }

  .status-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .badge {
    padding: 0.375rem 0.75rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 500;
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    background: var(--color-surface-secondary);
    color: var(--color-text);
  }

  .badge-success {
    background: var(--color-surface-secondary);
    color: var(--color-text);
  }

  .badge-warning {
    background: var(--color-surface-secondary);
    color: var(--color-text);
  }

  .badge-info {
    background: var(--color-surface-secondary);
    color: var(--color-text);
  }

  .warning-message {
    padding: 1rem;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    color: var(--color-text);
  }

  .warning-message p {
    margin: 0 0 0.5rem 0;
  }

  .warning-message code {
    background: var(--color-surface);
    color: var(--color-text);
  }

  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--color-border);
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  .btn-cancel,
  .btn-browse,
  .btn-use {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    min-height: 28px;
  }

  .btn-cancel {
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    color: var(--color-button-secondary-text);
  }

  .btn-cancel:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
  }

  .btn-cancel:active {
    background: var(--color-button-secondary-active);
  }

  .btn-cancel:focus-visible {
    outline: 2px solid var(--color-button-secondary-focus);
    outline-offset: 2px;
  }

  .btn-browse {
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    color: var(--color-button-secondary-text);
  }

  .btn-browse:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
  }

  .btn-browse:active {
    background: var(--color-button-secondary-active);
  }

  .btn-browse:focus-visible {
    outline: 2px solid var(--color-button-secondary-focus);
    outline-offset: 2px;
  }

  .btn-use {
    background: var(--color-button-primary-bg);
    border: 1px solid var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
  }

  .btn-use:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .btn-use:active:not(:disabled) {
    background: var(--color-button-primary-active);
    border-color: var(--color-button-primary-active);
  }

  .btn-use:focus-visible {
    outline: 2px solid var(--color-button-primary-focus);
    outline-offset: 2px;
  }

  .btn-use:disabled {
    background: var(--color-button-disabled-bg);
    color: var(--color-button-disabled-text);
    border-color: var(--color-button-disabled-border);
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
