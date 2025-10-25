// src/backends/types.rs

//! Backend trait and types

use std::process::ExitCode;

/// Git backend trait
pub trait GitBackend {
    fn add(&self, pathspec: &[&str]) -> Result<(), BackendError>;
    fn commit(&self, message: &str) -> Result<(), BackendError>;
    fn reset(&self, mode: ResetMode, target: &str) -> Result<(), BackendError>;
    fn tag(&self, name: &str, message: Option<&str>, annotated: bool) -> Result<(), BackendError>;
    fn rev_parse(&self, args: &[&str]) -> Result<String, BackendError>;
    fn push(&self, remote: &str, refspec: &str) -> Result<(), BackendError>;
}

#[derive(Debug)]
pub enum ResetMode {
    Soft,
    Mixed,
    Hard,
}

#[derive(Debug)]
pub struct BackendError {
    pub message: String,
}

impl BackendError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
