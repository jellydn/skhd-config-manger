<script lang="ts">
  import type { Shortcut, CreateShortcutRequest, ValidationResult } from '../types';
  import { validateShortcut as validateShortcutAPI } from '../services/tauri';
  import ApplicationPicker from './pickers/ApplicationPicker.svelte';
  import CommandPicker from './pickers/CommandPicker.svelte';

  interface Props {
    shortcut?: Shortcut;
    mode?: 'create' | 'edit' | 'duplicate';
    onSave: (data: CreateShortcutRequest & { id?: string }) => Promise<void>;
    onCancel: () => void;
  }

  let { shortcut, mode = shortcut ? 'edit' : 'create', onSave, onCancel }: Props = $props();

  const AVAILABLE_MODIFIERS = ['cmd', 'alt', 'shift', 'ctrl', 'fn'];

  let selectedModifiers = $state<string[]>(shortcut?.modifiers || []);
  let key = $state(shortcut?.key || '');
  let command = $state(shortcut?.command || '');
  let comment = $state(shortcut?.comment || '');
  let validationErrors = $state<string[]>([]);
  let validationWarnings = $state<string[]>([]);
  let saving = $state(false);
  let showAppPicker = $state(false);
  let showCommandPicker = $state(false);

  // Helper to compare arrays efficiently without JSON.stringify
  function arraysEqual(a: string[], b: string[]): boolean {
    if (a.length !== b.length) return false;
    return a.every((val, index) => val === b[index]);
  }

  // Track if form has changes (for duplicate mode)
  let hasChanges = $derived(
    mode === 'duplicate' ? (
      !arraysEqual(selectedModifiers, shortcut?.modifiers || []) ||
      key !== (shortcut?.key || '') ||
      command !== (shortcut?.command || '') ||
      comment !== (shortcut?.comment || '')
    ) : true
  );

  // Form configuration by mode
  const formConfig = {
    duplicate: { title: 'Duplicate Shortcut', buttonText: 'Create' },
    edit: { title: 'Edit Shortcut', buttonText: 'Update' },
    create: { title: 'Create Shortcut', buttonText: 'Create' }
  };

  let formTitle = $derived(formConfig[mode].title);
  let buttonText = $derived(formConfig[mode].buttonText);

  function toggleModifier(modifier: string) {
    if (selectedModifiers.includes(modifier)) {
      selectedModifiers = selectedModifiers.filter((m) => m !== modifier);
    } else {
      selectedModifiers = [...selectedModifiers, modifier];
    }
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!key.trim() || !command.trim()) {
      validationErrors = ['Key and command are required'];
      return;
    }

    try {
      saving = true;
      validationErrors = [];
      validationWarnings = [];

      const data: CreateShortcutRequest & { id?: string } = {
        modifiers: selectedModifiers,
        key: key.trim(),
        command: command.trim(),
        comment: comment.trim() || undefined,
      };

      // Only set ID for edit mode (not for duplicate or create)
      if (shortcut && mode === 'edit') {
        data.id = shortcut.id;
      }

      await onSave(data);
    } catch (err) {
      validationErrors = [err instanceof Error ? err.message : String(err)];
    } finally {
      saving = false;
    }
  }

  function handleKeyInput(event: KeyboardEvent) {
    // Capture keyboard input for key field
    if (event.target instanceof HTMLInputElement && event.target.id === 'key-input') {
      event.preventDefault();
      const keyName = event.key.toLowerCase();

      // Map special keys
      const keyMap: Record<string, string> = {
        enter: 'return',
        escape: 'escape',
        arrowup: 'up',
        arrowdown: 'down',
        arrowleft: 'left',
        arrowright: 'right',
        ' ': 'space',
      };

      key = keyMap[keyName] || keyName;
    }
  }

  function handleAppSelect(selectedCommand: string) {
    command = selectedCommand;
    showAppPicker = false;
  }

  function handleAppPickerCancel() {
    showAppPicker = false;
  }

  function handleCommandSelect(selectedCommand: string) {
    command = selectedCommand;
    showCommandPicker = false;
  }

  function handleCommandPickerCancel() {
    showCommandPicker = false;
  }
</script>

<div class="shortcut-form">
  <h3>{formTitle}</h3>

  {#if validationErrors.length > 0}
    <div class="validation-errors">
      {#each validationErrors as error}
        <div class="error-message">{error}</div>
      {/each}
    </div>
  {/if}

  {#if validationWarnings.length > 0}
    <div class="validation-warnings">
      {#each validationWarnings as warning}
        <div class="warning-message">{warning}</div>
      {/each}
    </div>
  {/if}

  <form onsubmit={handleSubmit}>
    <fieldset class="form-group">
      <legend>Modifiers</legend>
      <div class="modifier-buttons">
        {#each AVAILABLE_MODIFIERS as modifier}
          <button
            type="button"
            class="modifier-btn"
            class:active={selectedModifiers.includes(modifier)}
            onclick={() => toggleModifier(modifier)}
          >
            {modifier}
          </button>
        {/each}
      </div>
    </fieldset>

    <div class="form-group">
      <label for="key-input">Key</label>
      <input
        id="key-input"
        type="text"
        bind:value={key}
        onkeydown={handleKeyInput}
        placeholder="Press a key or type (e.g., return, f, escape)"
        required
      />
      <small>Press a key to capture it, or type the key name</small>
    </div>

    <div class="form-group">
      <div class="field-header">
        <label for="command-input">Command</label>
        <div class="picker-buttons">
          <button
            type="button"
            class="btn-picker"
            onclick={() => (showCommandPicker = true)}
            title="Browse command templates"
          >
            📋 Templates
          </button>
          <button
            type="button"
            class="btn-picker"
            onclick={() => (showAppPicker = true)}
            title="Browse installed applications"
          >
            📱 Applications
          </button>
        </div>
      </div>
      <textarea
        id="command-input"
        bind:value={command}
        placeholder="Shell command to execute (e.g., open -a Terminal)"
        rows="3"
        required
      ></textarea>
      <small>Shell command that will be executed when the shortcut is triggered</small>
    </div>

    <div class="form-group">
      <label for="comment-input">Comment (optional)</label>
      <input
        id="comment-input"
        type="text"
        bind:value={comment}
        placeholder="Description of what this shortcut does"
      />
    </div>

    <div class="form-actions">
      <button type="button" class="btn-cancel" onclick={onCancel} disabled={saving}>
        Cancel
      </button>
      <button type="submit" class="btn-save" disabled={saving || (mode === 'duplicate' && !hasChanges)}>
        {saving ? 'Saving...' : buttonText}
      </button>
    </div>
  </form>
</div>

{#if showAppPicker}
  <ApplicationPicker onSelect={handleAppSelect} onCancel={handleAppPickerCancel} />
{/if}

{#if showCommandPicker}
  <CommandPicker onSelect={handleCommandSelect} onCancel={handleCommandPickerCancel} />
{/if}

<style>
  .shortcut-form {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    max-width: 600px;
    margin: 0 auto;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #1d1d1f;
  }

  .validation-errors,
  .validation-warnings {
    margin-bottom: 1.5rem;
    padding: 1rem;
    border-radius: 8px;
  }

  .validation-errors {
    background: #fff3f3;
    border: 1px solid #ff3b30;
  }

  .validation-warnings {
    background: #fff9e6;
    border: 1px solid #ff9500;
  }

  .error-message {
    color: #ff3b30;
    font-size: 0.875rem;
  }

  .warning-message {
    color: #ff9500;
    font-size: 0.875rem;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  fieldset {
    border: none;
    padding: 0;
    margin: 0 0 1.5rem 0;
  }

  legend {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #1d1d1f;
    font-size: 0.875rem;
    padding: 0;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #1d1d1f;
    font-size: 0.875rem;
  }

  .modifier-buttons {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .modifier-btn {
    padding: 0.5rem 1rem;
    border: 2px solid #d2d2d7;
    background: white;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    color: #666;
  }

  .modifier-btn:hover {
    border-color: #007aff;
    color: #007aff;
  }

  .modifier-btn.active {
    background: #007aff;
    border-color: #007aff;
    color: white;
  }

  input,
  textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d2d2d7;
    border-radius: 8px;
    font-size: 1rem;
    font-family: inherit;
    transition: border-color 0.2s;
  }

  textarea {
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    resize: vertical;
  }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: #007aff;
  }

  small {
    display: block;
    margin-top: 0.25rem;
    color: #666;
    font-size: 0.75rem;
  }

  .field-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }

  .picker-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .btn-picker {
    padding: 0.375rem 0.75rem;
    border: 1px solid #d2d2d7;
    background: white;
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    color: #007aff;
  }

  .btn-picker:hover {
    background: #f5f5f7;
    border-color: #007aff;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 2rem;
  }

  button[type='submit'],
  .btn-cancel {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }

  .btn-cancel {
    background: #f5f5f7;
    color: #1d1d1f;
  }

  .btn-cancel:hover:not(:disabled) {
    background: #e8e8ed;
  }

  .btn-save {
    background: #007aff;
    color: white;
  }

  .btn-save:hover:not(:disabled) {
    background: #0051d5;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  @media (prefers-color-scheme: dark) {
    .shortcut-form {
      background: #2a2a2a;
    }

    h3,
    label,
    legend {
      color: #f5f5f7;
    }

    .modifier-btn {
      background: #1e1e1e;
      border-color: #4a4a4a;
      color: #aaa;
    }

    .modifier-btn:hover {
      border-color: #007aff;
      color: #007aff;
    }

    input,
    textarea {
      background: #1e1e1e;
      border-color: #4a4a4a;
      color: #f5f5f7;
    }

    .btn-cancel {
      background: #3a3a3a;
      color: #f5f5f7;
    }

    .btn-cancel:hover:not(:disabled) {
      background: #4a4a4a;
    }

    small {
      color: #999;
    }

    .btn-picker {
      background: #1e1e1e;
      border-color: #4a4a4a;
    }

    .btn-picker:hover {
      background: #3a3a3a;
      border-color: #007aff;
    }
  }
</style>
