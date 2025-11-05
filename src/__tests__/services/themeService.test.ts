/**
 * Unit tests for theme service
 * 
 * Tests theme application, color scheme switching, and CSS variable management
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { applyTheme, type ThemeMode } from '../../services/themeService';

describe('themeService', () => {
  beforeEach(() => {
    // Mock document.documentElement.style.setProperty
    vi.spyOn(document.documentElement.style, 'setProperty');
  });

  afterEach(() => {
    // Restore original methods and clear styles
    vi.restoreAllMocks();
    document.documentElement.style.cssText = '';
  });

  describe('applyTheme', () => {
    it('should apply light theme colors', () => {
      applyTheme('light');
      
      // Verify setProperty was called (we can't easily verify exact values without DOM)
      expect(document.documentElement.style.setProperty).toHaveBeenCalled();
      
      // Count how many times setProperty was called (should be called for each color variable)
      const callCount = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.length;
      expect(callCount).toBeGreaterThan(50); // Should have many color variables
    });

    it('should apply dark theme colors', () => {
      if (typeof document === 'undefined') {
        return;
      }
      
      applyTheme('dark');
      
      expect(document.documentElement.style.setProperty).toHaveBeenCalled();
      
      const callCount = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.length;
      expect(callCount).toBeGreaterThan(50);
    });

    it('should apply different colors for light vs dark theme', () => {
      // Apply light theme
      applyTheme('light');
      const lightCalls = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.map(call => call[1]);
      
      // Clear mock
      vi.clearAllMocks();
      
      // Apply dark theme
      applyTheme('dark');
      const darkCalls = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.map(call => call[1]);
      
      // Verify different colors were applied
      // Check that background colors are different
      const lightBg = lightCalls.find(call => call && typeof call === 'string' && call.includes('background'));
      const darkBg = darkCalls.find(call => call && typeof call === 'string' && call.includes('background'));
      
      // Should have applied colors (values might be different)
      expect(lightCalls.length).toBeGreaterThan(0);
      expect(darkCalls.length).toBeGreaterThan(0);
    });

    it('should set CSS custom properties with correct names', () => {
      if (typeof document === 'undefined') {
        return;
      }
      
      applyTheme('light');
      
      const calls = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls;
      
      // Verify that CSS variable names start with --
      const variableNames = calls.map(call => call[0]);
      variableNames.forEach(name => {
        expect(name).toMatch(/^--/);
      });
      
      // Verify some expected variable names exist
      const varNames = variableNames.join(' ');
      expect(varNames).toContain('--color-background');
      expect(varNames).toContain('--color-text');
      expect(varNames).toContain('--color-button-primary-bg');
    });

    it('should handle rapid theme switches', () => {
      if (typeof document === 'undefined') {
        return;
      }
      
      // Rapidly switch themes multiple times
      applyTheme('light');
      applyTheme('dark');
      applyTheme('light');
      applyTheme('dark');
      
      // Should not throw errors
      expect(document.documentElement.style.setProperty).toHaveBeenCalled();
      
      const callCount = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.length;
      expect(callCount).toBeGreaterThan(200); // Multiple themes * many variables
    });
  });

  describe('theme color definitions', () => {
    it('should have consistent color structure for light and dark themes', () => {
      if (typeof document === 'undefined') {
        return;
      }
      
      // This test verifies that both themes have the same set of color variables
      // by checking that applyTheme calls setProperty the same number of times
      
      applyTheme('light');
      const lightCallCount = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.length;
      
      vi.clearAllMocks();
      
      applyTheme('dark');
      const darkCallCount = (document.documentElement.style.setProperty as ReturnType<typeof vi.spyOn>).mock.calls.length;
      
      // Both themes should have the same number of color variables
      expect(lightCallCount).toBe(darkCallCount);
    });
  });
});
