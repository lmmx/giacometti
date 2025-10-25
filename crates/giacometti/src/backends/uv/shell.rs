// src/backends/uv/shell.rs

//! Shell-based uv backend (MVP)

use super::types::{UvBackend, UvBackendError};
use std::process::Command;

/// Shell backend that executes uv commands via the system uv binary
pub struct ShellBackend;

impl ShellBackend {
    /// Create a new shell backend
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Run a uv command and return success/failure
    ///
    /// # Errors
    ///
    /// Returns an error if the uv command fails or cannot be executed
    #[allow(clippy::unused_self)]
    fn run(&self, args: &[&str]) -> Result<(), UvBackendError> {
        match Command::new("uv").args(args).status() {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(UvBackendError::new("UV command failed")),
            Err(e) => Err(UvBackendError::new(format!("Failed to execute uv: {e}"))),
        }
    }

    /// Run a uv command and capture its output
    ///
    /// # Errors
    ///
    /// Returns an error if the uv command fails or cannot be executed
    #[allow(clippy::unused_self)]
    fn run_output(&self, args: &[&str]) -> Result<String, UvBackendError> {
        match Command::new("uv").args(args).output() {
            Ok(output) if output.status.success() => {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
            Ok(_) => Err(UvBackendError::new("UV command failed")),
            Err(e) => Err(UvBackendError::new(format!("Failed to execute uv: {e}"))),
        }
    }
}

impl Default for ShellBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl UvBackend for ShellBackend {
    fn bump_version(&self, level: &str) -> Result<(), UvBackendError> {
        self.run(&["version", "--bump", level])
    }

    fn get_version(&self) -> Result<String, UvBackendError> {
        self.run_output(&["version", "--short"])
    }
}
