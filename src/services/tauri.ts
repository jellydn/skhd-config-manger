/**
 * Tauri API wrapper service
 * Provides type-safe access to Tauri commands
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  ConfigFile,
  Shortcut,
  CreateShortcutRequest,
  UpdateShortcutRequest,
  ValidationResult,
  Backup,
  TestResult,
} from '../types';

/**
 * Configuration Management Commands
 */

/**
 * Detect the active skhd configuration file path
 * Returns the config file path that the running skhd service is using
 */
export async function detectActiveConfig(): Promise<string> {
  return invoke<string>('detect_active_config');
}

/**
 * Load skhd configuration from file
 * @param filePath Optional custom path (defaults to ~/.config/skhd/skhdrc)
 */
export async function loadConfig(filePath?: string): Promise<ConfigFile> {
  return invoke<ConfigFile>('load_config', { filePath });
}

/**
 * Save configuration to file
 * @param config Configuration to save
 */
export async function saveConfig(config: ConfigFile): Promise<void> {
  return invoke('save_config', { config });
}

/**
 * Reload configuration from disk (discarding in-memory changes)
 */
export async function reloadConfig(): Promise<ConfigFile> {
  return invoke<ConfigFile>('reload_config');
}

/**
 * Import configuration from custom file location via file picker
 */
export async function importConfig(): Promise<ConfigFile> {
  return invoke<ConfigFile>('import_config');
}

/**
 * Export configuration to custom file location via file picker
 * @returns Path where configuration was exported
 */
export async function exportConfig(): Promise<string> {
  return invoke<string>('export_config');
}

/**
 * Shortcut Management Commands
 */

/**
 * Create a new shortcut
 * @param request Shortcut creation request
 */
export async function createShortcut(request: CreateShortcutRequest): Promise<Shortcut> {
  return invoke<Shortcut>('create_shortcut', { request });
}

/**
 * Update an existing shortcut
 * @param request Shortcut update request
 */
export async function updateShortcut(request: UpdateShortcutRequest): Promise<Shortcut> {
  return invoke<Shortcut>('update_shortcut', { request });
}

/**
 * Delete a shortcut by ID
 * @param id Shortcut ID
 */
export async function deleteShortcut(id: string): Promise<void> {
  return invoke('delete_shortcut', { id });
}

/**
 * Validation Commands
 */

/**
 * Validate a shortcut
 * @param shortcut Shortcut to validate
 */
export async function validateShortcut(shortcut: Shortcut): Promise<ValidationResult> {
  return invoke<ValidationResult>('validate_shortcut', { shortcut });
}

/**
 * Validate entire configuration
 * @param config Configuration to validate
 */
export async function validateConfig(config: ConfigFile): Promise<ValidationResult> {
  return invoke<ValidationResult>('validate_config', { config });
}

/**
 * Backup Commands
 */

/**
 * Create a backup of the current configuration
 * @param description Optional description for the backup
 */
export async function createBackup(description?: string): Promise<Backup> {
  return invoke<Backup>('create_backup', { description });
}

/**
 * List all available backups
 */
export async function listBackups(): Promise<Backup[]> {
  return invoke<Backup[]>('list_backups');
}

/**
 * Restore configuration from a backup
 * @param backupPath Path to backup file
 */
export async function restoreBackup(backupPath: string): Promise<void> {
  return invoke('restore_backup', { backupPath });
}

/**
 * Testing Commands
 */

/**
 * Test a shortcut (syntax check and preview)
 * @param shortcutId ID of the shortcut to test
 */
export async function testShortcut(shortcutId: string): Promise<TestResult> {
  return invoke<TestResult>('test_shortcut', { shortcutId });
}

/**
 * Execute a shortcut's command in test mode
 * @param shortcutId ID of the shortcut to execute
 */
export async function executeTestCommand(shortcutId: string): Promise<TestResult> {
  return invoke<TestResult>('execute_test_command', { shortcutId });
}
