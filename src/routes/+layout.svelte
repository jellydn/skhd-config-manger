<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';

  // Current route path
  let currentPath = $derived($page.url.pathname);

  // Sidebar collapse state
  let isCollapsed = $state(false);

  // Load collapse state from localStorage
  onMount(() => {
    const saved = localStorage.getItem('sidebarCollapsed');
    if (saved !== null) {
      isCollapsed = saved === 'true';
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
      <button class="collapse-btn" onclick={toggleSidebar} aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="4 17 10 11 4 5"></polyline>
          <line x1="12" y1="19" x2="20" y2="19"></line>
        </svg>
        {#if !isCollapsed}
          <span>Service Manager</span>
        {/if}
      </a>
    </nav>

    <div class="sidebar-footer">
      <div class="service-status">
        <div class="status-indicator status-running"></div>
        {#if !isCollapsed}
          <span class="status-text">skhd running</span>
        {/if}
      </div>
    </div>
  </aside>

  <!-- Main Content -->
  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Helvetica Neue', sans-serif;
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
    background: #1c1c1c;
    border-right: 1px solid #2d2d2d;
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
    border-bottom: 1px solid #2d2d2d;
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
    color: #ffffff;
    margin: 0;
    text-transform: uppercase;
    opacity: 0.6;
    white-space: nowrap;
  }

  .collapse-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.5);
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
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
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
    color: rgba(255, 255, 255, 0.75);
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
    background: rgba(255, 255, 255, 0.06);
    color: rgba(255, 255, 255, 0.95);
  }

  .nav-item.active {
    background: rgba(10, 132, 255, 0.15);
    color: #0a84ff;
    font-weight: 500;
  }

  .nav-item.active svg {
    opacity: 1;
  }

  .sidebar-footer {
    padding: 12px 16px;
    border-top: 1px solid #2d2d2d;
  }

  .sidebar.collapsed .sidebar-footer {
    padding: 12px;
    display: flex;
    justify-content: center;
  }

  .service-status {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
  }

  .sidebar.collapsed .service-status {
    justify-content: center;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-running {
    background: #30d158;
    box-shadow: 0 0 6px rgba(48, 209, 88, 0.4);
  }

  .status-text {
    font-weight: 500;
  }

  /* Main Content Area */
  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background: #1e1e1e;
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
    background: rgba(255, 255, 255, 0.15);
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: rgba(255, 255, 255, 0.25);
  }
</style>
