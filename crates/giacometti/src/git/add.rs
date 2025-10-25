// src/git/add.rs

//! Git add operations

use crate::backends::types::{BackendError, GitBackend};

pub fn add_all(backend: &dyn GitBackend) -> Result<(), BackendError> {
    backend.add(&["--all"])
}
