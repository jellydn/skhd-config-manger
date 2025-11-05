# Phase 1: Data Model - System Theme Support

**Feature**: 008-system-theme
**Date**: 2025-11-02

## Entities

### ThemeState

Represents the current active theme mode synchronized with macOS system preferences.

**Fields**:
- `mode`: ThemeMode (enum) - Current theme mode (light or dark)
- `detected_at`: DateTime (ISO 8601) - When theme was last detected/changed
- `source`: ThemeSource (enum) - How theme was determined (system, fallback, etc.)

**Validation Rules**:
- `mode` must be valid ThemeMode value
- `detected_at` must be valid ISO 8601 datetime
- `source` must be valid ThemeSource value

**State Transitions**:
```
Initial ? Light (on launch, system in light mode)
Initial ? Dark (on launch, system in dark mode, or fallback)
Light ? Dark (system theme changed)
Dark ? Light (system theme changed)
```

**Relationships**:
- One active ThemeState instance per application session
- Updated via system theme change notifications or manual detection
- No persistence - always syncs with macOS system preferences

### ThemeMode (Enum)

Represents the available theme modes.

**Values**:
- `light` - Light mode (default macOS light appearance)
- `dark` - Dark mode (default macOS dark appearance)

**Display Mapping**:
- `light` ? Apply light theme CSS variables
- `dark` ? Apply dark theme CSS variables

**Constraints**:
- Only two modes supported (no custom themes per spec)
- Mode is controlled exclusively by macOS system settings (no manual override)

### ThemeSource (Enum)

Indicates how the current theme was determined.

**Values**:
- `system` - Detected from macOS system preferences (primary source)
- `fallback` - Fallback to dark mode when system detection fails
- `media_query` - Detected from CSS media query (browser-level fallback)

**Usage**:
- Used for debugging and logging theme detection issues
- Helps identify when fallback mechanisms are active

### ThemeColors

Represents the set of color values for backgrounds, text, borders, and interactive elements that vary based on active theme.

**Structure**:
Not a traditional entity - implemented as CSS custom properties (CSS variables) in `:root`:

```css
:root {
  --color-background: <light-or-dark-value>;
  --color-surface: <light-or-dark-value>;
  --color-text: <light-or-dark-value>;
  /* ... additional color variables ... */
}
```

**Color Categories**:
- **Backgrounds**: `--color-background`, `--color-surface`, `--color-surface-secondary`
- **Text**: `--color-text`, `--color-text-secondary`, `--color-text-tertiary`
- **Borders**: `--color-border`, `--color-border-hover`
- **Inputs**: `--color-input-bg`, `--color-input-border`, `--color-input-focus-*`
- **Buttons**: `--color-button-primary-*`, `--color-button-secondary-*`
- **Modals**: `--color-modal-backdrop`, `--color-modal-bg`, `--color-modal-border`
- **Scrollbars**: `--color-scrollbar-track`, `--color-scrollbar-thumb`, `--color-scrollbar-thumb-hover`
- **Forms**: `--color-form-bg`, `--color-form-shadow`

**Validation Rules**:
- All color values must be valid CSS color formats (hex, rgb, rgba)
- Light theme colors must provide sufficient contrast for readability
- Dark theme colors must provide sufficient contrast for readability
- No hardcoded color values should exist in component styles (use CSS variables)

**State Transitions**:
```
Light Colors ? Dark Colors (theme changes to dark)
Dark Colors ? Light Colors (theme changes to light)
```

**Relationships**:
- Managed by `themeService.ts` in frontend
- Updated via JavaScript `document.documentElement.style.setProperty()`
- All UI components reference these variables, not hardcoded colors

## Data Flow

### Theme Detection Flow

1. **Application Launch**:
   - Backend: Rust command `get_system_theme()` called
   - Backend: Reads `AppleInterfaceStyle` from NSUserDefaults
   - Backend: Returns `"light"` or `"dark"` string
   - Frontend: Receives theme via Tauri command
   - Frontend: Updates ThemeState and applies CSS variables

2. **Runtime Theme Change**:
   - Backend: NSDistributedNotificationCenter receives `AppleInterfaceThemeChangedNotification`
   - Backend: Emits Tauri event `theme-changed` with new theme
   - Frontend: Listens to `theme-changed` event
   - Frontend: Updates ThemeState and applies CSS variables
   - Frontend: CSS transitions smooth the visual change

3. **Fallback Handling**:
   - If system detection fails: Default to `dark` mode
   - If event monitoring fails: Poll every 1-2 seconds as fallback
   - If JavaScript fails: CSS `@media (prefers-color-scheme: dark)` provides browser-level fallback

## Validation and Constraints

### Theme Detection Constraints

- Theme detection must complete within 100ms on launch (performance requirement)
- Theme updates must complete within 2 seconds (spec requirement)
- Theme detection failures must not crash the application
- Theme detection failures must default to dark mode (maintains current behavior)

### CSS Variable Constraints

- All hardcoded colors must be replaced with CSS variable references
- CSS variables must be defined for both light and dark themes
- No color values should be hardcoded in component styles
- CSS transitions should be smooth (< 200ms) for theme changes

### State Consistency Constraints

- ThemeState must always reflect current macOS system preference
- ThemeState must update immediately when system preference changes
- ThemeState must not persist independent of system settings (no manual override)
- CSS variables must always match current ThemeState mode

## Edge Cases Handled

1. **System theme detection unavailable**: Default to dark mode, log error
2. **Theme change during modal/form input**: Update theme anyway, transitions handle gracefully
3. **Application minimized during theme change**: Update theme when window becomes visible
4. **Rapid theme switching**: Handle all changes, last change wins
5. **macOS version doesn't support theme APIs**: Fallback to CSS media query detection
