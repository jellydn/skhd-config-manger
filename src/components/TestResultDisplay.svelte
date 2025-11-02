<script lang="ts">
  import type { TestResult } from '../types';
  import { getExecutionConfig } from '../services/tauri';
  import { onMount } from 'svelte';

  interface Props {
    result: TestResult;
    onClose: () => void;
  }

  let { result, onClose }: Props = $props();
  let timeoutSeconds = $state(30); // Default fallback
  let maxOutputLength = $state(10000); // Default fallback

  onMount(async () => {
    try {
      const config = await getExecutionConfig();
      timeoutSeconds = config.timeout_seconds;
      maxOutputLength = config.max_output_length;
    } catch (error) {
      console.error('Failed to load execution config:', error);
      // Keep default values on error
    }
  });

  function getExitCodeDescription(code: number | null | undefined): string {
    if (code === null || code === undefined) return '';
    if (code === 0) return '✓ Success';
    if (code === 1) return '✗ General error';
    if (code === 2) return '✗ Misuse of shell command';
    if (code === 126) return '✗ Command cannot execute (permission denied)';
    if (code === 127) return '✗ Command not found';
    if (code === 128) return '✗ Invalid exit argument';
    if (code >= 129 && code <= 159) return `✗ Terminated by signal ${code - 128}`;
    return `✗ Failed (exit code ${code})`;
  }
</script>

<div class="test-result-display">
  <div class="result-header">
    <h3>{result.executed ? 'Execution Result' : 'Test Result'}</h3>
    <button class="btn-close" onclick={onClose}>✕</button>
  </div>

  {#if result.executed}
    <!-- Execution Results -->
    <div class="result-status" class:valid={result.exit_code === 0} class:invalid={result.exit_code !== 0 || result.timed_out}>
      {#if result.timed_out}
        ⏱️ Command timed out ({timeoutSeconds} seconds)
      {:else if result.exit_code === 0}
        ✅ Command executed successfully
      {:else}
        {getExitCodeDescription(result.exit_code)}
      {/if}
    </div>

    {#if result.execution_duration_ms !== undefined}
      <div class="execution-info">
        <strong>Duration:</strong> {result.execution_duration_ms}ms
      </div>
    {/if}

    {#if result.stdout}
      <div class="output-section">
        <h4>Standard Output (stdout):</h4>
        <pre class="stdout">{result.stdout}</pre>
      </div>
    {/if}

    {#if result.stderr}
      <div class="output-section">
        <h4>Standard Error (stderr):</h4>
        <pre class="stderr">{result.stderr}</pre>
      </div>
    {/if}

    {#if result.output_truncated}
      <div class="truncation-notice">
        <strong>⚠️ Output Truncated</strong>
        <p>Output was limited to {maxOutputLength.toLocaleString()} characters. The full output may be longer.</p>
      </div>
    {/if}
  {:else}
    <!-- Syntax Check Results -->
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
  {/if}

  <div class="result-footer">
    <small>{result.executed ? 'Executed' : 'Tested'} at: {new Date(result.timestamp).toLocaleString()}</small>
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

  .execution-info {
    padding: 0.75rem;
    background: #f8f8f8;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .execution-info strong {
    color: #1d1d1f;
    margin-right: 0.5rem;
  }

  .output-section {
    margin-bottom: 1rem;
  }

  .output-section h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #666;
    font-weight: 600;
  }

  .stdout {
    background: #f8f8f8;
    border-left: 3px solid #34c759;
  }

  .stderr {
    background: #fff3f3;
    border-left: 3px solid #ff3b30;
    color: #d32f2f;
  }

  .truncation-notice {
    padding: 0.75rem;
    background: #fff9e6;
    border: 1px solid #ffc107;
    border-radius: 6px;
    color: #856404;
    font-size: 0.875rem;
    margin-top: 1rem;
  }

  .truncation-notice strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .truncation-notice p {
    margin: 0;
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

    .execution-info {
      background: #1e1e1e;
    }

    .execution-info strong {
      color: #f5f5f7;
    }

    .output-section h4 {
      color: #999;
    }

    .stdout {
      background: #1e1e1e;
      border-left-color: #34c759;
    }

    .stderr {
      background: #3a1a1a;
      border-left-color: #ff3b30;
      color: #ff5c5c;
    }

    .truncation-notice {
      background: #3a3020;
      border-color: #ffc107;
      color: #ffd966;
    }
  }
</style>
