// src/git/release.rs

//! Release automation workflow

use crate::backends::git::shell::ShellBackend as GitShellBackend;
use crate::backends::git::types::GitBackend;
use crate::backends::uv::shell::ShellBackend as UvShellBackend;
use crate::backends::uv::types::UvBackend;
use crate::git;
use std::process::ExitCode;

pub fn run(bump_level: &str) -> ExitCode {
    let git_backend = GitShellBackend::new();
    let uv_backend = UvShellBackend::new();

    if let Err(e) = run_release(&git_backend, &uv_backend, bump_level) {
        eprintln!("❌ Release failed: {}", e.message);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run_release<G: GitBackend, U: UvBackend>(
    git_backend: &G,
    uv_backend: &U,
    bump_level: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("📦 Starting release (bump: {bump_level})");

    // 1. Bump version
    uv_backend.bump_version(bump_level)?;

    // 2. Temp commit to extract version
    git::add::add_all(git_backend)?;
    git::commit::commit(git_backend, "chore(temp): version check")?;

    // 3. Get version
    let version = uv_backend.get_version()?;
    eprintln!("✂️  Version: {version}");

    // 4. Reset and real commit
    git::reset::soft_reset(git_backend, "HEAD~1")?;
    git::add::add_all(git_backend)?;
    git::commit::commit(git_backend, &format!("chore(release): bump 🐍 -> v{version}"))?;

    // 5. Tag
    git::tag::annotated_tag(git_backend, &format!("py-{version}"), &format!("Python Release {version}"))?;

    // 6. Push
    let branch = git::rev_parse::get_current_branch(git_backend)?;
    git::push::push(git_backend, "origin", &branch)?;
    git::push::push(git_backend, "origin", &format!("py-{version}"))?;

    eprintln!("✅ Release complete: v{version}");
    Ok(())
}
