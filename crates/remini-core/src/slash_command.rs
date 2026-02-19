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
        "/clear" => Ok(Some("Screen cleared (stub).".to_string())),
        "/quit" | "/exit" => Ok(Some("Session ended (stub).".to_string())),
        "/help" | "/?" => Ok(Some(
            "Available commands:\n/about\n/auth\n/clear\n/help\n/model [set <name>]\n/quit\n/tools [desc|nodesc]\n@<path>\n!<command>".to_string(),
        )),
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
    fn auth_command_returns_methods() {
        let result = execute_slash_command("/auth")
            .expect("auth command should succeed")
            .expect("auth command should return content");
        assert!(result.contains("Login with Google"));
        assert!(result.contains("GEMINI_API_KEY"));
    }

    #[test]
    fn clear_command_returns_message() {
        let result = execute_slash_command("/clear")
            .expect("clear command should succeed")
            .expect("clear command should return content");
        assert!(result.contains("Screen cleared"));
    }

    #[test]
    fn unknown_slash_command_returns_error() {
        let result = execute_slash_command("/unknown");
        assert!(result.is_err());
    }
}
