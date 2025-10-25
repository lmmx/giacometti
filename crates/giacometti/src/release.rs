// src/release.rs

//! Release automation workflow

use std::process::{Command, ExitCode};

/// Run the release workflow (MVP: exact replica of `just release`)
#[must_use]
pub fn run(bump_level: &str) -> ExitCode {
    eprintln!("📦 Starting release workflow (bump: {bump_level})");

    // Steps match .just/ship.just exactly
    if !bump_version(bump_level) {
        return ExitCode::FAILURE;
    }
    if !stage_all() {
        return ExitCode::FAILURE;
    }
    if !temp_commit() {
        return ExitCode::FAILURE;
    }

    let Some(version) = get_version() else {
        return ExitCode::FAILURE;
    };

    if !reset_soft() {
        return ExitCode::FAILURE;
    }
    if !stage_all() {
        return ExitCode::FAILURE;
    }
    if !real_commit(&version) {
        return ExitCode::FAILURE;
    }
    if !create_tag(&version) {
        return ExitCode::FAILURE;
    }

    let Some(branch) = get_branch() else {
        return ExitCode::FAILURE;
    };

    if !push_commit(&branch) {
        return ExitCode::FAILURE;
    }
    if !push_tag(&version) {
        return ExitCode::FAILURE;
    }

    eprintln!("✅ Release complete: v{version}");
    ExitCode::SUCCESS
}

fn bump_version(level: &str) -> bool {
    eprintln!("📈 Bumping version: {level}");
    run_cmd("uv", &["version", "--bump", level])
}

fn stage_all() -> bool {
    run_cmd("git", &["add", "--all"])
}

fn temp_commit() -> bool {
    run_cmd("git", &["commit", "-m", "chore(temp): version check"])
}

fn get_version() -> Option<String> {
    eprintln!("✂️  Extracting version");
    Command::new("uv")
        .args(["version", "--short"])
        .output()
        .ok()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn reset_soft() -> bool {
    run_cmd("git", &["reset", "--soft", "HEAD~1"])
}

fn real_commit(version: &str) -> bool {
    eprintln!("✅ Creating release commit");
    let msg = format!("chore(release): bump 🐍 -> v{version}");
    run_cmd("git", &["commit", "-m", &msg])
}

fn create_tag(version: &str) -> bool {
    eprintln!("🏷️  Creating tag");
    let tag = format!("py-{version}");
    let msg = format!("Python Release {version}");
    run_cmd("git", &["tag", "-a", &tag, "-m", &msg])
}

fn get_branch() -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn push_commit(branch: &str) -> bool {
    eprintln!("🚀 Pushing commit to {branch}");
    run_cmd("git", &["push", "origin", branch])
}

fn push_tag(version: &str) -> bool {
    eprintln!("🚀 Pushing tag");
    let tag = format!("py-{version}");
    run_cmd("git", &["push", "origin", &tag])
}

fn run_cmd(cmd: &str, args: &[&str]) -> bool {
    match Command::new(cmd).args(args).status() {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("❌ Failed to run {cmd}: {e}");
            false
        }
    }
}
