// src/git/rev_parse.rs

//! Git rev-parse operations

use crate::backends::git::types::{GitBackendError, GitBackend};

/// Get the name of the current branch
///
/// # Errors
///
/// Returns an error if the git rev-parse operation fails
pub fn get_current_branch<B: GitBackend>(backend: &B) -> Result<String, GitBackendError> {
    backend.rev_parse(&["--abbrev-ref", "HEAD"])
}
