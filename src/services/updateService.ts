import { writable } from 'svelte/store';
import { isTauri } from '@tauri-apps/api/core';
import { check, type DownloadEvent, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

const AUTOMATIC_CHECK_KEY = 'updates.automaticCheck';
const AUTOMATIC_DOWNLOAD_KEY = 'updates.automaticDownload';
const LAST_CHECK_KEY = 'updates.lastCheck';
const CHECK_INTERVAL_MS = 24 * 60 * 60 * 1000;

export type UpdatePhase =
  | 'idle'
  | 'checking'
  | 'up-to-date'
  | 'available'
  | 'downloading'
  | 'ready'
  | 'installing'
  | 'error';

export interface UpdateState {
  phase: UpdatePhase;
  version: string | null;
  notes: string | null;
  downloadedBytes: number;
  totalBytes: number | null;
  message: string;
}

const initialState: UpdateState = {
  phase: 'idle',
  version: null,
  notes: null,
  downloadedBytes: 0,
  totalBytes: null,
  message: 'Updates are checked against the latest stable GitHub release.',
};

export const updateState = writable<UpdateState>(initialState);

let pendingUpdate: Update | null = null;
let operationActive = false;

function readBoolean(key: string, defaultValue: boolean): boolean {
  if (typeof localStorage === 'undefined') return defaultValue;
  const value = localStorage.getItem(key);
  return value === null ? defaultValue : value === 'true';
}

export function getUpdatePreferences() {
  return {
    automaticCheck: readBoolean(AUTOMATIC_CHECK_KEY, true),
    automaticDownload: readBoolean(AUTOMATIC_DOWNLOAD_KEY, false),
  };
}

export function setAutomaticCheck(enabled: boolean): void {
  localStorage.setItem(AUTOMATIC_CHECK_KEY, String(enabled));
}

export function setAutomaticDownload(enabled: boolean): void {
  localStorage.setItem(AUTOMATIC_DOWNLOAD_KEY, String(enabled));
}

export function shouldRunAutomaticCheck(
  now: number,
  lastCheck: number | null,
  enabled: boolean
): boolean {
  return enabled && (lastCheck === null || now - lastCheck >= CHECK_INTERVAL_MS);
}

export function parseLastCheck(rawLastCheck: string | null): number | null {
  if (rawLastCheck === null) return null;
  const lastCheck = Number(rawLastCheck);
  return Number.isFinite(lastCheck) ? lastCheck : null;
}

export async function checkForUpdates(userInitiated = true): Promise<void> {
  if (operationActive) return;
  operationActive = true;
  updateState.set({ ...initialState, phase: 'checking', message: 'Checking for updates…' });

  try {
    const update = await check({ timeout: 30_000 });
    localStorage.setItem(LAST_CHECK_KEY, String(Date.now()));

    if (!update) {
      updateState.set({
        ...initialState,
        phase: userInitiated ? 'up-to-date' : 'idle',
        message: userInitiated ? 'Keybinder is up to date.' : initialState.message,
      });
      return;
    }

    pendingUpdate = update;
    updateState.set({
      ...initialState,
      phase: 'available',
      version: update.version,
      notes: update.body ?? null,
      message: `Keybinder ${update.version} is available.`,
    });

    if (getUpdatePreferences().automaticDownload) {
      operationActive = false;
      await downloadUpdate();
    }
  } catch (error) {
    updateState.set({
      ...initialState,
      phase: 'error',
      message: `Update check failed: ${String(error)}`,
    });
  } finally {
    operationActive = false;
  }
}

export async function downloadUpdate(): Promise<void> {
  if (!pendingUpdate || operationActive) return;
  operationActive = true;
  let downloadedBytes = 0;
  let totalBytes: number | null = null;

  try {
    await pendingUpdate.download((event: DownloadEvent) => {
      if (event.event === 'Started') {
        totalBytes = event.data.contentLength ?? null;
      } else if (event.event === 'Progress') {
        downloadedBytes += event.data.chunkLength;
      }

      updateState.update((state) => ({
        ...state,
        phase: 'downloading',
        downloadedBytes,
        totalBytes,
        message: `Downloading Keybinder ${state.version}…`,
      }));
    });

    updateState.update((state) => ({
      ...state,
      phase: 'ready',
      message: `Keybinder ${state.version} is ready to install.`,
    }));
  } catch (error) {
    updateState.update((state) => ({
      ...state,
      phase: 'error',
      message: `Update download failed: ${String(error)}`,
    }));
  } finally {
    operationActive = false;
  }
}

export async function installUpdate(): Promise<void> {
  if (!pendingUpdate || operationActive) return;
  operationActive = true;
  updateState.update((state) => ({
    ...state,
    phase: 'installing',
    message: `Installing Keybinder ${state.version}…`,
  }));

  try {
    await pendingUpdate.install();
    await relaunch();
  } catch (error) {
    updateState.update((state) => ({
      ...state,
      phase: 'error',
      message: `Update installation failed: ${String(error)}`,
    }));
    operationActive = false;
  }
}

export async function checkForUpdatesAutomatically(): Promise<void> {
  if (!isTauri()) return;

  const { automaticCheck } = getUpdatePreferences();
  const lastCheck = parseLastCheck(localStorage.getItem(LAST_CHECK_KEY));

  if (shouldRunAutomaticCheck(Date.now(), lastCheck, automaticCheck)) {
    await checkForUpdates(false);
  }
}
