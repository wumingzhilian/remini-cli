use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanModeAction {
    Entered,
    Exited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanModeResult {
    pub action: PlanModeAction,
    pub message: String,
    pub plan_path: Option<PathBuf>,
}

pub fn enter_plan_mode(reason: Option<&str>) -> PlanModeResult {
    let message = reason
        .map(|reason| format!("Switching to Plan mode: {reason}"))
        .unwrap_or_else(|| "Switching to Plan mode".to_string());
    PlanModeResult {
        action: PlanModeAction::Entered,
        message,
        plan_path: None,
    }
}

pub fn exit_plan_mode(
    plan_path: &Path,
    plans_dir: &Path,
    cwd: &Path,
) -> Result<PlanModeResult, String> {
    if plan_path.as_os_str().is_empty() {
        return Err("plan_path is required".to_string());
    }

    let resolved_plan_path = normalize_path(cwd.join(plan_path));
    let resolved_plans_dir = normalize_path(if plans_dir.is_absolute() {
        plans_dir.to_path_buf()
    } else {
        cwd.join(plans_dir)
    });

    if !resolved_plan_path.starts_with(&resolved_plans_dir) {
        return Err("Access denied: plan path must be within the plans directory".to_string());
    }

    Ok(PlanModeResult {
        action: PlanModeAction::Exited,
        message: format!(
            "Plan approved from {}. Returning to default approval mode.",
            resolved_plan_path.display()
        ),
        plan_path: Some(resolved_plan_path),
    })
}

fn normalize_path(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_plan_mode_returns_reason_when_present() {
        let result = enter_plan_mode(Some("safe planning"));
        assert_eq!(result.action, PlanModeAction::Entered);
        assert!(result.message.contains("safe planning"));
    }

    #[test]
    fn exit_plan_mode_accepts_path_inside_plans_dir() {
        let cwd = Path::new("/workspace");
        let result = exit_plan_mode(
            Path::new(".gemini/plans/plan.md"),
            Path::new(".gemini/plans"),
            cwd,
        )
        .expect("plan path should be accepted");

        assert_eq!(result.action, PlanModeAction::Exited);
        assert_eq!(
            result.plan_path.as_deref(),
            Some(Path::new("/workspace/.gemini/plans/plan.md"))
        );
    }

    #[test]
    fn exit_plan_mode_rejects_path_outside_plans_dir() {
        let cwd = Path::new("/workspace");
        let err = exit_plan_mode(Path::new("plan.md"), Path::new(".gemini/plans"), cwd)
            .expect_err("plan path should be rejected");
        assert!(err.contains("Access denied"));
    }
}
