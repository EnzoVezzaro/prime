//! Agent-Native Semantic Representation extensions

use serde::{Deserialize, Serialize};
use crate::hash::EntityId;

/// Effect types for functions/methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    Pure,
    IO,
    Read,
    Write,
    Allocate,
    Fail,
    Diverge,
}

/// Capability required by an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: Option<String>,
}

/// Semantic metadata attached to an Entity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SemanticMetadata {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub effects: Vec<Effect>,
    pub capabilities: Vec<Capability>,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub invariants: Vec<String>,
    pub errors: Vec<String>,
    pub external_resources: Vec<String>,
    pub evidence_refs: Vec<EvidenceRef>,
}

/// Evidence reference for a semantic fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source: String,
    pub location: Option<String>,
    pub kind: EvidenceKind,
}

/// Kind of evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    StaticAnalysis,
    AstAnalysis,
    RuntimeObservation,
    TestResult,
    BenchmarkResult,
    CommitInfo,
}

/// Extended entity with semantic metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEntity {
    pub base_id: EntityId,
    pub semantic: SemanticMetadata,
}

impl SemanticMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effects.push(effect);
        self
    }
}
