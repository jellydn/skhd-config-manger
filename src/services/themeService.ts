/**
 * Theme service for managing macOS system theme detection and application
 * 
 * This service provides functions to apply theme colors via CSS variables
 * and manages theme state synchronization with macOS system preferences.
 */

/**
 * Theme mode type - represents the current theme state
 */
export type ThemeMode = 'light' | 'dark';

/**
 * Light theme color definitions
 */
const lightThemeColors = {
  '--color-background': '#ffffff',
  '--color-surface': '#ffffff',
  '--color-surface-secondary': '#f9fafb',
  '--color-border': '#e5e7eb',
  '--color-border-hover': '#3b82f6',
  '--color-text': '#111827',
  '--color-text-secondary': '#6b7280',
  '--color-text-tertiary': '#9ca3af',
  '--color-input-bg': '#fafafa',
  '--color-input-border': '#e5e5e7',
  '--color-input-focus-border': '#007aff',
  '--color-input-focus-bg': '#ffffff',
  '--color-input-focus-shadow': 'rgba(0, 122, 255, 0.1)',
  '--color-button-primary-bg': '#007aff',
  '--color-button-primary-hover': '#0051d5',
  '--color-button-primary-active': '#0040b3',
  '--color-button-primary-text': '#ffffff',
  '--color-button-primary-focus': 'rgba(0, 122, 255, 0.3)',
  '--color-button-secondary-bg': '#f5f5f7',
  '--color-button-secondary-hover': '#e8e8ed',
  '--color-button-secondary-active': '#d1d5db',
  '--color-button-secondary-border': '#d1d5db',
  '--color-button-secondary-text': '#111827',
  '--color-button-secondary-focus': 'rgba(0, 122, 255, 0.2)',
  '--color-button-disabled-bg': '#f9fafb',
  '--color-button-disabled-text': '#9ca3af',
  '--color-button-disabled-border': '#e5e7eb',
  '--color-modal-backdrop': 'rgba(0, 0, 0, 0.5)',
  '--color-modal-bg': '#ffffff',
  '--color-modal-border': '#e5e7eb',
  '--color-scrollbar-track': '#f0f0f0',
  '--color-scrollbar-thumb': '#b0b0b0',
  '--color-scrollbar-thumb-hover': '#909090',
  '--color-form-bg': '#ffffff',
  '--color-form-shadow': 'rgba(0, 0, 0, 0.08)',
  // Status colors
  '--color-status-success': '#34c759',
  '--color-status-success-bg': 'rgba(52, 199, 89, 0.15)',
  '--color-status-success-border': 'rgba(52, 199, 89, 0.3)',
  '--color-status-error': '#ff3b30',
  '--color-status-error-bg': 'rgba(255, 59, 48, 0.15)',
  '--color-status-error-border': 'rgba(255, 59, 48, 0.3)',
  '--color-status-warning': '#ff9500',
  '--color-status-warning-bg': 'rgba(255, 149, 0, 0.1)',
  '--color-status-warning-border': 'rgba(255, 149, 0, 0.3)',
  '--color-status-stopped': '#8e8e93',
  '--color-status-unknown': '#636366',
  // Log level colors
  '--color-log-error': '#d70015',
  '--color-log-warn': '#ff9500',
  '--color-log-info': '#007aff',
  '--color-log-debug': '#6b7280',
  '--color-log-default': '#6b7280',
  // Button variants
  '--color-button-success-bg': 'rgba(52, 199, 89, 0.15)',
  '--color-button-success-border': 'rgba(52, 199, 89, 0.3)',
  '--color-button-success-text': '#34c759',
  '--color-button-success-hover-bg': 'rgba(52, 199, 89, 0.25)',
  '--color-button-success-hover-border': 'rgba(52, 199, 89, 0.4)',
} as const;

/**
 * Dark theme color definitions
 */
const darkThemeColors = {
  '--color-background': '#1e1e1e',
  '--color-surface': '#1e1e1e',
  '--color-surface-secondary': '#1f2937',
  '--color-border': '#374151',
  '--color-border-hover': '#3b82f6',
  '--color-text': '#f9fafb',
  '--color-text-secondary': '#9ca3af',
  '--color-text-tertiary': '#6b7280',
  '--color-input-bg': '#2a2a2a',
  '--color-input-border': '#3a3a3a',
  '--color-input-focus-border': '#007aff',
  '--color-input-focus-bg': '#1e1e1e',
  '--color-input-focus-shadow': 'rgba(0, 122, 255, 0.2)',
  '--color-button-primary-bg': '#007aff',
  '--color-button-primary-hover': '#0051d5',
  '--color-button-primary-active': '#0040b3',
  '--color-button-primary-text': '#ffffff',
  '--color-button-primary-focus': 'rgba(0, 122, 255, 0.4)',
  '--color-button-secondary-bg': '#2a2a2a',
  '--color-button-secondary-hover': '#3a3a3a',
  '--color-button-secondary-active': '#4a4a4a',
  '--color-button-secondary-border': '#3a3a3a',
  '--color-button-secondary-text': '#f5f5f7',
  '--color-button-secondary-focus': 'rgba(0, 122, 255, 0.3)',
  '--color-button-disabled-bg': '#1f2937',
  '--color-button-disabled-text': '#6b7280',
  '--color-button-disabled-border': '#374151',
  '--color-modal-backdrop': 'rgba(0, 0, 0, 0.7)',
  '--color-modal-bg': '#1f2937',
  '--color-modal-border': '#374151',
  '--color-scrollbar-track': '#2a2a2a',
  '--color-scrollbar-thumb': '#505050',
  '--color-scrollbar-thumb-hover': '#606060',
  '--color-form-bg': '#1e1e1e',
  '--color-form-shadow': 'rgba(0, 0, 0, 0.4)',
  // Status colors
  '--color-status-success': '#30d158',
  '--color-status-success-bg': 'rgba(48, 209, 88, 0.15)',
  '--color-status-success-border': 'rgba(48, 209, 88, 0.3)',
  '--color-status-error': '#ff3b30',
  '--color-status-error-bg': 'rgba(255, 59, 48, 0.15)',
  '--color-status-error-border': 'rgba(255, 59, 48, 0.3)',
  '--color-status-warning': '#ff9500',
  '--color-status-warning-bg': 'rgba(255, 149, 0, 0.1)',
  '--color-status-warning-border': 'rgba(255, 149, 0, 0.3)',
  '--color-status-stopped': '#8e8e93',
  '--color-status-unknown': '#636366',
  // Log level colors
  '--color-log-error': '#f48771',
  '--color-log-warn': '#dcdcaa',
  '--color-log-info': '#4fc1ff',
  '--color-log-debug': '#b5b5b5',
  '--color-log-default': '#d4d4d4',
  // Button variants
  '--color-button-success-bg': 'rgba(48, 209, 88, 0.15)',
  '--color-button-success-border': 'rgba(48, 209, 88, 0.3)',
  '--color-button-success-text': '#30d158',
  '--color-button-success-hover-bg': 'rgba(48, 209, 88, 0.25)',
  '--color-button-success-hover-border': 'rgba(48, 209, 88, 0.4)',
} as const;

/**
 * Apply theme colors to the document root element
 * 
 * Updates all CSS custom properties (CSS variables) on :root to match
 * the specified theme. This enables dynamic theme switching without
 * page reload.
 * 
 * @param theme - The theme mode to apply ('light' or 'dark')
 */
export function applyTheme(theme: ThemeMode): void {
  const root = document.documentElement;
  const colors = theme === 'dark' ? darkThemeColors : lightThemeColors;
  
  // Update all CSS variables atomically
  Object.entries(colors).forEach(([key, value]) => {
    root.style.setProperty(key, value);
  });
}
