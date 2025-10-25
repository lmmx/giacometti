// src/backends/types.rs

//! Shared backend types and traits

/// Common error trait for all backend errors
pub trait BackendError: std::error::Error {
    /// Get the error message
    fn message(&self) -> &str;
}
