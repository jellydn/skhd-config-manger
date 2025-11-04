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
    <div class="modal-dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
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
    overflow-y: auto;
  }

  .error {
    color: #dc2626;
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
    color: #374151;
  }

  code {
    padding: 0.75rem 1rem;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
    color: #111827;
    word-break: break-all;
  }

  .file-path {
    color: #6b7280;
  }

  .command-preview {
    color: #059669;
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
  }

  .badge-success {
    background: #d1fae5;
    color: #065f46;
  }

  .badge-warning {
    background: #fef3c7;
    color: #92400e;
  }

  .badge-info {
    background: #dbeafe;
    color: #1e40af;
  }

  .warning-message {
    padding: 1rem;
    background: #fef3c7;
    border: 1px solid #fbbf24;
    border-radius: 8px;
    color: #92400e;
  }

  .warning-message p {
    margin: 0 0 0.5rem 0;
  }

  .warning-message code {
    background: #fff;
    color: #92400e;
  }

  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid #e5e7eb;
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
    transition: all 0.15s;
  }

  .btn-cancel {
    background: #f3f4f6;
    border: 1px solid #d1d5db;
    color: #374151;
  }

  .btn-cancel:hover {
    background: #e5e7eb;
    border-color: #9ca3af;
  }

  .btn-browse {
    background: white;
    border: 1px solid #d1d5db;
    color: #374151;
  }

  .btn-browse:hover {
    background: #f9fafb;
    border-color: #9ca3af;
  }

  .btn-use {
    background: #3b82f6;
    border: 1px solid #3b82f6;
    color: white;
  }

  .btn-use:hover:not(:disabled) {
    background: #2563eb;
    border-color: #2563eb;
  }

  .btn-use:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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

    .error {
      color: #fca5a5;
    }

    .detail-label {
      color: #d1d5db;
    }

    code {
      background: #111827;
      border-color: #374151;
      color: #f9fafb;
    }

    .file-path {
      color: #9ca3af;
    }

    .command-preview {
      color: #34d399;
    }

    .badge-success {
      background: #065f46;
      color: #d1fae5;
    }

    .badge-warning {
      background: #92400e;
      color: #fef3c7;
    }

    .badge-info {
      background: #1e40af;
      color: #dbeafe;
    }

    .warning-message {
      background: #451a03;
      border-color: #92400e;
      color: #fef3c7;
    }

    .warning-message code {
      background: #1c1917;
      color: #fef3c7;
    }

    .modal-footer {
      border-top-color: #374151;
    }

    .btn-cancel,
    .btn-browse {
      background: #374151;
      border-color: #4b5563;
      color: #f9fafb;
    }

    .btn-cancel:hover,
    .btn-browse:hover {
      background: #4b5563;
      border-color: #6b7280;
    }
  }
</style>
