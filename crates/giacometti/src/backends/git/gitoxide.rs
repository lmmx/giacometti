// src/backends/git/gitoxide.rs

//! Gitoxide-based git backend (not yet implemented)

use super::types::{GitBackend, GitBackendError, ResetMode};

/// Gitoxide backend for pure-Rust git operations (not yet implemented)
pub struct GitoxideBackend;

impl GitoxideBackend {
    /// Create a new gitoxide backend
    ///
    /// # Errors
    ///
    /// Currently always returns an error as this backend is not yet implemented
    pub fn new() -> Result<Self, GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }
}

impl GitBackend for GitoxideBackend {
    fn add(&self, _pathspec: &[&str]) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }

    fn commit(&self, _message: &str) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }

    fn reset(&self, _mode: ResetMode, _target: &str) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }

    fn tag(&self, _name: &str, _message: Option<&str>, _annotated: bool) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }

    fn rev_parse(&self, _args: &[&str]) -> Result<String, GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }

    fn push(&self, _remote: &str, _refspec: &str) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("Gitoxide backend not yet implemented"))
    }
}
