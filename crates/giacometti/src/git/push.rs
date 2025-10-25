// src/git/push.rs

//! Git push operations

use crate::backends::git::types::{BackendError, GitBackend};

/// Push a refspec to a remote repository
///
/// # Errors
///
/// Returns an error if the git push operation fails
pub fn push<B: GitBackend>(backend: &B, remote: &str, refspec: &str) -> Result<(), BackendError> {
    backend.push(remote, refspec)
}
