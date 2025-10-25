// src/backends/uv/types.rs

//! UV backend trait and types

use std::fmt;

/// UV backend trait for abstracting uv operations
///
/// Implementations provide different ways to execute uv commands:
/// - `ShellBackend`: shells out to uv binary
/// - `RustCrateBackend`: uses uv as a Rust crate (not yet implemented)
pub trait UvBackend {
    /// Bump the version in pyproject.toml
    ///
    /// # Errors
    ///
    /// Returns an error if the uv version bump operation fails
    fn bump_version(&self, level: &str) -> Result<(), UvBackendError>;

    /// Get the current version from pyproject.toml
    ///
    /// # Errors
    ///
    /// Returns an error if the uv version read operation fails
    fn get_version(&self) -> Result<String, UvBackendError>;
}

/// Error returned by UV backend operations
#[derive(Debug)]
pub struct UvBackendError {
    /// Error message
    pub message: String,
}

impl UvBackendError {
    /// Create a new UV backend error
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for UvBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UvBackendError {}

impl crate::backends::types::BackendError for UvBackendError {
    fn message(&self) -> &str {
        &self.message
    }
}
