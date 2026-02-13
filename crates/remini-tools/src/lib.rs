use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolErrorKind {
    NotFound,
    NotFile,
    NotDirectory,
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolError {
    pub kind: ToolErrorKind,
    pub message: String,
}

impl Display for ToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ToolError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
}

pub fn read_file(path: &Path) -> Result<String, ToolError> {
    if !path.exists() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFound,
            message: format!("Path does not exist: {}", path.display()),
        });
    }

    if !path.is_file() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFile,
            message: format!("Path is not a file: {}", path.display()),
        });
    }

    fs::read_to_string(path).map_err(|err| map_io_error(err, path))
}

pub fn list_directory(path: &Path) -> Result<Vec<DirectoryEntry>, ToolError> {
    if !path.exists() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFound,
            message: format!("Path does not exist: {}", path.display()),
        });
    }

    if !path.is_dir() {
        return Err(ToolError {
            kind: ToolErrorKind::NotDirectory,
            message: format!("Path is not a directory: {}", path.display()),
        });
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(path).map_err(|err| map_io_error(err, path))? {
        let entry = entry.map_err(|err| map_io_error(err, path))?;
        let entry_path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|err| map_io_error(err, &entry_path))?;
        entries.push(DirectoryEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry_path,
            is_directory: metadata.is_dir(),
        });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}

fn map_io_error(err: io::Error, path: &Path) -> ToolError {
    ToolError {
        kind: ToolErrorKind::Io,
        message: format!("I/O error at {}: {}", path.display(), err),
    }
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
    fn read_file_success() {
        let temp_dir = make_temp_dir("remini-tools-read-file-success");
        let file_path = temp_dir.join("sample.txt");
        fs::write(&file_path, "hello remini").expect("failed to write test file");

        let result = read_file(&file_path).expect("read_file should succeed");
        assert_eq!(result, "hello remini");

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn read_file_reports_not_found() {
        let temp_dir = make_temp_dir("remini-tools-read-file-not-found");
        let file_path = temp_dir.join("missing.txt");

        let result = read_file(&file_path).expect_err("read_file should fail");
        assert_eq!(result.kind, ToolErrorKind::NotFound);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn list_directory_success_sorted() {
        let temp_dir = make_temp_dir("remini-tools-list-directory-success");
        fs::write(temp_dir.join("z.txt"), "z").expect("failed to write test file");
        fs::create_dir_all(temp_dir.join("a-dir")).expect("failed to create test dir");
        fs::write(temp_dir.join("m.txt"), "m").expect("failed to write test file");

        let entries = list_directory(&temp_dir).expect("list_directory should succeed");
        let names: Vec<&str> = entries.iter().map(|entry| entry.name.as_str()).collect();
        assert_eq!(names, vec!["a-dir", "m.txt", "z.txt"]);

        let first = entries.first().expect("directory should have entries");
        assert!(first.is_directory);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn list_directory_reports_not_directory() {
        let temp_dir = make_temp_dir("remini-tools-list-directory-not-directory");
        let file_path = temp_dir.join("single.txt");
        fs::write(&file_path, "file").expect("failed to write test file");

        let result = list_directory(&file_path).expect_err("list_directory should fail");
        assert_eq!(result.kind, ToolErrorKind::NotDirectory);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }
}
