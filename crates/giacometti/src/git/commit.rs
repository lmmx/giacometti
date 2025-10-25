// src/git/commit.rs

//! Git commit operations

use crate::backends::types::{BackendError, GitBackend};

pub fn commit(backend: &dyn GitBackend, message: &str) -> Result<(), BackendError> {
    backend.commit(message)
}
