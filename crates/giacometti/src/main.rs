// src/main.rs

//! Giacometti - Hardened Git CLI with policy enforcement
//!
//! A security-focused Git wrapper that enforces policies with principle of least privilege.

mod policy;

use std::convert::TryFrom;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: gcmti <git-command> [args...]");
        eprintln!("\nExamples:");
        eprintln!("  gcmti status");
        eprintln!("  gcmti commit -m 'message'");
        eprintln!("  gcmti push origin main");
        return ExitCode::FAILURE;
    }

    // Enforce policy before executing
    if let Err(e) = policy::enforce(&args) {
        eprintln!("Policy violation: {e}");
        return ExitCode::FAILURE;
    }

    run_git(&args)
}

/// Execute git command with policy enforcement
fn run_git(args: &[String]) -> ExitCode {
    let status = Command::new("git").args(args).status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                ExitCode::SUCCESS
            } else {
                // Preserve git's exit code safely and without truncation
                exit_status
                    .code()
                    .and_then(|code| u8::try_from(code).ok())
                    .map_or(ExitCode::FAILURE, ExitCode::from)
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to execute git command: {e}");
            eprintln!("\nMake sure git is installed and available in your PATH.");
            ExitCode::FAILURE
        }
    }
}
