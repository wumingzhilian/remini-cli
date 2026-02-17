use std::process::Command;

pub fn execute_bang_command(input: &str) -> Result<Option<String>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('!') {
        return Ok(None);
    }

    let command = trimmed[1..].trim();
    if command.is_empty() {
        return Ok(None);
    }

    let output = Command::new("sh")
        .arg("-lc")
        .arg(command)
        .output()
        .map_err(|err| format!("Failed to execute shell command: {err}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            return Err(format!(
                "Shell command failed with status: {}",
                output
                    .status
                    .code()
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "signal".to_string())
            ));
        }
        return Err(stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(Some(stdout))
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
