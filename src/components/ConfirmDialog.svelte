<script lang="ts">
  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = 'Confirm',
    cancelLabel = 'Cancel',
    onConfirm,
    onCancel,
  }: Props = $props();

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onCancel();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && open) {
      onCancel();
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
    <div class="confirm-dialog">
      <h2>{title}</h2>
      <p>{message}</p>
      <div class="button-group">
        <button class="btn-cancel" onclick={onCancel}>
          {cancelLabel}
        </button>
        <button class="btn-confirm" onclick={onConfirm}>
          {confirmLabel}
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
  }

  .confirm-dialog {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    max-width: 400px;
    width: 100%;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
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

  h2 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #1d1d1f;
  }

  p {
    margin: 0 0 1.5rem 0;
    color: #666;
    line-height: 1.5;
  }

  .button-group {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  button {
    padding: 0.625rem 1.5rem;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid;
  }

  .btn-cancel {
    background: #f5f5f7;
    color: #1d1d1f;
    border-color: #d2d2d7;
  }

  .btn-cancel:hover {
    background: #e8e8ed;
  }

  .btn-confirm {
    background: #ff3b30;
    color: white;
    border-color: #ff3b30;
  }

  .btn-confirm:hover {
    background: #d93025;
    border-color: #d93025;
  }

  @media (prefers-color-scheme: dark) {
    .modal-backdrop {
      background: rgba(0, 0, 0, 0.7);
    }

    .confirm-dialog {
      background: #2a2a2a;
    }

    h2 {
      color: #f5f5f7;
    }

    p {
      color: #999;
    }

    .btn-cancel {
      background: #3a3a3a;
      border-color: #4a4a4a;
      color: #f5f5f7;
    }

    .btn-cancel:hover {
      background: #4a4a4a;
    }
  }
</style>
