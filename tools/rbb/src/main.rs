//! Student CLI for the Rust by Building course.
//!
//! Minimal-viable surface: status / open / test / next.
//! `watch` is a thin wrapper around `test` on a timer for now — full
//! filesystem-watcher support is a later polish step.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use rbb_core::{discover_lessons, exercises, find_lesson, find_repo_root, LessonId, LessonStatus};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "rbb", about = "Rust by Building — student CLI", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Show all lessons and your progress.
    Status,
    /// Print a lesson's README to stdout.
    Open { lesson: String },
    /// Compile + run every exercise in a lesson, report pass/fail per file.
    Check { lesson: String },
    /// Run tests for a lesson's project.
    Test { lesson: String },
    /// Rerun tests every time something changes.
    Watch { lesson: String },
    /// Jump to the next lesson you haven't finished.
    Next,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = find_repo_root(&std::env::current_dir()?)
        .context("run rbb from inside your rust-by-building checkout")?;

    match cli.cmd {
        Cmd::Status           => cmd_status(&root),
        Cmd::Open { lesson }  => cmd_open(&root, &lesson),
        Cmd::Check { lesson } => {
            let all_ok = cmd_check(&root, &lesson)?;
            if !all_ok { std::process::exit(1); }
            Ok(())
        }
        Cmd::Test { lesson }  => {
            // Propagate test failure as a non-zero exit code so shell
            // pipelines and CI actually notice. `cmd_test` itself does
            // NOT exit — watch mode also calls it and must survive.
            let passed = cmd_test(&root, &lesson)?;
            if !passed { std::process::exit(1); }
            Ok(())
        }
        Cmd::Watch { lesson } => cmd_watch(&root, &lesson),
        Cmd::Next             => cmd_next(&root),
    }
}

fn parse_id(s: &str) -> Result<LessonId> { s.parse() }

fn cmd_status(root: &PathBuf) -> Result<()> {
    let lessons = discover_lessons(root)?;
    let progress = rbb_core::progress::load()?;

    println!("{:>3}  {:<20} {:>7}  {:<5}  {}", "id", "lesson", "ex", "proj", "status");
    for l in lessons {
        let lp = progress.lessons.get(&l.id);
        let total_ex = exercises::discover(&l.path).map(|v| v.len() as u32).unwrap_or(0);
        let passed_ex = lp.map(|p| p.exercises_passed()).unwrap_or(0);
        let proj = lp.map(|p| p.project_passing).unwrap_or(false);
        let status = lp.map(|p| p.status).unwrap_or_default();

        let ex_col = format!("{}/{}", passed_ex, total_ex);
        let proj_col = if proj { "✓".green().to_string() } else { "-".dimmed().to_string() };
        let status_col = match status {
            LessonStatus::NotStarted => "not started".dimmed().to_string(),
            LessonStatus::InProgress => "in progress".yellow().to_string(),
            LessonStatus::Done       => "done".green().to_string(),
        };
        println!("{:>3}  {:<20} {:>7}  {:<5}  {}", l.id, l.slug, ex_col, proj_col, status_col);
    }
    Ok(())
}

/// Compile + run every exercise in the lesson. Persists per-exercise
/// outcome to the student's progress file. Returns whether all passed.
fn cmd_check(root: &PathBuf, lesson: &str) -> Result<bool> {
    let id = parse_id(lesson)?;
    let l = find_lesson(root, id)?;
    let ex_list = exercises::discover(&l.path)?;

    if ex_list.is_empty() {
        println!("no exercises in lesson {id}");
        return Ok(true);
    }

    let mut progress = rbb_core::progress::load()?;
    let lp = progress.lessons.entry(id).or_default();
    let mut all_ok = true;

    for ex in &ex_list {
        let (result, log) = exercises::run(ex)?;
        let (mark, color) = match result {
            exercises::ExerciseResult::Passed        => ("✓", "green"),
            exercises::ExerciseResult::CompileFailed => ("✗ compile", "red"),
            exercises::ExerciseResult::TestFailed    => ("✗ tests",   "red"),
        };
        let line = format!("  [{mark}] ex{}_{}", ex.index, ex.slug);
        match color {
            "green" => println!("{}", line.green()),
            _       => { println!("{}", line.red()); all_ok = false; }
        }
        if !matches!(result, exercises::ExerciseResult::Passed) && !log.is_empty() {
            for l in log.lines().take(10) {
                println!("      {}", l.dimmed());
            }
        }
        lp.exercises.insert(ex.index, result);
    }

    if lp.status == LessonStatus::NotStarted {
        lp.status = LessonStatus::InProgress;
    }
    let passing = lp.exercises_passed();
    progress.last_active = Some(now_rfc3339());
    rbb_core::progress::save(&progress)?;

    println!("\n{} of {} exercises passing", passing, ex_list.len());
    Ok(all_ok)
}

fn cmd_open(root: &PathBuf, lesson: &str) -> Result<()> {
    let id = parse_id(lesson)?;
    let l = find_lesson(root, id)?;
    let readme = l.path.join("README.md");
    let text = std::fs::read_to_string(&readme)
        .with_context(|| format!("reading {readme:?}"))?;
    println!("{text}");
    Ok(())
}

/// Runs the lesson's project tests. Returns whether they passed.
/// Does NOT exit the process — the caller decides how to react.
fn cmd_test(root: &PathBuf, lesson: &str) -> Result<bool> {
    let id = parse_id(lesson)?;
    let l = find_lesson(root, id)?;
    let project = l.path.join("project").join("Cargo.toml");
    if !project.exists() {
        anyhow::bail!("lesson {id} has no project at {project:?}");
    }

    let status = Command::new("cargo")
        .args(["test", "--manifest-path"])
        .arg(&project)
        .status()?;

    if status.success() {
        mark_project_passing(id)?;
        println!("{}", "all tests passed".green());
        Ok(true)
    } else {
        println!("{}", "tests failed".red());
        Ok(false)
    }
}

fn cmd_watch(root: &PathBuf, lesson: &str) -> Result<()> {
    use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let id = parse_id(lesson)?;
    let l = find_lesson(root, id)?;

    println!("watching {} — Ctrl-C to stop", l.path.display().cyan());

    // Initial run so the student sees state without having to touch a file.
    let _ = cmd_test(root, lesson);

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(300), move |res| {
        // Forward anything (Ok or Err) — cmd_test itself reports compile
        // status, so a spurious wake is cheap.
        let _ = tx.send(res);
    })?;

    debouncer.watcher().watch(&l.path, RecursiveMode::Recursive)?;

    for _ in rx {
        // Clear screen between runs so feedback replaces itself. ANSI
        // CSI H + J: cursor to 0,0 and erase below.
        print!("\x1b[H\x1b[2J");
        let _ = cmd_test(root, lesson);
    }
    Ok(())
}

fn cmd_next(root: &PathBuf) -> Result<()> {
    let lessons = discover_lessons(root)?;
    let progress = rbb_core::progress::load()?;

    for l in lessons {
        let done = progress.lessons.get(&l.id).map(|lp| lp.status) == Some(LessonStatus::Done);
        if !done {
            println!("next up: lesson {} — {}", l.id, l.slug);
            println!("  rbb open {}", l.id);
            return Ok(());
        }
    }
    println!("{}", "all lessons complete".green());
    Ok(())
}

fn mark_project_passing(id: LessonId) -> Result<()> {
    let mut p = rbb_core::progress::load()?;
    let lp = p.lessons.entry(id).or_default();
    lp.project_passing = true;
    lp.status = LessonStatus::Done;
    p.last_active = Some(now_rfc3339());
    rbb_core::progress::save(&p)
}

fn now_rfc3339() -> String {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("@{t}")
}
