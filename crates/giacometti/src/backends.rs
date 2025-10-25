// src/backends/mod.rs

//! Git backends

pub mod types;
pub mod shell;
pub mod gitoxide;
pub mod github_api;

pub use types::{BackendError, GitBackend, ResetMode};
pub use shell::ShellBackend;
pub use gitoxide::GitoxideBackend;
pub use github_api::GitHubApiBackend;
