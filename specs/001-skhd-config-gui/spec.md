# Feature Specification: skhd Configuration GUI

**Feature Branch**: `001-skhd-config-gui`
**Created**: 2025-11-01
**Status**: Draft
**Input**: User description: "gui for my config"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Existing Configuration (Priority: P1)

Users need to view their current skhd keyboard shortcut configuration in a readable, organized format rather than opening a text editor.

**Why this priority**: This is the foundation - users must be able to see their configuration before editing it. Delivers immediate value by providing a better viewing experience than raw text files.

**Independent Test**: Can be fully tested by launching the app and verifying that existing skhd configuration is displayed correctly. Delivers value even without editing capabilities.

**Acceptance Scenarios**:

1. **Given** a user has an existing skhd config file at `~/.config/skhd/skhdrc`, **When** they launch the application, **Then** the configuration is parsed and displayed in an organized, readable format showing all keyboard shortcuts and their associated commands
2. **Given** a user has no existing skhd config file, **When** they launch the application, **Then** they see an empty state with a message explaining how to create their first shortcut
3. **Given** a user has a skhd config file with syntax errors, **When** they launch the application, **Then** they see a clear error message indicating which line has the problem and what the issue is, without corrupting the file

---

### User Story 2 - Edit Keyboard Shortcuts (Priority: P2)

Users need to modify existing keyboard shortcuts or add new ones through a visual interface instead of manually editing text syntax.

**Why this priority**: Core editing functionality - builds on viewing to enable configuration changes. This is where the GUI provides significant value over text editing.

**Independent Test**: Can be tested by opening an existing configuration, making changes (add, edit, delete shortcuts), and verifying that changes are saved correctly to the config file while maintaining valid syntax.

**Acceptance Scenarios**:

1. **Given** a user is viewing their configuration, **When** they click on an existing shortcut entry, **Then** they can modify the keyboard combination, the command, or delete the entry entirely
2. **Given** a user wants to add a new shortcut, **When** they click "Add Shortcut", **Then** they are presented with an interface to define the keyboard combination and associated command, with real-time validation that the combination isn't already in use
3. **Given** a user has made changes to their configuration, **When** they save the changes, **Then** a backup of the previous configuration is created automatically, and the new configuration is written only if it passes validation
4. **Given** a user has made unsaved changes, **When** they attempt to close the application, **Then** they are prompted to save, discard, or cancel the action

---

### User Story 3 - Test Shortcuts Before Saving (Priority: P3)

Users need to verify that keyboard shortcuts work as intended before committing them to their active configuration.

**Why this priority**: Enhances confidence and reduces errors, but users can still accomplish their goals without this feature by saving and testing manually.

**Independent Test**: Can be tested by creating or modifying a shortcut, triggering the test mode, and verifying that the shortcut executes in a sandboxed environment without affecting the active configuration.

**Acceptance Scenarios**:

1. **Given** a user has created or modified a shortcut, **When** they click "Test Shortcut", **Then** the application temporarily activates that specific shortcut and displays feedback when the keyboard combination is pressed, showing whether it triggered correctly
2. **Given** a user is testing a shortcut that launches an application, **When** the shortcut is triggered, **Then** the application launches and the GUI shows confirmation that the shortcut executed successfully

---

### User Story 4 - Configuration Initialization (Priority: P1)

Users need clear, unambiguous control over how configurations are initialized - whether detecting existing configs or creating blank ones.

**Why this priority**: Foundation for user workflow. Ambiguous initialization leads to confusion and potential data loss. Critical for first-run experience.

**Independent Test**: Can be tested by launching app with and without existing config, verifying each action does exactly what it promises.

**Acceptance Scenarios**:

1. **Given** user launches app with no existing config, **When** they click "Detect Active Config", **Then** system searches standard locations and displays "No configuration found" message with option to create new
2. **Given** user launches app with existing config at `~/.config/skhd/skhdrc`, **When** they click "Detect Active Config", **Then** system loads the existing configuration
3. **Given** user launches app (regardless of existing config), **When** they click "Create Blank Config", **Then** system creates an empty configuration without checking for existing files
4. **Given** user has existing config, **When** they click "Create Blank Config", **Then** system creates blank config and allows user to choose save location (preventing accidental overwrite of existing config)
5. **Given** user clicks "Import Existing File...", **When** they select a file, **Then** system loads that specific file regardless of standard locations

**Design Decision (Discovered Edge Case)**:

**Problem**: Original spec ambiguous - "Create New Config" button auto-detected existing config, conflicting with user's explicit intent to create new.

**Solution**: Separate actions with clear intent:
- **"Detect Active Config"**: Auto-detection of standard locations
- **"Create Blank Config"**: Always blank, never auto-detect
- **"Import Existing File..."**: Manual file selection

**Rationale**:
- Respects explicit user choice (no surprises)
- Supports experimentation without touching existing config
- Allows recovery from corrupted config (start fresh)
- Clear, predictable behavior for each action

---

### Edge Cases

- What happens when the skhd config file is being edited by another process while the GUI is open?
- How does the system handle keyboard combinations that conflict with system shortcuts?
- What happens if the user doesn't have write permissions to the skhd config directory?
- How does the app behave when the skhd service is not running or not installed?
- What happens with Unicode characters or special symbols in commands or comments?
- How are multi-line commands or comments handled in the GUI display?
- **RESOLVED**: What happens when user clicks "Create New Config" but existing config detected? → Separated into distinct "Detect Active" vs "Create Blank" actions (User Story 4)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST load and parse the skhd configuration file from the standard location (`~/.config/skhd/skhdrc`)
- **FR-002**: System MUST display all keyboard shortcuts in a structured, searchable list showing the key combination and associated command
- **FR-003**: Users MUST be able to add new keyboard shortcuts by specifying the key combination and command through GUI controls
- **FR-004**: Users MUST be able to edit existing keyboard shortcuts (modify key combination or command)
- **FR-005**: Users MUST be able to delete keyboard shortcuts from the configuration
- **FR-006**: System MUST validate keyboard shortcut syntax before saving to prevent invalid configurations
- **FR-007**: System MUST create an automatic backup of the existing configuration before any save operation
- **FR-008**: System MUST detect and warn users about duplicate keyboard combinations
- **FR-009**: System MUST provide undo/redo functionality for configuration changes within a session
- **FR-010**: System MUST display clear error messages when configuration parsing fails, indicating the specific line and issue
- **FR-011**: System MUST support standard keyboard modifiers (cmd, alt, shift, ctrl, fn) and their combinations
- **FR-012**: System MUST preserve comments and formatting from the original config file where possible
- **FR-013**: Users MUST be able to search/filter shortcuts by key combination or command text
- **FR-014**: System MUST prompt users to save before closing if unsaved changes exist
- **FR-015**: System MUST reload configuration when external changes to the config file are detected
- **FR-016**: Application MUST request necessary file system permissions on first launch
- **FR-017**: System MUST handle read-only mode gracefully when write permissions are unavailable
- **FR-018**: System MUST provide separate "Detect Active Config" action that searches standard locations for existing configuration
- **FR-019**: "Detect Active Config" MUST NOT create new files - only detect and load existing configurations
- **FR-020**: System MUST provide separate "Create Blank Config" action that creates empty configuration
- **FR-021**: "Create Blank Config" MUST NOT auto-detect existing configs - always creates blank regardless of existing files
- **FR-022**: "Create Blank Config" MUST allow user to specify save location to prevent accidental overwrite of existing config
- **FR-023**: System MUST provide "Import Existing File..." action for manual file selection
- **FR-024**: Welcome screen MUST clearly label each action to communicate intent: "Detect Active Config", "Create Blank Config", "Import Existing File..."

### Key Entities _(include if feature involves data)_

- **Keyboard Shortcut**: Represents a single skhd configuration entry with a key combination (modifiers + key) and an associated shell command or action
- **Configuration File**: Represents the entire skhd config file, containing a collection of shortcuts, comments, and metadata (file path, last modified timestamp, backup status)
- **Key Combination**: Represents a specific keyboard input pattern consisting of zero or more modifiers (cmd, alt, shift, ctrl, fn) and a primary key
- **Backup**: Represents a timestamped copy of the configuration file created before modifications

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can view their complete skhd configuration within 2 seconds of launching the application
- **SC-002**: Users can add or edit a keyboard shortcut and save the configuration in under 30 seconds
- **SC-003**: 100% of valid skhd configurations are parsed correctly without data loss
- **SC-004**: Zero user-reported incidents of configuration file corruption due to the application
- **SC-005**: Application successfully detects and warns about 100% of duplicate keyboard combinations before saving
- **SC-006**: 95% of users can complete their first configuration edit without consulting documentation
- **SC-007**: Application launch time is under 2 seconds on macOS 11+ systems
- **SC-008**: Application memory footprint remains under 50MB during typical editing sessions
- **SC-009**: All configuration changes are backed up before saving with timestamp for recovery

## Assumptions

- Users have skhd installed and configured on their macOS system
- The skhd configuration file follows standard skhd syntax conventions
- Users understand basic keyboard shortcut concepts (modifiers, key combinations)
- The skhd configuration file location follows the standard `~/.config/skhd/skhdrc` path or users know their custom path
- Users have basic familiarity with shell commands or understand what the commands in their config do
- Application will use standard macOS file system APIs for reading/writing configuration files
- Dark mode support will follow system preferences automatically (standard macOS behavior)
- Keyboard shortcut capture will follow standard macOS event handling patterns
- Configuration file encoding is UTF-8 (standard for text config files on macOS)
