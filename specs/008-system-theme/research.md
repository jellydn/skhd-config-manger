# Phase 0: Research - System Theme Support

**Feature**: 008-system-theme
**Date**: 2025-11-02
**Status**: Complete

## Research Areas

### 1. macOS System Theme Detection in Rust/Tauri

**Question**: How can we detect the current macOS system theme (light/dark mode) from Rust in a Tauri application?

**Decision**: Use macOS `UserDefaults` API via Objective-C runtime bindings or `defaults` command-line tool

**Rationale**:
- macOS stores appearance preference in `NSUserDefaults` under key `AppleInterfaceStyle`
- Value is `"Dark"` for dark mode, absent or `nil` for light mode
- Two approaches available:
  1. **Objective-C runtime bindings** (objc crate): Direct API access, more efficient
  2. **Command-line `defaults`**: Simpler, no FFI required, good fallback
- Tauri apps run on macOS and can use native APIs via Rust FFI

**Alternatives Considered**:
- **CSS `prefers-color-scheme` media query**: Only reactive, doesn't provide proactive detection or programmatic access
- **Tauri plugin/window APIs**: Tauri v2 doesn't expose system theme APIs directly
- **Web-based detection**: `window.matchMedia('(prefers-color-scheme: dark)')` works but requires polling, not ideal for Tauri

**Implementation Notes**:
- Use `objc` crate for direct `NSUserDefaults` access (most efficient)
- Fallback to `defaults read -g AppleInterfaceStyle` command if objc unavailable
- Return `"light"` or `"dark"` string from Tauri command
- Handle case where preference is not set (defaults to light mode)

**Code Pattern**:
```rust
// Option 1: Using objc crate (recommended)
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};

fn get_system_theme() -> Result<String, String> {
    let user_defaults = Class::get("NSUserDefaults").ok_or("NSUserDefaults not available")?;
    let standard_defaults: *mut Object = unsafe { msg_send![user_defaults, standardUserDefaults] };
    let style: *mut Object = unsafe { msg_send![standard_defaults, objectForKey: @"AppleInterfaceStyle"] };
    
    if style.is_null() {
        Ok("light".to_string())
    } else {
        // Extract string value - check if it's "Dark"
        Ok("dark".to_string())
    }
}

// Option 2: Using defaults command (fallback)
fn get_system_theme_command() -> Result<String, String> {
    let output = std::process::Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleInterfaceStyle")
        .output()?;
    
    match String::from_utf8_lossy(&output.stdout).trim() {
        "Dark" => Ok("dark".to_string()),
        _ => Ok("light".to_string()),
    }
}
```

### 2. Real-Time Theme Change Monitoring

**Question**: How can we detect when macOS system theme changes during application runtime?

**Decision**: Use macOS `NSDistributedNotificationCenter` to listen for `AppleInterfaceThemeChangedNotification` events

**Rationale**:
- macOS posts `AppleInterfaceThemeChangedNotification` when system appearance changes
- Can subscribe via `NSDistributedNotificationCenter` using objc crate
- Tauri event system can bridge notification to frontend
- Alternative: Poll theme every few seconds (less efficient, higher latency)

**Alternatives Considered**:
- **Polling every 1-2 seconds**: Works but inefficient, higher latency (up to 2s delay)
- **CSS media query listener**: Frontend-only, requires window focus, not ideal for background monitoring
- **Tauri window events**: No system theme change events in Tauri v2

**Implementation Notes**:
- Use `objc` crate to subscribe to `NSDistributedNotificationCenter`
- Create background task/service in Rust that monitors notifications
- Emit Tauri event (`theme-changed`) when notification received
- Frontend subscribes to event and updates CSS variables reactively
- Fallback: If notification API unavailable, poll every 1-2 seconds

**Code Pattern**:
```rust
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};

fn setup_theme_monitor(app_handle: tauri::AppHandle) {
    // Subscribe to AppleInterfaceThemeChangedNotification
    // When received, emit Tauri event with new theme
    // Frontend listens and updates CSS variables
}
```

### 3. CSS Variable Update Strategy

**Question**: What's the best way to update CSS variables dynamically when theme changes?

**Decision**: Use JavaScript to update CSS custom properties on `:root` element, with CSS transitions for smooth changes

**Rationale**:
- CSS variables (`--color-*`) already defined in `+layout.svelte`
- JavaScript can update `document.documentElement.style.setProperty()` efficiently
- CSS transitions can smooth theme changes (< 200ms transition)
- No DOM manipulation needed, just CSS variable updates
- Works with existing `@media (prefers-color-scheme: dark)` as fallback

**Alternatives Considered**:
- **Separate CSS classes**: Would require class toggling on body/html, more complex
- **Inline styles per element**: Too invasive, breaks existing CSS structure
- **CSS-in-JS**: Overkill for this use case, adds dependencies

**Implementation Notes**:
- Create `themeService.ts` that manages theme state
- On theme change event from Tauri, update all CSS variables at once
- Use `requestAnimationFrame` to batch updates for smooth transition
- Ensure all hardcoded colors in components use CSS variables instead
- Add CSS transition on `:root` for smooth theme switching:
  ```css
  :root {
    transition: background-color 0.2s ease, color 0.2s ease;
  }
  ```

**Code Pattern**:
```typescript
// themeService.ts
export function applyTheme(theme: 'light' | 'dark') {
  const root = document.documentElement;
  const colors = theme === 'dark' ? darkThemeColors : lightThemeColors;
  
  Object.entries(colors).forEach(([key, value]) => {
    root.style.setProperty(`--color-${key}`, value);
  });
}
```

### 4. Handling Hardcoded Colors

**Question**: How do we identify and replace all hardcoded color values in the codebase?

**Decision**: Systematic refactoring to replace hardcoded colors with CSS variable references

**Rationale**:
- Current codebase has hardcoded colors in `+layout.svelte` (e.g., `background: #1e1e1e`)
- Need to audit all components for hardcoded colors
- Replace with CSS variable references (e.g., `background: var(--color-background)`)
- Maintain existing CSS variable definitions, just ensure they're used consistently

**Alternatives Considered**:
- **Leave hardcoded colors**: Would break theme switching, unacceptable
- **Sass/LESS variables**: Adds build complexity, CSS variables are sufficient
- **Theme-specific CSS files**: Over-complicated, CSS variables handle this elegantly

**Implementation Notes**:
- Audit all `.svelte` files for hardcoded hex/rgb colors
- Replace with appropriate CSS variable references
- Test both light and dark themes after refactoring
- Ensure no visual regressions (colors look correct in both themes)

**Files Requiring Updates**:
- `src/routes/+layout.svelte` - Remove hardcoded body/sidebar colors
- All component files - Check for hardcoded colors in `<style>` blocks
- Any inline styles in templates

### 5. Fallback and Error Handling

**Question**: What happens if theme detection fails or macOS version doesn't support it?

**Decision**: Graceful fallback to dark mode (current default) with silent error handling

**Rationale**:
- Application currently defaults to dark theme
- If theme detection fails, default to dark mode maintains current behavior
- Don't show errors to users (theme detection failure is non-critical)
- Log errors for debugging purposes
- CSS `@media (prefers-color-scheme: dark)` provides browser-level fallback

**Alternatives Considered**:
- **Show error message**: Theme is non-critical, unnecessary user concern
- **Default to light mode**: Would break existing user experience
- **Disable theme feature**: Poor UX, detection failures should be rare

**Implementation Notes**:
- Wrap theme detection in try-catch, return "dark" on error
- Log errors to console for developers
- Keep CSS media query as ultimate fallback (browser handles if JS fails)
- Test on macOS 11+ to ensure APIs are available

## Resolved Clarifications

All `NEEDS CLARIFICATION` markers from Technical Context are now resolved:

1. **macOS system theme detection API/library**: ? Resolved - Use `objc` crate for NSUserDefaults access, fallback to `defaults` command
2. **Real-time theme monitoring**: ? Resolved - Use NSDistributedNotificationCenter for event-based monitoring
3. **CSS variable update approach**: ? Resolved - JavaScript updates CSS custom properties on `:root`

## Technology Decisions Summary

| Decision | Technology | Rationale |
|----------|-----------|-----------|
| Theme Detection | `objc` crate + NSUserDefaults | Direct macOS API access, efficient |
| Theme Monitoring | NSDistributedNotificationCenter | Event-based, real-time updates |
| CSS Updates | JavaScript + CSS Variables | Efficient, works with existing structure |
| Fallback | Dark mode + CSS media query | Maintains current behavior, multiple fallback layers |

## Dependencies to Add

- **objc crate**: For macOS Objective-C runtime bindings (NSUserDefaults, NSDistributedNotificationCenter)
  - Version: Latest stable (0.2.x)
  - Purpose: Access macOS system APIs for theme detection
  - Alternative: Command-line `defaults` tool (slower, polling-based)

## Implementation Constraints

- Must work on macOS 11+ (Big Sur minimum)
- Theme detection must not block UI thread (<100ms target)
- Theme updates must complete within 2 seconds per spec requirement
- All existing UI components must be audited for hardcoded colors
- Must handle theme detection failures gracefully (default to dark mode)
