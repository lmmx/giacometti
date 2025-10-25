// src/lib.rs

//! Giacometti - Hardened Git CLI with policy enforcement
//!
//! A security-focused Git wrapper that enforces policies with principle of least privilege.

pub mod backends;
pub mod git;
pub mod policy;
pub mod release;

#[cfg(test)]
mod tests;
