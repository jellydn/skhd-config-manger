<script lang="ts">
  import type { Shortcut, CreateShortcutRequest, ValidationResult } from '../types';
  import { validateShortcut as validateShortcutAPI } from '../services/tauri';
  import ApplicationPicker from './pickers/ApplicationPicker.svelte';
  import CommandPicker from './pickers/CommandPicker.svelte';
  import FilePicker from './pickers/FilePicker.svelte';

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
  let showFilePicker = $state(false);

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

  function handleFileSelect(selectedCommand: string) {
    command = selectedCommand;
    showFilePicker = false;
  }

  function handleFilePickerCancel() {
    showFilePicker = false;
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
        <button
          type="button"
          class="btn-picker"
          onclick={() => (showFilePicker = true)}
          title="Browse files and scripts"
        >
          📁 Files
        </button>
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

{#if showFilePicker}
  <FilePicker onSelect={handleFileSelect} onCancel={handleFilePickerCancel} />
{/if}

<style>
  .shortcut-form {
    background: var(--color-form-bg);
    border-radius: 16px;
    padding: 2.5rem;
    max-width: 650px;
    margin: 0 auto;
    box-shadow: 0 8px 24px var(--color-form-shadow);
  }

  h3 {
    margin: 0 0 2rem 0;
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--color-text);
    letter-spacing: -0.02em;
  }

  .validation-errors,
  .validation-warnings {
    margin-bottom: 1.5rem;
    padding: 1rem;
    border-radius: 8px;
    background: var(--color-surface-secondary);
    border: 1px solid var(--color-border);
  }

  .validation-errors {
    background: var(--color-surface-secondary);
    border-color: var(--color-border);
  }

  .validation-warnings {
    background: var(--color-surface-secondary);
    border-color: var(--color-border);
  }

  .error-message {
    color: var(--color-text);
    font-size: 0.875rem;
  }

  .warning-message {
    color: var(--color-text);
    font-size: 0.875rem;
  }

  .form-group {
    margin-bottom: 2rem;
  }

  fieldset {
    border: none;
    padding: 0;
    margin: 0 0 2rem 0;
  }

  legend {
    display: block;
    margin-bottom: 0.75rem;
    font-weight: 600;
    color: var(--color-text);
    font-size: 0.9rem;
    padding: 0;
    letter-spacing: -0.01em;
  }

  label {
    display: block;
    margin-bottom: 0.75rem;
    font-weight: 600;
    color: var(--color-text);
    font-size: 0.9rem;
    letter-spacing: -0.01em;
  }

  .modifier-buttons {
    display: flex;
    gap: 0.625rem;
    flex-wrap: wrap;
  }

  .modifier-btn {
    padding: 0.625rem 1.25rem;
    border: 1.5px solid var(--color-input-border);
    background: var(--color-input-bg);
    border-radius: 10px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    color: var(--color-text);
    letter-spacing: -0.01em;
  }

  .modifier-btn:hover {
    border-color: var(--color-input-focus-border);
    background: var(--color-input-focus-bg);
    color: var(--color-text);
    transform: translateY(-1px);
  }

  .modifier-btn.active {
    background: var(--color-button-primary-bg);
    border-color: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
    box-shadow: 0 2px 8px var(--color-input-focus-shadow);
  }

  input,
  textarea {
    width: 100%;
    padding: 0.875rem 1rem;
    border: 1.5px solid var(--color-input-border);
    border-radius: 10px;
    font-size: 0.95rem;
    font-family: inherit;
    transition: all 0.15s ease;
    background: var(--color-input-bg);
    box-sizing: border-box;
  }

  textarea {
    font-family: 'SF Mono', Monaco, 'Courier New', monospace;
    resize: vertical;
    line-height: 1.5;
  }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: var(--color-input-focus-border);
    background: var(--color-input-focus-bg);
    box-shadow: 0 0 0 3px var(--color-input-focus-shadow);
  }

  small {
    display: block;
    margin-top: 0.5rem;
    color: var(--color-text-secondary);
    font-size: 0.8rem;
    line-height: 1.4;
  }

  .picker-buttons {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 0.75rem;
    flex-wrap: wrap;
  }

  .btn-picker {
    padding: 0.5rem 0.875rem;
    border: 1.5px solid var(--color-input-border);
    background: var(--color-button-secondary-bg);
    border-radius: 8px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    color: var(--color-button-secondary-text);
    white-space: nowrap;
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    letter-spacing: -0.01em;
  }

  .btn-picker:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-input-focus-border);
    color: var(--color-text);
    transform: translateY(-1px);
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
    margin-top: 2.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--color-border);
  }

  button[type='submit'],
  .btn-cancel {
    padding: 0.875rem 2rem;
    border-radius: 10px;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    border: none;
    letter-spacing: -0.01em;
  }

  .btn-cancel {
    background: var(--color-button-secondary-bg);
    color: var(--color-button-secondary-text);
    border: 1.5px solid var(--color-button-secondary-border);
  }

  .btn-cancel:hover:not(:disabled) {
    background: var(--color-button-secondary-hover);
    color: var(--color-text);
    border-color: var(--color-button-secondary-border);
    transform: translateY(-1px);
  }

  .btn-save {
    background: var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
    box-shadow: 0 2px 8px var(--color-input-focus-shadow);
  }

  .btn-save:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px var(--color-input-focus-shadow);
  }

  button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    transform: none !important;
  }

  input,
  textarea {
    color: var(--color-text);
  }
</style>
