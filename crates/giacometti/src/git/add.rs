// src/git/add.rs

//! Git add operations

use crate::backends::git::types::{BackendError, GitBackend};

/// Stage all changes for commit
///
/// Equivalent to `git add --all`
///
/// # Errors
///
/// Returns an error if the git add operation fails
pub fn add_all<B: GitBackend>(backend: &B) -> Result<(), BackendError> {
    backend.add(&["--all"])
}
