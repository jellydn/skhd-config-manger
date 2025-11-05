/**
 * Theme service for managing macOS system theme detection and application
 * 
 * This service provides functions to apply theme colors via CSS variables
 * and manages theme state synchronization with macOS system preferences.
 * 
 * Uses Kanagawa color scheme (https://github.com/rebelot/kanagawa.nvim)
 * - Dark theme: Kanagawa Wave variant
 * - Light theme: Kanagawa Lotus variant
 */

/**
 * Theme mode type - represents the current theme state
 */
export type ThemeMode = 'light' | 'dark';

/**
 * Light theme color definitions (Kanagawa Lotus)
 */
const lightThemeColors = {
  // Base colors - Kanagawa Lotus palette
  '--color-background': '#FAF6F3', // lotusWave1 - warm white
  '--color-surface': '#FFFFFF', // pure white for cards
  '--color-surface-secondary': '#F5E9DD', // lotusWave2 - light beige
  '--color-border': '#D5C9A1', // lotusWave5 - light border
  '--color-border-hover': '#957FB8', // lotusViolet - purple accent
  '--color-text': '#545464', // lotusWave7 - dark gray
  '--color-text-secondary': '#727169', // lotusWave6 - medium gray
  '--color-text-tertiary': '#9C9C93', // lotusWave5 - light gray
  // Input colors
  '--color-input-bg': '#FFF9F0', // lotusWave0 - very light beige
  '--color-input-border': '#D5C9A1', // lotusWave5
  '--color-input-focus-border': '#7E9CD8', // lotusBlue - blue accent
  '--color-input-focus-bg': '#FFFFFF',
  '--color-input-focus-shadow': 'rgba(126, 156, 216, 0.2)',
  // Button colors
  '--color-button-primary-bg': '#7E9CD8', // lotusBlue
  '--color-button-primary-hover': '#6A8BC5', // darker blue
  '--color-button-primary-active': '#5A7BB5', // even darker
  '--color-button-primary-text': '#FFFFFF',
  '--color-button-primary-focus': 'rgba(126, 156, 216, 0.3)',
  '--color-button-secondary-bg': '#F5E9DD', // lotusWave2
  '--color-button-secondary-hover': '#E8D9C8', // darker beige
  '--color-button-secondary-active': '#DBC9B3', // even darker
  '--color-button-secondary-border': '#D5C9A1', // lotusWave5
  '--color-button-secondary-text': '#545464', // lotusWave7
  '--color-button-secondary-focus': 'rgba(126, 156, 216, 0.2)',
  '--color-button-disabled-bg': '#F5E9DD',
  '--color-button-disabled-text': '#9C9C93',
  '--color-button-disabled-border': '#D5C9A1',
  // Modal colors
  '--color-modal-backdrop': 'rgba(84, 84, 100, 0.5)',
  '--color-modal-bg': '#FFFFFF',
  '--color-modal-border': '#D5C9A1',
  // Scrollbar colors
  '--color-scrollbar-track': '#F5E9DD',
  '--color-scrollbar-thumb': '#C8A384', // lotusPeach2
  '--color-scrollbar-thumb-hover': '#B8967A',
  // Form colors
  '--color-form-bg': '#FFFFFF',
  '--color-form-shadow': 'rgba(84, 84, 100, 0.1)',
  // Status colors - Kanagawa accent colors
  '--color-status-success': '#76946A', // lotusGreen
  '--color-status-success-bg': 'rgba(118, 148, 106, 0.15)',
  '--color-status-success-border': 'rgba(118, 148, 106, 0.3)',
  '--color-status-error': '#C34043', // lotusRed
  '--color-status-error-bg': 'rgba(195, 64, 67, 0.15)',
  '--color-status-error-border': 'rgba(195, 64, 67, 0.3)',
  '--color-status-warning': '#C0A36E', // lotusYellow
  '--color-status-warning-bg': 'rgba(192, 163, 110, 0.15)',
  '--color-status-warning-border': 'rgba(192, 163, 110, 0.3)',
  '--color-status-stopped': '#727169', // lotusWave6
  '--color-status-unknown': '#9C9C93', // lotusWave5
  // Log level colors
  '--color-log-error': '#C34043', // lotusRed
  '--color-log-warn': '#C0A36E', // lotusYellow
  '--color-log-info': '#7E9CD8', // lotusBlue
  '--color-log-debug': '#727169', // lotusWave6
  '--color-log-default': '#545464', // lotusWave7
  // Button variants
  '--color-button-success-bg': 'rgba(118, 148, 106, 0.15)',
  '--color-button-success-border': 'rgba(118, 148, 106, 0.3)',
  '--color-button-success-text': '#76946A',
  '--color-button-success-hover-bg': 'rgba(118, 148, 106, 0.25)',
  '--color-button-success-hover-border': 'rgba(118, 148, 106, 0.4)',
} as const;

/**
 * Dark theme color definitions (Kanagawa Wave)
 */
const darkThemeColors = {
  // Base colors - Kanagawa Wave palette
  '--color-background': '#1F1F28', // waveBg - dark blue-gray
  '--color-surface': '#1F1F28', // waveBg
  '--color-surface-secondary': '#2A2A37', // waveBg2 - slightly lighter
  '--color-border': '#363646', // waveBlue2 - blue-gray border
  '--color-border-hover': '#957FB8', // waveViolet - purple accent
  '--color-text': '#DCD7BA', // waveFg - warm beige
  '--color-text-secondary': '#C8C093', // waveFg2 - lighter beige
  '--color-text-tertiary': '#9CAB88', // waveGreen2 - muted green-gray
  // Input colors
  '--color-input-bg': '#2A2A37', // waveBg2
  '--color-input-border': '#363646', // waveBlue2
  '--color-input-focus-border': '#7E9CD8', // waveBlue - blue accent
  '--color-input-focus-bg': '#1F1F28',
  '--color-input-focus-shadow': 'rgba(126, 156, 216, 0.3)',
  // Button colors
  '--color-button-primary-bg': '#7E9CD8', // waveBlue
  '--color-button-primary-hover': '#6A8BC5', // darker blue
  '--color-button-primary-active': '#5A7BB5', // even darker
  '--color-button-primary-text': '#1F1F28', // dark text for contrast on blue
  '--color-button-primary-focus': 'rgba(126, 156, 216, 0.4)',
  '--color-button-secondary-bg': '#2A2A37', // waveBg2
  '--color-button-secondary-hover': '#363646', // waveBlue2
  '--color-button-secondary-active': '#414350', // darker
  '--color-button-secondary-border': '#363646', // waveBlue2
  '--color-button-secondary-text': '#DCD7BA', // waveFg
  '--color-button-secondary-focus': 'rgba(126, 156, 216, 0.3)',
  '--color-button-disabled-bg': '#2A2A37',
  '--color-button-disabled-text': '#727169', // waveGray
  '--color-button-disabled-border': '#363646',
  // Modal colors
  '--color-modal-backdrop': 'rgba(0, 0, 0, 0.7)',
  '--color-modal-bg': '#2A2A37', // waveBg2
  '--color-modal-border': '#363646', // waveBlue2
  // Scrollbar colors
  '--color-scrollbar-track': '#2A2A37',
  '--color-scrollbar-thumb': '#54546D', // waveBlue1
  '--color-scrollbar-thumb-hover': '#6A6A85',
  // Form colors
  '--color-form-bg': '#1F1F28',
  '--color-form-shadow': 'rgba(0, 0, 0, 0.5)',
  // Status colors - Kanagawa accent colors
  '--color-status-success': '#76946A', // waveGreen
  '--color-status-success-bg': 'rgba(118, 148, 106, 0.15)',
  '--color-status-success-border': 'rgba(118, 148, 106, 0.3)',
  '--color-status-error': '#C34043', // waveRed
  '--color-status-error-bg': 'rgba(195, 64, 67, 0.15)',
  '--color-status-error-border': 'rgba(195, 64, 67, 0.3)',
  '--color-status-warning': '#C0A36E', // waveYellow
  '--color-status-warning-bg': 'rgba(192, 163, 110, 0.15)',
  '--color-status-warning-border': 'rgba(192, 163, 110, 0.3)',
  '--color-status-stopped': '#727169', // waveGray
  '--color-status-unknown': '#9CAB88', // waveGreen2
  // Log level colors
  '--color-log-error': '#C34043', // waveRed
  '--color-log-warn': '#C0A36E', // waveYellow
  '--color-log-info': '#7E9CD8', // waveBlue
  '--color-log-debug': '#727169', // waveGray
  '--color-log-default': '#DCD7BA', // waveFg
  // Button variants
  '--color-button-success-bg': 'rgba(118, 148, 106, 0.15)',
  '--color-button-success-border': 'rgba(118, 148, 106, 0.3)',
  '--color-button-success-text': '#76946A',
  '--color-button-success-hover-bg': 'rgba(118, 148, 106, 0.25)',
  '--color-button-success-hover-border': 'rgba(118, 148, 106, 0.4)',
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
