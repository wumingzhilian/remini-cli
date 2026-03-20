use std::path::PathBuf;

use remini_tools::{
    glob_search, grep_search, list_directory, read_file, read_many_files, replace_in_file,
    write_file, DirectoryEntry, FileContent, GrepMatch, ReplaceResult, ToolError, WriteFileResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRequest {
    ReadFile { path: PathBuf },
    ReadManyFiles { path: PathBuf, max_files: usize },
    WriteFile { path: PathBuf, content: String },
    Replace {
        path: PathBuf,
        old_string: String,
        new_string: String,
        allow_multiple: bool,
    },
    ListDirectory { path: PathBuf },
    GlobSearch { root: PathBuf, pattern: String },
    GrepSearch { root: PathBuf, query: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolResponse {
    ReadFile(String),
    ReadManyFiles(Vec<FileContent>),
    WriteFile(WriteFileResult),
    Replace(ReplaceResult),
    ListDirectory(Vec<DirectoryEntry>),
    GlobSearch(Vec<PathBuf>),
    GrepSearch(Vec<GrepMatch>),
}

pub fn builtin_tool_descriptors() -> &'static [ToolDescriptor] {
    &[
        ToolDescriptor {
            name: "glob",
            description: "match files by wildcard pattern",
        },
        ToolDescriptor {
            name: "grep_search",
            description: "search text within files",
        },
        ToolDescriptor {
            name: "list_directory",
            description: "list files and directories",
        },
        ToolDescriptor {
            name: "read_file",
            description: "read text content from a file",
        },
        ToolDescriptor {
            name: "read_many_files",
            description: "read text content from many files",
        },
        ToolDescriptor {
            name: "write_file",
            description: "write text content to a file",
        },
        ToolDescriptor {
            name: "replace",
            description: "replace exact text within a file",
        },
    ]
}

pub fn format_tool_list(include_descriptions: bool) -> String {
    let body = builtin_tool_descriptors()
        .iter()
        .map(|tool| {
            if include_descriptions {
                format!("{} - {}", tool.name, tool.description)
            } else {
                tool.name.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("Available tools:\n{body}")
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
            ToolRequest::WriteFile { path, content } => {
                let result = write_file(&path, &content)?;
                Ok(ToolResponse::WriteFile(result))
            }
            ToolRequest::Replace {
                path,
                old_string,
                new_string,
                allow_multiple,
            } => {
                let result = replace_in_file(&path, &old_string, &new_string, allow_multiple)?;
                Ok(ToolResponse::Replace(result))
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
    fn write_file_request_works() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-write-file");
        let file = temp_dir.join("nested").join("note.txt");

        let registry = ToolRegistry;
        let response = registry
            .execute(ToolRequest::WriteFile {
                path: file.clone(),
                content: "saved".to_string(),
            })
            .expect("tool request should succeed");

        let result = match response {
            ToolResponse::WriteFile(result) => result,
            _ => panic!("expected write-file response"),
        };
        assert_eq!(result.path, file);
        assert_eq!(result.bytes_written, 5);
        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn replace_request_works() {
        let temp_dir = make_temp_dir("remini-core-tool-registry-replace");
        let file = temp_dir.join("note.txt");
        fs::write(&file, "before").expect("failed to write fixture");

        let registry = ToolRegistry;
        let response = registry
            .execute(ToolRequest::Replace {
                path: file,
                old_string: "before".to_string(),
                new_string: "after".to_string(),
                allow_multiple: false,
            })
            .expect("tool request should succeed");

        let result = match response {
            ToolResponse::Replace(result) => result,
            _ => panic!("expected replace response"),
        };
        assert_eq!(result.occurrences, 1);
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

    #[test]
    fn descriptors_include_read_only_tools() {
        let names = builtin_tool_descriptors()
            .iter()
            .map(|tool| tool.name)
            .collect::<Vec<_>>();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"read_many_files"));
        assert!(names.contains(&"write_file"));
        assert!(names.contains(&"replace"));
        assert!(names.contains(&"grep_search"));
    }

    #[test]
    fn format_tool_list_can_include_descriptions() {
        let output = format_tool_list(true);
        assert!(output.contains("Available tools"));
        assert!(output.contains("read_file - read text content"));
    }
}
