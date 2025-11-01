# Feature Specification: Configuration Import/Export

**Feature Branch**: `002-config-import-export`
**Created**: 2025-11-01
**Status**: Draft
**Input**: User description: "Allow to load configuration and reload from UI. So I had a way to import or export config."

## User Scenarios & Testing _(mandatory)_

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Import Configuration from Custom Location (Priority: P1)

As a user managing multiple skhd configurations, I want to load a configuration file from a custom location so that I can test different configurations or use configurations stored in version control.

**Why this priority**: This is the core functionality requested and provides immediate value for users who maintain multiple configurations. It enables the primary use case of switching between different skhd setups without manual file copying.

**Independent Test**: Can be fully tested by selecting a custom skhdrc file from any location on the file system and verifying it loads correctly in the GUI, delivering the ability to manage configurations from anywhere.

**Acceptance Scenarios**:

1. **Given** I have a custom skhd configuration at `/Users/me/dotfiles/skhdrc`, **When** I click "Import Configuration" and select that file, **Then** the GUI loads and displays all shortcuts from that file
2. **Given** I have loaded a custom configuration, **When** I make edits and save, **Then** changes are written to the custom location, not the default `~/.config/skhd/skhdrc`
3. **Given** I select a file with parse errors, **When** import completes, **Then** the GUI displays parse errors clearly and shows successfully parsed shortcuts
4. **Given** I click "Import Configuration", **When** I cancel the file dialog, **Then** the current configuration remains unchanged

---

### User Story 2 - Export Current Configuration (Priority: P2)

As a user who has customized my skhd configuration through the GUI, I want to export my current configuration to a file so that I can back it up, share it, or use it on another machine.

**Why this priority**: This complements the import functionality and enables portability and backup workflows. It's P2 because users can still manually copy the file, but having a GUI export greatly improves UX.

**Independent Test**: Can be tested by making edits in the GUI, clicking "Export Configuration", selecting a destination, and verifying the exported file contains all changes and is a valid skhd configuration.

**Acceptance Scenarios**:

1. **Given** I have made changes to shortcuts in the GUI, **When** I click "Export Configuration" and choose a destination, **Then** a valid skhdrc file is saved with all my changes
2. **Given** I export to an existing file, **When** the save dialog appears, **Then** I am warned about overwriting and can confirm or cancel
3. **Given** I have unsaved changes, **When** I click "Export Configuration", **Then** the export includes the current in-memory state (unsaved changes)
4. **Given** I export successfully, **When** export completes, **Then** a success notification appears with the file path

---

### User Story 3 - Reload from Default Location (Priority: P1)

As a user who has made changes I want to discard, I want to reload the configuration from the default location so that I can revert to the last saved state without restarting the application.

**Why this priority**: This is essential for error recovery and provides a "reset" mechanism. Users need this to undo unwanted changes, making it equally critical as import functionality.

**Independent Test**: Can be tested by making changes to the configuration in the GUI, clicking "Reload", and verifying the GUI returns to the state of the file on disk, discarding in-memory changes.

**Acceptance Scenarios**:

1. **Given** I have made unsaved changes to shortcuts, **When** I click "Reload Configuration", **Then** I am warned about losing unsaved changes and can confirm or cancel
2. **Given** I confirm reload, **When** reload completes, **Then** all unsaved changes are discarded and the GUI shows the file state from disk
3. **Given** the file on disk has been modified externally, **When** I click "Reload", **Then** the GUI loads the latest version from disk
4. **Given** the file has been deleted externally, **When** I click "Reload", **Then** an error message appears explaining the file is missing

---

### Edge Cases

- What happens when the imported file is not a valid skhd configuration (wrong format, syntax errors)?
- How does the system handle permission errors when reading from or writing to a custom location?
- What happens if the user imports a very large configuration file (>10,000 lines)?
- How does the system handle file paths with special characters or spaces?
- What happens if the user tries to export to a read-only directory?
- How does the system behave if the default configuration file doesn't exist on reload?
- What happens if the file is modified externally while the user is editing in the GUI?

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide a file picker dialog to select skhd configuration files from any location
- **FR-002**: System MUST load and parse configuration files from custom locations, not just the default path
- **FR-003**: System MUST display the current configuration file path in the UI header
- **FR-004**: System MUST track which file is currently loaded (default or custom location)
- **FR-005**: System MUST save changes to the currently loaded file location, not always to default
- **FR-006**: System MUST provide an "Export Configuration" button that opens a save file dialog
- **FR-007**: System MUST export the current in-memory configuration state to the selected file
- **FR-008**: System MUST provide a "Reload Configuration" button in the header
- **FR-009**: System MUST warn users before discarding unsaved changes on reload
- **FR-010**: System MUST reload configuration from the currently tracked file location
- **FR-011**: System MUST handle file permission errors gracefully with clear error messages
- **FR-012**: System MUST validate exported files are valid skhd configuration format

### Key Entities _(include if feature involves data)_

- **ConfigurationSource**: Tracks the file path of the currently loaded configuration (default or custom)
- **FileMetadata**: Contains file path, last modified timestamp, and file size for tracking

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can successfully import a configuration file from any location and see all shortcuts displayed within 3 seconds
- **SC-002**: Users can export their configuration to any writable location and verify the exported file is valid
- **SC-003**: Users can reload configuration from disk and discard unsaved changes with a single click
- **SC-004**: 100% of file operation errors (permissions, missing files, invalid formats) display clear error messages
- **SC-005**: The UI always displays the current configuration file path, allowing users to know which file they are editing
- **SC-006**: All file operations (import, export, reload) complete without data loss or corruption
