// src/git/rev_parse.rs

//! Git rev-parse operations

use crate::backends::types::{BackendError, GitBackend};

/// Get the name of the current branch
///
/// # Errors
///
/// Returns an error if the git rev-parse operation fails
pub fn get_current_branch<B: GitBackend>(backend: &B) -> Result<String, BackendError> {
    backend.rev_parse(&["--abbrev-ref", "HEAD"])
}
