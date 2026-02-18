pub fn execute_slash_command(input: &str) -> Result<Option<String>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('/') {
        return Ok(None);
    }

    let command = trimmed.split_whitespace().next().unwrap_or(trimmed);
    match command {
        "/about" => Ok(Some(format!(
            "remini-cli v{} (phase2 bootstrap)",
            env!("CARGO_PKG_VERSION")
        ))),
        "/help" | "/?" => Ok(Some(
            "Available commands:\n/about\n/help\n/tools\n@<path>\n!<command>".to_string(),
        )),
        "/tools" => Ok(Some(
            "Available tools:\nlist_directory\nread_file\nglob\ngrep_search".to_string(),
        )),
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
    fn unknown_slash_command_returns_error() {
        let result = execute_slash_command("/unknown");
        assert!(result.is_err());
    }
}
