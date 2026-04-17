//! Per-student progress file. Lives at `~/.rbb/progress.json`.

use crate::{exercises::ExerciseResult, LessonId, LessonStatus};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Progress {
    /// When the student last touched anything. Used by `rbb-admin progress`
    /// to sort the dashboard.
    pub last_active: Option<String>,
    pub lessons: BTreeMap<LessonId, LessonProgress>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LessonProgress {
    pub project_passing: bool,
    pub status: LessonStatus,
    /// Per-exercise result keyed by exercise index. Populated by
    /// `rbb check <lesson>` (and by `rbb watch` on every rerun). An
    /// exercise not listed here has not been checked yet.
    #[serde(default)]
    pub exercises: BTreeMap<u32, ExerciseResult>,
}

impl LessonProgress {
    pub fn exercises_passed(&self) -> u32 {
        self.exercises.values().filter(|r| **r == ExerciseResult::Passed).count() as u32
    }
}

pub fn progress_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME is unset")?;
    Ok(PathBuf::from(home).join(".rbb").join("progress.json"))
}

pub fn load() -> Result<Progress> {
    let path = progress_path()?;
    if !path.exists() {
        return Ok(Progress::default());
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {path:?}"))?;
    Ok(serde_json::from_str(&text).unwrap_or_default())
}

pub fn save(p: &Progress) -> Result<()> {
    let path = progress_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(p)?;
    std::fs::write(&path, text).with_context(|| format!("writing {path:?}"))?;
    Ok(())
}
