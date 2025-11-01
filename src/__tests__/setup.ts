/**
 * Vitest setup file
 * Runs before all tests
 */

import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// Mock Tauri API for frontend tests
const mockTauri = {
  invoke: vi.fn(),
};

if (typeof window !== 'undefined') {
  Object.defineProperty(window, '__TAURI__', {
    value: mockTauri,
    writable: true,
  });
}
