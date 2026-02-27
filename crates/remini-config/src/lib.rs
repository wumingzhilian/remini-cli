#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalMode {
    Default,
    AutoEdit,
    Plan,
    Yolo,
}

impl ApprovalMode {
    pub const ALLOWED_VALUES: [&'static str; 4] = ["default", "auto_edit", "plan", "yolo"];

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "default" => Some(Self::Default),
            "auto_edit" => Some(Self::AutoEdit),
            "plan" => Some(Self::Plan),
            "yolo" => Some(Self::Yolo),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::AutoEdit => "auto_edit",
            Self::Plan => "plan",
            Self::Yolo => "yolo",
        }
    }
}

impl Default for ApprovalMode {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub default_approval_mode: ApprovalMode,
    pub sandbox_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_approval_mode: ApprovalMode::Default,
            sandbox_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliOverrides {
    pub approval_mode: Option<ApprovalMode>,
    pub sandbox_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveSettings {
    pub approval_mode: ApprovalMode,
    pub sandbox_enabled: bool,
}

pub fn resolve_settings(settings: &Settings, overrides: &CliOverrides) -> EffectiveSettings {
    EffectiveSettings {
        approval_mode: overrides
            .approval_mode
            .unwrap_or(settings.default_approval_mode),
        sandbox_enabled: overrides
            .sandbox_enabled
            .unwrap_or(settings.sandbox_enabled),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_approval_modes() {
        assert_eq!(ApprovalMode::parse("default"), Some(ApprovalMode::Default));
        assert_eq!(
            ApprovalMode::parse("auto_edit"),
            Some(ApprovalMode::AutoEdit)
        );
        assert_eq!(ApprovalMode::parse("plan"), Some(ApprovalMode::Plan));
        assert_eq!(ApprovalMode::parse("yolo"), Some(ApprovalMode::Yolo));
    }

    #[test]
    fn rejects_unknown_approval_mode() {
        assert_eq!(ApprovalMode::parse("unknown"), None);
    }

    #[test]
    fn resolve_settings_uses_defaults_when_no_override() {
        let settings = Settings::default();
        let effective = resolve_settings(&settings, &CliOverrides::default());
        assert_eq!(effective.approval_mode, ApprovalMode::Default);
        assert!(!effective.sandbox_enabled);
    }

    #[test]
    fn resolve_settings_prefers_cli_overrides() {
        let settings = Settings {
            default_approval_mode: ApprovalMode::Plan,
            sandbox_enabled: false,
        };
        let overrides = CliOverrides {
            approval_mode: Some(ApprovalMode::AutoEdit),
            sandbox_enabled: Some(true),
        };

        let effective = resolve_settings(&settings, &overrides);
        assert_eq!(effective.approval_mode, ApprovalMode::AutoEdit);
        assert!(effective.sandbox_enabled);
    }
}
