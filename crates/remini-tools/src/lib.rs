use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

const SKIPPED_DIRECTORY_NAMES: &[&str] = &[".git", "node_modules", "target"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolErrorKind {
    NotFound,
    NotFile,
    NotDirectory,
    InvalidInput,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrepMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteFileResult {
    pub path: PathBuf,
    pub bytes_written: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceResult {
    pub path: PathBuf,
    pub occurrences: usize,
    pub bytes_written: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCommandResult {
    pub command: String,
    pub status_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
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

pub fn write_file(path: &Path, content: &str) -> Result<WriteFileResult, ToolError> {
    if path.exists() && !path.is_file() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFile,
            message: format!("Path is not a file: {}", path.display()),
        });
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|err| map_io_error(err, parent))?;
        }
    }

    fs::write(path, content).map_err(|err| map_io_error(err, path))?;
    Ok(WriteFileResult {
        path: path.to_path_buf(),
        bytes_written: content.len(),
    })
}

pub fn replace_in_file(
    path: &Path,
    old_string: &str,
    new_string: &str,
    allow_multiple: bool,
) -> Result<ReplaceResult, ToolError> {
    if old_string.is_empty() {
        return Err(ToolError {
            kind: ToolErrorKind::InvalidInput,
            message: "old_string must not be empty".to_string(),
        });
    }

    let current = read_file(path)?;
    let occurrences = current.matches(old_string).count();
    if occurrences == 0 {
        return Err(ToolError {
            kind: ToolErrorKind::InvalidInput,
            message: format!("old_string was not found in {}", path.display()),
        });
    }

    if occurrences > 1 && !allow_multiple {
        return Err(ToolError {
            kind: ToolErrorKind::InvalidInput,
            message: format!(
                "old_string matched {occurrences} times in {}. Set allow_multiple to replace all matches.",
                path.display()
            ),
        });
    }

    let updated = current.replace(old_string, new_string);
    fs::write(path, &updated).map_err(|err| map_io_error(err, path))?;

    Ok(ReplaceResult {
        path: path.to_path_buf(),
        occurrences,
        bytes_written: updated.len(),
    })
}

pub fn run_shell_command(command: &str, cwd: &Path) -> Result<ShellCommandResult, ToolError> {
    let command = command.trim();
    if command.is_empty() {
        return Err(ToolError {
            kind: ToolErrorKind::InvalidInput,
            message: "command must not be empty".to_string(),
        });
    }

    if !cwd.exists() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFound,
            message: format!("Path does not exist: {}", cwd.display()),
        });
    }

    if !cwd.is_dir() {
        return Err(ToolError {
            kind: ToolErrorKind::NotDirectory,
            message: format!("Path is not a directory: {}", cwd.display()),
        });
    }

    let mut process = shell_command(command);
    let output = process
        .current_dir(cwd)
        .output()
        .map_err(|err| map_io_error(err, cwd))?;

    Ok(ShellCommandResult {
        command: command.to_string(),
        status_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

#[cfg(windows)]
fn shell_command(command: &str) -> Command {
    let mut process = Command::new("cmd");
    process.arg("/C").arg(command);
    process
}

#[cfg(not(windows))]
fn shell_command(command: &str) -> Command {
    let mut process = Command::new("sh");
    process.arg("-lc").arg(command);
    process
}

pub fn read_many_files(path: &Path, max_files: usize) -> Result<Vec<FileContent>, ToolError> {
    if !path.exists() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFound,
            message: format!("Path does not exist: {}", path.display()),
        });
    }

    let mut files = Vec::new();
    if path.is_file() {
        files.push(path.to_path_buf());
    } else if path.is_dir() {
        collect_files(path, &mut files)?;
    } else {
        return Err(ToolError {
            kind: ToolErrorKind::Io,
            message: format!("Unsupported path type: {}", path.display()),
        });
    }

    files.sort();
    if files.len() > max_files {
        files.truncate(max_files);
    }

    let mut result = Vec::new();
    for file in files {
        let content = match fs::read_to_string(&file) {
            Ok(text) => text,
            Err(_) => continue,
        };
        result.push(FileContent {
            path: file,
            content,
        });
    }

    Ok(result)
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

pub fn grep_search(root: &Path, query: &str) -> Result<Vec<GrepMatch>, ToolError> {
    if !root.exists() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFound,
            message: format!("Path does not exist: {}", root.display()),
        });
    }

    if !root.is_dir() {
        return Err(ToolError {
            kind: ToolErrorKind::NotDirectory,
            message: format!("Path is not a directory: {}", root.display()),
        });
    }

    let mut files = Vec::new();
    collect_files(root, &mut files)?;

    let mut matches = Vec::new();
    for file in files {
        let content = match fs::read_to_string(&file) {
            Ok(text) => text,
            Err(_) => continue,
        };

        for (index, line) in content.lines().enumerate() {
            if line.contains(query) {
                matches.push(GrepMatch {
                    path: file.clone(),
                    line_number: index + 1,
                    line: line.to_string(),
                });
            }
        }
    }

    matches.sort_by(|a, b| {
        let path_cmp = a.path.cmp(&b.path);
        if path_cmp == std::cmp::Ordering::Equal {
            a.line_number.cmp(&b.line_number)
        } else {
            path_cmp
        }
    });

    Ok(matches)
}

pub fn glob_search(root: &Path, pattern: &str) -> Result<Vec<PathBuf>, ToolError> {
    if !root.exists() {
        return Err(ToolError {
            kind: ToolErrorKind::NotFound,
            message: format!("Path does not exist: {}", root.display()),
        });
    }

    if !root.is_dir() {
        return Err(ToolError {
            kind: ToolErrorKind::NotDirectory,
            message: format!("Path is not a directory: {}", root.display()),
        });
    }

    let mut files = Vec::new();
    collect_files(root, &mut files)?;

    let mut matches = Vec::new();
    for file in files {
        if let Ok(relative) = file.strip_prefix(root) {
            let normalized = relative.to_string_lossy().replace('\\', "/");
            if wildcard_match(pattern, &normalized) {
                matches.push(file);
            }
        }
    }

    matches.sort();
    Ok(matches)
}

fn collect_files(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), ToolError> {
    for entry in fs::read_dir(root).map_err(|err| map_io_error(err, root))? {
        let entry = entry.map_err(|err| map_io_error(err, root))?;
        let path = entry.path();
        let metadata = entry.metadata().map_err(|err| map_io_error(err, &path))?;
        if metadata.is_file() {
            out.push(path);
        } else if metadata.is_dir() {
            if should_skip_directory(&path) {
                continue;
            }
            collect_files(&path, out)?;
        }
    }
    Ok(())
}

fn should_skip_directory(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| SKIPPED_DIRECTORY_NAMES.contains(&name))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternToken {
    Literal(char),
    Star,
    DoubleStar,
    Question,
}

fn tokenize_pattern(pattern: &str) -> Vec<PatternToken> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    tokens.push(PatternToken::DoubleStar);
                    i += 2;
                } else {
                    tokens.push(PatternToken::Star);
                    i += 1;
                }
            }
            '?' => {
                tokens.push(PatternToken::Question);
                i += 1;
            }
            c => {
                tokens.push(PatternToken::Literal(c));
                i += 1;
            }
        }
    }
    tokens
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
    let tokens = tokenize_pattern(pattern);
    let text_chars: Vec<char> = text.chars().collect();
    let p_len = tokens.len();
    let t_len = text_chars.len();

    let mut dp = vec![vec![false; t_len + 1]; p_len + 1];
    dp[0][0] = true;

    for i in 1..=p_len {
        if matches!(tokens[i - 1], PatternToken::Star | PatternToken::DoubleStar) {
            dp[i][0] = dp[i - 1][0];
        }
    }

    for i in 1..=p_len {
        for j in 1..=t_len {
            match tokens[i - 1] {
                PatternToken::Star => {
                    dp[i][j] = dp[i - 1][j] || (text_chars[j - 1] != '/' && dp[i][j - 1]);
                }
                PatternToken::DoubleStar => {
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                }
                PatternToken::Question => {
                    dp[i][j] = text_chars[j - 1] != '/' && dp[i - 1][j - 1];
                }
                PatternToken::Literal(c) => {
                    if c == text_chars[j - 1] {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }
    }

    dp[p_len][t_len]
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
    fn write_file_creates_parent_directories() {
        let temp_dir = make_temp_dir("remini-tools-write-file");
        let file_path = temp_dir.join("nested").join("note.txt");

        let result = write_file(&file_path, "hello write").expect("write_file should succeed");
        assert_eq!(result.bytes_written, "hello write".len());
        assert_eq!(
            fs::read_to_string(&file_path).expect("file should exist"),
            "hello write"
        );

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn replace_in_file_replaces_single_match() {
        let temp_dir = make_temp_dir("remini-tools-replace-single");
        let file_path = temp_dir.join("sample.txt");
        fs::write(&file_path, "hello old world").expect("failed to write fixture");

        let result = replace_in_file(&file_path, "old", "new", false)
            .expect("replace_in_file should succeed");
        assert_eq!(result.occurrences, 1);
        assert_eq!(
            fs::read_to_string(&file_path).expect("file should exist"),
            "hello new world"
        );

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn replace_in_file_rejects_multiple_matches_by_default() {
        let temp_dir = make_temp_dir("remini-tools-replace-multiple");
        let file_path = temp_dir.join("sample.txt");
        fs::write(&file_path, "old old").expect("failed to write fixture");

        let err = replace_in_file(&file_path, "old", "new", false)
            .expect_err("replace_in_file should fail for multiple matches");
        assert_eq!(err.kind, ToolErrorKind::InvalidInput);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn replace_in_file_allows_multiple_matches_when_requested() {
        let temp_dir = make_temp_dir("remini-tools-replace-allow-multiple");
        let file_path = temp_dir.join("sample.txt");
        fs::write(&file_path, "old old").expect("failed to write fixture");

        let result = replace_in_file(&file_path, "old", "new", true)
            .expect("replace_in_file should succeed");
        assert_eq!(result.occurrences, 2);
        assert_eq!(
            fs::read_to_string(&file_path).expect("file should exist"),
            "new new"
        );

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn run_shell_command_captures_stdout_and_status() {
        let temp_dir = make_temp_dir("remini-tools-shell-success");

        let result = run_shell_command("printf shell-ok", &temp_dir)
            .expect("run_shell_command should succeed");
        assert_eq!(result.status_code, Some(0));
        assert_eq!(result.stdout, "shell-ok");
        assert_eq!(result.stderr, "");

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn run_shell_command_captures_non_zero_status() {
        let temp_dir = make_temp_dir("remini-tools-shell-failure");

        let result = run_shell_command("exit 7", &temp_dir)
            .expect("run_shell_command should return command result");
        assert_eq!(result.status_code, Some(7));

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

    #[test]
    fn grep_search_finds_matches() {
        let temp_dir = make_temp_dir("remini-tools-grep-search");
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir).expect("failed to create src dir");
        fs::write(src_dir.join("a.txt"), "hello\nneedle line\nbye").expect("failed to write file");
        fs::write(src_dir.join("b.txt"), "needle once").expect("failed to write file");
        fs::write(src_dir.join("c.txt"), "nothing here").expect("failed to write file");

        let matches = grep_search(&temp_dir, "needle").expect("grep_search should succeed");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 2);
        assert_eq!(matches[1].line_number, 1);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn glob_search_matches_patterns() {
        let temp_dir = make_temp_dir("remini-tools-glob-search");
        let src_dir = temp_dir.join("src");
        let docs_dir = temp_dir.join("docs");
        fs::create_dir_all(&src_dir).expect("failed to create src dir");
        fs::create_dir_all(&docs_dir).expect("failed to create docs dir");
        fs::write(src_dir.join("main.rs"), "fn main() {}").expect("failed to write file");
        fs::write(src_dir.join("lib.rs"), "pub fn x() {}").expect("failed to write file");
        fs::write(docs_dir.join("guide.md"), "# Guide").expect("failed to write file");

        let rs_matches = glob_search(&temp_dir, "src/*.rs").expect("glob_search should succeed");
        assert_eq!(rs_matches.len(), 2);

        let md_matches = glob_search(&temp_dir, "*.md").expect("glob_search should succeed");
        assert_eq!(md_matches.len(), 0);

        let nested_md_matches =
            glob_search(&temp_dir, "docs/*.md").expect("glob_search should succeed");
        assert_eq!(nested_md_matches.len(), 1);

        let recursive_md_matches =
            glob_search(&temp_dir, "**/*.md").expect("glob_search should succeed");
        assert_eq!(recursive_md_matches.len(), 1);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn read_many_files_reads_directory_files() {
        let temp_dir = make_temp_dir("remini-tools-read-many-files");
        fs::write(temp_dir.join("a.txt"), "a").expect("failed to write test file");
        fs::write(temp_dir.join("b.txt"), "b").expect("failed to write test file");
        fs::create_dir_all(temp_dir.join("nested")).expect("failed to create nested dir");
        fs::write(temp_dir.join("nested").join("c.txt"), "c").expect("failed to write test file");

        let contents =
            read_many_files(&temp_dir, 10).expect("read_many_files should succeed for directory");
        assert_eq!(contents.len(), 3);
        assert!(contents.iter().any(|item| item.content == "a"));
        assert!(contents.iter().any(|item| item.content == "b"));
        assert!(contents.iter().any(|item| item.content == "c"));

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn read_many_files_skips_generated_directories() {
        let temp_dir = make_temp_dir("remini-tools-read-many-skip-generated");
        fs::write(temp_dir.join("root.txt"), "root").expect("failed to write root file");
        fs::create_dir_all(temp_dir.join("target")).expect("failed to create target dir");
        fs::write(temp_dir.join("target").join("generated.txt"), "generated")
            .expect("failed to write generated file");
        fs::create_dir_all(temp_dir.join("node_modules")).expect("failed to create node_modules");
        fs::write(temp_dir.join("node_modules").join("dep.txt"), "dep")
            .expect("failed to write dep file");

        let contents =
            read_many_files(&temp_dir, 10).expect("read_many_files should succeed for directory");
        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0].content, "root");

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn read_many_files_honors_limit() {
        let temp_dir = make_temp_dir("remini-tools-read-many-limit");
        fs::write(temp_dir.join("a.txt"), "a").expect("failed to write test file");
        fs::write(temp_dir.join("b.txt"), "b").expect("failed to write test file");
        fs::write(temp_dir.join("c.txt"), "c").expect("failed to write test file");

        let contents =
            read_many_files(&temp_dir, 2).expect("read_many_files should succeed for directory");
        assert_eq!(contents.len(), 2);

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }
}
