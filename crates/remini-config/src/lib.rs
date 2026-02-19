use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

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

#[derive(Debug, Deserialize, Default)]
struct SettingsFile {
    #[serde(default)]
    general: GeneralSettingsFile,
    #[serde(default)]
    tools: ToolsSettingsFile,
}

#[derive(Debug, Deserialize, Default)]
struct GeneralSettingsFile {
    #[serde(rename = "defaultApprovalMode")]
    default_approval_mode: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct ToolsSettingsFile {
    sandbox: Option<bool>,
}

fn read_settings_file(path: &Path) -> Result<Option<SettingsFile>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path)
        .map_err(|err| format!("Cannot read settings file {}: {}", path.display(), err))?;
    let parsed: SettingsFile = serde_json::from_str(&raw)
        .map_err(|err| format!("Invalid JSON in settings file {}: {}", path.display(), err))?;
    Ok(Some(parsed))
}

fn apply_settings_file(
    settings: &mut Settings,
    file: SettingsFile,
    source: &Path,
) -> Result<(), String> {
    if let Some(raw_mode) = file.general.default_approval_mode {
        if let Some(mode) = ApprovalMode::parse(&raw_mode) {
            settings.default_approval_mode = mode;
        } else {
            return Err(format!(
                "Invalid defaultApprovalMode '{}' in {}. Allowed values: {}",
                raw_mode,
                source.display(),
                ApprovalMode::ALLOWED_VALUES.join(", ")
            ));
        }
    }

    if let Some(sandbox) = file.tools.sandbox {
        settings.sandbox_enabled = sandbox;
    }

    Ok(())
}

pub fn load_settings_from_paths(
    user_path: &Path,
    workspace_path: &Path,
) -> Result<Settings, String> {
    let mut settings = Settings::default();

    if let Some(user_file) = read_settings_file(user_path)? {
        apply_settings_file(&mut settings, user_file, user_path)?;
    }

    if let Some(workspace_file) = read_settings_file(workspace_path)? {
        apply_settings_file(&mut settings, workspace_file, workspace_path)?;
    }

    Ok(settings)
}

fn user_settings_path_from_home(home: &Path) -> PathBuf {
    home.join(".gemini").join("settings.json")
}

pub fn load_settings_for_workspace(workspace_dir: &Path) -> Result<Settings, String> {
    let workspace_path = workspace_dir.join(".gemini").join("settings.json");

    let user_path = env::var("HOME")
        .map(|home| user_settings_path_from_home(Path::new(&home)))
        .unwrap_or_else(|_| PathBuf::from("__remini_missing_home__/settings.json"));

    load_settings_from_paths(&user_path, &workspace_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_temp_dir(prefix: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        let dir = env::temp_dir().join(format!("{}-{}-{}", prefix, std::process::id(), timestamp));
        fs::create_dir_all(&dir).expect("failed to create temp dir");
        dir
    }

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

    #[test]
    fn load_settings_returns_defaults_when_files_are_missing() {
        let temp_dir = make_temp_dir("remini-config-missing");
        let user_path = temp_dir.join("user-settings.json");
        let workspace_path = temp_dir.join("workspace-settings.json");

        let settings = load_settings_from_paths(&user_path, &workspace_path)
            .expect("settings loading should succeed");
        assert_eq!(settings.default_approval_mode, ApprovalMode::Default);
        assert!(!settings.sandbox_enabled);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn workspace_settings_override_user_settings() {
        let temp_dir = make_temp_dir("remini-config-merge");
        let user_path = temp_dir.join("user.json");
        let workspace_path = temp_dir.join("workspace.json");

        fs::write(
            &user_path,
            r#"{"general":{"defaultApprovalMode":"plan"},"tools":{"sandbox":true}}"#,
        )
        .expect("failed to write user settings");
        fs::write(
            &workspace_path,
            r#"{"general":{"defaultApprovalMode":"auto_edit"},"tools":{"sandbox":false}}"#,
        )
        .expect("failed to write workspace settings");

        let settings = load_settings_from_paths(&user_path, &workspace_path)
            .expect("settings loading should succeed");
        assert_eq!(settings.default_approval_mode, ApprovalMode::AutoEdit);
        assert!(!settings.sandbox_enabled);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn invalid_mode_in_settings_returns_error() {
        let temp_dir = make_temp_dir("remini-config-invalid-mode");
        let user_path = temp_dir.join("user.json");
        let workspace_path = temp_dir.join("workspace.json");

        fs::write(
            &workspace_path,
            r#"{"general":{"defaultApprovalMode":"invalid_mode"}}"#,
        )
        .expect("failed to write workspace settings");

        let err = load_settings_from_paths(&user_path, &workspace_path)
            .expect_err("loading should fail for invalid mode");
        assert!(err.contains("Invalid defaultApprovalMode"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }
}
