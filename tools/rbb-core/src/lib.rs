//! Shared types and helpers for the student `rbb` CLI and the admin
//! `rbb-admin` CLI. Keep this crate free of side effects (no process
//! spawning, no stdio) — just data modeling and pure logic.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub mod exercises;
pub mod progress;

/// A lesson directory on disk, like `lessons/03-functions/`.
#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: LessonId,
    pub slug: String,        // "functions"
    pub path: PathBuf,       // absolute path to lesson dir
}

/// Numeric identifier of a lesson. Parsed from the directory prefix, e.g.
/// `lessons/03-functions/` → `LessonId(3)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LessonId(pub u32);

impl std::fmt::Display for LessonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for LessonId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(LessonId(s.trim_start_matches('0').parse().unwrap_or(0)))
    }
}

/// Discover all lessons under `<root>/lessons/`. Sorted by `LessonId`.
pub fn discover_lessons(root: &Path) -> Result<Vec<Lesson>> {
    let dir = root.join("lessons");
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).with_context(|| format!("reading {dir:?}"))? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() { continue; }

        let name = entry.file_name().to_string_lossy().into_owned();
        // Expect "NN-slug" form.
        let Some((num, slug)) = name.split_once('-') else { continue; };
        let Ok(id) = num.parse::<u32>() else { continue; };

        out.push(Lesson {
            id: LessonId(id),
            slug: slug.to_string(),
            path,
        });
    }
    out.sort_by_key(|l| l.id);
    Ok(out)
}

/// Locate a lesson by its numeric id.
pub fn find_lesson(root: &Path, id: LessonId) -> Result<Lesson> {
    discover_lessons(root)?
        .into_iter()
        .find(|l| l.id == id)
        .ok_or_else(|| anyhow!("no lesson with id {id}"))
}

/// Walks up from `start` looking for a directory that contains the
/// workspace `Cargo.toml` with a `[workspace]` table. Returns that dir.
pub fn find_repo_root(start: &Path) -> Result<PathBuf> {
    let mut cur = start.canonicalize()?;
    loop {
        let cargo = cur.join("Cargo.toml");
        if cargo.exists() {
            let text = std::fs::read_to_string(&cargo)?;
            if text.contains("[workspace]") {
                return Ok(cur);
            }
        }
        let Some(parent) = cur.parent() else { break; };
        cur = parent.to_path_buf();
    }
    Err(anyhow!("not inside a rust-by-building workspace"))
}

/// Result summary of running a lesson's tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOutcome {
    pub passed: u32,
    pub failed: u32,
    pub log: String,
}

impl TestOutcome {
    pub fn ok(&self) -> bool { self.failed == 0 && self.passed > 0 }
}

/// Thin wrapper so both CLIs can pretty-print the same state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CourseStatus {
    pub lessons: BTreeMap<LessonId, LessonStatus>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum LessonStatus {
    #[default]
    NotStarted,
    InProgress,
    Done,
}
