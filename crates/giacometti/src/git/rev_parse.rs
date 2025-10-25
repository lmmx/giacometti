// src/git/rev_parse.rs

//! Git rev-parse operations

use crate::backends::types::{BackendError, GitBackend};

pub fn get_current_branch(backend: &dyn GitBackend) -> Result<String, BackendError> {
    backend.rev_parse(&["--abbrev-ref", "HEAD"])
}
