// src/git/commit.rs

//! Git commit operations

use crate::backends::git::types::{BackendError, GitBackend};

/// Create a commit with the given message
///
/// # Errors
///
/// Returns an error if the git commit operation fails
pub fn commit<B: GitBackend>(backend: &B, message: &str) -> Result<(), BackendError> {
    backend.commit(message)
}
