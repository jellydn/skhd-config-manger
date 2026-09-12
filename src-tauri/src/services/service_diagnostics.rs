use crate::models::{
    AccessibilityPermission, InputMonitoringPermission, ServiceState, SkhdVariant,
};

pub(super) type ZigStatus = (
    ServiceState,
    Option<u32>,
    AccessibilityPermission,
    InputMonitoringPermission,
    Option<String>,
);

pub(super) fn config_requires_grabber(content: &str) -> bool {
    content.lines().any(|line| {
        let directive = line.trim_start();
        directive.strip_prefix(".remap").is_some_and(|content| {
            content.starts_with(char::is_whitespace) && content.contains('{')
        })
    })
}

pub(super) fn parse_zig_status(output: &str) -> ZigStatus {
    let daemon = named_status_line(output, "Daemon running:");
    let hotkeys = named_status_line(output, "Hotkeys functional:");
    let input_monitoring = named_status_line(output, "Input Monitoring:");
    let registration = named_status_line(output, "Registration status:");

    let pid = daemon.and_then(|line| {
        line.split_once("Yes (PID ")?
            .1
            .trim_end_matches(')')
            .parse()
            .ok()
    });
    let mut state = match daemon {
        Some(line) if line.contains("Yes (PID ") => ServiceState::Running,
        Some(line) if line.contains("No (") => ServiceState::Stopped,
        _ => ServiceState::Unknown,
    };
    let accessibility = match hotkeys {
        Some(line) if line.contains("Yes (event tap active)") => AccessibilityPermission::Granted,
        Some(line) if line.contains("accessibility denied") => AccessibilityPermission::Denied,
        _ => AccessibilityPermission::Unknown,
    };
    let input_monitoring = match input_monitoring {
        Some(line) if line.ends_with("Granted") => InputMonitoringPermission::Granted,
        Some(line) if line.contains("Denied") => InputMonitoringPermission::Denied,
        _ => InputMonitoringPermission::Unknown,
    };
    let error_message = if registration.is_some_and(|line| line.contains("requires user approval"))
    {
        state = ServiceState::Error;
        Some(
            "skhd.zig needs approval in System Settings → General → Login Items & Extensions."
                .to_string(),
        )
    } else if accessibility == AccessibilityPermission::Denied {
        state = ServiceState::Error;
        Some("skhd.zig: Accessibility permission is denied.".to_string())
    } else if input_monitoring == InputMonitoringPermission::Denied {
        state = ServiceState::Error;
        Some("skhd.zig: Input Monitoring permission is denied.".to_string())
    } else if hotkeys.is_some_and(|line| line.contains("event tap registered but disabled")) {
        state = ServiceState::Error;
        Some("skhd.zig: The event tap is disabled. Restart the service.".to_string())
    } else if state == ServiceState::Unknown {
        Some("skhd.zig returned an unrecognized service status.".to_string())
    } else {
        None
    };

    (state, pid, accessibility, input_monitoring, error_message)
}

fn named_status_line<'a>(output: &'a str, name: &str) -> Option<&'a str> {
    output
        .lines()
        .find(|line| line.trim_start().starts_with(name))
        .map(str::trim)
}

pub(super) fn accessibility_guidance(variant: SkhdVariant) -> String {
    match variant {
        SkhdVariant::Original => "Add the skhd executable used by the launch agent to System Settings → Privacy & Security → Accessibility, enable it, then restart the service. Granting Keybinder or Terminal does not grant the launchd service.".to_string(),
        SkhdVariant::Zig => "Add and enable /Applications/skhd.app in both Accessibility and Input Monitoring under System Settings → Privacy & Security. If service registration needs approval, also enable skhd under General → Login Items & Extensions. Then restart the service.".to_string(),
    }
}

pub(super) fn is_accessibility_denial(message: &str) -> bool {
    let normalized = message.to_lowercase();
    normalized.contains("must be run with accessibility access")
        || normalized.contains("accessibilitypermissiondenied")
        || normalized.contains("accessibility permission denied")
}

pub(super) fn current_accessibility_denial(log_tail: &str) -> Option<String> {
    log_tail
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .filter(|line| is_accessibility_denial(line))
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_zig_status_uses_named_status_fields() {
        let output = "  Registration status:  enabled\n  Daemon running:       Yes (PID 4242)\n  Hotkeys functional:   Yes (event tap active)\n  Input Monitoring:     Granted\n";
        let (state, pid, accessibility, input_monitoring, error) = parse_zig_status(output);
        assert_eq!(state, ServiceState::Running);
        assert_eq!(pid, Some(4242));
        assert_eq!(accessibility, AccessibilityPermission::Granted);
        assert_eq!(input_monitoring, InputMonitoringPermission::Granted);
        assert_eq!(error, None);
    }

    #[test]
    fn parse_zig_status_reports_permission_and_registration_failures() {
        let denied = "  Registration status:  enabled\n  Daemon running:       Yes (PID 42)\n  Hotkeys functional:   No (accessibility denied — see remediation below)\n  Input Monitoring:     Denied (events suppressed — see remediation below)\n";
        let (state, _, accessibility, input_monitoring, error) = parse_zig_status(denied);
        assert_eq!(state, ServiceState::Error);
        assert_eq!(accessibility, AccessibilityPermission::Denied);
        assert_eq!(input_monitoring, InputMonitoringPermission::Denied);
        assert!(error.unwrap().contains("Accessibility"));

        let approval = "  Registration status:  requires user approval in System Settings → Login Items & Extensions\n  Daemon running:       No (LaunchAgent not loaded)\n  Hotkeys functional:   Unknown (daemon not running or window server unavailable)\n  Input Monitoring:     Unknown (will prompt on first key event)\n";
        let (state, _, _, _, error) = parse_zig_status(approval);
        assert_eq!(state, ServiceState::Error);
        assert!(error.unwrap().contains("Login Items & Extensions"));
    }

    #[test]
    fn accessibility_denial_matches_daemon_errors() {
        assert!(is_accessibility_denial(
            "skhd: must be run with accessibility access! abort.."
        ));
        assert!(is_accessibility_denial(
            "error.AccessibilityPermissionDenied"
        ));
        assert!(!is_accessibility_denial("service exited with code 1"));
    }

    #[test]
    fn only_current_log_diagnostic_can_report_accessibility_denial() {
        assert_eq!(
            current_accessibility_denial(
                "skhd: configuration error\nskhd: must be run with accessibility access! abort..\n"
            )
            .as_deref(),
            Some("skhd: must be run with accessibility access! abort..")
        );
        assert_eq!(
            current_accessibility_denial(
                "skhd: must be run with accessibility access! abort..\nskhd: configuration error\n"
            ),
            None
        );
    }

    #[test]
    fn accessibility_guidance_names_the_correct_permission_target() {
        let original = accessibility_guidance(SkhdVariant::Original);
        assert!(original.contains("skhd executable"));
        assert!(original.contains("Granting Keybinder or Terminal does not grant"));

        let zig = accessibility_guidance(SkhdVariant::Zig);
        assert!(zig.contains("/Applications/skhd.app"));
        assert!(zig.contains("Input Monitoring"));
    }

    #[test]
    fn config_requires_grabber_only_for_block_remaps() {
        assert!(config_requires_grabber(
            ".remap caps_lock [device builtin] {\n  tap: escape\n}\n"
        ));
        assert!(!config_requires_grabber(
            ".remap caps_lock [device builtin] : escape\n"
        ));
        assert!(!config_requires_grabber("# .remap caps_lock {\n"));
    }
}
