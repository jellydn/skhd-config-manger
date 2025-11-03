# Manual Test Checklist - Feature 006: Service Manager

**Branch**: `006-service-log-reload`
**Date**: 2025-11-03
**Tester**: _________________

## Pre-Testing Setup

- [ ] skhd service is installed (`which skhd`)
- [ ] skhd service is running (`launchctl list | grep skhd`)
- [ ] Active config exists at `~/.skhdrc` or `~/.config/skhd/skhdrc`
- [ ] Backup current config before testing

---

## User Story 1 - View Service Logs (Priority: P1)

### Test Case 1.1: Initial Log Display
**Objective**: Verify logs display on page load

**Steps**:
1. Open Keybinder application
2. Navigate to "Service Manager" page
3. Observe log viewer area

**Expected Results**:
- [ ] Logs are visible in chronological order
- [ ] Timestamps are displayed in readable format (HH:MM:SS)
- [ ] Log levels are visible ([ERROR], [WARN], [INFO], [DEBUG])
- [ ] Footer shows "Total logs: X"
- [ ] Status shows "● Streaming" or "○ Stopped"

**Actual Result**: _________________

---

### Test Case 1.2: Real-Time Log Streaming
**Objective**: Verify new logs appear automatically

**Steps**:
1. Start log stream if not already streaming (click "Start Stream")
2. Trigger skhd activity (press a configured hotkey)
3. Observe log viewer

**Expected Results**:
- [ ] New log entries appear within 1 second
- [ ] Auto-scroll takes viewer to bottom
- [ ] Streaming status shows "● Streaming"
- [ ] No performance lag with rapid log generation

**Actual Result**: _________________

---

### Test Case 1.3: Color-Coded Log Levels
**Objective**: Verify visual distinction between log levels

**Steps**:
1. Examine logs in viewer
2. Look for different log levels (ERROR, WARN, INFO, DEBUG)

**Expected Results**:
- [ ] ERROR logs are red (#f48771)
- [ ] WARN logs are yellow (#dcdcaa)
- [ ] INFO logs are blue (#4fc1ff)
- [ ] DEBUG logs are gray (#b5b5b5)
- [ ] Color coding is consistent and readable

**Actual Result**: _________________

---

### Test Case 1.4: Virtual Scrolling Performance
**Objective**: Verify performance with large log volumes

**Steps**:
1. Let log stream run until 100+ logs accumulated
2. Scroll up and down through logs
3. Observe performance and responsiveness

**Expected Results**:
- [ ] Scrolling is smooth (no lag)
- [ ] Only visible logs are rendered (check DOM)
- [ ] Total log count is accurate in footer
- [ ] No memory issues or slowdown

**Actual Result**: _________________

---

### Test Case 1.5: Sort Order Toggle
**Objective**: Verify sort order changes work correctly

**Steps**:
1. Click sort toggle button (should show "↑ Oldest First" or "↓ Newest First")
2. Observe log order
3. Click again to toggle back
4. Refresh page and check persistence

**Expected Results**:
- [ ] Button label updates correctly
- [ ] Logs reverse order when toggled
- [ ] Newest timestamp is first (descending) or last (ascending)
- [ ] Sort preference persists after page refresh (localStorage)

**Actual Result**: _________________

---

### Test Case 1.6: Start/Stop Stream Control
**Objective**: Verify stream control buttons work

**Steps**:
1. Click "Stop Stream" button
2. Trigger skhd activity
3. Observe log viewer
4. Click "Start Stream" button
5. Trigger skhd activity again

**Expected Results**:
- [ ] Stream stops (status shows "○ Stopped")
- [ ] No new logs appear while stopped
- [ ] Stream resumes when started
- [ ] New logs appear after restart
- [ ] Button toggles between "Start Stream" and "Stop Stream"

**Actual Result**: _________________

---

### Test Case 1.7: Clear Logs
**Objective**: Verify clear logs functionality

**Steps**:
1. Ensure some logs are displayed
2. Click "Clear Logs" button
3. Observe log viewer and footer

**Expected Results**:
- [ ] All logs are removed from viewer
- [ ] Footer shows "Total logs: 0"
- [ ] Clear button is disabled (no logs to clear)
- [ ] New logs can still be streamed after clearing

**Actual Result**: _________________

---

### Test Case 1.8: Auto-Scroll Behavior
**Objective**: Verify auto-scroll pause on user interaction

**Steps**:
1. Ensure auto-scroll checkbox is checked
2. Let logs stream (should auto-scroll to bottom)
3. Manually scroll up
4. Observe behavior as new logs arrive

**Expected Results**:
- [ ] Logs auto-scroll to bottom when checkbox is checked
- [ ] Auto-scroll checkbox unchecks when user scrolls manually
- [ ] New logs don't force scroll when auto-scroll is off
- [ ] Checking auto-scroll box resumes auto-scroll behavior

**Actual Result**: _________________

---

## User Story 2 - Reload Service (Priority: P2)

### Test Case 2.1: Service Status Display
**Objective**: Verify service status is displayed correctly

**Steps**:
1. Observe toolbar header
2. Check service status indicator

**Expected Results**:
- [ ] Status shows "Running" (or current state)
- [ ] Green dot indicator for Running state
- [ ] PID is displayed (e.g., "PID: 61934")
- [ ] Status updates every 5 seconds automatically

**Actual Result**: _________________

---

### Test Case 2.2: Successful Service Reload
**Objective**: Verify reload button works with valid config

**Steps**:
1. Ensure skhd service is running
2. Note current PID
3. Click "Reload Service" button
4. Observe status changes

**Expected Results**:
- [ ] Button shows spinner/loading state
- [ ] Status changes to "Reloading" or similar
- [ ] After ~2-5 seconds, status returns to "Running"
- [ ] PID changes (new process)
- [ ] Logs show service restart messages
- [ ] No error messages displayed

**Actual Result**: _________________

---

### Test Case 2.3: Service Reload with Configuration Error
**Objective**: Verify error handling for invalid configs

**Steps**:
1. Manually edit `~/.skhdrc` to introduce syntax error
2. Click "Reload Service" button
3. Observe error handling

**Expected Results**:
- [ ] Service attempts to reload
- [ ] Error status is shown (red dot)
- [ ] Error message displays with helpful guidance
- [ ] Service status indicates "Error" state
- [ ] Logs show error details

**Actual Result**: _________________

---

### Test Case 2.4: Concurrent Reload Prevention
**Objective**: Verify only one reload can happen at a time

**Steps**:
1. Click "Reload Service" button
2. Quickly click "Reload Service" again while first reload is in progress
3. Observe behavior

**Expected Results**:
- [ ] Button is disabled during reload
- [ ] Second click has no effect
- [ ] Only one reload process runs
- [ ] Reload lock prevents concurrent operations

**Actual Result**: _________________

---

## User Story 3 - Import Configuration Before Reload (Priority: P3)

### Test Case 3.1: Display Active Config Path
**Objective**: Verify active config path is shown

**Steps**:
1. Navigate to Service Manager page
2. Locate "Active Config" display

**Expected Results**:
- [ ] "Active Config:" label is visible
- [ ] Path shows actual skhd config location (e.g., `~/.skhdrc`)
- [ ] Path is in monospace font
- [ ] Path is readable and correctly formatted

**Actual Result**: _________________

---

### Test Case 3.2: Import Config File Picker
**Objective**: Verify import button opens file picker

**Steps**:
1. Click "Import Config" button
2. Observe file picker dialog

**Expected Results**:
- [ ] Native macOS file picker opens
- [ ] Dialog title is "Import skhd Configuration"
- [ ] Default directory is `~/.config/skhd/` or home
- [ ] Can browse to any location
- [ ] Can cancel without error

**Actual Result**: _________________

---

### Test Case 3.3: Import Config Success Flow
**Objective**: Verify successful config import shows pending state

**Steps**:
1. Click "Import Config" button
2. Select a valid skhd configuration file (different from current)
3. Observe UI changes

**Expected Results**:
- [ ] Success message appears: "Imported: /path/to/file"
- [ ] Display changes to "Loaded Config:" (orange border)
- [ ] Shows imported file path
- [ ] Hint text appears: "(Click 'Reload Service' to apply)"
- [ ] Success message disappears after 5 seconds
- [ ] Service still using old config (not yet reloaded)

**Actual Result**: _________________

---

### Test Case 3.4: Import Config Cancel
**Objective**: Verify canceling import doesn't cause errors

**Steps**:
1. Click "Import Config" button
2. Click "Cancel" in file picker dialog
3. Observe UI

**Expected Results**:
- [ ] No error message displayed
- [ ] Active config display unchanged
- [ ] Application remains functional
- [ ] Can retry import without issue

**Actual Result**: _________________

---

### Test Case 3.5: Import Invalid Config File
**Objective**: Verify error handling for invalid files

**Steps**:
1. Create a text file with invalid skhd syntax
2. Click "Import Config"
3. Select the invalid file
4. Observe error handling

**Expected Results**:
- [ ] Error message displays: "Failed to import: ..."
- [ ] Error message includes parse error details
- [ ] Active config unchanged
- [ ] Error message disappears after 10 seconds
- [ ] Can retry with valid file

**Actual Result**: _________________

---

### Test Case 3.6: Import and Reload Workflow
**Objective**: Verify imported config is applied after reload

**Steps**:
1. Note current active config path
2. Import a different valid config file
3. Verify "Loaded Config" shows with orange styling
4. Click "Reload Service" button
5. Wait for reload to complete
6. Observe config display

**Expected Results**:
- [ ] During reload: Shows loading state
- [ ] After reload: "Active Config:" (normal styling, no orange)
- [ ] Active config path remains `~/.skhdrc` (the standard location)
- [ ] Imported config content is now in `~/.skhdrc`
- [ ] Service is using the imported config
- [ ] Test a hotkey from imported config to confirm

**Actual Result**: _________________

---

### Test Case 3.7: Reload Without Import
**Objective**: Verify reload works with existing active config

**Steps**:
1. Do NOT import any config
2. Ensure only "Active Config:" is displayed (no pending import)
3. Click "Reload Service"
4. Observe behavior

**Expected Results**:
- [ ] Service reloads successfully
- [ ] Active config path unchanged
- [ ] Service continues using same config
- [ ] No errors or warnings
- [ ] Hotkeys still work as before

**Actual Result**: _________________

---

## Edge Cases & Error Scenarios

### Test Case E.1: Service Not Running
**Objective**: Test behavior when skhd is not running

**Steps**:
1. Stop skhd service: `launchctl unload ~/Library/LaunchAgents/com.koekeishiya.skhd.plist`
2. Refresh application
3. Observe status and try reload

**Expected Results**:
- [ ] Status shows "Stopped" or "Unknown" with gray dot
- [ ] Error message: "skhd service not found..."
- [ ] Reload button attempts to start service
- [ ] Helpful error messages guide user to install/start skhd

**Actual Result**: _________________

---

### Test Case E.2: Log File Not Exists
**Objective**: Test behavior when log file doesn't exist

**Steps**:
1. Stop skhd service
2. Delete `/tmp/skhd_<username>.err.log`
3. Start log stream
4. Observe error handling

**Expected Results**:
- [ ] Error message indicates log file not found
- [ ] Guidance suggests starting skhd service
- [ ] Application doesn't crash
- [ ] Can retry after starting service

**Actual Result**: _________________

---

### Test Case E.3: Large Log File (>10MB)
**Objective**: Verify performance with large logs

**Steps**:
1. Generate large skhd log file (manually or via script)
2. Start log stream
3. Observe performance

**Expected Results**:
- [ ] Virtual scrolling prevents UI freeze
- [ ] Only last 1000 logs kept in memory (FIFO)
- [ ] Scrolling remains smooth
- [ ] Application responsive

**Actual Result**: _________________

---

### Test Case E.4: Missing Config File
**Objective**: Test behavior when no config exists

**Steps**:
1. Temporarily move `~/.skhdrc` and `~/.config/skhd/skhdrc`
2. Refresh application
3. Observe active config display

**Expected Results**:
- [ ] Error message lists searched locations
- [ ] Guidance to create config file
- [ ] Import still works
- [ ] Can import and reload to create new config

**Actual Result**: _________________

---

### Test Case E.5: Accessibility - Keyboard Navigation
**Objective**: Verify keyboard navigation works

**Steps**:
1. Use Tab key to navigate through controls
2. Use Enter/Space to activate buttons
3. Use arrow keys in log viewer (if focused)

**Expected Results**:
- [ ] All interactive elements are keyboard accessible
- [ ] Focus indicators are visible
- [ ] Log viewer can be focused (tabindex="0")
- [ ] Buttons respond to Enter/Space keys
- [ ] Tab order is logical

**Actual Result**: _________________

---

### Test Case E.6: Accessibility - Screen Reader
**Objective**: Verify screen reader announces changes

**Steps**:
1. Enable VoiceOver (Cmd+F5)
2. Navigate to Service Manager
3. Trigger service reload
4. Import config
5. Observe announcements

**Expected Results**:
- [ ] Service status changes are announced
- [ ] Import success/error messages announced
- [ ] Log entries have proper labels
- [ ] ARIA live regions work correctly
- [ ] All interactive elements have labels

**Actual Result**: _________________

---

### Test Case E.7: Dark Mode Consistency
**Objective**: Verify dark theme across all states

**Steps**:
1. Check macOS system appearance is Dark Mode
2. Observe all UI elements, states, and interactions

**Expected Results**:
- [ ] All backgrounds use dark colors (#1c1c1c, #1e1e1e, #252525)
- [ ] Text is readable with sufficient contrast
- [ ] Status indicators use macOS system colors
- [ ] No light mode elements visible
- [ ] Hover states visible but subtle

**Actual Result**: _________________

---

## Test Summary

**Total Test Cases**: 26
**Passed**: ___ / 26
**Failed**: ___ / 26
**Blocked**: ___ / 26

### Critical Issues Found:
1. _________________
2. _________________
3. _________________

### Minor Issues Found:
1. _________________
2. _________________
3. _________________

### Notes:
_________________
_________________
_________________

**Tester Signature**: _________________
**Date Completed**: _________________
**Ready for Merge**: [ ] Yes  [ ] No  [ ] With Fixes
