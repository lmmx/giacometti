// src/backends/uv/rust_crate.rs

//! Rust crate-based uv backend (not yet implemented)

use super::types::{UvBackend, UvBackendError};

/// Rust crate backend for direct uv operations (not yet implemented)
pub struct RustCrateBackend;

impl RustCrateBackend {
    /// Create a new Rust crate backend
    ///
    /// # Errors
    ///
    /// Currently always returns an error as this backend is not yet implemented
    pub fn new() -> Result<Self, UvBackendError> {
        Err(UvBackendError::new("Rust crate backend not yet implemented"))
    }
}

impl UvBackend for RustCrateBackend {
    fn bump_version(&self, _level: &str) -> Result<(), UvBackendError> {
        Err(UvBackendError::new("Rust crate backend not yet implemented"))
    }

    fn get_version(&self) -> Result<String, UvBackendError> {
        Err(UvBackendError::new("Rust crate backend not yet implemented"))
    }
}
