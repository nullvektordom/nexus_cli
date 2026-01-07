//! Planning Catalyst - AI-powered planning document generation
//!
//! This module provides intelligent generation of planning documents (02-05)
//! from a user's vision document (01) using `DeepSeek` R1 with sequential thinking.

pub mod engine;
pub mod generator;
pub mod prompts;
pub mod validation;

pub use engine::CatalystEngine;
pub use generator::DocumentType;
