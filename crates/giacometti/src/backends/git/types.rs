// src/backends/git/types.rs

//! Backend trait and types

use std::fmt;

/// Git backend trait for abstracting git operations
///
/// Implementations provide different ways to execute git commands:
/// - `ShellBackend`: shells out to git binary
/// - `GitoxideBackend`: uses gitoxide library (not yet implemented)
/// - `GitHubApiBackend`: uses GitHub API for verified commits (not yet implemented)
pub trait GitBackend {
    /// Stage files for commit
    ///
    /// # Errors
    ///
    /// Returns an error if the git add operation fails
    fn add(&self, pathspec: &[&str]) -> Result<(), GitBackendError>;

    /// Create a commit with the given message
    ///
    /// # Errors
    ///
    /// Returns an error if the git commit operation fails
    fn commit(&self, message: &str) -> Result<(), GitBackendError>;

    /// Reset the current HEAD to the specified state
    ///
    /// # Errors
    ///
    /// Returns an error if the git reset operation fails
    fn reset(&self, mode: ResetMode, target: &str) -> Result<(), GitBackendError>;

    /// Create a git tag
    ///
    /// # Errors
    ///
    /// Returns an error if the git tag operation fails
    fn tag(&self, name: &str, message: Option<&str>, annotated: bool) -> Result<(), GitBackendError>;

    /// Execute git rev-parse with the given arguments
    ///
    /// # Errors
    ///
    /// Returns an error if the git rev-parse operation fails
    fn rev_parse(&self, args: &[&str]) -> Result<String, GitBackendError>;

    /// Push to a remote repository
    ///
    /// # Errors
    ///
    /// Returns an error if the git push operation fails
    fn push(&self, remote: &str, refspec: &str) -> Result<(), GitBackendError>;
}

/// Git reset mode
#[derive(Debug, Clone, Copy)]
pub enum ResetMode {
    /// Soft reset - move HEAD only, keep index and working tree
    Soft,
    /// Mixed reset - move HEAD and reset index, keep working tree (default)
    Mixed,
    /// Hard reset - move HEAD, reset index and working tree (destructive)
    Hard,
}

/// Error returned by git backend operations
#[derive(Debug)]
pub struct GitBackendError {
    /// Error message
    pub message: String,
}

impl GitBackendError {
    /// Create a new git backend error
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GitBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GitBackendError {}

impl crate::backends::types::BackendError for GitBackendError {
    fn message(&self) -> &str {
        &self.message
    }
}
