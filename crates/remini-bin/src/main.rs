use std::io::IsTerminal;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use remini_config::{resolve_settings, ApprovalMode, CliOverrides, Settings};
use remini_core::{
    decide_run_mode, normalize_query, startup_notice, OutputFormat, RunMode, RunRequest,
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

    #[arg(long = "approval-mode")]
    approval_mode: Option<String>,

    #[arg(short = 'o', long = "output-format", value_enum)]
    output_format: Option<CliOutputFormat>,
}

fn main() -> ExitCode {
    let args = CliArgs::parse();

    if args.prompt.is_some() && !args.query.is_empty() {
        eprintln!("Cannot use both a positional prompt and the --prompt (-p) flag together");
        return ExitCode::from(1);
    }

    if args.prompt.is_some() && args.prompt_interactive.is_some() {
        eprintln!("Cannot use both --prompt (-p) and --prompt-interactive (-i) together");
        return ExitCode::from(1);
    }

    let approval_mode = if let Some(raw_mode) = args.approval_mode.as_deref() {
        match ApprovalMode::parse(raw_mode) {
            Some(mode) => Some(mode),
            None => {
                eprintln!(
                    "Invalid approval mode: {raw_mode}. Valid values are: {}",
                    ApprovalMode::ALLOWED_VALUES.join(", ")
                );
                return ExitCode::from(1);
            }
        }
    } else {
        None
    };

    let base_settings = Settings::default();
    let effective_settings = resolve_settings(
        &base_settings,
        &CliOverrides {
            approval_mode,
            sandbox_enabled: if args.sandbox { Some(true) } else { None },
        },
    );

    let request = RunRequest {
        query: normalize_query(&args.query),
        prompt: args.prompt,
        prompt_interactive: args.prompt_interactive,
        output_format: args.output_format.map(OutputFormat::from),
    };
    let stdin_is_tty = std::io::stdin().is_terminal();
    let mode = decide_run_mode(&request, stdin_is_tty);

    if let Some(message) = startup_notice(&request, stdin_is_tty) {
        eprintln!("{message}");
    }

    match mode {
        RunMode::Interactive => {
            println!(
                "remini interactive mode bootstrap complete (Phase 1 skeleton, approval-mode={}).",
                effective_settings.approval_mode.as_str()
            );
            ExitCode::SUCCESS
        }
        RunMode::Headless => {
            if let Some(prompt) = request.prompt.as_ref().or(request.query.as_ref()) {
                println!("{prompt}");
            }
            ExitCode::SUCCESS
        }
    }
}
