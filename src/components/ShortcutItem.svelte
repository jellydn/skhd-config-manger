<script lang="ts">
  import type { Shortcut } from '../types';

  interface Props {
    shortcut: Shortcut;
    onEdit?: (shortcut: Shortcut) => void;
    onDelete?: (id: string) => void;
    onTest?: (id: string) => void;
    onDuplicate?: (shortcut: Shortcut) => void;
  }

  let { shortcut, onEdit, onDelete, onTest, onDuplicate }: Props = $props();

  function formatModifiers(modifiers: string[]): string {
    if (modifiers.length === 0) return '';
    return modifiers.join(' + ') + ' +';
  }
</script>

<div class="shortcut-item">
  <div class="shortcut-keys">
    {#if shortcut.modifiers.length > 0}
      <span class="modifiers">{formatModifiers(shortcut.modifiers)}</span>
    {/if}
    <span class="key">{shortcut.key}</span>
  </div>

  <div class="shortcut-command">
    <code>{shortcut.command}</code>
  </div>

  {#if shortcut.comment}
    <div class="shortcut-comment">
      {shortcut.comment}
    </div>
  {/if}

  <div class="shortcut-actions">
    {#if onTest}
      <button type="button" class="btn-test" onclick={() => onTest(shortcut.id)} title="Test">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="5 3 19 12 5 21 5 3"></polygon>
        </svg>
      </button>
    {/if}
    {#if onDuplicate}
      <button type="button" class="btn-duplicate" onclick={() => onDuplicate(shortcut)} title="Duplicate">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
      </button>
    {/if}
    {#if onEdit}
      <button type="button" class="btn-edit" onclick={() => onEdit(shortcut)} title="Edit">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
        </svg>
      </button>
    {/if}
    {#if onDelete}
      <button type="button" class="btn-delete" onclick={() => onDelete(shortcut.id)} title="Delete">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
          <line x1="10" y1="11" x2="10" y2="17"></line>
          <line x1="14" y1="11" x2="14" y2="17"></line>
        </svg>
      </button>
    {/if}
  </div>
</div>

<style>
  .shortcut-item {
    display: grid;
    grid-template-columns: 280px 1fr 240px;
    gap: 1rem;
    align-items: start;
    padding: 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    background: white;
    margin-bottom: 0.5rem;
    transition: box-shadow 0.2s;
  }

  .shortcut-item:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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
    color: #666;
    font-weight: 500;
  }

  .key {
    background: #f5f5f5;
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    border: 1px solid #ddd;
    font-weight: 600;
    color: #333;
  }

  .shortcut-command {
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
  }

  .shortcut-command code {
    background: #f8f8f8;
    padding: 0.5rem;
    border-radius: 4px;
    display: block;
    color: #2c3e50;
  }

  .shortcut-comment {
    grid-column: 1 / -1;
    font-size: 0.875rem;
    color: #666;
    font-style: italic;
  }

  .shortcut-actions {
    display: flex;
    gap: 0.5rem;
  }

  button {
    padding: 0.4rem 0.5rem;
    border-radius: 6px;
    border: 1px solid transparent;
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  button svg {
    width: 16px;
    height: 16px;
  }

  .btn-test {
    background: #34c759;
    color: white;
  }

  .btn-test:hover {
    background: #28a745;
  }

  .btn-edit {
    background: #007aff;
    color: white;
  }

  .btn-edit:hover {
    background: #0051d5;
  }

  .btn-duplicate {
    background: #8e8e93;
    color: white;
  }

  .btn-duplicate:hover {
    background: #636366;
  }

  .btn-delete {
    background: transparent;
    color: #ff3b30;
    border-color: #ff3b30;
  }

  .btn-delete:hover {
    background: #ff3b30;
    color: white;
  }

  @media (prefers-color-scheme: dark) {
    .shortcut-item {
      background: #1e1e1e;
      border-color: #3a3a3a;
    }

    .key {
      background: #2a2a2a;
      border-color: #4a4a4a;
      color: #e0e0e0;
    }

    .shortcut-command code {
      background: #2a2a2a;
      color: #e0e0e0;
    }

    .modifiers {
      color: #aaa;
    }

    .btn-duplicate {
      background: #636366;
    }

    .btn-duplicate:hover {
      background: #8e8e93;
    }

    .shortcut-comment {
      color: #999;
    }
  }
</style>
