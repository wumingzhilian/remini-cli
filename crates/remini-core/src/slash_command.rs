const COMMAND_HELP_TEXT: &str = "Available commands:\n/about\n/auth\n/bug\n/clear\n/commands\n/compress [note]\n/copy [message-id]\n/directory [list|add <path>|remove <path>]\n/docs\n/editor [status|open <path>]\n/help\n/ide [status|enable|disable]\n/init\n/model [set <name>]\n/privacy\n/quit\n/resume [session|latest]\n/settings [show|set <key> <value>]\n/stats [session|model|tools]\n/terminal-setup [check|install]\n/theme [list|set <name>]\n/tools [desc|nodesc]\n/vim [status|on|off]\n@<path>\n!<command>";

pub fn execute_slash_command(input: &str) -> Result<Option<String>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('/') {
        return Ok(None);
    }

    let mut parts = trimmed.split_whitespace();
    let command = parts.next().unwrap_or(trimmed);
    match command {
        "/about" => Ok(Some(format!(
            "remini-cli v{} (phase2 bootstrap)",
            env!("CARGO_PKG_VERSION")
        ))),
        "/auth" => Ok(Some(
            "Auth methods (stub):\n- Login with Google\n- GEMINI_API_KEY\n- Vertex AI".to_string(),
        )),
        "/bug" => Ok(Some(
            "Bug report (stub): please open an issue with steps to reproduce and logs.".to_string(),
        )),
        "/directory" => {
            let action = parts.next().unwrap_or("list");
            match action {
                "list" => Ok(Some(
                    "Directory context (stub):\n- . (workspace root)".to_string(),
                )),
                "add" => {
                    if let Some(path) = parts.next() {
                        Ok(Some(format!(
                            "Added directory: {path} (session-only stub). Use --include-directories on startup for persistent include paths."
                        )))
                    } else {
                        Err("Usage: /directory add <path>".to_string())
                    }
                }
                "remove" => {
                    if let Some(path) = parts.next() {
                        Ok(Some(format!(
                            "Removed directory: {path} (session-only stub)."
                        )))
                    } else {
                        Err("Usage: /directory remove <path>".to_string())
                    }
                }
                other => Err(format!(
                    "Unsupported /directory option: {other}. Use list, add <path>, or remove <path>."
                )),
            }
        }
        "/copy" => {
            let target = parts.next().unwrap_or("last");
            Ok(Some(format!(
                "Copied message {target} to clipboard (stub)."
            )))
        }
        "/compress" => {
            let note = parts.collect::<Vec<_>>().join(" ");
            if note.is_empty() {
                Ok(Some(
                    "Compression complete (stub): summarized current conversation.".to_string(),
                ))
            } else {
                Ok(Some(format!(
                    "Compression complete (stub): summarized current conversation with note: {note}"
                )))
            }
        }
        "/docs" => Ok(Some(
            "Docs (stub): visit https://github.com/wumingzhilian/remini-cli#readme".to_string(),
        )),
        "/editor" => {
            let action = parts.next().unwrap_or("status");
            match action {
                "status" => Ok(Some(
                    "Editor (stub): no active editor bridge connected.".to_string(),
                )),
                "open" => {
                    if let Some(path) = parts.next() {
                        Ok(Some(format!("Editor open request: {path} (stub).")))
                    } else {
                        Err("Usage: /editor open <path>".to_string())
                    }
                }
                other => Err(format!(
                    "Unsupported /editor option: {other}. Use status or open <path>."
                )),
            }
        }
        "/ide" => {
            let action = parts.next().unwrap_or("status");
            match action {
                "status" => Ok(Some("IDE companion (stub): disabled.".to_string())),
                "enable" => Ok(Some(
                    "IDE companion enabled for current session (stub).".to_string(),
                )),
                "disable" => Ok(Some(
                    "IDE companion disabled for current session (stub).".to_string(),
                )),
                other => Err(format!(
                    "Unsupported /ide option: {other}. Use status, enable, or disable."
                )),
            }
        }
        "/theme" => {
            let action = parts.next().unwrap_or("list");
            match action {
                "list" => Ok(Some(
                    "Available themes (stub): default, light, dark".to_string(),
                )),
                "set" => {
                    if let Some(name) = parts.next() {
                        Ok(Some(format!(
                            "Theme set to {name} for current session (stub)."
                        )))
                    } else {
                        Err("Usage: /theme set <name>".to_string())
                    }
                }
                other => Err(format!(
                    "Unsupported /theme option: {other}. Use list or set <name>."
                )),
            }
        }
        "/terminal-setup" => {
            let action = parts.next().unwrap_or("check");
            match action {
                "check" => Ok(Some(
                    "Terminal setup (stub): shell integration status unknown.".to_string(),
                )),
                "install" => Ok(Some(
                    "Terminal setup install requested (stub): follow shell instructions."
                        .to_string(),
                )),
                other => Err(format!(
                    "Unsupported /terminal-setup option: {other}. Use check or install."
                )),
            }
        }
        "/vim" => {
            let action = parts.next().unwrap_or("status");
            match action {
                "status" => Ok(Some("Vim mode (stub): off.".to_string())),
                "on" => Ok(Some(
                    "Vim mode enabled for current session (stub).".to_string(),
                )),
                "off" => Ok(Some(
                    "Vim mode disabled for current session (stub).".to_string(),
                )),
                other => Err(format!(
                    "Unsupported /vim option: {other}. Use status, on, or off."
                )),
            }
        }
        "/init" => Ok(Some(
            "Init (stub): generated starter files guidance for remini-cli in current workspace."
                .to_string(),
        )),
        "/privacy" => Ok(Some(
            "Privacy (stub): review telemetry, data retention, and output safety settings."
                .to_string(),
        )),
        "/settings" => {
            let action = parts.next().unwrap_or("show");
            match action {
                "show" => Ok(Some(
                    "Settings (stub): model=auto, approvalMode=default, sandbox=false".to_string(),
                )),
                "set" => {
                    let key = parts.next();
                    let value = parts.next();
                    match (key, value) {
                        (Some(key), Some(value)) => Ok(Some(format!(
                            "Setting updated: {key}={value} (session-only stub)."
                        ))),
                        _ => Err("Usage: /settings set <key> <value>".to_string()),
                    }
                }
                other => Err(format!(
                    "Unsupported /settings option: {other}. Use show or set <key> <value>."
                )),
            }
        }
        "/clear" => Ok(Some("Screen cleared (stub).".to_string())),
        "/quit" | "/exit" => Ok(Some("Session ended (stub).".to_string())),
        "/resume" => {
            let session = parts.next().unwrap_or("latest");
            Ok(Some(format!("Resumed session: {session} (stub).")))
        }
        "/help" | "/?" => Ok(Some(COMMAND_HELP_TEXT.to_string())),
        "/commands" => Ok(Some(COMMAND_HELP_TEXT.to_string())),
        "/model" => {
            let action = parts.next();
            match action {
                None => Ok(Some("Current model: auto (stub)".to_string())),
                Some("set") => {
                    if let Some(model_name) = parts.next() {
                        Ok(Some(format!(
                            "Model set to {model_name} (session-only stub)"
                        )))
                    } else {
                        Err("Usage: /model set <model-name>".to_string())
                    }
                }
                Some(other) => Err(format!(
                    "Unsupported /model option: {other}. Use /model or /model set <name>."
                )),
            }
        }
        "/tools" => {
            let mode = parts.next().unwrap_or("nodesc");
            match mode {
                "desc" | "descriptions" => Ok(Some(
                    "Available tools:\nlist_directory - list files and directories\nread_file - read text content from a file\nglob - match files by wildcard pattern\ngrep_search - search text within files".to_string(),
                )),
                "nodesc" | "nodescriptions" => Ok(Some(
                    "Available tools:\nlist_directory\nread_file\nglob\ngrep_search".to_string(),
                )),
                other => Err(format!(
                    "Unsupported /tools option: {other}. Use desc or nodesc."
                )),
            }
        }
        "/stats" => {
            let view = parts.next().unwrap_or("session");
            match view {
                "session" => Ok(Some(
                    "Session stats (stub): duration=0s, turns=0, toolCalls=0".to_string(),
                )),
                "model" => Ok(Some(
                    "Model stats (stub): model=auto, inputTokens=0, outputTokens=0".to_string(),
                )),
                "tools" => Ok(Some(
                    "Tool stats (stub): list_directory=0, read_file=0, glob=0, grep_search=0"
                        .to_string(),
                )),
                other => Err(format!(
                    "Unsupported /stats option: {other}. Use session, model, or tools."
                )),
            }
        }
        _ => Err(format!("Unknown slash command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_slash_input_returns_none() {
        let result = execute_slash_command("hello").expect("should not fail");
        assert_eq!(result, None);
    }

    #[test]
    fn help_command_returns_output() {
        let result = execute_slash_command("/help")
            .expect("help command should succeed")
            .expect("help command should return content");
        assert!(result.contains("Available commands"));
        assert!(result.contains("/commands"));
    }

    #[test]
    fn commands_command_returns_output() {
        let result = execute_slash_command("/commands")
            .expect("commands command should succeed")
            .expect("commands command should return content");
        assert!(result.contains("Available commands"));
        assert!(result.contains("/model"));
        assert!(result.contains("/resume"));
        assert!(result.contains("/directory"));
        assert!(result.contains("/compress"));
        assert!(result.contains("/docs"));
        assert!(result.contains("/editor"));
        assert!(result.contains("/ide"));
        assert!(result.contains("/init"));
        assert!(result.contains("/privacy"));
        assert!(result.contains("/terminal-setup"));
        assert!(result.contains("/theme"));
        assert!(result.contains("/vim"));
    }

    #[test]
    fn about_command_returns_version() {
        let result = execute_slash_command("/about")
            .expect("about command should succeed")
            .expect("about command should return content");
        assert!(result.contains("remini-cli v"));
    }

    #[test]
    fn tools_command_returns_output() {
        let result = execute_slash_command("/tools")
            .expect("tools command should succeed")
            .expect("tools command should return content");
        assert!(result.contains("read_file"));
        assert!(result.contains("grep_search"));
    }

    #[test]
    fn tools_desc_returns_descriptions() {
        let result = execute_slash_command("/tools desc")
            .expect("tools desc should succeed")
            .expect("tools desc should return content");
        assert!(result.contains("read_file - read text content"));
    }

    #[test]
    fn model_command_returns_current_model() {
        let result = execute_slash_command("/model")
            .expect("model command should succeed")
            .expect("model command should return content");
        assert!(result.contains("Current model: auto"));
    }

    #[test]
    fn model_set_command_returns_confirmation() {
        let result = execute_slash_command("/model set gemini-2.5-flash")
            .expect("model set command should succeed")
            .expect("model set command should return content");
        assert!(result.contains("Model set to gemini-2.5-flash"));
    }

    #[test]
    fn quit_command_returns_message() {
        let result = execute_slash_command("/quit")
            .expect("quit command should succeed")
            .expect("quit command should return content");
        assert!(result.contains("Session ended"));
    }

    #[test]
    fn resume_command_defaults_to_latest() {
        let result = execute_slash_command("/resume")
            .expect("resume command should succeed")
            .expect("resume command should return content");
        assert!(result.contains("Resumed session: latest"));
    }

    #[test]
    fn resume_command_accepts_session_id() {
        let result = execute_slash_command("/resume 8")
            .expect("resume command should succeed")
            .expect("resume command should return content");
        assert!(result.contains("Resumed session: 8"));
    }

    #[test]
    fn auth_command_returns_methods() {
        let result = execute_slash_command("/auth")
            .expect("auth command should succeed")
            .expect("auth command should return content");
        assert!(result.contains("Login with Google"));
        assert!(result.contains("GEMINI_API_KEY"));
    }

    #[test]
    fn bug_command_returns_report_hint() {
        let result = execute_slash_command("/bug")
            .expect("bug command should succeed")
            .expect("bug command should return content");
        assert!(result.contains("Bug report"));
    }

    #[test]
    fn clear_command_returns_message() {
        let result = execute_slash_command("/clear")
            .expect("clear command should succeed")
            .expect("clear command should return content");
        assert!(result.contains("Screen cleared"));
    }

    #[test]
    fn directory_command_defaults_to_list() {
        let result = execute_slash_command("/directory")
            .expect("directory command should succeed")
            .expect("directory command should return content");
        assert!(result.contains("Directory context"));
    }

    #[test]
    fn directory_add_requires_path() {
        let result = execute_slash_command("/directory add");
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("should fail"),
            "Usage: /directory add <path>"
        );
    }

    #[test]
    fn directory_add_accepts_path() {
        let result = execute_slash_command("/directory add docs")
            .expect("directory add command should succeed")
            .expect("directory add command should return content");
        assert!(result.contains("Added directory: docs"));
    }

    #[test]
    fn directory_remove_accepts_path() {
        let result = execute_slash_command("/directory remove docs")
            .expect("directory remove command should succeed")
            .expect("directory remove command should return content");
        assert!(result.contains("Removed directory: docs"));
    }

    #[test]
    fn copy_command_defaults_to_last() {
        let result = execute_slash_command("/copy")
            .expect("copy command should succeed")
            .expect("copy command should return content");
        assert!(result.contains("Copied message last"));
    }

    #[test]
    fn copy_command_accepts_message_id() {
        let result = execute_slash_command("/copy 12")
            .expect("copy command should succeed")
            .expect("copy command should return content");
        assert!(result.contains("Copied message 12"));
    }

    #[test]
    fn compress_command_defaults_to_summary() {
        let result = execute_slash_command("/compress")
            .expect("compress command should succeed")
            .expect("compress command should return content");
        assert!(result.contains("Compression complete"));
    }

    #[test]
    fn compress_command_accepts_note() {
        let result = execute_slash_command("/compress keep key decisions")
            .expect("compress command should succeed")
            .expect("compress command should return content");
        assert!(result.contains("keep key decisions"));
    }

    #[test]
    fn docs_command_returns_link_hint() {
        let result = execute_slash_command("/docs")
            .expect("docs command should succeed")
            .expect("docs command should return content");
        assert!(result.contains("Docs (stub)"));
    }

    #[test]
    fn editor_command_defaults_to_status() {
        let result = execute_slash_command("/editor")
            .expect("editor command should succeed")
            .expect("editor command should return content");
        assert!(result.contains("Editor (stub)"));
    }

    #[test]
    fn editor_open_requires_path() {
        let result = execute_slash_command("/editor open");
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("should fail"),
            "Usage: /editor open <path>"
        );
    }

    #[test]
    fn editor_open_accepts_path() {
        let result = execute_slash_command("/editor open src/main.rs")
            .expect("editor open should succeed")
            .expect("editor open should return content");
        assert!(result.contains("src/main.rs"));
    }

    #[test]
    fn ide_command_defaults_to_status() {
        let result = execute_slash_command("/ide")
            .expect("ide command should succeed")
            .expect("ide command should return content");
        assert!(result.contains("IDE companion"));
    }

    #[test]
    fn ide_enable_and_disable_work() {
        let enabled = execute_slash_command("/ide enable")
            .expect("ide enable should succeed")
            .expect("ide enable should return content");
        let disabled = execute_slash_command("/ide disable")
            .expect("ide disable should succeed")
            .expect("ide disable should return content");
        assert!(enabled.contains("enabled"));
        assert!(disabled.contains("disabled"));
    }

    #[test]
    fn theme_command_lists_themes() {
        let result = execute_slash_command("/theme")
            .expect("theme command should succeed")
            .expect("theme command should return content");
        assert!(result.contains("Available themes"));
    }

    #[test]
    fn theme_set_requires_name() {
        let result = execute_slash_command("/theme set");
        assert!(result.is_err());
        assert_eq!(result.expect_err("should fail"), "Usage: /theme set <name>");
    }

    #[test]
    fn theme_set_accepts_name() {
        let result = execute_slash_command("/theme set dark")
            .expect("theme set should succeed")
            .expect("theme set should return content");
        assert!(result.contains("Theme set to dark"));
    }

    #[test]
    fn terminal_setup_defaults_to_check() {
        let result = execute_slash_command("/terminal-setup")
            .expect("terminal-setup should succeed")
            .expect("terminal-setup should return content");
        assert!(result.contains("Terminal setup (stub)"));
    }

    #[test]
    fn terminal_setup_install_is_supported() {
        let result = execute_slash_command("/terminal-setup install")
            .expect("terminal-setup install should succeed")
            .expect("terminal-setup install should return content");
        assert!(result.contains("install requested"));
    }

    #[test]
    fn vim_command_defaults_to_status() {
        let result = execute_slash_command("/vim")
            .expect("vim should succeed")
            .expect("vim should return content");
        assert!(result.contains("Vim mode"));
    }

    #[test]
    fn vim_mode_on_and_off_work() {
        let enabled = execute_slash_command("/vim on")
            .expect("vim on should succeed")
            .expect("vim on should return content");
        let disabled = execute_slash_command("/vim off")
            .expect("vim off should succeed")
            .expect("vim off should return content");
        assert!(enabled.contains("enabled"));
        assert!(disabled.contains("disabled"));
    }

    #[test]
    fn init_command_returns_setup_hint() {
        let result = execute_slash_command("/init")
            .expect("init command should succeed")
            .expect("init command should return content");
        assert!(result.contains("Init (stub)"));
    }

    #[test]
    fn privacy_command_returns_policy_hint() {
        let result = execute_slash_command("/privacy")
            .expect("privacy command should succeed")
            .expect("privacy command should return content");
        assert!(result.contains("Privacy (stub)"));
    }

    #[test]
    fn settings_command_defaults_to_show() {
        let result = execute_slash_command("/settings")
            .expect("settings command should succeed")
            .expect("settings command should return content");
        assert!(result.contains("Settings (stub)"));
    }

    #[test]
    fn settings_set_requires_key_and_value() {
        let result = execute_slash_command("/settings set model");
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("should fail"),
            "Usage: /settings set <key> <value>"
        );
    }

    #[test]
    fn settings_set_accepts_key_and_value() {
        let result = execute_slash_command("/settings set model gemini-2.5-flash")
            .expect("settings set should succeed")
            .expect("settings set should return content");
        assert!(result.contains("Setting updated: model=gemini-2.5-flash"));
    }

    #[test]
    fn stats_command_defaults_to_session_view() {
        let result = execute_slash_command("/stats")
            .expect("stats command should succeed")
            .expect("stats command should return content");
        assert!(result.contains("Session stats"));
    }

    #[test]
    fn stats_tools_view_returns_tool_metrics() {
        let result = execute_slash_command("/stats tools")
            .expect("stats tools command should succeed")
            .expect("stats tools command should return content");
        assert!(result.contains("Tool stats"));
    }

    #[test]
    fn unknown_slash_command_returns_error() {
        let result = execute_slash_command("/unknown");
        assert!(result.is_err());
    }
}
