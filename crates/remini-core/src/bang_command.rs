use std::path::Path;

use remini_tools::run_shell_command;

pub fn execute_bang_command(input: &str) -> Result<Option<String>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('!') {
        return Ok(None);
    }

    let command = trimmed[1..].trim();
    if command.is_empty() {
        return Ok(None);
    }

    let output = run_shell_command(command, Path::new("."))
        .map_err(|err| format!("Failed to execute shell command: {err}"))?;

    if output.status_code != Some(0) {
        let stderr = output.stderr.trim().to_string();
        if stderr.is_empty() {
            return Err(format!(
                "Shell command failed with status: {}",
                output
                    .status_code
                    .map_or_else(|| "signal".to_string(), |code| code.to_string())
            ));
        }
        return Err(stderr);
    }

    Ok(Some(output.stdout))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_bang_input_returns_none() {
        let result = execute_bang_command("hello").expect("should not fail");
        assert_eq!(result, None);
    }

    #[test]
    fn bang_command_executes() {
        let result = execute_bang_command("!printf 'hello bang'")
            .expect("shell command should succeed")
            .expect("should return output");
        assert_eq!(result, "hello bang");
    }

    #[test]
    fn bang_command_propagates_failure() {
        let result = execute_bang_command("!false");
        assert!(result.is_err());
    }
}
