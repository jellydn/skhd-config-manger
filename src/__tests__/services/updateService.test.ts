import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import {
  getUpdatePreferences,
  isMissingUpdaterFeed,
  latestReleaseHasUpdaterFeed,
  parseLastCheck,
  setAutomaticCheck,
  setAutomaticDownload,
  shouldRunAutomaticCheck,
} from '../../services/updateService';

const originalFetch = globalThis.fetch;

afterEach(() => {
  globalThis.fetch = originalFetch;
});

describe('updateService preferences', () => {
  beforeEach(() => {
    if (typeof localStorage === 'undefined') {
      const values = new Map<string, string>();
      Object.defineProperty(globalThis, 'localStorage', {
        configurable: true,
        value: {
          clear: () => values.clear(),
          getItem: (key: string) => values.get(key) ?? null,
          setItem: (key: string, value: string) => values.set(key, value),
        },
      });
    }
    localStorage.clear();
  });

  it('enables daily checks but not downloads by default', () => {
    expect(getUpdatePreferences()).toEqual({
      automaticCheck: true,
      automaticDownload: false,
    });
  });

  it('persists both update preferences', () => {
    setAutomaticCheck(false);
    setAutomaticDownload(true);

    expect(getUpdatePreferences()).toEqual({
      automaticCheck: false,
      automaticDownload: true,
    });
  });
});

describe('automatic update interval', () => {
  const day = 24 * 60 * 60 * 1000;

  it('checks on first launch and after one day', () => {
    expect(shouldRunAutomaticCheck(day, null, true)).toBe(true);
    expect(shouldRunAutomaticCheck(day, 0, true)).toBe(true);
  });

  it('does not check early or when disabled', () => {
    expect(shouldRunAutomaticCheck(day - 1, 0, true)).toBe(false);
    expect(shouldRunAutomaticCheck(day * 2, 0, false)).toBe(false);
  });

  it('treats corrupt saved timestamps as a first check', () => {
    expect(parseLastCheck(null)).toBeNull();
    expect(parseLastCheck('not-a-timestamp')).toBeNull();
    expect(parseLastCheck('86400000')).toBe(86_400_000);
  });
});

describe('updater feed availability', () => {
  it('detects whether the latest release contains updater metadata', async () => {
    globalThis.fetch = (async () =>
      ({
        ok: true,
        json: async () => ({ assets: [{ name: 'keybinder.dmg' }, { name: 'latest.json' }] }),
      }) as Response) as typeof fetch;

    await expect(latestReleaseHasUpdaterFeed()).resolves.toBe(true);
  });

  it('reports a confirmed missing updater feed', async () => {
    globalThis.fetch = (async () =>
      ({
        ok: true,
        json: async () => ({ assets: [{ name: 'keybinder_0.5.0_universal_darwin.dmg' }] }),
      }) as Response) as typeof fetch;

    await expect(latestReleaseHasUpdaterFeed()).resolves.toBe(false);
  });

  it('keeps API failures distinct from a missing feed', async () => {
    globalThis.fetch = (async () => ({ ok: false }) as Response) as typeof fetch;

    await expect(latestReleaseHasUpdaterFeed()).resolves.toBeNull();
  });

  it('classifies only the known error with a confirmed missing feed', () => {
    const missingFeedError = new Error('Could not fetch a valid release JSON from the remote');

    expect(isMissingUpdaterFeed(missingFeedError, false)).toBe(true);
    expect(isMissingUpdaterFeed(missingFeedError, true)).toBe(false);
    expect(isMissingUpdaterFeed(missingFeedError, null)).toBe(false);
    expect(isMissingUpdaterFeed(new Error('network unavailable'), false)).toBe(false);
  });
});
