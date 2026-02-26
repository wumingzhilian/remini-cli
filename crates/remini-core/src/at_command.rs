use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::tool_registry::{ToolRegistry, ToolRequest, ToolResponse};

fn resolve_path_candidates(
    raw_path: &str,
    cwd: &Path,
    include_directories: &[PathBuf],
) -> Vec<PathBuf> {
    let path = PathBuf::from(raw_path);
    if path.is_absolute() {
        return vec![path];
    }

    let mut candidates = Vec::with_capacity(include_directories.len() + 1);
    candidates.push(cwd.join(&path));
    for include_dir in include_directories {
        let base = if include_dir.is_absolute() {
            include_dir.clone()
        } else {
            cwd.join(include_dir)
        };
        candidates.push(base.join(&path));
    }

    let mut unique = Vec::with_capacity(candidates.len());
    let mut seen = HashSet::new();
    for candidate in candidates {
        if seen.insert(candidate.clone()) {
            unique.push(candidate);
        }
    }
    unique
}

pub fn expand_at_command(
    input: &str,
    cwd: &Path,
    include_directories: &[PathBuf],
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

    let candidate_paths = resolve_path_candidates(raw_path, cwd, include_directories);
    for path in &candidate_paths {
        let read_many_result = registry.execute(ToolRequest::ReadManyFiles {
            path: path.clone(),
            max_files: 64,
        });
        if let Ok(ToolResponse::ReadManyFiles(files)) = read_many_result {
            if files.is_empty() {
                continue;
            }

            let body = files
                .into_iter()
                .map(|file| format!("## {}\n{}", file.path.display(), file.content))
                .collect::<Vec<_>>()
                .join("\n\n");

            let payload = if tail.is_empty() {
                body
            } else {
                format!("{body}\n\n{tail}")
            };
            return Ok(Some(payload));
        }
    }

    let searched_paths = candidate_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    Err(format!(
        "Failed to resolve @ command path: {raw_path}. Searched: {searched_paths}"
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
        let result = expand_at_command("hello world", Path::new("."), &[], &registry)
            .expect("should not fail");
        assert_eq!(result, None);
    }

    #[test]
    fn expands_file_input() {
        let temp_dir = make_temp_dir("remini-core-at-command-file");
        let file = temp_dir.join("note.txt");
        fs::write(&file, "hello from file").expect("failed to write fixture");

        let registry = ToolRegistry;
        let result = expand_at_command("@note.txt summarize", &temp_dir, &[], &registry)
            .expect("should work");
        let payload = result.expect("expected expanded payload");
        assert!(payload.contains("## "));
        assert!(payload.contains("hello from file"));
        assert!(payload.contains("summarize"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn expands_directory_input() {
        let temp_dir = make_temp_dir("remini-core-at-command-dir");
        fs::write(temp_dir.join("a.txt"), "a").expect("failed to write fixture");
        fs::write(temp_dir.join("b.txt"), "b").expect("failed to write fixture");

        let registry = ToolRegistry;
        let result = expand_at_command("@.", &temp_dir, &[], &registry).expect("should work");
        let payload = result.expect("expected expanded payload");
        assert!(payload.contains("## "));
        assert!(payload.contains("a"));
        assert!(payload.contains("b"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn expands_from_include_directory_when_missing_in_cwd() {
        let temp_dir = make_temp_dir("remini-core-at-command-include");
        let include_dir = temp_dir.join("extra");
        fs::create_dir_all(&include_dir).expect("failed to create include dir");
        fs::write(include_dir.join("note.txt"), "from include").expect("failed to write fixture");

        let registry = ToolRegistry;
        let result = expand_at_command(
            "@note.txt summarize",
            &temp_dir,
            &[PathBuf::from("extra")],
            &registry,
        )
        .expect("should work");
        let payload = result.expect("expected expanded payload");
        assert!(payload.contains("from include"));
        assert!(payload.contains("summarize"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn prefers_cwd_before_include_directories() {
        let temp_dir = make_temp_dir("remini-core-at-command-prefer-cwd");
        let include_dir = temp_dir.join("extra");
        fs::create_dir_all(&include_dir).expect("failed to create include dir");
        fs::write(temp_dir.join("note.txt"), "cwd version").expect("failed to write cwd fixture");
        fs::write(include_dir.join("note.txt"), "include version")
            .expect("failed to write include fixture");

        let registry = ToolRegistry;
        let result = expand_at_command(
            "@note.txt summarize",
            &temp_dir,
            &[include_dir.clone()],
            &registry,
        )
        .expect("should work");
        let payload = result.expect("expected expanded payload");
        assert!(payload.contains("cwd version"));
        assert!(!payload.contains("include version"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }
}
