<script lang="ts">
  import type { Shortcut } from '../types';

  interface Props {
    shortcut: Shortcut;
    onEdit?: (shortcut: Shortcut) => void;
    onDelete?: (id: string) => void;
    onTest?: (id: string) => void;
    onDuplicate?: (shortcut: Shortcut) => void;
    onCancelExecution?: (id: string) => void;
    isExecuting?: boolean;
  }

  let { shortcut, onEdit, onDelete, onTest, onDuplicate, onCancelExecution, isExecuting = false }: Props = $props();

  function formatModifiers(modifiers: string[]): string {
    if (modifiers.length === 0) return '';
    return modifiers.join(' + ') + ' +';
  }

  function handleTest() {
    if (!onTest) return;
    onTest(shortcut.id);
  }

  function handleCancelClick() {
    if (!onCancelExecution) return;
    onCancelExecution(shortcut.id);
  }
</script>

<div class="shortcut-item">
  <div class="shortcut-main">
    <div class="shortcut-keys">
      {#if shortcut.modifiers.length > 0}
        <span class="modifiers">{formatModifiers(shortcut.modifiers)}</span>
      {/if}
      <span class="key">{shortcut.key}</span>
    </div>

    <div class="shortcut-command">
      <code>{shortcut.command}</code>
    </div>

    <div class="shortcut-actions">
    {#if onTest}
      {#if isExecuting && onCancelExecution}
        <button
          type="button"
          class="btn-cancel"
          onclick={handleCancelClick}
          title="Cancel execution"
          aria-label="Cancel command execution"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="15" y1="9" x2="9" y2="15"></line>
            <line x1="9" y1="9" x2="15" y2="15"></line>
          </svg>
        </button>
      {/if}
      <button
        type="button"
        class="btn-test"
        class:executing={isExecuting}
        onclick={handleTest}
        disabled={isExecuting}
        title={isExecuting ? 'Executing...' : 'Execute'}
        aria-label={isExecuting ? 'Executing command' : 'Execute shortcut command'}
      >
        {#if isExecuting}
          <svg class="spinner" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <line x1="12" y1="2" x2="12" y2="6"></line>
            <line x1="12" y1="18" x2="12" y2="22"></line>
            <line x1="4.93" y1="4.93" x2="7.76" y2="7.76"></line>
            <line x1="16.24" y1="16.24" x2="19.07" y2="19.07"></line>
            <line x1="2" y1="12" x2="6" y2="12"></line>
            <line x1="18" y1="12" x2="22" y2="12"></line>
            <line x1="4.93" y1="19.07" x2="7.76" y2="16.24"></line>
            <line x1="16.24" y1="7.76" x2="19.07" y2="4.93"></line>
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <polygon points="5 3 19 12 5 21 5 3"></polygon>
          </svg>
        {/if}
      </button>
    {/if}
    {#if onDuplicate}
      <button type="button" class="btn-duplicate" onclick={() => onDuplicate(shortcut)} title="Duplicate" aria-label="Duplicate shortcut">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
      </button>
    {/if}
    {#if onEdit}
      <button type="button" class="btn-edit" onclick={() => onEdit(shortcut)} title="Edit" aria-label="Edit shortcut">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
        </svg>
      </button>
    {/if}
    {#if onDelete}
      <button type="button" class="btn-delete" onclick={() => onDelete(shortcut.id)} title="Delete" aria-label="Delete shortcut">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
          <line x1="10" y1="11" x2="10" y2="17"></line>
          <line x1="14" y1="11" x2="14" y2="17"></line>
        </svg>
      </button>
    {/if}
    </div>
  </div>

  {#if shortcut.comment}
    <div class="shortcut-comment">
      {shortcut.comment}
    </div>
  {/if}
</div>

<style>
  .shortcut-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    border: 1px solid var(--color-border);
    border-radius: 8px;
    background: var(--color-surface-secondary);
    margin-bottom: 0.5rem;
    transition: all 0.2s;
  }

  .shortcut-main {
    display: grid;
    grid-template-columns: minmax(180px, 240px) 1fr minmax(180px, 200px);
    gap: 1rem;
    align-items: start;
  }

  .shortcut-item:hover {
    background: var(--color-surface);
    border-color: var(--color-border-hover);
  }

  .shortcut-keys {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex-wrap: wrap;
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
  }

  .modifiers {
    color: var(--color-text-secondary);
    font-weight: 500;
  }

  .key {
    background: var(--color-surface);
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    border: 1px solid var(--color-border);
    font-weight: 600;
    color: var(--color-text);
  }

  .shortcut-command {
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
  }

  .shortcut-command code {
    background: var(--color-surface);
    padding: 0.5rem;
    border-radius: 4px;
    display: block;
    color: var(--color-text);
  }

  .shortcut-comment {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
    font-style: italic;
    padding-left: 0.5rem;
    border-left: 3px solid var(--color-border);
  }

  .shortcut-actions {
    display: flex;
    gap: 0.5rem;
    position: relative;
    z-index: 1;
  }

  button {
    padding: 0.4rem 0.5rem;
    border-radius: 6px;
    border: 1px solid transparent;
    font-size: 0.8125rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 36px;
  }

  button svg {
    width: 16px;
    height: 16px;
  }

  .btn-test {
    background: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
    border-color: var(--color-button-primary-bg);
  }

  .btn-test:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .btn-test:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-test.executing {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  .btn-cancel {
    background: var(--color-button-secondary-bg);
    color: var(--color-button-secondary-text);
    border-color: var(--color-button-secondary-border);
  }

  .btn-cancel:hover {
    background: var(--color-button-secondary-hover);
    color: var(--color-text);
    border-color: var(--color-button-secondary-border);
  }

  .btn-edit {
    background: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
    border-color: var(--color-button-primary-bg);
  }

  .btn-edit:hover {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .btn-duplicate {
    background: var(--color-button-secondary-bg);
    color: var(--color-button-secondary-text);
    border-color: var(--color-button-secondary-border);
  }

  .btn-duplicate:hover {
    background: var(--color-button-secondary-hover);
    color: var(--color-text);
    border-color: var(--color-button-secondary-border);
  }

  .btn-delete {
    background: var(--color-button-secondary-bg);
    color: var(--color-button-secondary-text);
    border-color: var(--color-button-secondary-border);
  }

  .btn-delete:hover {
    background: var(--color-button-secondary-hover);
    color: var(--color-text);
    border-color: var(--color-button-secondary-border);
  }
</style>
