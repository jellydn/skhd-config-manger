import { beforeEach, describe, expect, it } from 'vitest';
import {
  getUpdatePreferences,
  parseLastCheck,
  setAutomaticCheck,
  setAutomaticDownload,
  shouldRunAutomaticCheck,
} from '../../services/updateService';

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
