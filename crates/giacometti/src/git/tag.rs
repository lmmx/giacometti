// src/git/tag.rs

//! Git tag operations

use crate::backends::git::types::{BackendError, GitBackend};

/// Create an annotated tag with a message
///
/// # Errors
///
/// Returns an error if the git tag operation fails
pub fn annotated_tag<B: GitBackend>(
    backend: &B,
    name: &str,
    message: &str,
) -> Result<(), BackendError> {
    backend.tag(name, Some(message), true)
}
