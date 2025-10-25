// src/backends/github_api.rs

//! GitHub API-based backend for verified commits (not yet implemented)

use super::types::{BackendError, GitBackend, ResetMode};

pub struct GitHubApiBackend;

impl GitHubApiBackend {
    pub fn new() -> Result<Self, BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }
}

impl GitBackend for GitHubApiBackend {
    fn add(&self, _pathspec: &[&str]) -> Result<(), BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }

    fn commit(&self, _message: &str) -> Result<(), BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }

    fn reset(&self, _mode: ResetMode, _target: &str) -> Result<(), BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }

    fn tag(&self, _name: &str, _message: Option<&str>, _annotated: bool) -> Result<(), BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }

    fn rev_parse(&self, _args: &[&str]) -> Result<String, BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }

    fn push(&self, _remote: &str, _refspec: &str) -> Result<(), BackendError> {
        Err(BackendError::new("GitHub API backend not yet implemented"))
    }
}
