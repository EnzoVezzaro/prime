//! Prime Core - Fundamental data structures for the Prime knowledge graph
//!
//! This crate defines the core types that represent a codebase as a compact,
//! language-agnostic knowledge artifact optimized for agent consumption.

pub mod types;
pub mod hash;
pub mod confidence;
pub mod language;
pub mod agent;
pub mod par;
pub mod projection;

pub use types::*;
pub use hash::{ContentHash, EntityId, HashChain, FileHashTracker, EntityId as HashEntityId};
pub use confidence::*;
pub use language::*;
pub use agent::*;
pub use par::*;
pub use projection::*;

pub use serde_json;