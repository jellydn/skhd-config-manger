<script lang="ts">
  import type { Shortcut } from '../types';
  import ShortcutItem from './ShortcutItem.svelte';

  interface Props {
    shortcuts: Shortcut[];
    onEdit?: (shortcut: Shortcut) => void;
    onDelete?: (id: string) => void;
    onTest?: (id: string) => void;
    onDuplicate?: (shortcut: Shortcut) => void;
    executingShortcutId?: string | null;
    onCancelExecution?: (id: string) => void;
  }

  let { shortcuts, onEdit, onDelete, onTest, onDuplicate, executingShortcutId, onCancelExecution }: Props =
    $props();

  // Group shortcuts by category (you can enhance this later)
  let sortedShortcuts = $derived([...shortcuts].sort((a, b) => a.line_number - b.line_number));
</script>

<div class="shortcut-list">
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
    max-width: 100%;
    margin: 0;
    padding: 0;
  }

  .list-content {
    display: flex;
    flex-direction: column;
  }
</style>
