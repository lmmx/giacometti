// src/policy.rs

//! Policy enforcement for git operations

use std::fmt;

/// Policy violation error
#[derive(Debug)]
pub struct PolicyError {
    message: String,
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl PolicyError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Enforce security policies on git command
pub fn enforce(args: &[String]) -> Result<(), PolicyError> {
    if args.is_empty() {
        return Err(PolicyError::new("No command provided"));
    }

    let command = &args[0];

    // For now, implement a simple allowlist
    match command.as_str() {
        // Read-only operations (safe)
        "status" | "log" | "diff" | "show" | "branch" | "remote" => Ok(()),

        // Write operations (need more checks)
        "commit" | "add" | "push" | "pull" => {
            Ok(())
        }

        // Dangerous operations (blocked by default)
        "reset" | "rebase" | "force" | "push --force" => {
            Err(PolicyError::new(format!(
                "Command '{command}' is blocked by security policy",
            )))
        }

        // Unknown commands (block by default for least privilege)
        _ => Err(PolicyError::new(format!(
            "Command '{command}' is not in the allowlist",
        ))),
    }
}
