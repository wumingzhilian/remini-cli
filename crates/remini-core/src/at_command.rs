use std::path::{Path, PathBuf};

use crate::tool_registry::{ToolRegistry, ToolRequest, ToolResponse};

fn resolve_path(raw_path: &str, cwd: &Path) -> PathBuf {
    let path = PathBuf::from(raw_path);
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

pub fn expand_at_command(
    input: &str,
    cwd: &Path,
    registry: &ToolRegistry,
) -> Result<Option<String>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('@') {
        return Ok(None);
    }

    let remainder = &trimmed[1..];
    let (raw_path, tail) = if let Some((path, rest)) = remainder.split_once(' ') {
        (path.trim(), rest.trim())
    } else {
        (remainder.trim(), "")
    };

    if raw_path.is_empty() {
        return Ok(None);
    }

    let path = resolve_path(raw_path, cwd);
    let read_file_result = registry.execute(ToolRequest::ReadFile { path: path.clone() });
    if let Ok(ToolResponse::ReadFile(content)) = read_file_result {
        let payload = if tail.is_empty() {
            content
        } else {
            format!("{content}\n\n{tail}")
        };
        return Ok(Some(payload));
    }

    let list_dir_result = registry.execute(ToolRequest::ListDirectory { path: path.clone() });
    if let Ok(ToolResponse::ListDirectory(entries)) = list_dir_result {
        let listing = entries
            .into_iter()
            .map(|entry| entry.name)
            .collect::<Vec<_>>()
            .join("\n");
        let payload = if tail.is_empty() {
            listing
        } else {
            format!("{listing}\n\n{tail}")
        };
        return Ok(Some(payload));
    }

    Err(format!(
        "Failed to resolve @ command path: {}",
        path.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
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
    fn returns_none_for_non_at_input() {
        let registry = ToolRegistry;
        let result =
            expand_at_command("hello world", Path::new("."), &registry).expect("should not fail");
        assert_eq!(result, None);
    }

    #[test]
    fn expands_file_input() {
        let temp_dir = make_temp_dir("remini-core-at-command-file");
        let file = temp_dir.join("note.txt");
        fs::write(&file, "hello from file").expect("failed to write fixture");

        let registry = ToolRegistry;
        let result =
            expand_at_command("@note.txt summarize", &temp_dir, &registry).expect("should work");
        assert_eq!(result, Some("hello from file\n\nsummarize".to_string()));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn expands_directory_input() {
        let temp_dir = make_temp_dir("remini-core-at-command-dir");
        fs::write(temp_dir.join("a.txt"), "a").expect("failed to write fixture");
        fs::write(temp_dir.join("b.txt"), "b").expect("failed to write fixture");

        let registry = ToolRegistry;
        let result = expand_at_command("@.", &temp_dir, &registry).expect("should work");
        let payload = result.expect("expected expanded payload");
        assert!(payload.contains("a.txt"));
        assert!(payload.contains("b.txt"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }
}
