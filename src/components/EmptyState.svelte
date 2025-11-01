<script lang="ts">
  interface Props {
    onCreateFirst?: () => void;
  }

  let { onCreateFirst }: Props = $props();
</script>

<div class="empty-state">
  <div class="empty-icon">⌨️</div>
  <h2>No Shortcuts Found</h2>
  <p>Your skhd configuration file appears to be empty.</p>
  <p class="hint">
    Start by creating your first keyboard shortcut to control your macOS environment.
  </p>

  {#if onCreateFirst}
    <button class="btn-primary" onclick={onCreateFirst}> Create First Shortcut </button>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
    min-height: 400px;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }

  h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: #1d1d1f;
  }

  p {
    color: #666;
    margin: 0.5rem 0;
    max-width: 500px;
  }

  .hint {
    font-size: 0.875rem;
    font-style: italic;
  }

  .btn-primary {
    margin-top: 2rem;
    padding: 0.75rem 2rem;
    background: #007aff;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-primary:hover {
    background: #0051d5;
  }

  @media (prefers-color-scheme: dark) {
    h2 {
      color: #f5f5f7;
    }

    p {
      color: #999;
    }
  }
</style>
