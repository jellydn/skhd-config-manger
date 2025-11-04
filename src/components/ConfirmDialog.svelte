<script lang="ts">
  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    variant?: 'default' | 'danger';
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = 'Confirm',
    cancelLabel = 'Cancel',
    variant = 'default',
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
        <button class="btn-confirm" class:btn-danger={variant === 'danger'} onclick={onConfirm}>
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
    background: var(--color-modal-backdrop);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
    backdrop-filter: blur(4px);
  }

  .confirm-dialog {
    background: var(--color-modal-bg);
    border-radius: 12px;
    padding: 2rem;
    max-width: 400px;
    width: 100%;
    box-shadow: 0 10px 40px var(--color-form-shadow);
    animation: slideIn 0.2s ease-out;
    border: 1px solid var(--color-modal-border);
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
    color: var(--color-text);
  }

  p {
    margin: 0 0 1.5rem 0;
    color: var(--color-text-secondary);
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
    background: var(--color-button-secondary-bg);
    color: var(--color-button-secondary-text);
    border-color: var(--color-button-secondary-border);
  }

  .btn-cancel:hover {
    background: var(--color-button-secondary-hover);
  }

  .btn-confirm {
    background: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
    border-color: var(--color-button-primary-bg);
  }

  .btn-confirm:hover {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .btn-danger {
    background: var(--color-button-primary-bg) !important;
    color: var(--color-button-primary-text) !important;
    border-color: var(--color-button-primary-bg) !important;
    font-weight: 600 !important;
    animation: pulseWarning 2s infinite;
  }

  .btn-danger:hover {
    background: var(--color-button-primary-hover) !important;
    border-color: var(--color-button-primary-hover) !important;
  }

  @keyframes pulseWarning {
    0%, 100% {
      box-shadow: 0 0 0 0 var(--color-input-focus-shadow);
    }
    50% {
      box-shadow: 0 0 0 8px var(--color-input-focus-shadow);
    }
  }
</style>
