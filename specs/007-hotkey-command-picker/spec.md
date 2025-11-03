# Feature Specification: Command and Application Picker for Hotkey Setup

**Feature Branch**: `007-hotkey-command-picker`
**Created**: 2025-11-03
**Status**: Draft
**Input**: User description: "improve the hotkey setup by selecting the application or find the command or by name or something more friendly and other than just a text writer"

## Clarifications

### Session 2025-11-03

- Q: How does the system handle applications with duplicate names (e.g., multiple versions of the same app)? → A: Show all duplicate applications with distinguishing information (path or version) appended to the name
- Q: What happens when searching returns no results? → A: Display a simple "No results found" message with no further action
- Q: What happens when a selected application or script is moved or deleted after the hotkey is created? → A: Allow hotkey to remain configured but mark it as inactive/disabled if the target is missing, with visual indicator in the list
- Q: What happens when the user's system has hundreds of applications installed (performance of application list)? → A: Load all applications at once and rely on browser/system performance (may cause lag with 500+ apps)
- Q: How does the system handle commands that require elevated privileges (sudo)? → A: Allow all commands including sudo without special handling or warnings (user responsible for understanding implications)

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Browse and Select Applications (Priority: P1)

Users want to assign keyboard shortcuts to launch applications without having to remember or type the exact application path. They should be able to browse available applications on their system and select them visually.

**Why this priority**: This is the most common use case for keyboard shortcuts - launching applications. Without this, users must manually type application paths, which is error-prone and creates friction in the setup process.

**Independent Test**: Can be fully tested by opening the hotkey editor, clicking an "application picker" button, browsing the list of installed applications, selecting one, and verifying the correct application path is inserted into the command field. Delivers immediate value by making application shortcuts accessible to non-technical users.

**Acceptance Scenarios**:

1. **Given** the user is creating a new hotkey, **When** they click the "Browse Applications" button, **Then** a searchable list of installed macOS applications appears
2. **Given** the application picker is open, **When** the user types in the search field, **Then** the list filters to show only matching application names
3. **Given** the user selects an application from the list, **When** they confirm the selection, **Then** the correct command to launch that application is inserted into the command field
4. **Given** the user has selected an application, **When** they view the hotkey in the list, **Then** the application name is displayed in a user-friendly format (not just the raw path)

---

### User Story 2 - Search and Select Shell Commands (Priority: P2)

Users want to assign keyboard shortcuts to common shell commands and scripts without having to know the exact command syntax. They should be able to search for commands by name or description.

**Why this priority**: After application launching, executing shell commands is the second most common hotkey use case. This enables users to discover and use system commands they might not know about.

**Independent Test**: Can be tested by opening the command picker, searching for common commands (e.g., "volume", "brightness", "screenshot"), selecting one, and verifying the correct command syntax is inserted. Delivers value by exposing system capabilities to users.

**Acceptance Scenarios**:

1. **Given** the user is creating a new hotkey, **When** they click the "Browse Commands" button, **Then** a searchable list of common shell commands appears organized by category
2. **Given** the command picker is open, **When** the user searches for a capability (e.g., "volume"), **Then** all commands related to that capability are shown with descriptions
3. **Given** the user selects a command, **When** they confirm the selection, **Then** the command syntax is inserted into the command field
4. **Given** a command requires parameters, **When** the user selects it, **Then** they are prompted to provide the necessary parameter values

---

### User Story 3 - Browse File System for Scripts and Executables (Priority: P3)

Users want to assign keyboard shortcuts to custom scripts or executables located anywhere on their file system. They should be able to navigate their file system and select files visually.

**Why this priority**: This handles advanced use cases for users with custom scripts. It's lower priority because fewer users need this compared to launching applications or using built-in commands.

**Independent Test**: Can be tested by opening the file browser, navigating to a directory containing scripts, selecting an executable file, and verifying the correct file path is inserted into the command field. Delivers value for power users with custom automation.

**Acceptance Scenarios**:

1. **Given** the user is creating a new hotkey, **When** they click the "Browse Files" button, **Then** a file browser dialog opens
2. **Given** the file browser is open, **When** the user navigates to a directory and selects an executable file, **Then** the full path to that file is inserted into the command field
3. **Given** the user selects a script file, **When** they confirm the selection, **Then** the command includes the appropriate interpreter if needed (e.g., "bash /path/to/script.sh")
4. **Given** the file browser is open, **When** the user tries to select a non-executable file, **Then** they receive a warning that the file may not be executable

---

### User Story 4 - Quick Command Templates (Priority: P2)

Users want access to pre-configured command templates for common tasks (volume control, brightness, window management) that they can customize rather than building from scratch.

**Why this priority**: This accelerates setup for common use cases and helps users discover what's possible with keyboard shortcuts. It's equal priority with shell command search because it serves a similar discovery function.

**Independent Test**: Can be tested by opening the template picker, selecting a template (e.g., "Increase Volume"), customizing any parameters if needed, and verifying the complete command is inserted. Delivers immediate value through preset configurations.

**Acceptance Scenarios**:

1. **Given** the user is creating a new hotkey, **When** they click the "Use Template" button, **Then** a list of common command templates appears organized by category
2. **Given** the template picker is open, **When** the user selects a template, **Then** they see a preview of what the command will do
3. **Given** a template has customizable parameters, **When** the user selects it, **Then** they can adjust the parameter values before inserting
4. **Given** the user confirms a template, **When** it's inserted into the command field, **Then** they can still edit it manually if needed

---

### Edge Cases

- The system loads all installed applications at once in the picker, relying on browser/system performance (may experience lag with 500+ applications)
- When multiple applications have the same name, the system displays all versions with distinguishing information (file path or version number) appended to help users select the correct one
- When a configured application or script is moved or deleted, the hotkey remains in the configuration but is marked as inactive/disabled with a visual indicator in the list, preserving the configuration in case the file is restored
- Commands requiring elevated privileges (sudo) are allowed without special handling or warnings; users are responsible for understanding the implications
- When search returns no results in any picker (applications, commands, templates), the system displays a "No results found" message
- How does the system display applications that don't have standard bundle structures?
- What happens when a script file doesn't have execute permissions?
- How does the system handle paths with spaces or special characters?

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide a visual application picker that lists all installed macOS applications
- **FR-002**: System MUST load all applications at once when the picker is opened
- **FR-003**: System MUST allow users to search/filter the application list by name in real-time
- **FR-004**: System MUST display applications with their icons and human-readable names
- **FR-005**: System MUST insert the correct command syntax to launch the selected application
- **FR-006**: System MUST provide a command browser that categorizes common shell commands (e.g., System, Media, Window Management)
- **FR-007**: System MUST allow users to search commands by name or description
- **FR-008**: System MUST display command descriptions to help users understand what each command does
- **FR-009**: System MUST provide a file browser for selecting custom scripts or executables
- **FR-010**: System MUST validate that selected files are executable or warn the user if they may not be
- **FR-011**: System MUST provide a template library with pre-configured commands for common tasks
- **FR-012**: System MUST allow users to preview what a template command will do before inserting it
- **FR-013**: System MUST support parameter customization for commands and templates that require it
- **FR-014**: System MUST properly escape paths with spaces and special characters when inserting commands
- **FR-015**: System MUST preserve the ability to manually edit command text after using pickers/templates
- **FR-016**: System MUST allow commands with elevated privileges (sudo) to be configured without special validation or warnings
- **FR-017**: System MUST mark hotkeys as inactive/disabled when their target application or file no longer exists, with visual indicators in the hotkey list
- **FR-018**: System MUST preserve the configuration of inactive/disabled hotkeys to allow recovery if the target file is restored
- **FR-019**: System MUST organize template commands into logical categories (e.g., Volume Control, Window Management, Screenshots)
- **FR-020**: System MUST show the raw command syntax alongside the user-friendly description
- **FR-021**: System MUST display all applications with duplicate names and append distinguishing information (file path or version number) to each entry
- **FR-022**: System MUST display a "No results found" message when search queries return no matching results in any picker

### Key Entities

- **Application**: Represents an installed macOS application with properties including display name, icon, bundle identifier, and executable path
- **Command Template**: Represents a pre-configured command pattern with properties including name, description, category, command syntax, and optional parameters
- **Command Parameter**: Represents a configurable value in a command template with properties including parameter name, description, data type, default value, and validation rules
- **Command Category**: Represents a logical grouping of commands or templates (e.g., "System", "Media", "Window Management")

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can create an application launch hotkey in under 30 seconds without typing any paths
- **SC-002**: 90% of users successfully create their first hotkey using the visual pickers without referring to documentation
- **SC-003**: Users can discover and configure at least 5 different types of commands through the template library
- **SC-004**: Search results appear within 200 milliseconds of typing in any picker
- **SC-005**: The application picker correctly identifies and lists at least 95% of installed applications on a standard macOS system
- **SC-006**: Zero syntax errors occur when using commands inserted through pickers or templates
- **SC-007**: Users report a 50% reduction in time spent creating hotkeys compared to the text-only approach

## Assumptions

- Users are running macOS 11 or later where standard application locations are used
- The system has permission to access the Applications folder and user's home directory
- Common system commands (volume, brightness, etc.) have consistent availability across supported macOS versions
- Users understand basic concepts of keyboard shortcuts and application launching
- The command template library will start with 20-30 common commands and can be expanded over time
- Users with custom scripts are comfortable with basic file system navigation
- Applications are installed in standard locations (/Applications, ~/Applications, /System/Applications)

## Dependencies

- Existing skhd configuration file format and parser
- Existing hotkey editing UI components
- macOS system APIs for discovering installed applications
- File system access permissions for browsing applications and scripts

## Out of Scope

- Creating or editing shell scripts within the application
- Teaching users shell scripting or command syntax
- Validating whether commands will actually work on the user's system
- Automatic command suggestions based on user behavior
- Cloud-based command template sharing or marketplace
- Integration with third-party application stores
- Custom icon creation or editing for applications
- Modifying or configuring applications themselves (only launching them)
