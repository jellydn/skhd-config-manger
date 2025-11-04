<script lang="ts">
  import { commandTemplateService } from '../../services/commandTemplateService';
  import type { CommandCategory, CommandTemplate } from '../../types';

  interface Props {
    onSelect: (command: string) => void;
    onCancel: () => void;
  }

  let { onSelect, onCancel }: Props = $props();

  let categories = $state<CommandCategory[]>([]);
  let templates = $state<CommandTemplate[]>([]);
  let selectedCategory = $state<string | null>(null);
  let selectedTemplate = $state<CommandTemplate | null>(null);
  let parameterValues = $state<Record<string, string>>({});
  let searchQuery = $state('');
  let loading = $state(true);
  let error = $state<string | null>(null);
  let generating = $state(false);

  // Load categories and templates
  $effect(() => {
    Promise.all([
      commandTemplateService.listCategories(),
      commandTemplateService.listTemplates(),
    ])
      .then(([cats, tmps]) => {
        categories = cats;
        templates = tmps;
        loading = false;
      })
      .catch((err) => {
        console.error('Failed to load templates:', err);
        error = err.toString();
        loading = false;
      });
  });

  // Filtered templates based on category and search
  const filteredTemplates = $derived(() => {
    let result = templates;

    // Filter by category
    if (selectedCategory) {
      result = result.filter((t) => t.category_id === selectedCategory);
    }

    // Filter by search query
    result = commandTemplateService.searchTemplates(result, searchQuery);

    return result;
  });

  function handleCategorySelect(categoryId: string | null) {
    selectedCategory = categoryId;
    selectedTemplate = null;
    parameterValues = {};
    searchQuery = '';
  }

  function handleTemplateSelect(template: CommandTemplate) {
    selectedTemplate = template;
    // Initialize parameter values with defaults
    parameterValues = {};
    template.parameters.forEach((param) => {
      parameterValues[param.name] = param.default_value;
    });
  }

  function handleBack() {
    if (selectedTemplate) {
      selectedTemplate = null;
      parameterValues = {};
    } else if (selectedCategory) {
      selectedCategory = null;
    }
  }

  async function handleGenerate() {
    if (!selectedTemplate) return;

    generating = true;
    error = null;

    try {
      const command = await commandTemplateService.generateCommand(
        selectedTemplate.id,
        parameterValues
      );
      onSelect(command);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      generating = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      if (selectedTemplate) {
        handleBack();
      } else {
        onCancel();
      }
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onCancel();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="modal-backdrop"
  onclick={handleBackdropClick}
  onkeydown={(e) => e.key === 'Enter' && handleBackdropClick(e as unknown as MouseEvent)}
  role="dialog"
  aria-modal="true"
  aria-label="Command Template Picker"
  tabindex="-1"
>
  <div class="modal-dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
    <div class="modal-header">
      <div class="header-left">
        {#if selectedTemplate || selectedCategory}
          <button class="back-btn" onclick={handleBack} aria-label="Back">←</button>
        {/if}
        <h2>
          {#if selectedTemplate}
            {selectedTemplate.name}
          {:else if selectedCategory}
            {categories.find((c) => c.id === selectedCategory)?.name || 'Templates'}
          {:else}
            Command Templates
          {/if}
        </h2>
      </div>
      <button class="close-btn" onclick={onCancel} aria-label="Close">✕</button>
    </div>

    <div class="modal-body">
      {#if loading}
        <div class="loading">
          <div class="spinner"></div>
          <p>Loading templates...</p>
        </div>
      {:else if error && !generating}
        <div class="error">
          <p>Failed to load templates: {error}</p>
        </div>
      {:else if selectedTemplate}
        <!-- Parameter Form View -->
        <div class="template-details">
          <p class="template-description">{selectedTemplate.description}</p>

          {#if selectedTemplate.parameters.length > 0}
            <div class="parameters-form">
              <h3>Parameters</h3>
              {#each selectedTemplate.parameters as param (param.name)}
                <div class="parameter-field">
                  <label for={param.name}>
                    {param.description}
                  </label>

                  {#if param.enum_values && param.enum_values.length > 0}
                    <!-- Dropdown for enum values -->
                    <select
                      id={param.name}
                      bind:value={parameterValues[param.name]}
                      class="param-input"
                    >
                      {#each param.enum_values as enumValue}
                        <option value={enumValue}>{enumValue}</option>
                      {/each}
                    </select>
                  {:else}
                    <!-- Text input for other types -->
                    <input
                      type={param.data_type === 'integer' ? 'number' : 'text'}
                      id={param.name}
                      bind:value={parameterValues[param.name]}
                      placeholder={param.default_value}
                      min={param.min_value}
                      max={param.max_value}
                      class="param-input"
                    />
                  {/if}

                  {#if param.min_value !== undefined || param.max_value !== undefined}
                    <span class="param-hint">
                      Range: {param.min_value ?? '−∞'} to {param.max_value ?? '∞'}
                    </span>
                  {/if}
                </div>
              {/each}
            </div>
          {:else}
            <p class="no-params">This template has no parameters.</p>
          {/if}

          {#if error && generating}
            <div class="error">
              <p>{error}</p>
            </div>
          {/if}
        </div>
      {:else if selectedCategory}
        <!-- Template List View (filtered by category) -->
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search templates..."
          class="search-input"
        />

        {#if filteredTemplates().length === 0}
          <div class="empty">
            <p>No templates found</p>
            {#if searchQuery}
              <p class="hint">Try a different search term</p>
            {/if}
          </div>
        {:else}
          <ul class="template-list">
            {#each filteredTemplates() as template (template.id)}
              <li class="template-item">
                <button class="template-button" onclick={() => handleTemplateSelect(template)}>
                  <div class="template-info">
                    <span class="template-name">{template.name}</span>
                    <span class="template-description">{template.description}</span>
                  </div>
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      {:else}
        <!-- All Templates List View (no category selected) -->
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search all templates..."
          class="search-input"
        />

        {#if filteredTemplates().length === 0}
          <div class="empty">
            <p>No templates found</p>
            {#if searchQuery}
              <p class="hint">Try a different search term</p>
            {/if}
          </div>
        {:else}
          <!-- Category Filter Chips -->
          <div class="category-chips">
            {#each categories as category (category.id)}
              <button
                class="category-chip"
                onclick={() => handleCategorySelect(category.id)}
                title={category.description}
              >
                <span class="chip-icon">{category.icon}</span>
                <span class="chip-name">{category.name}</span>
              </button>
            {/each}
          </div>

          <ul class="template-list">
            {#each filteredTemplates() as template (template.id)}
              <li class="template-item">
                <button class="template-button" onclick={() => handleTemplateSelect(template)}>
                  <div class="template-icon">
                    {categories.find((c) => c.id === template.category_id)?.icon || '⚙️'}
                  </div>
                  <div class="template-info">
                    <span class="template-name">{template.name}</span>
                    <span class="template-description">{template.description}</span>
                  </div>
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      {/if}
    </div>

    <div class="modal-footer">
      {#if selectedTemplate}
        <button class="cancel-btn" onclick={handleBack}>Back</button>
        <button
          class="generate-btn"
          onclick={handleGenerate}
          disabled={generating}
        >
          {generating ? 'Generating...' : 'Use This Command'}
        </button>
      {:else}
        <button class="cancel-btn" onclick={onCancel}>Cancel</button>
      {/if}
    </div>
  </div>
</div>

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
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .modal-dialog {
    background: var(--color-modal-bg);
    border-radius: 12px;
    box-shadow: 0 20px 60px var(--color-form-shadow);
    width: 700px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
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

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
    border-bottom: 1px solid var(--color-border);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .back-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: 0.25rem;
    line-height: 1;
    transition: color 0.15s;
  }

  .back-btn:hover {
    color: var(--color-text);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: 0.25rem;
    line-height: 1;
    transition: color 0.15s;
  }

  .close-btn:hover {
    color: var(--color-text);
  }

  .modal-body {
    padding: 1.5rem;
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .modal-body::-webkit-scrollbar {
    width: 12px;
  }

  .modal-body::-webkit-scrollbar-track {
    background: var(--color-scrollbar-track);
    border-radius: 0;
  }

  .modal-body::-webkit-scrollbar-thumb {
    background: var(--color-scrollbar-thumb);
    border-radius: 6px;
    border: 2px solid var(--color-scrollbar-track);
  }

  .modal-body::-webkit-scrollbar-thumb:hover {
    background: var(--color-scrollbar-thumb-hover);
  }

  .search-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--color-input-border);
    border-radius: 8px;
    font-size: 0.875rem;
    transition: border-color 0.15s, box-shadow 0.15s;
    margin-bottom: 1rem;
    box-sizing: border-box;
    background: var(--color-input-bg);
    color: var(--color-text);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--color-input-focus-border);
    box-shadow: 0 0 0 3px var(--color-input-focus-shadow);
    background: var(--color-input-focus-bg);
  }

  .loading,
  .error,
  .empty {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem 1rem;
    color: var(--color-text-secondary);
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-input-focus-border);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error {
    color: var(--color-text);
  }

  .hint {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
  }

  /* Category Chips */
  .category-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .category-chip {
    padding: 0.5rem 0.875rem;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 20px;
    cursor: pointer;
    transition: all 0.15s;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.8rem;
  }

  .category-chip:hover {
    border-color: var(--color-input-focus-border);
    background: var(--color-input-focus-bg);
  }

  .chip-icon {
    font-size: 1rem;
    line-height: 1;
  }

  .chip-name {
    font-weight: 500;
    color: var(--color-text);
  }

  /* Template List */
  .template-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .template-item {
    margin: 0;
    padding: 0;
  }

  .template-button {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
  }

  .template-button:hover {
    background: var(--color-surface-secondary);
    border-color: var(--color-input-focus-border);
  }

  .template-icon {
    font-size: 1.5rem;
    line-height: 1;
    flex-shrink: 0;
  }

  .template-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .template-name {
    font-weight: 500;
    color: var(--color-text);
    font-size: 0.9rem;
  }

  .template-description {
    color: var(--color-text-secondary);
    font-size: 0.8rem;
  }

  /* Template Details */
  .template-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .template-details > .template-description {
    color: var(--color-text-secondary);
    font-size: 0.9rem;
    margin: 0;
  }

  .parameters-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .parameters-form h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .parameter-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .parameter-field label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
  }

  .param-input {
    padding: 0.625rem 0.875rem;
    border: 1px solid var(--color-input-border);
    border-radius: 6px;
    font-size: 0.875rem;
    transition: border-color 0.15s, box-shadow 0.15s;
    background: var(--color-input-bg);
    color: var(--color-text);
  }

  .param-input:focus {
    outline: none;
    border-color: var(--color-input-focus-border);
    box-shadow: 0 0 0 3px var(--color-input-focus-shadow);
    background: var(--color-input-focus-bg);
  }

  .param-hint {
    font-size: 0.75rem;
    color: var(--color-text-secondary);
  }

  .no-params {
    color: var(--color-text-secondary);
    font-size: 0.9rem;
    font-style: italic;
  }

  .template-details > .template-description {
    color: var(--color-text-secondary);
    font-size: 0.9rem;
    margin: 0;
  }

  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--color-border);
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  .cancel-btn,
  .generate-btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }

  .cancel-btn {
    background: var(--color-button-secondary-bg);
    border: 1px solid var(--color-button-secondary-border);
    color: var(--color-button-secondary-text);
  }

  .cancel-btn:hover {
    background: var(--color-button-secondary-hover);
    border-color: var(--color-button-secondary-border);
  }

  .generate-btn {
    background: var(--color-button-primary-bg);
    border: 1px solid var(--color-button-primary-bg);
    color: var(--color-button-primary-text);
  }

  .generate-btn:hover:not(:disabled) {
    background: var(--color-button-primary-hover);
    border-color: var(--color-button-primary-hover);
  }

  .generate-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
