//! Genesis Module
//!
//! The Genesis module handles PROJECT GENESIS - the creation of foundational planning
//! documents for brand new projects.
//!
//! This is distinct from Catalyst (task-based generation) and operates at the
//! architectural/system level.

pub mod engine;
pub mod prompts;

pub use engine::GenesisEngine;
pub use prompts::{build_genesis_user_prompt, parse_genesis_response, GENESIS_SYSTEM_PROMPT};
