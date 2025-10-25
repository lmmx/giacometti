// src/git/push.rs

//! Git push operations

use crate::backends::types::{BackendError, GitBackend};

pub fn push(backend: &dyn GitBackend, remote: &str, refspec: &str) -> Result<(), BackendError> {
    backend.push(remote, refspec)
}
