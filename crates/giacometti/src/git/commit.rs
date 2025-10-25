// src/git/commit.rs

//! Git commit operations

use crate::backends::types::{BackendError, GitBackend};

/// Create a commit with the given message
///
/// # Errors
///
/// Returns an error if the git commit operation fails
pub fn commit(backend: &dyn GitBackend, message: &str) -> Result<(), BackendError> {
    backend.commit(message)
}
