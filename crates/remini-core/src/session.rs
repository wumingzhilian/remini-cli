use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTurn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredSession {
    pub id: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SessionStore {
    root: PathBuf,
}

impl SessionStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn append_turn(&self, session_id: &str, turn: &SessionTurn) -> Result<PathBuf, String> {
        validate_session_id(session_id)?;
        fs::create_dir_all(&self.root)
            .map_err(|err| format!("Cannot create session dir {}: {err}", self.root.display()))?;

        let path = self.session_path(session_id);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|err| format!("Cannot open session file {}: {err}", path.display()))?;
        let json = serde_json::to_string(turn)
            .map_err(|err| format!("Cannot serialize session turn: {err}"))?;
        writeln!(file, "{json}")
            .map_err(|err| format!("Cannot write session file {}: {err}", path.display()))?;
        Ok(path)
    }

    pub fn read_turns(&self, session_id: &str) -> Result<Vec<SessionTurn>, String> {
        validate_session_id(session_id)?;
        let path = self.session_path(session_id);
        let file = fs::File::open(&path)
            .map_err(|err| format!("Cannot open session file {}: {err}", path.display()))?;
        let reader = BufReader::new(file);
        let mut turns = Vec::new();
        for line in reader.lines() {
            let line =
                line.map_err(|err| format!("Cannot read session file {}: {err}", path.display()))?;
            if line.trim().is_empty() {
                continue;
            }
            let turn = serde_json::from_str(&line)
                .map_err(|err| format!("Invalid session record in {}: {err}", path.display()))?;
            turns.push(turn);
        }
        Ok(turns)
    }

    pub fn list_sessions(&self) -> Result<Vec<StoredSession>, String> {
        if !self.root.exists() {
            return Ok(Vec::new());
        }

        let mut sessions = Vec::new();
        for entry in fs::read_dir(&self.root)
            .map_err(|err| format!("Cannot read session dir {}: {err}", self.root.display()))?
        {
            let entry = entry.map_err(|err| {
                format!(
                    "Cannot read session dir entry {}: {err}",
                    self.root.display()
                )
            })?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("jsonl") {
                continue;
            }
            if let Some(stem) = path.file_stem().and_then(|value| value.to_str()) {
                sessions.push(StoredSession {
                    id: stem.to_string(),
                    path,
                });
            }
        }
        sessions.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(sessions)
    }

    pub fn latest_session(&self) -> Result<Option<StoredSession>, String> {
        Ok(self.list_sessions()?.into_iter().last())
    }

    fn session_path(&self, session_id: &str) -> PathBuf {
        self.root.join(format!("{session_id}.jsonl"))
    }
}

fn validate_session_id(session_id: &str) -> Result<(), String> {
    if session_id.is_empty() {
        return Err("session_id must not be empty".to_string());
    }

    let valid = session_id
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'));
    if !valid || session_id == "." || session_id == ".." || Path::new(session_id).is_absolute() {
        return Err("session_id contains unsupported characters".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
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
    fn append_and_read_session_turns() {
        let temp_dir = make_temp_dir("remini-core-session-read");
        let store = SessionStore::new(&temp_dir);

        store
            .append_turn(
                "session-1",
                &SessionTurn {
                    role: "user".to_string(),
                    content: "hello".to_string(),
                },
            )
            .expect("append should succeed");

        let turns = store
            .read_turns("session-1")
            .expect("read turns should succeed");
        assert_eq!(turns.len(), 1);
        assert_eq!(turns[0].content, "hello");

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn list_and_latest_sessions_are_sorted_by_id() {
        let temp_dir = make_temp_dir("remini-core-session-list");
        let store = SessionStore::new(&temp_dir);
        let turn = SessionTurn {
            role: "assistant".to_string(),
            content: "ok".to_string(),
        };
        store.append_turn("session-a", &turn).expect("append a");
        store.append_turn("session-b", &turn).expect("append b");

        let sessions = store.list_sessions().expect("list should succeed");
        assert_eq!(sessions[0].id, "session-a");
        assert_eq!(
            store
                .latest_session()
                .expect("latest should succeed")
                .unwrap()
                .id,
            "session-b"
        );

        fs::remove_dir_all(temp_dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn rejects_path_like_session_ids() {
        let store = SessionStore::new("unused");
        let err = store
            .append_turn(
                "../escape",
                &SessionTurn {
                    role: "user".to_string(),
                    content: "bad".to_string(),
                },
            )
            .expect_err("path-like session id should fail");
        assert!(err.contains("unsupported"));
    }
}
