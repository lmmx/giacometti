// src/git/reset.rs

//! Git reset operations

use crate::backends::types::{BackendError, GitBackend, ResetMode};

pub fn soft_reset(backend: &dyn GitBackend, target: &str) -> Result<(), BackendError> {
    backend.reset(ResetMode::Soft, target)
}
