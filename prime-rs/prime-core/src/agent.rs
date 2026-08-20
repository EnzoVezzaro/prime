//! Agent-Native Interface types for Prime
//!
//! This module defines the standardized response envelope, tool request/response types,
//! and source escalation metadata for the agent-facing interface.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::confidence::{Confidence, FactProvenance};
use crate::hash::EntityId;
use crate::types::{SymbolKind, RelationKind, Range};
use crate::language::Language;
use crate::types::Entity;

// ─── Response Status ────────────────────────────────────────────────────────────

/// Status of a Prime tool response
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    /// Prime has complete knowledge for this request
    Complete,
    /// Prime has partial knowledge; source escalation may be needed
    Partial,
    /// Prime cannot determine coverage
    Unknown,
    /// The requested operation is not supported
    Unsupported,
    /// An error occurred
    Error,
}

impl ResponseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseStatus::Complete => "complete",
            ResponseStatus::Partial => "partial",
            ResponseStatus::Unknown => "unknown",
            ResponseStatus::Unsupported => "unsupported",
            ResponseStatus::Error => "error",
        }
    }
}

impl std::fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ─── Agent Confidence ───────────────────────────────────────────────────────────

/// Agent-facing confidence level for knowledge results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentConfidence {
    /// Directly observable in source; verified by analyzer
    Exact,
    /// Statically derived with high certainty
    Derived,
    /// Inferred from patterns; may be incomplete
    Inferred,
    /// Cannot determine confidence
    Unknown,
}

impl AgentConfidence {
    /// Map from internal Confidence to agent-facing confidence
    pub fn from_confidence(c: Confidence) -> Self {
        match c {
            Confidence::Exact => AgentConfidence::Exact,
            Confidence::High => AgentConfidence::Derived,
            Confidence::Medium | Confidence::Low => AgentConfidence::Inferred,
            Confidence::Unknown => AgentConfidence::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AgentConfidence::Exact => "exact",
            AgentConfidence::Derived => "derived",
            AgentConfidence::Inferred => "inferred",
            AgentConfidence::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for AgentConfidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ─── Provenance ─────────────────────────────────────────────────────────────────

/// Provenance of a fact in an agent response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvenance {
    /// Repository revision (commit hash)
    pub revision: Option<String>,
    /// Source location (file:line)
    pub location: Option<String>,
    /// How this fact was derived
    pub source: String,
    /// Analyzer that produced this fact
    pub analyzer: Option<String>,
}

impl AgentProvenance {
    pub fn from_fact_provenance(fp: &FactProvenance) -> Self {
        Self {
            revision: None,
            location: fp.evidence.clone(),
            source: format!("{:?}", fp.source),
            analyzer: Some(fp.derived_by.clone()),
        }
    }
}

// ─── Telemetry ──────────────────────────────────────────────────────────────────

/// Telemetry data for a Prime tool invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    /// Tool name that was invoked
    pub tool: String,
    /// Target entity/operation
    pub target: String,
    /// Response status
    pub status: ResponseStatus,
    /// Latency in milliseconds
    pub latency_ms: f64,
    /// Bytes transferred
    pub bytes: usize,
    /// Whether source escalation is required
    pub source_required: bool,
    /// Timestamp
    pub timestamp: u64,
}

impl TelemetryData {
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

// ─── Response Envelope ──────────────────────────────────────────────────────────

/// Standard response envelope for all Prime tool results
///
/// Every Prime tool result uses this envelope to provide consistent
/// status, coverage, provenance, and escalation metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeEnvelope<T> {
    /// Response status
    pub status: ResponseStatus,

    /// Whether source code access is required to answer fully
    pub source_required: bool,

    /// Coverage of Prime's knowledge for this request (0.0 – 1.0)
    /// 1.0 = Prime has complete knowledge
    /// 0.0 = Prime has no relevant knowledge
    pub coverage: f64,

    /// The actual result payload
    pub result: T,

    /// What Prime knows is missing from its knowledge
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<String>,

    /// Provenance of the returned facts
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<AgentProvenance>,

    /// Warnings about the result
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,

    /// Optional telemetry for this invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<TelemetryData>,
}

impl<T> PrimeEnvelope<T> {
    /// Create a complete response
    pub fn complete(result: T) -> Self {
        Self {
            status: ResponseStatus::Complete,
            source_required: false,
            coverage: 1.0,
            result,
            missing: Vec::new(),
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: None,
        }
    }

    /// Create a partial response
    pub fn partial(result: T, missing: Vec<String>) -> Self {
        let coverage = if missing.is_empty() { 1.0 } else { 0.5 };
        Self {
            status: ResponseStatus::Partial,
            source_required: true,
            coverage,
            result,
            missing,
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: None,
        }
    }

    /// Create an unknown coverage response
    pub fn unknown(result: T) -> Self {
        Self {
            status: ResponseStatus::Unknown,
            source_required: true,
            coverage: 0.0,
            result,
            missing: vec!["coverage cannot be determined".to_string()],
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: None,
        }
    }

    /// Create an error response
    pub fn error(message: String) -> PrimeEnvelope<()> {
        PrimeEnvelope {
            status: ResponseStatus::Error,
            source_required: false,
            coverage: 0.0,
            result: (),
            missing: Vec::new(),
            provenance: Vec::new(),
            warnings: vec![message],
            telemetry: None,
        }
    }

    /// Add telemetry to the envelope
    pub fn with_telemetry(mut self, telemetry: TelemetryData) -> Self {
        self.telemetry = Some(telemetry);
        self
    }

    /// Add provenance to the envelope
    pub fn with_provenance(mut self, provenance: Vec<AgentProvenance>) -> Self {
        self.provenance = provenance;
        self
    }

    /// Add warnings to the envelope
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Check if the result is usable without source escalation
    pub fn is_sufficient(&self) -> bool {
        self.status == ResponseStatus::Complete && !self.source_required
    }

    /// Estimate token cost of the envelope overhead (not the result)
    pub fn envelope_tokens(&self) -> usize {
        // status + source_required + coverage + missing + warnings ≈ 20 tokens
        let mut tokens = 20;
        tokens += self.missing.iter().map(|s| s.len() / 4).sum::<usize>();
        tokens += self.warnings.iter().map(|s| s.len() / 4).sum::<usize>();
        tokens
    }
}

// ─── Tool Request ───────────────────────────────────────────────────────────────

/// Intent of a Prime tool request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolIntent {
    /// Search for entities by keyword
    Search,
    /// Look up an entity by qualified name
    Lookup,
    /// Get knowledge context for an entity
    Context,
    /// Get relationships (callers, callees, deps, dependents)
    Relationships,
    /// Get dependency graph
    Dependencies,
    /// Analyze impact of changes
    Impact,
    /// Get architecture overview
    Architecture,
}

impl ToolIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolIntent::Search => "search",
            ToolIntent::Lookup => "lookup",
            ToolIntent::Context => "context",
            ToolIntent::Relationships => "relationships",
            ToolIntent::Dependencies => "dependencies",
            ToolIntent::Impact => "impact",
            ToolIntent::Architecture => "architecture",
        }
    }
}

/// Detail level for responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DetailLevel {
    /// Minimal information (name, kind, confidence)
    Minimal,
    /// Standard information (signature, relations)
    Standard,
    /// Full information (documentation, all relations)
    Full,
}

impl Default for DetailLevel {
    fn default() -> Self {
        DetailLevel::Standard
    }
}

/// Scope of a relationship query
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationScope {
    /// Direct relationships only
    Direct,
    /// Transitive relationships (1 level deep)
    Transitive,
    /// All reachable relationships
    All,
}

impl Default for RelationScope {
    fn default() -> Self {
        RelationScope::Direct
    }
}

/// A request to a Prime tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    /// The intent of this request
    pub intent: ToolIntent,

    /// Target entity (qualified name, search term, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Scope for relationship queries
    #[serde(default)]
    pub scope: RelationScope,

    /// Which dimensions to include (dependencies, callers, tests, etc.)
    #[serde(default)]
    pub dimensions: Vec<String>,

    /// Maximum results to return
    #[serde(default = "default_limit")]
    pub limit: usize,

    /// Maximum depth for transitive queries
    #[serde(default = "default_depth")]
    pub depth: usize,

    /// Detail level for the response
    #[serde(default)]
    pub detail: DetailLevel,

    /// Specific fields to include (if empty, return all)
    #[serde(default)]
    pub fields: Vec<String>,

    /// Token budget for the response
    #[serde(default = "default_token_budget")]
    pub token_budget: usize,
}

fn default_limit() -> usize { 20 }
fn default_depth() -> usize { 1 }
fn default_token_budget() -> usize { 8192 }

impl Default for ToolRequest {
    fn default() -> Self {
        Self {
            intent: ToolIntent::Search,
            target: None,
            scope: RelationScope::default(),
            dimensions: Vec::new(),
            limit: default_limit(),
            depth: default_depth(),
            detail: DetailLevel::default(),
            fields: Vec::new(),
            token_budget: default_token_budget(),
        }
    }
}

// ─── Entity Detail ──────────────────────────────────────────────────────────────

/// Detailed entity information for agent consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDetail {
    pub id: u64,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub language: Language,
    pub range: Option<Range>,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: AgentConfidence,
    pub relation_counts: HashMap<RelationKind, usize>,
}

impl EntityDetail {
    /// Create an EntityDetail from an Entity and its ID
    pub fn from_entity(id: &EntityId, entity: &Entity) -> Self {
        Self {
            id: id.0,
            kind: entity.kind,
            name: entity.name.clone(),
            qualified_name: entity.qualified_name.clone(),
            language: entity.language,
            range: Some(entity.range),
            signature: entity.signature.clone(),
            documentation: entity.documentation.clone(),
            confidence: AgentConfidence::from_confidence(entity.confidence),
            relation_counts: HashMap::new(),
        }
    }

    /// Create an EntityDetail from raw fields
    pub fn new(
        id: u64,
        kind: SymbolKind,
        name: String,
        qualified_name: String,
        language: Language,
        range: Option<Range>,
        signature: Option<String>,
        documentation: Option<String>,
        confidence: AgentConfidence,
        relation_counts: HashMap<RelationKind, usize>,
    ) -> Self {
        Self { id, kind, name, qualified_name, language, range, signature, documentation, confidence, relation_counts }
    }
}

// ─── Context Result ─────────────────────────────────────────────────────────────

/// Result of a prime_context operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextResult {
    /// The target entity
    pub target: EntityDetail,

    /// Direct dependencies
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<EntityDetail>,

    /// Entities that depend on this
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependents: Vec<EntityDetail>,

    /// Functions that call this
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub callers: Vec<EntityDetail>,

    /// Functions this calls
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub callees: Vec<EntityDetail>,

    /// Tests that cover this
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tests: Vec<EntityDetail>,

    /// What this returns (for functions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,

    /// What this may throw (for functions)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub may_throw: Vec<String>,
}

// ─── Dependency Result ──────────────────────────────────────────────────────────

/// Result of a prime_dependencies operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyResult {
    /// The target entity
    pub target: EntityDetail,

    /// Direct dependencies
    pub dependencies: Vec<EntityDetail>,

    /// Dependency chain (if transitive)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chain: Vec<Vec<EntityDetail>>,
}

// ─── Impact Result ──────────────────────────────────────────────────────────────

/// Result of a prime_impact operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactResult {
    /// The target entity being changed
    pub target: EntityDetail,

    /// Directly affected entities
    pub direct_impact: Vec<EntityDetail>,

    /// Transitively affected entities
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transitive_impact: Vec<EntityDetail>,

    /// Risk assessment
    pub risk: ImpactRisk,

    /// Affected file count
    pub affected_files: usize,
}

/// Risk level for impact analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImpactRisk {
    /// Changes are localized
    Low,
    /// Changes affect multiple modules
    Medium,
    /// Changes affect core infrastructure
    High,
    /// Cannot determine impact
    Unknown,
}

// ─── Architecture Result ────────────────────────────────────────────────────────

/// Result of a prime_architecture operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureResult {
    /// Subsystem name
    pub name: String,

    /// Key entities in this subsystem
    pub entities: Vec<EntityDetail>,

    /// Dependencies on other subsystems
    pub dependencies: Vec<String>,

    /// Dependent subsystems
    pub dependents: Vec<String>,

    /// Module/file organization
    pub modules: Vec<ModuleInfo>,
}

/// Module information for architecture view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub path: String,
    pub entity_count: usize,
    pub language: Language,
}

// ─── Context Handle ─────────────────────────────────────────────────────────────

/// Application-level handle for multi-step retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextHandle {
    /// Unique handle ID
    pub id: String,

    /// The target entity
    pub target: String,

    /// What has been retrieved so far
    pub retrieved: Vec<String>,

    /// Token budget used so far
    pub used_tokens: usize,

    /// Token budget remaining
    pub remaining_tokens: usize,
}

// ─── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_complete() {
        let env = PrimeEnvelope::complete("test result");
        assert_eq!(env.status, ResponseStatus::Complete);
        assert!(!env.source_required);
        assert_eq!(env.coverage, 1.0);
        assert!(env.is_sufficient());
    }

    #[test]
    fn test_envelope_partial() {
        let env = PrimeEnvelope::partial("partial", vec!["missing info".to_string()]);
        assert_eq!(env.status, ResponseStatus::Partial);
        assert!(env.source_required);
        assert!(!env.is_sufficient());
    }

    #[test]
    fn test_envelope_error() {
        let env = PrimeEnvelope::<()>::error("something went wrong".to_string());
        assert_eq!(env.status, ResponseStatus::Error);
        assert!(!env.source_required);
    }

    #[test]
    fn test_agent_confidence_mapping() {
        assert_eq!(AgentConfidence::from_confidence(Confidence::Exact), AgentConfidence::Exact);
        assert_eq!(AgentConfidence::from_confidence(Confidence::High), AgentConfidence::Derived);
        assert_eq!(AgentConfidence::from_confidence(Confidence::Medium), AgentConfidence::Inferred);
        assert_eq!(AgentConfidence::from_confidence(Confidence::Unknown), AgentConfidence::Unknown);
    }

    #[test]
    fn test_tool_request_defaults() {
        let req = ToolRequest::default();
        assert_eq!(req.intent, ToolIntent::Search);
        assert_eq!(req.limit, 20);
        assert_eq!(req.depth, 1);
        assert_eq!(req.detail, DetailLevel::Standard);
        assert_eq!(req.token_budget, 8192);
    }

    #[test]
    fn test_envelope_serialization() {
        let env = PrimeEnvelope::complete("hello");
        let json = serde_json::to_string(&env).unwrap();
        assert!(json.contains("\"status\":\"complete\""));
        assert!(json.contains("\"source_required\":false"));
        assert!(json.contains("\"coverage\":1.0"));
    }
}
