<script lang="ts">
  import type { Shortcut } from '../types';
  import ShortcutItem from './ShortcutItem.svelte';

  interface Props {
    shortcuts: Shortcut[];
    onEdit?: (shortcut: Shortcut) => void;
    onDelete?: (id: string) => void;
    onTest?: (id: string) => void;
  }

  let { shortcuts, onEdit, onDelete, onTest }: Props = $props();

  // Group shortcuts by category (you can enhance this later)
  let sortedShortcuts = $derived(
    [...shortcuts].sort((a, b) => a.line_number - b.line_number)
  );
</script>

<div class="shortcut-list">
  <div class="list-header">
    <h2>Keyboard Shortcuts</h2>
    <div class="shortcut-count">
      {shortcuts.length} {shortcuts.length === 1 ? 'shortcut' : 'shortcuts'}
    </div>
  </div>

  <div class="list-content">
    {#each sortedShortcuts as shortcut (shortcut.id)}
      <ShortcutItem
        {shortcut}
        {onEdit}
        {onDelete}
        {onTest}
      />
    {/each}
  </div>
</div>

<style>
  .shortcut-list {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
  }

  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid #e0e0e0;
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
