/**
 * Log service for managing skhd service log streaming
 *
 * This service provides:
 * - Starting and stopping log stream
 * - Listening for new log entries via events
 * - Managing log buffer and state
 * - Type-safe interfaces for log operations
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { LogEntry } from '../types';

/**
 * Start streaming logs from the skhd service
 *
 * This command spawns a background process that tails the skhd service logs
 * and emits "log-entry" events for each new log line.
 *
 * @throws {string} If stream is already running or failed to start
 *
 * @example
 * ```typescript
 * try {
 *   await startLogStream();
 *   console.log('Log stream started');
 * } catch (error) {
 *   console.error('Failed to start log stream:', error);
 * }
 * ```
 */
export async function startLogStream(): Promise<void> {
  return invoke('start_log_stream');
}

/**
 * Stop the log stream and clean up resources
 *
 * This command kills the background log stream process and stops
 * emitting log-entry events.
 *
 * @throws {string} If stream is not running or failed to stop
 *
 * @example
 * ```typescript
 * try {
 *   await stopLogStream();
 *   console.log('Log stream stopped');
 * } catch (error) {
 *   console.error('Failed to stop log stream:', error);
 * }
 * ```
 */
export async function stopLogStream(): Promise<void> {
  return invoke('stop_log_stream');
}

/**
 * Check if the log stream is currently running
 *
 * @returns {Promise<boolean>} True if stream is active, false otherwise
 *
 * @example
 * ```typescript
 * const isRunning = await isLogStreamRunning();
 * console.log('Stream status:', isRunning ? 'running' : 'stopped');
 * ```
 */
export async function isLogStreamRunning(): Promise<boolean> {
  return invoke('is_log_stream_running');
}

/**
 * Get recent logs from the skhd log file
 *
 * This retrieves historical logs from `/tmp/skhd_<username>.err.log`.
 * Useful for loading logs that were generated before the stream started.
 *
 * @param {number} limit - Maximum number of log lines to retrieve (default: 100)
 * @returns {Promise<LogEntry[]>} Array of parsed log entries
 *
 * @example
 * ```typescript
 * try {
 *   const logs = await getRecentLogs(100);
 *   console.log('Loaded', logs.length, 'historical logs');
 * } catch (error) {
 *   console.error('Failed to load logs:', error);
 * }
 * ```
 */
export async function getRecentLogs(limit?: number): Promise<LogEntry[]> {
  return invoke<LogEntry[]>('get_recent_logs', { limit });
}

/**
 * Listen for new log entries from the stream
 *
 * This function sets up an event listener for "log-entry" events
 * emitted by the backend log tailer.
 *
 * @param {function} callback - Function to call with each new log entry
 * @returns {Promise<UnlistenFn>} Function to call to stop listening
 *
 * @example
 * ```typescript
 * const unlisten = await onLogEntry((entry) => {
 *   console.log(`[${entry.level}] ${entry.message}`);
 * });
 *
 * // Later, stop listening:
 * unlisten();
 * ```
 */
export async function onLogEntry(
  callback: (entry: LogEntry) => void
): Promise<UnlistenFn> {
  return listen<LogEntry>('log-entry', (event) => {
    callback(event.payload);
  });
}

/**
 * Format a log entry timestamp for display
 *
 * @param {string} timestamp - ISO 8601 timestamp string
 * @returns {string} Formatted time string (HH:MM:SS)
 *
 * @example
 * ```typescript
 * const formatted = formatLogTimestamp('2025-11-02T10:15:30Z');
 * // Returns: "10:15:30"
 * ```
 */
export function formatLogTimestamp(timestamp: string): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

/**
 * Get CSS class for log level color coding
 *
 * @param {LogLevel} level - Log level (ERROR, WARN, INFO, DEBUG)
 * @returns {string} Tailwind CSS class for text color
 *
 * @example
 * ```typescript
 * const colorClass = getLogLevelClass('ERROR');
 * // Returns: "text-red-600"
 * ```
 */
export function getLogLevelClass(level: string): string {
  switch (level) {
    case 'ERROR':
      return 'log-level-error';
    case 'WARN':
      return 'log-level-warn';
    case 'INFO':
      return 'log-level-info';
    case 'DEBUG':
      return 'log-level-debug';
    default:
      return 'log-level-default';
  }
}
