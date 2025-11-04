<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    onClose: () => void;
    children: Snippet;
  }

  let { open, onClose, children }: Props = $props();

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && open) {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    onkeydown={handleKeydown}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-content">
      {@render children()}
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
  }

  .modal-content {
    max-height: 90vh;
    overflow-y: auto;
    animation: slideIn 0.2s ease-out;
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.2) transparent;
  }

  .modal-content::-webkit-scrollbar {
    width: 12px;
  }

  .modal-content::-webkit-scrollbar-track {
    background: var(--color-scrollbar-track);
    border-radius: 0;
  }

  .modal-content::-webkit-scrollbar-thumb {
    background: var(--color-scrollbar-thumb);
    border-radius: 6px;
    border: 2px solid var(--color-scrollbar-track);
  }

  .modal-content::-webkit-scrollbar-thumb:hover {
    background: var(--color-scrollbar-thumb-hover);
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

</style>
