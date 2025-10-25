// src/tests/mock/git.rs

use crate::backends::types::{BackendError, GitBackend, ResetMode};
use super::MockBackend;

impl GitBackend for MockBackend {
    fn add(&self, _: &[&str]) -> Result<(), BackendError> {
        Err(BackendError::new("not implemented"))
    }

    fn commit(&self, _: &str) -> Result<(), BackendError> {
        Err(BackendError::new("not implemented"))
    }

    fn reset(&self, _: ResetMode, _: &str) -> Result<(), BackendError> {
        Err(BackendError::new("not implemented"))
    }

    fn tag(&self, _: &str, _: Option<&str>, _: bool) -> Result<(), BackendError> {
        Err(BackendError::new("not implemented"))
    }

    fn rev_parse(&self, args: &[&str]) -> Result<String, BackendError> {
        assert_eq!(args, &["--abbrev-ref", "HEAD"]);
        Ok(self.branch_name.clone())
    }

    fn push(&self, _: &str, _: &str) -> Result<(), BackendError> {
        Err(BackendError::new("not implemented"))
    }
}
