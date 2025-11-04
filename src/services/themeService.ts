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
  '--color-button-primary-text': '#ffffff',
  '--color-button-secondary-bg': '#f5f5f7',
  '--color-button-secondary-hover': '#e8e8ed',
  '--color-button-secondary-border': '#e5e5e7',
  '--color-button-secondary-text': '#1d1d1f',
  '--color-modal-backdrop': 'rgba(0, 0, 0, 0.5)',
  '--color-modal-bg': '#ffffff',
  '--color-modal-border': '#e5e7eb',
  '--color-scrollbar-track': '#f0f0f0',
  '--color-scrollbar-thumb': '#b0b0b0',
  '--color-scrollbar-thumb-hover': '#909090',
  '--color-form-bg': '#ffffff',
  '--color-form-shadow': 'rgba(0, 0, 0, 0.08)',
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
  '--color-button-primary-text': '#ffffff',
  '--color-button-secondary-bg': '#2a2a2a',
  '--color-button-secondary-hover': '#3a3a3a',
  '--color-button-secondary-border': '#3a3a3a',
  '--color-button-secondary-text': '#f5f5f7',
  '--color-modal-backdrop': 'rgba(0, 0, 0, 0.7)',
  '--color-modal-bg': '#1f2937',
  '--color-modal-border': '#374151',
  '--color-scrollbar-track': '#2a2a2a',
  '--color-scrollbar-thumb': '#505050',
  '--color-scrollbar-thumb-hover': '#606060',
  '--color-form-bg': '#1e1e1e',
  '--color-form-shadow': 'rgba(0, 0, 0, 0.4)',
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
