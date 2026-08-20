//! Confidence levels for derived knowledge

use serde::{Deserialize, Serialize};
use std::fmt;

/// Confidence level for derived facts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Confidence {
    /// Directly observable in source code
    Exact = 4,
    /// Statically derivable with high certainty
    High = 3,
    /// Reasonably deducible but not certain
    Medium = 2,
    /// Plausible but uncertain
    Low = 1,
    /// Cannot be determined
    Unknown = 0,
}

impl Confidence {
    pub fn as_str(&self) -> &'static str {
        match self {
            Confidence::Exact => "exact",
            Confidence::High => "high",
            Confidence::Medium => "medium",
            Confidence::Low => "low",
            Confidence::Unknown => "unknown",
        }
    }

    pub fn threshold_for_agent(&self) -> bool {
        // Agent consumes facts at or above this level by default
        *self >= Confidence::Medium
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Confidence::Unknown
    }
}

impl fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Provenance information for a derived fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactProvenance {
    pub confidence: Confidence,
    pub source: ProvenanceSource,
    pub evidence: Option<String>,  // File:line or other reference
    pub derived_at: u64,           // Unix timestamp
    pub derived_by: String,        // Tool/analyzer name
}

/// Source of the fact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ProvenanceSource {
    /// Explicit in source code (type annotations, declarations)
    Explicit = 1,
    /// Derived from static analysis (type inference, call graph)
    StaticAnalysis = 2,
    /// Discovered by language analyzer (imports, calls)
    Discovered = 3,
    /// Inferred from patterns/heuristics
    Inferred = 4,
    /// From .acc-memory.md or similar
    Memory = 5,
    /// User-provided
    Manual = 6,
}