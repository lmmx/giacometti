// src/backends/gitoxide.rs

//! Gitoxide-based git backend (not yet implemented)

use super::types::{BackendError, GitBackend, ResetMode};

pub struct GitoxideBackend;

impl GitoxideBackend {
    pub fn new() -> Result<Self, BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }
}

impl GitBackend for GitoxideBackend {
    fn add(&self, _pathspec: &[&str]) -> Result<(), BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }

    fn commit(&self, _message: &str) -> Result<(), BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }

    fn reset(&self, _mode: ResetMode, _target: &str) -> Result<(), BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }

    fn tag(&self, _name: &str, _message: Option<&str>, _annotated: bool) -> Result<(), BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }

    fn rev_parse(&self, _args: &[&str]) -> Result<String, BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }

    fn push(&self, _remote: &str, _refspec: &str) -> Result<(), BackendError> {
        Err(BackendError::new("Gitoxide backend not yet implemented"))
    }
}
