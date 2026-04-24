pub mod at_command;
pub mod bang_command;
pub mod exit_codes;
pub mod plan_mode;
pub mod slash_command;
pub mod tool_registry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunMode {
    Interactive,
    Headless,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    StreamJson,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RunRequest {
    pub query: Option<String>,
    pub model: Option<String>,
    pub resume: Option<String>,
    pub include_directories: Vec<String>,
    pub prompt: Option<String>,
    pub prompt_interactive: Option<String>,
    pub output_format: Option<OutputFormat>,
}

pub const POSITIONAL_QUERY_NOTICE: &str = "Positional arguments now default to interactive mode. To run in non-interactive mode, use the --prompt (-p) flag.";

pub fn normalize_query(raw_query: &[String]) -> Option<String> {
    if raw_query.is_empty() {
        None
    } else {
        Some(raw_query.join(" "))
    }
}

pub fn decide_run_mode(request: &RunRequest, stdin_is_tty: bool) -> RunMode {
    if request.prompt.is_some() {
        return RunMode::Headless;
    }

    if request.prompt_interactive.is_some() {
        return RunMode::Interactive;
    }

    if request.query.is_some() {
        if stdin_is_tty {
            return RunMode::Interactive;
        }
        return RunMode::Headless;
    }

    if stdin_is_tty {
        RunMode::Interactive
    } else {
        RunMode::Headless
    }
}

pub fn startup_notice(request: &RunRequest, stdin_is_tty: bool) -> Option<&'static str> {
    if request.query.is_some() && request.prompt.is_none() && stdin_is_tty {
        return Some(POSITIONAL_QUERY_NOTICE);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query_none_for_empty() {
        assert_eq!(normalize_query(&[]), None);
    }

    #[test]
    fn normalize_query_joins_by_space() {
        let query = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(normalize_query(&query), Some("hello world".to_string()));
    }

    #[test]
    fn prompt_forces_headless() {
        let request = RunRequest {
            prompt: Some("status".to_string()),
            ..Default::default()
        };
        assert_eq!(decide_run_mode(&request, true), RunMode::Headless);
    }

    #[test]
    fn prompt_interactive_forces_interactive() {
        let request = RunRequest {
            prompt_interactive: Some("status".to_string()),
            ..Default::default()
        };
        assert_eq!(decide_run_mode(&request, false), RunMode::Interactive);
    }

    #[test]
    fn positional_query_is_interactive_on_tty() {
        let request = RunRequest {
            query: Some("hello".to_string()),
            ..Default::default()
        };
        assert_eq!(decide_run_mode(&request, true), RunMode::Interactive);
    }

    #[test]
    fn positional_query_is_headless_on_non_tty() {
        let request = RunRequest {
            query: Some("hello".to_string()),
            ..Default::default()
        };
        assert_eq!(decide_run_mode(&request, false), RunMode::Headless);
    }

    #[test]
    fn startup_notice_exists_for_positional_query_on_tty() {
        let request = RunRequest {
            query: Some("hello".to_string()),
            ..Default::default()
        };
        assert_eq!(
            startup_notice(&request, true),
            Some(POSITIONAL_QUERY_NOTICE)
        );
    }

    #[test]
    fn startup_notice_not_shown_when_prompt_is_used() {
        let request = RunRequest {
            query: Some("hello".to_string()),
            prompt: Some("status".to_string()),
            ..Default::default()
        };
        assert_eq!(startup_notice(&request, true), None);
    }
}
