<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/stores';
  import type { Snippet } from 'svelte';
  import { applyTheme } from '../services/themeService';
  import { getSystemTheme, startThemeMonitor, stopThemeMonitor } from '../services/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { checkForUpdatesAutomatically } from '../services/updateService';

  // Props
  let { children }: { children: Snippet } = $props();

  // Current route path
  let currentPath = $derived($page.url.pathname);

  // Sidebar collapse state
  let isCollapsed = $state(false);

  // Store unsubscribe function for cleanup
  let unsubscribeThemeListener: (() => void) | null = $state(null);

  // Load collapse state from localStorage and detect system theme on mount
  onMount(async () => {
    // Load sidebar collapse state
    const saved = localStorage.getItem('sidebarCollapsed');
    if (saved !== null) {
      isCollapsed = saved === 'true';
    }

    // Detect and apply system theme on launch
    try {
      const theme = await getSystemTheme();
      applyTheme(theme);
    } catch (error) {
      // Fallback to dark mode on error (maintains current default behavior)
      console.error('Failed to detect system theme, defaulting to dark mode:', error);
      applyTheme('dark');
    }

    // Start theme monitoring for real-time updates
    try {
      await startThemeMonitor();
      console.log('Theme monitoring started');
    } catch (error) {
      console.error('Failed to start theme monitoring:', error);
    }

    // Listen for theme change events
    try {
      const unsubscribe = await listen<{ theme: string; timestamp: string }>(
        'theme-changed',
        (event) => {
          const newTheme = event.payload.theme as 'light' | 'dark';
          console.log('System theme changed to:', newTheme);
          applyTheme(newTheme);
        }
      );
      unsubscribeThemeListener = unsubscribe;
    } catch (error) {
      console.error('Failed to set up theme change listener:', error);
    }

    void checkForUpdatesAutomatically();
  });

  // Cleanup on destroy
  onDestroy(async () => {
    if (unsubscribeThemeListener) {
      unsubscribeThemeListener();
    }
    try {
      await stopThemeMonitor();
      console.log('Theme monitoring stopped');
    } catch (error) {
      console.error('Failed to stop theme monitoring:', error);
    }
  });

  // Toggle sidebar collapse
  function toggleSidebar() {
    isCollapsed = !isCollapsed;
    localStorage.setItem('sidebarCollapsed', String(isCollapsed));
  }
</script>

<div class="app">
  <!-- Sidebar Navigation -->
  <aside class="sidebar" class:collapsed={isCollapsed}>
    <div class="sidebar-header">
      {#if !isCollapsed}
        <h1 class="app-title">Keybinder</h1>
      {/if}
      <button
        class="collapse-btn"
        onclick={toggleSidebar}
        aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          {#if isCollapsed}
            <polyline points="9 18 15 12 9 6"></polyline>
          {:else}
            <polyline points="15 18 9 12 15 6"></polyline>
          {/if}
        </svg>
      </button>
    </div>

    <nav class="sidebar-nav">
      <a
        href="/"
        class="nav-item"
        class:active={currentPath === '/'}
        aria-current={currentPath === '/' ? 'page' : undefined}
        title="Shortcuts"
      >
        <!-- Keyboard icon -->
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <rect x="2" y="4" width="20" height="16" rx="2"></rect>
          <path d="M6 8h.01M10 8h.01M14 8h.01M18 8h.01M8 12h.01M12 12h.01M16 12h.01M7 16h10"></path>
        </svg>
        {#if !isCollapsed}
          <span>Shortcuts</span>
        {/if}
      </a>

      <a
        href="/logs"
        class="nav-item"
        class:active={currentPath === '/logs'}
        aria-current={currentPath === '/logs' ? 'page' : undefined}
        title="Service Manager"
      >
        <!-- Terminal/Activity icon -->
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <polyline points="4 17 10 11 4 5"></polyline>
          <line x1="12" y1="19" x2="20" y2="19"></line>
        </svg>
        {#if !isCollapsed}
          <span>Service Manager</span>
        {/if}
      </a>

      <a
        href="/settings"
        class="nav-item"
        class:active={currentPath === '/settings'}
        aria-current={currentPath === '/settings' ? 'page' : undefined}
        title="Settings"
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="3"></circle>
          <path
            d="M19.4 15a1.7 1.7 0 0 0 .34 1.88l.06.06-2.83 2.83-.06-.06A1.7 1.7 0 0 0 15 19.4a1.7 1.7 0 0 0-1 .6 1.7 1.7 0 0 0-.4 1.1V21h-4v-.09A1.7 1.7 0 0 0 8.6 19.4a1.7 1.7 0 0 0-1.88.34l-.06.06-2.83-2.83.06-.06A1.7 1.7 0 0 0 4.6 15a1.7 1.7 0 0 0-.6-1 1.7 1.7 0 0 0-1.1-.4H3v-4h.09A1.7 1.7 0 0 0 4.6 8.6a1.7 1.7 0 0 0-.34-1.88l-.06-.06 2.83-2.83.06.06A1.7 1.7 0 0 0 9 4.6a1.7 1.7 0 0 0 1-.6 1.7 1.7 0 0 0 .4-1.1V3h4v.09A1.7 1.7 0 0 0 15.4 4.6a1.7 1.7 0 0 0 1.88-.34l.06-.06 2.83 2.83-.06.06A1.7 1.7 0 0 0 19.4 9c.12.39.33.74.6 1 .29.28.68.42 1.09.4H21v4h-.09c-.41-.02-.8.12-1.09.4-.27.26-.48.61-.6 1Z"
          ></path>
        </svg>
        {#if !isCollapsed}
          <span>Settings</span>
        {/if}
      </a>
    </nav>
  </aside>

  <!-- Main Content -->
  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
  /* Color System - Light and Dark Theme Variables */
  /* Default to light theme - will be overridden by themeService.ts */
  /* Add smooth transitions for theme changes */
  :global(:root) {
    transition:
      background-color 0.2s ease,
      color 0.2s ease,
      border-color 0.2s ease;
    /* Light Theme Colors (default) */
    --color-background: #ffffff;
    --color-surface: #ffffff;
    --color-surface-secondary: #f9fafb;
    --color-border: #e5e7eb;
    --color-border-hover: #3b82f6;
    --color-text: #111827;
    --color-text-secondary: #6b7280;
    --color-text-tertiary: #9ca3af;

    /* Input Colors */
    --color-input-bg: #fafafa;
    --color-input-border: #e5e5e7;
    --color-input-focus-border: #007aff;
    --color-input-focus-bg: #ffffff;
    --color-input-focus-shadow: rgba(0, 122, 255, 0.1);

    /* Button Colors */
    --color-button-primary-bg: #007aff;
    --color-button-primary-hover: #0051d5;
    --color-button-primary-text: #ffffff;
    --color-button-secondary-bg: #f5f5f7;
    --color-button-secondary-hover: #e8e8ed;
    --color-button-secondary-border: #e5e5e7;
    --color-button-secondary-text: #1d1d1f;

    /* Modal Colors */
    --color-modal-backdrop: rgba(0, 0, 0, 0.5);
    --color-modal-bg: #ffffff;
    --color-modal-border: #e5e7eb;

    /* Scrollbar Colors */
    --color-scrollbar-track: #f0f0f0;
    --color-scrollbar-thumb: #b0b0b0;
    --color-scrollbar-thumb-hover: #909090;

    /* Form Colors */
    --color-form-bg: #ffffff;
    --color-form-shadow: rgba(0, 0, 0, 0.08);
  }

  /* Note: Theme is now handled via JavaScript (themeService.ts) */
  /* CSS variables are set dynamically based on macOS system theme */

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    background: #1e1e1e;
    color: #ffffff;
    overflow: hidden;
  }

  .app {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  /* Sidebar - Native macOS style */
  .sidebar {
    width: 200px;
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    transition: width 0.3s ease;
  }

  .sidebar.collapsed {
    width: 60px;
  }

  .sidebar-header {
    padding: 20px 16px 12px;
    border-bottom: 1px solid var(--color-border);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
  }

  .sidebar.collapsed .sidebar-header {
    padding: 20px 12px 12px;
    justify-content: center;
  }

  .app-title {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.3px;
    color: var(--color-text);
    margin: 0;
    text-transform: uppercase;
    opacity: 0.6;
    white-space: nowrap;
  }

  .collapse-btn {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    padding: 4px;
    cursor: pointer;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .collapse-btn:hover {
    background: var(--color-surface-secondary);
    color: var(--color-text);
  }

  .sidebar-nav {
    flex: 1;
    padding: 8px;
    overflow-y: auto;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 10px;
    margin-bottom: 2px;
    border-radius: 6px;
    color: var(--color-text-secondary);
    text-decoration: none;
    font-size: 13px;
    font-weight: 400;
    transition: all 0.15s ease;
    cursor: pointer;
    white-space: nowrap;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: 8px;
  }

  .nav-item svg {
    opacity: 0.8;
    flex-shrink: 0;
  }

  .nav-item:hover {
    background: var(--color-surface-secondary);
    color: var(--color-text);
  }

  .nav-item.active {
    background: var(--color-surface-secondary);
    color: var(--color-border-hover);
    font-weight: 500;
  }

  .nav-item.active svg {
    opacity: 1;
  }

  /* Main Content Area */
  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background: var(--color-background);
  }

  /* Scrollbar styling for macOS look */
  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: transparent;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: var(--color-scrollbar-thumb);
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: var(--color-scrollbar-thumb-hover);
  }
</style>
