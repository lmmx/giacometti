// src/git/reset.rs

//! Git reset operations

use crate::backends::git::types::{GitBackendError, GitBackend, ResetMode};

/// Perform a soft reset to the target commit
///
/// Moves HEAD but keeps index and working tree unchanged
///
/// # Errors
///
/// Returns an error if the git reset operation fails
pub fn soft_reset<B: GitBackend>(backend: &B, target: &str) -> Result<(), GitBackendError> {
    backend.reset(ResetMode::Soft, target)
}
