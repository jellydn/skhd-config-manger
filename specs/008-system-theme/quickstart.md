# Developer Quickstart - System Theme Support

**Feature**: 008-system-theme
**Prerequisites**: Rust 1.75+, Node.js 18+, macOS 11+ (Big Sur)
**Estimated Setup Time**: 15 minutes

## Quick Start

### 1. Understand the Feature

This feature adds:
- **Automatic theme detection** on application launch
- **Real-time theme updates** when macOS system theme changes
- **Consistent theme application** across all UI components

**Architecture**:
- Rust backend: macOS NSUserDefaults access, theme change monitoring via NSDistributedNotificationCenter
- Svelte frontend: CSS variable management, theme state, reactive UI updates
- Tauri IPC: commands for theme detection, events for theme change notifications

### 2. Development Environment Setup

**Install objc crate dependency**:
```bash
cd src-tauri
# Add to Cargo.toml dependencies:
# objc = "0.2"
cargo add objc
```

**Run development server**:
```bash
# From repository root
bun run tauri dev
```

**Test theme detection**:
- Change macOS system theme in System Settings ? Appearance
- Verify application updates theme automatically

### 3. Key File Locations

**Backend (Rust)**:
```
src-tauri/src/
??? commands/
?   ??? theme.rs           # Theme detection commands (NEW)
??? services/
?   ??? theme_monitor.rs   # Theme change monitoring service (NEW)
??? lib.rs                 # Register theme commands (UPDATE)
```

**Frontend (Svelte)**:
```
src/
??? routes/
?   ??? +layout.svelte     # Theme initialization, CSS variables (UPDATE)
??? services/
?   ??? themeService.ts    # Theme state management (NEW)
??? components/            # All components - replace hardcoded colors (UPDATE)
```

### 4. Implementation Workflow

**Phase 1: Backend - Theme Detection** ?? ~2 hours
1. Create `src-tauri/src/commands/theme.rs`
2. Implement `get_system_theme()` command:
   - Use `objc` crate to access NSUserDefaults
   - Read `AppleInterfaceStyle` key
   - Return `"light"` or `"dark"` string
   - Fallback to `"dark"` on error
3. Write unit tests for theme detection (mocked NSUserDefaults)

**Phase 2: Backend - Theme Monitoring** ?? ~3 hours
1. Create `src-tauri/src/services/theme_monitor.rs`
2. Implement `ThemeMonitor` struct:
   - `start_monitoring(app_handle)` - Subscribe to NSDistributedNotificationCenter
   - `stop_monitoring()` - Unsubscribe from notifications
   - Emit `theme-changed` Tauri event on system theme change
3. Handle fallback to polling if notification API unavailable
4. Write integration tests for theme change events

**Phase 3: Backend - Command Registration** ?? ~30 minutes
1. Register commands in `src-tauri/src/lib.rs`:
   - `get_system_theme`
   - `start_theme_monitor`
   - `stop_theme_monitor`
2. Initialize theme monitor on app startup

**Phase 4: Frontend - Theme Service** ?? ~1 hour
1. Create `src/services/themeService.ts`
2. Implement theme state management:
   - `getTheme()` - Call Tauri command on launch
   - `applyTheme(theme)` - Update CSS variables
   - `listenForThemeChanges()` - Subscribe to `theme-changed` events
3. Define CSS variable mappings for light/dark themes

**Phase 5: Frontend - Layout Integration** ?? ~1 hour
1. Update `src/routes/+layout.svelte`:
   - Call `getTheme()` on mount
   - Call `start_theme_monitor()` on mount
   - Subscribe to `theme-changed` events
   - Remove hardcoded colors from body/sidebar styles
2. Ensure CSS variables are properly defined for both themes

**Phase 6: Frontend - Component Refactoring** ?? ~3-4 hours
1. Audit all `.svelte` component files for hardcoded colors
2. Replace hardcoded hex/rgb colors with CSS variable references:
   - `background: #1e1e1e` ? `background: var(--color-background)`
   - `color: #ffffff` ? `color: var(--color-text)`
3. Test each component in both light and dark themes
4. Verify no visual regressions

**Phase 7: Testing & Polish** ?? ~2 hours
1. Manual testing:
   - Launch app in light mode, verify light theme
   - Launch app in dark mode, verify dark theme
   - Change system theme while app running, verify update
   - Rapidly switch themes, verify all changes captured
2. Visual regression testing:
   - Screenshot all screens in light mode
   - Screenshot all screens in dark mode
   - Compare with baseline
3. Performance testing:
   - Verify theme detection <100ms
   - Verify theme updates <2s
   - Check no UI blocking during transitions

### 5. Testing the Feature

**Unit Tests (Rust)**:
```bash
cd src-tauri
cargo test theme
```

**Integration Tests**:
- Change macOS theme manually in System Settings
- Verify `theme-changed` event is received
- Verify CSS variables update correctly

**Performance Testing Methodology**:

**Theme Detection Performance (T046)**:
- Measure time from app mount to theme application completion
- Use `performance.now()` in browser console or Rust `std::time::Instant`
- Expected: <100ms from get_system_theme() call to CSS variable update
- Test both light and dark mode detection
- Test on macOS 11+ (Big Sur minimum)

**Theme Update Performance (T047)**:
- Measure time from theme-changed event receipt to visual completion
- Use browser DevTools Performance tab to record transition
- Trigger theme change via System Settings while app running
- Expected: <2 seconds from event to complete visual transition
- Verify CSS transitions complete smoothly (check for jank/frame drops)
- Test rapid theme switching (change 5 times rapidly, verify all handled)

**Measurement Approach**:
```typescript
// In themeService.ts, add performance markers:
export function applyTheme(theme: 'light' | 'dark') {
  const start = performance.now();
  const root = document.documentElement;
  // ... update CSS variables ...
  const end = performance.now();
  console.log(`Theme applied in ${end - start}ms`);
}
```

**Automated Performance Tests**:
- Add performance assertions to integration tests
- Fail build if detection >100ms or update >2s in CI
- Use `cargo bench` for Rust backend performance tests

**Manual Testing Checklist**:
- [ ] App launches with correct theme (matches system)
- [ ] Changing system theme updates app within 2 seconds
- [ ] All UI components use correct theme colors
- [ ] No hardcoded colors visible in either theme
- [ ] Theme transitions are smooth (no flickering)
- [ ] App handles theme detection failures gracefully

### 6. Common Issues & Solutions

**Issue**: Theme detection always returns "dark"
- **Solution**: Check NSUserDefaults access permissions, verify objc crate is linked correctly

**Issue**: Theme changes not detected during runtime
- **Solution**: Verify `start_theme_monitor()` was called, check NSDistributedNotificationCenter subscription

**Issue**: Some components still show hardcoded colors
- **Solution**: Audit component styles, ensure all colors use CSS variables

**Issue**: Theme transition is jarring/flickering
- **Solution**: Add CSS transitions to `:root`, ensure all CSS variables update atomically

**Issue**: CSS variables not updating
- **Solution**: Verify `applyTheme()` function updates `document.documentElement.style`, check CSS variable names match

### 7. Architecture Decisions

**Why objc crate?**
- Direct access to macOS NSUserDefaults and NSDistributedNotificationCenter
- More efficient than command-line tools
- Native macOS API integration

**Why CSS variables?**
- Already partially implemented in codebase
- Efficient updates (single DOM operation)
- Works with existing CSS structure
- Smooth transitions via CSS transitions

**Why event-based monitoring?**
- Real-time updates (no polling delay)
- Lower CPU usage than polling
- Native macOS notification system

**Why fallback to dark mode?**
- Maintains current application default behavior
- Non-critical feature (theme detection failure shouldn't break app)
- Users likely expect dark mode if detection fails

### 8. Next Steps After Implementation

1. **Performance Optimization**:
   - Profile theme detection performance
   - Optimize CSS variable updates if needed
   - Ensure no frame drops during transitions

2. **Accessibility**:
   - Verify theme colors meet WCAG contrast requirements
   - Test with accessibility tools (VoiceOver, etc.)

3. **Documentation**:
   - Update README with theme feature description
   - Document theme behavior for users
   - Add troubleshooting guide

4. **Future Enhancements** (Out of Scope):
   - Manual theme override (currently system-only)
   - Custom theme definitions
   - Theme-specific feature variations
