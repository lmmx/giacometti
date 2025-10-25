// src/backends/shell.rs

//! Shell-based git backend (MVP)

use super::types::{BackendError, GitBackend, ResetMode};
use std::process::Command;

pub struct ShellBackend;

impl ShellBackend {
    pub fn new() -> Self {
        Self
    }

    fn run(&self, args: &[&str]) -> Result<(), BackendError> {
        match Command::new("git").args(args).status() {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(BackendError::new("Git command failed")),
            Err(e) => Err(BackendError::new(format!("Failed to execute git: {e}"))),
        }
    }

    fn run_output(&self, args: &[&str]) -> Result<String, BackendError> {
        match Command::new("git").args(args).output() {
            Ok(output) if output.status.success() => {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
            Ok(_) => Err(BackendError::new("Git command failed")),
            Err(e) => Err(BackendError::new(format!("Failed to execute git: {e}"))),
        }
    }
}

impl GitBackend for ShellBackend {
    fn add(&self, pathspec: &[&str]) -> Result<(), BackendError> {
        let mut args = vec!["add"];
        args.extend(pathspec);
        self.run(&args)
    }

    fn commit(&self, message: &str) -> Result<(), BackendError> {
        self.run(&["commit", "-m", message])
    }

    fn reset(&self, mode: ResetMode, target: &str) -> Result<(), BackendError> {
        let mode_str = match mode {
            ResetMode::Soft => "--soft",
            ResetMode::Mixed => "--mixed",
            ResetMode::Hard => "--hard",
        };
        self.run(&["reset", mode_str, target])
    }

    fn tag(&self, name: &str, message: Option<&str>, annotated: bool) -> Result<(), BackendError> {
        let mut args = vec!["tag"];
        if annotated {
            args.push("-a");
        }
        args.push(name);
        if let Some(msg) = message {
            args.push("-m");
            args.push(msg);
        }
        self.run(&args)
    }

    fn rev_parse(&self, args: &[&str]) -> Result<String, BackendError> {
        let mut cmd_args = vec!["rev-parse"];
        cmd_args.extend(args);
        self.run_output(&cmd_args)
    }

    fn push(&self, remote: &str, refspec: &str) -> Result<(), BackendError> {
        self.run(&["push", remote, refspec])
    }
}
