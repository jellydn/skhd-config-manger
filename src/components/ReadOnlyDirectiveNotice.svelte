<script lang="ts">
  let { count }: { count: number } = $props();
</script>

<div class="directive-notice" role="status">
  This file contains {count} read-only {count === 1 ? 'directive' : 'directives'}. Keybinder
  preserves them unchanged, but does not edit them.
</div>

<style>
  .directive-notice {
    margin-bottom: 12px;
    padding: 10px 12px;
    color: var(--color-status-warning);
    background: var(--color-status-warning-bg);
    border: 1px solid var(--color-status-warning-border);
    border-radius: 6px;
    font-size: 12px;
  }
</style>
