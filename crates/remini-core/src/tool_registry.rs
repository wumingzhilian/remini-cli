use std::path::PathBuf;

use remini_tools::{
    glob_search, grep_search, list_directory, read_file, read_many_files, DirectoryEntry,
    FileContent, GrepMatch, ToolError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRequest {
    ReadFile { path: PathBuf },
    ReadManyFiles { path: PathBuf, max_files: usize },
    ListDirectory { path: PathBuf },
    GlobSearch { root: PathBuf, pattern: String },
    GrepSearch { root: PathBuf, query: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolResponse {
    ReadFile(String),
    ReadManyFiles(Vec<FileContent>),
    ListDirectory(Vec<DirectoryEntry>),
    GlobSearch(Vec<PathBuf>),
    GrepSearch(Vec<GrepMatch>),
}

#[derive(Debug, Default, Clone)]
pub struct ToolRegistry;

impl ToolRegistry {
    pub fn execute(&self, request: ToolRequest) -> Result<ToolResponse, ToolError> {
        match request {
            ToolRequest::ReadFile { path } => {
                let content = read_file(&path)?;
                Ok(ToolResponse::ReadFile(content))
            }
            ToolRequest::ReadManyFiles { path, max_files } => {
                let content = read_many_files(&path, max_files)?;
                Ok(ToolResponse::ReadManyFiles(content))
            }
            ToolRequest::ListDirectory { path } => {
                let entries = list_directory(&path)?;
                Ok(ToolResponse::ListDirectory(entries))
            }
            ToolRequest::GlobSearch { root, pattern } => {
                let entries = glob_search(&root, &pattern)?;
                Ok(ToolResponse::GlobSearch(entries))
            }
            ToolRequest::GrepSearch { root, query } => {
                let entries = grep_search(&root, &query)?;
                Ok(ToolResponse::GrepSearch(entries))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use remini_tools::ToolErrorKind;

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
    fn read_file_request_works() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-read-file");
        let file = temp_dir.join("note.txt");
        fs::write(&file, "hello").expect("failed to write fixture");

        let registry = ToolRegistry;
        let response = registry
            .execute(ToolRequest::ReadFile { path: file })
            .expect("tool request should succeed");

        assert_eq!(response, ToolResponse::ReadFile("hello".to_string()));
        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn list_directory_request_works() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-list-directory");
        fs::write(temp_dir.join("a.txt"), "a").expect("failed to write fixture");
        fs::write(temp_dir.join("b.txt"), "b").expect("failed to write fixture");

        let registry = ToolRegistry;
        let response = registry
            .execute(ToolRequest::ListDirectory {
                path: temp_dir.clone(),
            })
            .expect("tool request should succeed");

        let entries = match response {
            ToolResponse::ListDirectory(entries) => entries,
            _ => panic!("expected list directory response"),
        };
        assert_eq!(entries.len(), 2);
        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn read_many_files_request_works() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-read-many");
        fs::write(temp_dir.join("a.txt"), "a").expect("failed to write fixture");
        fs::write(temp_dir.join("b.txt"), "b").expect("failed to write fixture");

        let registry = ToolRegistry;
        let response = registry
            .execute(ToolRequest::ReadManyFiles {
                path: temp_dir.clone(),
                max_files: 10,
            })
            .expect("tool request should succeed");

        let files = match response {
            ToolResponse::ReadManyFiles(items) => items,
            _ => panic!("expected read-many-files response"),
        };
        assert_eq!(files.len(), 2);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn grep_search_request_works() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-grep");
        fs::write(temp_dir.join("a.txt"), "needle").expect("failed to write fixture");
        fs::write(temp_dir.join("b.txt"), "none").expect("failed to write fixture");

        let registry = ToolRegistry;
        let response = registry
            .execute(ToolRequest::GrepSearch {
                root: temp_dir.clone(),
                query: "needle".to_string(),
            })
            .expect("tool request should succeed");

        let matches = match response {
            ToolResponse::GrepSearch(matches) => matches,
            _ => panic!("expected grep response"),
        };
        assert_eq!(matches.len(), 1);
        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn missing_file_returns_error() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-error");
        let registry = ToolRegistry;
        let result = registry.execute(ToolRequest::ReadFile {
            path: temp_dir.join("missing.txt"),
        });
        let err = result.expect_err("expected read_file error");
        assert_eq!(err.kind, ToolErrorKind::NotFound);
        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }
}
