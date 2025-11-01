<script lang="ts">
  import type { ParseError } from '../types';

  interface Props {
    errors: ParseError[];
    onDismiss?: () => void;
  }

  let { errors, onDismiss }: Props = $props();
</script>

<div class="error-display">
  <div class="error-header">
    <div class="error-title">
      <span class="error-icon">⚠️</span>
      <h3>Configuration Errors ({errors.length})</h3>
    </div>
    {#if onDismiss}
      <button class="btn-dismiss" onclick={onDismiss}>✕</button>
    {/if}
  </div>

  <div class="error-list">
    {#each errors as error}
      <div class="error-item">
        <div class="error-location">
          Line {error.line_number}{error.column ? `, Column ${error.column}` : ''}
        </div>
        <div class="error-message">{error.message}</div>
        {#if error.line_content}
          <div class="error-code">
            <code>{error.line_content}</code>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .error-display {
    background: #fff3cd;
    border: 1px solid #ffc107;
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .error-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .error-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .error-icon {
    font-size: 1.25rem;
  }

  h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #856404;
  }

  .btn-dismiss {
    background: transparent;
    border: none;
    font-size: 1.25rem;
    color: #856404;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    transition: background 0.2s;
  }

  .btn-dismiss:hover {
    background: rgba(133, 100, 4, 0.1);
  }

  .error-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .error-item {
    background: white;
    padding: 0.75rem;
    border-radius: 6px;
    border-left: 3px solid #ffc107;
  }

  .error-location {
    font-size: 0.75rem;
    font-weight: 600;
    color: #856404;
    margin-bottom: 0.25rem;
    font-family: 'SF Mono', Monaco, monospace;
  }

  .error-message {
    color: #856404;
    margin-bottom: 0.5rem;
  }

  .error-code {
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
    background: #f8f8f8;
    padding: 0.5rem;
    border-radius: 4px;
    margin-top: 0.5rem;
  }

  .error-code code {
    color: #333;
  }

  @media (prefers-color-scheme: dark) {
    .error-display {
      background: #3a2f1f;
      border-color: #856404;
    }

    .error-item {
      background: #2a2a2a;
    }

    h3,
    .error-message,
    .error-location {
      color: #ffc107;
    }

    .error-code {
      background: #1a1a1a;
    }

    .error-code code {
      color: #e0e0e0;
    }

    .btn-dismiss {
      color: #ffc107;
    }

    .btn-dismiss:hover {
      background: rgba(255, 193, 7, 0.1);
    }
  }
</style>
