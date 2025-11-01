<script lang="ts">
  import type { TestResult } from '../types';

  interface Props {
    result: TestResult;
    onClose: () => void;
  }

  let { result, onClose }: Props = $props();
</script>

<div class="test-result-display">
  <div class="result-header">
    <h3>Test Result</h3>
    <button class="btn-close" onclick={onClose}>✕</button>
  </div>

  <div class="result-status" class:valid={result.syntax_valid} class:invalid={!result.syntax_valid}>
    {#if result.syntax_valid}
      ✅ Command syntax is valid
    {:else}
      ❌ Command has syntax errors
    {/if}
  </div>

  {#if result.syntax_error}
    <div class="syntax-error">
      <h4>Syntax Error:</h4>
      <pre>{result.syntax_error}</pre>
    </div>
  {/if}

  <div class="command-preview">
    <h4>Command Preview:</h4>
    <pre>{result.preview}</pre>
  </div>

  <div class="result-footer">
    <small>Tested at: {new Date(result.timestamp).toLocaleString()}</small>
  </div>
</div>

<style>
  .test-result-display {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    max-width: 90vw;
    width: 600px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    max-height: 80vh;
    overflow-y: auto;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #e0e0e0;
  }

  h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #1d1d1f;
  }

  .btn-close {
    background: transparent;
    border: none;
    font-size: 1.5rem;
    color: #666;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    transition: background 0.2s;
  }

  .btn-close:hover {
    background: #f5f5f7;
  }

  .result-status {
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
    font-weight: 500;
    font-size: 1rem;
  }

  .result-status.valid {
    background: #e6f7ec;
    color: #28a745;
    border: 1px solid #34c759;
  }

  .result-status.invalid {
    background: #ffebee;
    color: #d32f2f;
    border: 1px solid #ff3b30;
  }

  .syntax-error {
    margin-bottom: 1.5rem;
    padding: 1rem;
    background: #fff3f3;
    border: 1px solid #ff3b30;
    border-radius: 8px;
  }

  .syntax-error h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #ff3b30;
    font-weight: 600;
  }

  .command-preview {
    margin-bottom: 1.5rem;
  }

  .command-preview h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #666;
    font-weight: 600;
  }

  pre {
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    font-size: 0.875rem;
    line-height: 1.6;
    background: #f8f8f8;
    padding: 1rem;
    border-radius: 6px;
    overflow-x: auto;
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    word-break: break-word;
    color: #1d1d1f;
  }

  .result-footer {
    padding-top: 1rem;
    border-top: 1px solid #e0e0e0;
    text-align: right;
  }

  .result-footer small {
    color: #666;
    font-size: 0.75rem;
  }

  @media (prefers-color-scheme: dark) {
    .test-result-display {
      background: #2a2a2a;
    }

    .result-header {
      border-bottom-color: #3a3a3a;
    }

    h3 {
      color: #f5f5f7;
    }

    .btn-close {
      color: #999;
    }

    .btn-close:hover {
      background: #3a3a3a;
    }

    .result-status.valid {
      background: #1a3a25;
      color: #34c759;
    }

    .result-status.invalid {
      background: #3a1a1a;
      color: #ff3b30;
    }

    .syntax-error {
      background: #3a1a1a;
      border-color: #ff3b30;
    }

    .syntax-error h4 {
      color: #ff5c5c;
    }

    .command-preview h4 {
      color: #999;
    }

    pre {
      background: #1e1e1e;
      color: #f5f5f7;
    }

    .result-footer {
      border-top-color: #3a3a3a;
    }

    .result-footer small {
      color: #999;
    }
  }
</style>
