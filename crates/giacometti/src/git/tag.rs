// src/git/tag.rs

//! Git tag operations

use crate::backends::types::{BackendError, GitBackend};

pub fn annotated_tag(
    backend: &dyn GitBackend,
    name: &str,
    message: &str,
) -> Result<(), BackendError> {
    backend.tag(name, Some(message), true)
}
