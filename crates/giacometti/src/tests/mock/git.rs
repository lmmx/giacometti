// src/tests/mock/git.rs

use crate::backends::git::types::{GitBackendError, GitBackend, ResetMode};
use super::MockBackend;

impl GitBackend for MockBackend {
    fn add(&self, _: &[&str]) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("not implemented"))
    }

    fn commit(&self, _: &str) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("not implemented"))
    }

    fn reset(&self, _: ResetMode, _: &str) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("not implemented"))
    }

    fn tag(&self, _: &str, _: Option<&str>, _: bool) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("not implemented"))
    }

    fn rev_parse(&self, args: &[&str]) -> Result<String, GitBackendError> {
        assert_eq!(args, &["--abbrev-ref", "HEAD"]);
        Ok(self.branch_name.clone())
    }

    fn push(&self, _: &str, _: &str) -> Result<(), GitBackendError> {
        Err(GitBackendError::new("not implemented"))
    }
}
