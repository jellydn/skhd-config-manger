<script lang="ts">
  import type { Shortcut } from '../types';
  import ShortcutItem from './ShortcutItem.svelte';

  interface Props {
    shortcuts: Shortcut[];
    onEdit?: (shortcut: Shortcut) => void;
    onDelete?: (id: string) => void;
    onTest?: (id: string) => void;
    onCreate?: () => void;
    onSave?: () => void;
    isModified?: boolean;
    onDuplicate?: (shortcut: Shortcut) => void;
    executingShortcutId?: string | null;
    onCancelExecution?: (id: string) => void;
  }

  let { shortcuts, onEdit, onDelete, onTest, onCreate, onSave, isModified, onDuplicate, executingShortcutId, onCancelExecution }: Props =
    $props();

  // Group shortcuts by category (you can enhance this later)
  let sortedShortcuts = $derived([...shortcuts].sort((a, b) => a.line_number - b.line_number));
</script>

<div class="shortcut-list">
  <div class="list-header">
    <div class="header-left">
      <h2>Keyboard Shortcuts</h2>
      <div class="shortcut-count">
        {shortcuts.length}
        {shortcuts.length === 1 ? 'shortcut' : 'shortcuts'}
      </div>
    </div>
    <div class="header-actions">
      {#if onSave}
        <button class="btn-save" onclick={onSave} disabled={!isModified} title="Save Changes" aria-label="Save changes to configuration">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
          <span>Save Changes</span>
        </button>
      {/if}
      {#if onCreate}
        <button class="btn-create" onclick={onCreate} title="New Shortcut" aria-label="Create new shortcut">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
          <span>New Shortcut</span>
        </button>
      {/if}
    </div>
  </div>

  <div class="list-content">
    {#each sortedShortcuts as shortcut (shortcut.id)}
      <ShortcutItem
        {shortcut}
        {onEdit}
        {onDelete}
        {onTest}
        {onDuplicate}
        {onCancelExecution}
        isExecuting={executingShortcutId === shortcut.id}
      />
    {/each}
  </div>
</div>

<style>
  .shortcut-list {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
  }

  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid #e0e0e0;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
    flex-shrink: 0;
  }

  h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0;
    color: #1d1d1f;
  }

  .shortcut-count {
    font-size: 0.875rem;
    color: #666;
    background: #f5f5f7;
    padding: 0.5rem 1rem;
    border-radius: 16px;
  }

  .btn-save,
  .btn-create {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .btn-save svg,
  .btn-create svg {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }

  .btn-save {
    background: #34c759;
    color: white;
    border: 1px solid #34c759;
  }

  .btn-save:hover:not(:disabled) {
    background: #28a745;
    border-color: #28a745;
  }

  .btn-save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-create {
    background: #007aff;
    color: white;
    border: 1px solid #007aff;
  }

  .btn-create:hover {
    background: #0051d5;
    border-color: #0051d5;
  }

  .list-content {
    display: flex;
    flex-direction: column;
  }

  @media (prefers-color-scheme: dark) {
    h2 {
      color: #f5f5f7;
    }

    .list-header {
      border-bottom-color: #3a3a3a;
    }

    .shortcut-count {
      background: #2a2a2a;
      color: #aaa;
    }
  }
</style>
