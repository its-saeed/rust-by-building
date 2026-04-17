//! Discover and run standalone exercise files.
//!
//! Exercises live at `lessons/NN-slug/exercises/exK_*.rs`. Each file is
//! self-contained: `fn main`, optional `#[cfg(test)]` block, no external
//! crates. We compile each with `rustc --test` into a tempdir and run it
//! — pass means it compiled cleanly AND every test inside passed.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Exercise {
    /// Numeric index parsed from the filename prefix (`ex3_...` → 3).
    pub index: u32,
    /// Slug portion after the prefix (`ex3_early_return.rs` → `early_return`).
    pub slug: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExerciseResult {
    Passed,
    CompileFailed,
    TestFailed,
}

/// List every `exN_*.rs` under `<lesson_dir>/exercises/`, sorted by index.
pub fn discover(lesson_dir: &Path) -> Result<Vec<Exercise>> {
    let dir = lesson_dir.join("exercises");
    if !dir.exists() { return Ok(Vec::new()); }

    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).with_context(|| format!("reading {dir:?}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e != "rs").unwrap_or(true) { continue; }

        let fname = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        // Expect "exN_slug"
        let Some(rest) = fname.strip_prefix("ex") else { continue; };
        let Some((num_str, slug)) = rest.split_once('_') else { continue; };
        let Ok(index) = num_str.parse::<u32>() else { continue; };

        out.push(Exercise { index, slug: slug.to_string(), path });
    }
    out.sort_by_key(|e| e.index);
    Ok(out)
}

/// Compile an exercise with `rustc --test` and run it. Returns the
/// outcome plus captured stderr (for display on failure).
pub fn run(ex: &Exercise) -> Result<(ExerciseResult, String)> {
    let tmp = std::env::temp_dir().join(format!("rbb-ex-{}-{}", ex.index, ex.slug));
    std::fs::create_dir_all(&tmp)?;
    let bin = tmp.join("test-bin");

    // Compile.
    let compile = Command::new("rustc")
        .args(["--edition", "2021", "--test"])
        .arg(&ex.path)
        .arg("-o").arg(&bin)
        .output()?;

    if !compile.status.success() {
        let stderr = String::from_utf8_lossy(&compile.stderr).into_owned();
        return Ok((ExerciseResult::CompileFailed, stderr));
    }

    // Run.
    let run = Command::new(&bin).output()?;
    if run.status.success() {
        Ok((ExerciseResult::Passed, String::new()))
    } else {
        let mut combined = String::from_utf8_lossy(&run.stdout).into_owned();
        combined.push_str(&String::from_utf8_lossy(&run.stderr));
        Ok((ExerciseResult::TestFailed, combined))
    }
}
