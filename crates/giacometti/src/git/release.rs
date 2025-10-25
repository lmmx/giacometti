// src/git/release.rs

//! Release automation workflow

use crate::backends::shell::ShellBackend;
use crate::backends::types::GitBackend;
use crate::git;
use std::process::{Command, ExitCode};

pub fn run(bump_level: &str) -> ExitCode {
    let backend = ShellBackend::new();

    if let Err(e) = run_release(&backend, bump_level) {
        eprintln!("❌ Release failed: {}", e.message);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run_release<B: GitBackend>(backend: &B, bump_level: &str) -> Result<(), crate::backends::types::BackendError> {
    eprintln!("📦 Starting release (bump: {bump_level})");

    // 1. Bump version
    uv_bump(bump_level)?;

    // 2. Temp commit to extract version
    git::add::add_all(backend)?;
    git::commit::commit(backend, "chore(temp): version check")?;

    // 3. Get version
    let version = uv_get_version()?;
    eprintln!("✂️  Version: {version}");

    // 4. Reset and real commit
    git::reset::soft_reset(backend, "HEAD~1")?;
    git::add::add_all(backend)?;
    git::commit::commit(backend, &format!("chore(release): bump 🐍 -> v{version}"))?;

    // 5. Tag
    git::tag::annotated_tag(backend, &format!("py-{version}"), &format!("Python Release {version}"))?;

    // 6. Push
    let branch = git::rev_parse::get_current_branch(backend)?;
    git::push::push(backend, "origin", &branch)?;
    git::push::push(backend, "origin", &format!("py-{version}"))?;

    eprintln!("✅ Release complete: v{version}");
    Ok(())
}

fn uv_bump(level: &str) -> Result<(), crate::backends::types::BackendError> {
    // TODO: move to uv module
    match Command::new("uv").args(["version", "--bump", level]).status() {
        Ok(status) if status.success() => Ok(()),
        _ => Err(crate::backends::types::BackendError::new("uv bump failed")),
    }
}

fn uv_get_version() -> Result<String, crate::backends::types::BackendError> {
    // TODO: move to uv module
    match Command::new("uv").args(["version", "--short"]).output() {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
        _ => Err(crate::backends::types::BackendError::new("uv version failed")),
    }
}
