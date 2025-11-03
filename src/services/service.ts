/**
 * Service control for managing skhd service
 *
 * This service provides:
 * - Getting service status
 * - Reloading the service
 * - Managing service state
 */

import { invoke } from '@tauri-apps/api/core';
import type { ServiceStatus } from '../types';

/**
 * Get the current status of the skhd service
 *
 * @returns {Promise<ServiceStatus>} Current service status
 *
 * @example
 * ```typescript
 * const status = await getServiceStatus();
 * console.log('Service state:', status.state);
 * console.log('Service PID:', status.pid);
 * ```
 */
export async function getServiceStatus(): Promise<ServiceStatus> {
  return invoke('get_service_status');
}

/**
 * Reload the skhd service
 *
 * This will stop and start the service with the current configuration.
 *
 * @throws {string} If reload fails
 *
 * @example
 * ```typescript
 * try {
 *   await reloadService();
 *   console.log('Service reloaded successfully');
 * } catch (error) {
 *   console.error('Failed to reload service:', error);
 * }
 * ```
 */
export async function reloadService(): Promise<void> {
  return invoke('reload_service');
}
