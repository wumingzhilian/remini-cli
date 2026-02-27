use std::env;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use remini_config::{load_settings_for_workspace, resolve_settings, ApprovalMode, CliOverrides};
use remini_core::{
    at_command::expand_at_command,
    bang_command::execute_bang_command,
    decide_run_mode,
    exit_codes::{EXIT_GENERAL_ERROR, EXIT_INPUT_ERROR, EXIT_SUCCESS},
    normalize_query,
    slash_command::execute_slash_command,
    startup_notice,
    tool_registry::ToolRegistry,
    OutputFormat, RunMode, RunRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum CliOutputFormat {
    Text,
    Json,
    StreamJson,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(value: CliOutputFormat) -> Self {
        match value {
            CliOutputFormat::Text => OutputFormat::Text,
            CliOutputFormat::Json => OutputFormat::Json,
            CliOutputFormat::StreamJson => OutputFormat::StreamJson,
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "remini",
    version,
    about = "Remini CLI (Rust rewrite of gemini-cli)",
    long_about = "Remini CLI - Defaults to interactive mode. Use -p/--prompt for non-interactive mode."
)]
struct CliArgs {
    #[arg(value_name = "QUERY", num_args = 0..)]
    query: Vec<String>,

    #[arg(short = 'm', long = "model")]
    model: Option<String>,

    #[arg(short = 'p', long = "prompt")]
    prompt: Option<String>,

    #[arg(short = 'i', long = "prompt-interactive")]
    prompt_interactive: Option<String>,

    #[arg(short = 's', long = "sandbox", default_value_t = false)]
    sandbox: bool,

    #[arg(short = 'd', long = "debug", default_value_t = false)]
    debug: bool,

    #[arg(long = "approval-mode")]
    approval_mode: Option<String>,

    #[arg(short = 'y', long = "yolo", default_value_t = false)]
    yolo: bool,

    #[arg(
        short = 'r',
        long = "resume",
        num_args = 0..=1,
        default_missing_value = "latest",
        value_name = "SESSION"
    )]
    resume: Option<String>,

    #[arg(
        long = "include-directories",
        value_name = "DIR",
        action = clap::ArgAction::Append,
        value_delimiter = ','
    )]
    include_directories: Vec<String>,

    #[arg(short = 'o', long = "output-format", value_enum)]
    output_format: Option<CliOutputFormat>,
}

fn normalize_include_directories(raw: Vec<String>) -> Vec<String> {
    raw.into_iter()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

fn normalize_resume(raw: Option<String>) -> Option<String> {
    raw.map(|value| value.trim().to_string()).and_then(|value| {
        if value.is_empty() {
            Some("latest".to_string())
        } else {
            Some(value)
        }
    })
}

fn env_debug_truthy(raw: Option<&str>) -> bool {
    matches!(raw, Some("true" | "1"))
}

fn is_debug_mode(args_debug: bool) -> bool {
    if args_debug {
        return true;
    }

    env_debug_truthy(env::var("DEBUG").ok().as_deref())
        || env_debug_truthy(env::var("DEBUG_MODE").ok().as_deref())
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 8);
    for c in value.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(c),
        }
    }
    escaped
}

fn print_headless_output(output: &str, format: Option<&OutputFormat>, model: Option<&str>) {
    let model_name = model.unwrap_or("auto");
    match format.unwrap_or(&OutputFormat::Text) {
        OutputFormat::Text => println!("{output}"),
        OutputFormat::Json => {
            let escaped = json_escape(output);
            println!(
                "{{\"response\":\"{escaped}\",\"stats\":{{\"mode\":\"stub\",\"toolsUsed\":0,\"model\":\"{}\"}}}}",
                json_escape(model_name)
            );
        }
        OutputFormat::StreamJson => {
            let escaped = json_escape(output);
            println!(
                "{{\"type\":\"init\",\"sessionId\":\"local-dev\",\"model\":\"{}\"}}",
                json_escape(model_name)
            );
            println!("{{\"type\":\"message\",\"role\":\"assistant\",\"content\":\"{escaped}\"}}");
            println!("{{\"type\":\"result\",\"response\":\"{escaped}\"}}");
        }
    }
}

fn build_json_error(message: &str, code: u8) -> String {
    let escaped = json_escape(message);
    format!(
        "{{\"response\":\"\",\"stats\":{{\"mode\":\"stub\",\"toolsUsed\":0}},\"error\":{{\"message\":\"{escaped}\",\"code\":{code}}}}}"
    )
}

fn build_stream_json_error(message: &str, code: u8) -> Vec<String> {
    let escaped = json_escape(message);
    vec![
        "{\"type\":\"init\",\"sessionId\":\"local-dev\",\"model\":\"auto\"}".to_string(),
        format!("{{\"type\":\"error\",\"message\":\"{escaped}\",\"code\":{code}}}"),
        format!("{{\"type\":\"result\",\"error\":\"{escaped}\"}}"),
    ]
}

fn return_with_error(message: &str, format: Option<&OutputFormat>, code: u8) -> ExitCode {
    match format.unwrap_or(&OutputFormat::Text) {
        OutputFormat::Text => eprintln!("{message}"),
        OutputFormat::Json => println!("{}", build_json_error(message, code)),
        OutputFormat::StreamJson => {
            for line in build_stream_json_error(message, code) {
                println!("{line}");
            }
        }
    }
    ExitCode::from(code)
}

fn main() -> ExitCode {
    let args = CliArgs::parse();
    let output_format = args.output_format.map(OutputFormat::from);
    let include_directories = normalize_include_directories(args.include_directories.clone());
    let resume = normalize_resume(args.resume.clone());
    let debug_mode = is_debug_mode(args.debug);

    if args.prompt.is_some() && !args.query.is_empty() {
        return return_with_error(
            "Cannot use both a positional prompt and the --prompt (-p) flag together",
            output_format.as_ref(),
            EXIT_INPUT_ERROR,
        );
    }

    if args.prompt.is_some() && args.prompt_interactive.is_some() {
        return return_with_error(
            "Cannot use both --prompt (-p) and --prompt-interactive (-i) together",
            output_format.as_ref(),
            EXIT_INPUT_ERROR,
        );
    }

    if args.yolo && args.approval_mode.is_some() {
        return return_with_error(
            "Cannot use both --yolo (-y) and --approval-mode together. Use --approval-mode=yolo instead.",
            output_format.as_ref(),
            EXIT_INPUT_ERROR,
        );
    }

    let approval_mode = if args.yolo {
        Some(ApprovalMode::Yolo)
    } else if let Some(raw_mode) = args.approval_mode.as_deref() {
        match ApprovalMode::parse(raw_mode) {
            Some(mode) => Some(mode),
            None => {
                return return_with_error(
                    &format!(
                        "Invalid approval mode: {raw_mode}. Valid values are: {}",
                        ApprovalMode::ALLOWED_VALUES.join(", ")
                    ),
                    output_format.as_ref(),
                    EXIT_INPUT_ERROR,
                );
            }
        }
    } else {
        None
    };

    let base_settings = match load_settings_for_workspace(Path::new(".")) {
        Ok(settings) => settings,
        Err(err) => {
            return return_with_error(
                &format!("Failed to load settings: {err}"),
                output_format.as_ref(),
                EXIT_INPUT_ERROR,
            );
        }
    };
    let effective_settings = resolve_settings(
        &base_settings,
        &CliOverrides {
            approval_mode,
            sandbox_enabled: if args.sandbox { Some(true) } else { None },
        },
    );

    let request = RunRequest {
        query: normalize_query(&args.query),
        model: args.model,
        resume,
        include_directories,
        prompt: args.prompt,
        prompt_interactive: args.prompt_interactive,
        output_format: output_format.clone(),
    };
    let stdin_is_tty = std::io::stdin().is_terminal();
    let mode = decide_run_mode(&request, stdin_is_tty);

    if let Some(message) = startup_notice(&request, stdin_is_tty) {
        eprintln!("{message}");
    }
    if debug_mode {
        eprintln!("Debug mode enabled (stub).");
    }

    match mode {
        RunMode::Interactive => {
            if let Some(session) = request.resume.as_deref() {
                println!("Resuming session: {session} (stub)");
            }
            println!(
                "remini interactive mode bootstrap complete (Phase 1 skeleton, approval-mode={}).",
                effective_settings.approval_mode.as_str()
            );
            ExitCode::from(EXIT_SUCCESS)
        }
        RunMode::Headless => {
            if let Some(raw_input) = request.prompt.as_ref().or(request.query.as_ref()) {
                let response_text = match execute_slash_command(raw_input) {
                    Ok(Some(slash_output)) => slash_output,
                    Ok(None) => match execute_bang_command(raw_input) {
                        Ok(Some(shell_output)) => shell_output,
                        Ok(None) => {
                            let registry = ToolRegistry;
                            let include_dirs = request
                                .include_directories
                                .iter()
                                .map(PathBuf::from)
                                .collect::<Vec<_>>();
                            match expand_at_command(
                                raw_input,
                                Path::new("."),
                                &include_dirs,
                                &registry,
                            ) {
                                Ok(Some(expanded)) => expanded,
                                Ok(None) => raw_input.to_string(),
                                Err(err) => {
                                    return return_with_error(
                                        &err,
                                        request.output_format.as_ref(),
                                        EXIT_INPUT_ERROR,
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            return return_with_error(
                                &err,
                                request.output_format.as_ref(),
                                EXIT_GENERAL_ERROR,
                            );
                        }
                    },
                    Err(err) => {
                        return return_with_error(
                            &err,
                            request.output_format.as_ref(),
                            EXIT_INPUT_ERROR,
                        );
                    }
                };

                print_headless_output(
                    &response_text,
                    request.output_format.as_ref(),
                    request.model.as_deref(),
                );
            }
            ExitCode::from(EXIT_SUCCESS)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_escape_escapes_newlines_and_quotes() {
        let value = "line1\n\"quoted\"";
        let escaped = json_escape(value);
        assert_eq!(escaped, "line1\\n\\\"quoted\\\"");
    }

    #[test]
    fn output_format_mapping_works() {
        assert_eq!(
            OutputFormat::from(CliOutputFormat::Text),
            OutputFormat::Text
        );
        assert_eq!(
            OutputFormat::from(CliOutputFormat::Json),
            OutputFormat::Json
        );
        assert_eq!(
            OutputFormat::from(CliOutputFormat::StreamJson),
            OutputFormat::StreamJson
        );
    }

    #[test]
    fn build_json_error_contains_code() {
        let payload = build_json_error("bad input", EXIT_INPUT_ERROR);
        assert!(payload.contains("\"code\":42"));
        assert!(payload.contains("bad input"));
    }

    #[test]
    fn build_stream_json_error_contains_error_event() {
        let lines = build_stream_json_error("failure", EXIT_GENERAL_ERROR);
        assert_eq!(lines.len(), 3);
        assert!(lines[1].contains("\"type\":\"error\""));
        assert!(lines[1].contains("\"code\":1"));
    }

    #[test]
    fn yolo_conflict_error_message_is_stable() {
        let msg = "Cannot use both --yolo (-y) and --approval-mode together. Use --approval-mode=yolo instead.";
        let payload = build_json_error(msg, EXIT_INPUT_ERROR);
        assert!(payload.contains("--approval-mode=yolo"));
    }

    #[test]
    fn include_directories_are_trimmed() {
        let dirs = normalize_include_directories(vec![
            " ./one ".to_string(),
            "".to_string(),
            "two".to_string(),
            "   ".to_string(),
        ]);
        assert_eq!(dirs, vec!["./one".to_string(), "two".to_string()]);
    }

    #[test]
    fn normalize_resume_defaults_empty_to_latest() {
        assert_eq!(
            normalize_resume(Some("".to_string())),
            Some("latest".to_string())
        );
        assert_eq!(
            normalize_resume(Some("   ".to_string())),
            Some("latest".to_string())
        );
    }

    #[test]
    fn normalize_resume_keeps_explicit_value() {
        assert_eq!(
            normalize_resume(Some("5".to_string())),
            Some("5".to_string())
        );
    }

    #[test]
    fn parse_resume_without_value_defaults_to_latest() {
        let args = CliArgs::try_parse_from(["remini", "--resume"]).expect("parse should succeed");
        assert_eq!(args.resume.as_deref(), Some("latest"));
    }

    #[test]
    fn parse_resume_with_explicit_value() {
        let args = CliArgs::try_parse_from(["remini", "--resume", "session-42"])
            .expect("parse should succeed");
        assert_eq!(args.resume.as_deref(), Some("session-42"));
    }

    #[test]
    fn env_debug_truthy_handles_known_values() {
        assert!(env_debug_truthy(Some("true")));
        assert!(env_debug_truthy(Some("1")));
        assert!(!env_debug_truthy(Some("false")));
        assert!(!env_debug_truthy(Some("0")));
        assert!(!env_debug_truthy(None));
    }

    #[test]
    fn is_debug_mode_true_when_cli_flag_is_true() {
        assert!(is_debug_mode(true));
    }
}
