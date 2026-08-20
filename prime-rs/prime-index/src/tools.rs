//! Semantic tool operations for the agent-native interface
//!
//! Each operation corresponds to an MCP tool and returns `PrimeEnvelope<T>`.

use prime_core::{
    KnowledgeGraph, Entity, EntityId, RelationKind, SymbolKind, Language,
    Confidence, Range, Position,
    PrimeEnvelope, ResponseStatus, AgentConfidence, AgentProvenance,
    TelemetryData, ToolRequest, ToolIntent, DetailLevel, RelationScope,
    EntityDetail, ContextResult, DependencyResult, ImpactResult, ImpactRisk,
    ArchitectureResult, ModuleInfo,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::query::{QueryEngine, QueryOptions, EntitySummary};

/// Tool executor that wraps a QueryEngine and provides semantic operations
pub struct ToolExecutor {
    engine: Arc<QueryEngine>,
}

/// Helper: convert EntitySummary to EntityDetail
fn summary_to_detail(s: &EntitySummary) -> EntityDetail {
    EntityDetail::new(
        s.id.0,
        s.kind,
        s.name.clone(),
        s.qualified_name.clone(),
        s.language,
        s.range,
        s.signature.clone(),
        s.documentation.clone(),
        AgentConfidence::from_confidence(s.confidence),
        s.relation_counts.clone(),
    )
}

/// Helper: convert Entity to EntityDetail
fn entity_to_detail(id: &EntityId, e: &Entity) -> EntityDetail {
    EntityDetail::from_entity(id, e)
}

impl ToolExecutor {
    pub fn new(engine: Arc<QueryEngine>) -> Self {
        Self { engine }
    }

    pub fn from_graph(graph: KnowledgeGraph) -> Self {
        Self {
            engine: Arc::new(QueryEngine::new(graph)),
        }
    }

    pub fn engine(&self) -> &QueryEngine {
        &self.engine
    }

    pub fn execute(&self, request: &ToolRequest) -> prime_core::serde_json::Value {
        let result = match request.intent {
            ToolIntent::Search => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_search(target, request.limit);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
            ToolIntent::Lookup => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_lookup(target);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
            ToolIntent::Context => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_context(target, request.depth, request.token_budget);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
            ToolIntent::Relationships => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_relationships(target, &request.dimensions, request.scope, request.limit);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
            ToolIntent::Dependencies => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_dependencies(target, request.scope, request.depth);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
            ToolIntent::Impact => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_impact(target);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
            ToolIntent::Architecture => {
                let target = request.target.as_deref().unwrap_or("");
                let env = self.prime_architecture(target);
                prime_core::serde_json::to_value(&env).unwrap_or_default()
            }
        };

        result
    }

    /// prime_search — Search entities by keyword
    pub fn prime_search(&self, query: &str, limit: usize) -> PrimeEnvelope<Vec<EntityDetail>> {
        let start = Instant::now();

        if query.is_empty() {
            return PrimeEnvelope {
                status: ResponseStatus::Error,
                source_required: false,
                coverage: 0.0,
                result: Vec::new(),
                missing: Vec::new(),
                provenance: Vec::new(),
                warnings: vec!["Empty search query".to_string()],
                telemetry: None,
            };
        }

        let opts = QueryOptions {
            max_results: limit,
            min_confidence: Confidence::Low,
            ..Default::default()
        };

        let summaries = self.engine.search(query, &opts);
        let details: Vec<EntityDetail> = summaries.iter().map(summary_to_detail).collect();

        let coverage = if details.is_empty() { 0.0 } else { 1.0 };
        let status = if details.is_empty() {
            ResponseStatus::Unknown
        } else {
            ResponseStatus::Complete
        };

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        PrimeEnvelope {
            status,
            source_required: false,
            coverage,
            result: details,
            missing: Vec::new(),
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: Some(TelemetryData {
                tool: "prime_search".to_string(),
                target: query.to_string(),
                status,
                latency_ms: elapsed,
                bytes: 0,
                source_required: false,
                timestamp: TelemetryData::now(),
            }),
        }
    }

    /// prime_lookup — Find entity by qualified name
    pub fn prime_lookup(&self, qualified_name: &str) -> PrimeEnvelope<Option<EntityDetail>> {
        let start = Instant::now();

        let result = self.engine.find_by_qualified(qualified_name)
            .map(|s| summary_to_detail(&s));

        let (status, coverage, source_required) = match &result {
            Some(_) => (ResponseStatus::Complete, 1.0, false),
            None => (ResponseStatus::Unknown, 0.0, true),
        };

        let missing = if result.is_none() {
            vec![format!("Entity '{}' not found in Prime", qualified_name)]
        } else {
            Vec::new()
        };

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        PrimeEnvelope {
            status,
            source_required,
            coverage,
            result,
            missing,
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: Some(TelemetryData {
                tool: "prime_lookup".to_string(),
                target: qualified_name.to_string(),
                status,
                latency_ms: elapsed,
                bytes: 0,
                source_required,
                timestamp: TelemetryData::now(),
            }),
        }
    }

    /// prime_context — Get knowledge neighborhood for an entity
    pub fn prime_context(&self, qualified_name: &str, depth: usize, token_budget: usize) -> PrimeEnvelope<Option<ContextResult>> {
        let start = Instant::now();

        let entity_id = match self.engine.graph().find_by_qualified(qualified_name) {
            Some(id) => id,
            None => {
                return PrimeEnvelope {
                    status: ResponseStatus::Unknown,
                    source_required: true,
                    coverage: 0.0,
                    result: None,
                    missing: vec![format!("Entity '{}' not found", qualified_name)],
                    provenance: Vec::new(),
                    warnings: Vec::new(),
                    telemetry: None,
                };
            }
        };

        let entity = match self.engine.graph().entities.get(&entity_id) {
            Some(e) => e,
            None => {
                return PrimeEnvelope {
                    status: ResponseStatus::Error,
                    source_required: false,
                    coverage: 0.0,
                    result: None,
                    missing: Vec::new(),
                    provenance: Vec::new(),
                    warnings: vec!["Entity ID exists but entity data missing".to_string()],
                    telemetry: None,
                };
            }
        };

        let target = entity_to_detail(&entity_id, entity);

        let deps = self.engine.graph().dependencies(entity_id);
        let dependents = self.engine.graph().dependents(entity_id);
        let callers = self.engine.graph().callers(entity_id);
        let callees = self.engine.graph().callees(entity_id);

        let to_details = |ids: Vec<EntityId>| -> Vec<EntityDetail> {
            ids.into_iter()
                .filter_map(|id| self.engine.graph().entities.get(&id).map(|e| entity_to_detail(&id, e)))
                .collect()
        };

        let dependencies = to_details(deps);
        let dependents = to_details(dependents);
        let callers = to_details(callers);
        let callees = to_details(callees);

        let has_signature = entity.signature.is_some();
        let has_docs = entity.documentation.is_some();
        let has_relations = !dependencies.is_empty() || !callers.is_empty() || !callees.is_empty();

        let coverage = match (has_signature, has_docs, has_relations) {
            (true, true, true) => 1.0,
            (true, true, false) => 0.9,
            (true, false, true) => 0.8,
            (true, false, false) => 0.7,
            (false, _, true) => 0.6,
            (false, true, false) => 0.5,
            (false, false, true) => 0.4,
            (false, false, false) => 0.3,
        };

        let mut missing = Vec::new();
        if !has_signature {
            missing.push("function signature".to_string());
        }
        if !has_docs {
            missing.push("documentation".to_string());
        }

        let context_result = ContextResult {
            target,
            dependencies,
            dependents,
            callers,
            callees,
            tests: Vec::new(),
            returns: entity.signature.clone(),
            may_throw: Vec::new(),
        };

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        let source_required = coverage < 0.8;

        PrimeEnvelope {
            status: ResponseStatus::Complete,
            source_required,
            coverage,
            result: Some(context_result),
            missing,
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: Some(TelemetryData {
                tool: "prime_context".to_string(),
                target: qualified_name.to_string(),
                status: ResponseStatus::Complete,
                latency_ms: elapsed,
                bytes: 0,
                source_required,
                timestamp: TelemetryData::now(),
            }),
        }
    }

    /// prime_relationships — Get relationships for an entity
    pub fn prime_relationships(
        &self,
        qualified_name: &str,
        dimensions: &[String],
        scope: RelationScope,
        limit: usize,
    ) -> PrimeEnvelope<Vec<EntityDetail>> {
        let start = Instant::now();

        let entity_id = match self.engine.graph().find_by_qualified(qualified_name) {
            Some(id) => id,
            None => {
                return PrimeEnvelope {
                    status: ResponseStatus::Unknown,
                    source_required: true,
                    coverage: 0.0,
                    result: Vec::new(),
                    missing: vec![format!("Entity '{}' not found", qualified_name)],
                    provenance: Vec::new(),
                    warnings: Vec::new(),
                    telemetry: None,
                };
            }
        };

        let mut results = Vec::new();
        let want_deps = dimensions.is_empty() || dimensions.iter().any(|d| d == "dependencies");
        let want_callers = dimensions.is_empty() || dimensions.iter().any(|d| d == "callers");
        let want_callees = dimensions.is_empty() || dimensions.iter().any(|d| d == "callees");
        let want_dependents = dimensions.is_empty() || dimensions.iter().any(|d| d == "dependents");

        if want_deps {
            for id in self.engine.graph().dependencies(entity_id) {
                if let Some(e) = self.engine.graph().entities.get(&id) {
                    results.push(entity_to_detail(&id, e));
                }
            }
        }
        if want_dependents {
            for id in self.engine.graph().dependents(entity_id) {
                if let Some(e) = self.engine.graph().entities.get(&id) {
                    results.push(entity_to_detail(&id, e));
                }
            }
        }
        if want_callers {
            for id in self.engine.graph().callers(entity_id) {
                if let Some(e) = self.engine.graph().entities.get(&id) {
                    results.push(entity_to_detail(&id, e));
                }
            }
        }
        if want_callees {
            for id in self.engine.graph().callees(entity_id) {
                if let Some(e) = self.engine.graph().entities.get(&id) {
                    results.push(entity_to_detail(&id, e));
                }
            }
        }

        results.truncate(limit);

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        let coverage = if results.is_empty() { 0.5 } else { 1.0 };

        PrimeEnvelope {
            status: ResponseStatus::Complete,
            source_required: false,
            coverage,
            result: results,
            missing: Vec::new(),
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: Some(TelemetryData {
                tool: "prime_relationships".to_string(),
                target: qualified_name.to_string(),
                status: ResponseStatus::Complete,
                latency_ms: elapsed,
                bytes: 0,
                source_required: false,
                timestamp: TelemetryData::now(),
            }),
        }
    }

    /// prime_dependencies — Get dependency graph
    pub fn prime_dependencies(
        &self,
        qualified_name: &str,
        scope: RelationScope,
        depth: usize,
    ) -> PrimeEnvelope<Option<DependencyResult>> {
        let start = Instant::now();

        let entity_id = match self.engine.graph().find_by_qualified(qualified_name) {
            Some(id) => id,
            None => {
                return PrimeEnvelope {
                    status: ResponseStatus::Unknown,
                    source_required: true,
                    coverage: 0.0,
                    result: None,
                    missing: vec![format!("Entity '{}' not found", qualified_name)],
                    provenance: Vec::new(),
                    warnings: Vec::new(),
                    telemetry: None,
                };
            }
        };

        let entity = match self.engine.graph().entities.get(&entity_id) {
            Some(e) => e,
            None => return PrimeEnvelope {
                status: ResponseStatus::Error,
                source_required: false,
                coverage: 0.0,
                result: None,
                missing: Vec::new(),
                provenance: Vec::new(),
                warnings: vec!["Entity data missing".to_string()],
                telemetry: None,
            },
        };

        let target = entity_to_detail(&entity_id, entity);
        let deps = self.engine.graph().dependencies(entity_id);
        let dependencies: Vec<EntityDetail> = deps.into_iter()
            .filter_map(|id| self.engine.graph().entities.get(&id).map(|e| entity_to_detail(&id, e)))
            .collect();

        let result = DependencyResult {
            target,
            dependencies,
            chain: Vec::new(),
        };

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        PrimeEnvelope {
            status: ResponseStatus::Complete,
            source_required: false,
            coverage: 1.0,
            result: Some(result),
            missing: Vec::new(),
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: Some(TelemetryData {
                tool: "prime_dependencies".to_string(),
                target: qualified_name.to_string(),
                status: ResponseStatus::Complete,
                latency_ms: elapsed,
                bytes: 0,
                source_required: false,
                timestamp: TelemetryData::now(),
            }),
        }
    }

    /// prime_impact — Analyze impact of changes
    pub fn prime_impact(&self, qualified_name: &str) -> PrimeEnvelope<Option<ImpactResult>> {
        let start = Instant::now();

        let entity_id = match self.engine.graph().find_by_qualified(qualified_name) {
            Some(id) => id,
            None => {
                return PrimeEnvelope {
                    status: ResponseStatus::Unknown,
                    source_required: true,
                    coverage: 0.0,
                    result: None,
                    missing: vec![format!("Entity '{}' not found", qualified_name)],
                    provenance: Vec::new(),
                    warnings: Vec::new(),
                    telemetry: None,
                };
            }
        };

        let entity = match self.engine.graph().entities.get(&entity_id) {
            Some(e) => e,
            None => return PrimeEnvelope {
                status: ResponseStatus::Error,
                source_required: false,
                coverage: 0.0,
                result: None,
                missing: Vec::new(),
                provenance: Vec::new(),
                warnings: vec!["Entity data missing".to_string()],
                telemetry: None,
            },
        };

        let target = entity_to_detail(&entity_id, entity);

        let callers = self.engine.graph().callers(entity_id);
        let dependents = self.engine.graph().dependents(entity_id);

        let mut direct_ids: Vec<EntityId> = callers.into_iter().chain(dependents).collect();
        direct_ids.dedup();

        let direct_impact: Vec<EntityDetail> = direct_ids.iter()
            .filter_map(|id| self.engine.graph().entities.get(id).map(|e| entity_to_detail(id, e)))
            .collect();

        let affected_files: usize = direct_impact.iter()
            .filter(|d| d.range.is_some())
            .count();

        let risk = if direct_impact.len() > 10 {
            ImpactRisk::High
        } else if direct_impact.len() > 3 {
            ImpactRisk::Medium
        } else {
            ImpactRisk::Low
        };

        let result = ImpactResult {
            target,
            direct_impact,
            transitive_impact: Vec::new(),
            risk,
            affected_files,
        };

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        PrimeEnvelope {
            status: ResponseStatus::Complete,
            source_required: false,
            coverage: 0.8,
            result: Some(result),
            missing: vec!["transitive impact analysis".to_string()],
            provenance: Vec::new(),
            warnings: Vec::new(),
            telemetry: Some(TelemetryData {
                tool: "prime_impact".to_string(),
                target: qualified_name.to_string(),
                status: ResponseStatus::Complete,
                latency_ms: elapsed,
                bytes: 0,
                source_required: false,
                timestamp: TelemetryData::now(),
            }),
        }
    }

    /// prime_architecture — Get architecture overview
    pub fn prime_architecture(&self, qualified_name: &str) -> PrimeEnvelope<Option<ArchitectureResult>> {
        let start = Instant::now();

        let graph = self.engine.graph();

        if qualified_name.is_empty() {
            let entities: Vec<EntityDetail> = graph.entities.iter()
                .take(50)
                .map(|(id, e)| entity_to_detail(id, e))
                .collect();

            let modules: Vec<ModuleInfo> = graph.modules.iter()
                .map(|(_, m)| ModuleInfo {
                    name: m.name.clone(),
                    path: m.path.clone(),
                    entity_count: m.files.len(),
                    language: m.language,
                })
                .collect();

            let result = ArchitectureResult {
                name: graph.project.name.clone(),
                entities,
                dependencies: Vec::new(),
                dependents: Vec::new(),
                modules,
            };

            let elapsed = start.elapsed().as_secs_f64() * 1000.0;

            return PrimeEnvelope {
                status: ResponseStatus::Complete,
                source_required: false,
                coverage: 0.7,
                result: Some(result),
                missing: vec!["full architecture analysis".to_string()],
                provenance: Vec::new(),
                warnings: Vec::new(),
                telemetry: Some(TelemetryData {
                    tool: "prime_architecture".to_string(),
                    target: qualified_name.to_string(),
                    status: ResponseStatus::Complete,
                    latency_ms: elapsed,
                    bytes: 0,
                    source_required: false,
                    timestamp: TelemetryData::now(),
                }),
            };
        }

        match graph.find_by_qualified(qualified_name) {
            Some(entity_id) => {
                let entity = graph.entities.get(&entity_id);
                let deps = graph.dependencies(entity_id);
                let dependents = graph.dependents(entity_id);

                let dep_names: Vec<String> = deps.iter()
                    .filter_map(|id| graph.entities.get(id).map(|e| e.qualified_name.clone()))
                    .collect();

                let dependent_names: Vec<String> = dependents.iter()
                    .filter_map(|id| graph.entities.get(id).map(|e| e.qualified_name.clone()))
                    .collect();

                let result = ArchitectureResult {
                    name: qualified_name.to_string(),
                    entities: entity.map(|e| vec![entity_to_detail(&entity_id, e)]).unwrap_or_default(),
                    dependencies: dep_names,
                    dependents: dependent_names,
                    modules: Vec::new(),
                };

                let elapsed = start.elapsed().as_secs_f64() * 1000.0;

                PrimeEnvelope {
                    status: ResponseStatus::Complete,
                    source_required: false,
                    coverage: 0.6,
                    result: Some(result),
                    missing: vec!["full subsystem analysis".to_string()],
                    provenance: Vec::new(),
                    warnings: Vec::new(),
                    telemetry: Some(TelemetryData {
                        tool: "prime_architecture".to_string(),
                        target: qualified_name.to_string(),
                        status: ResponseStatus::Complete,
                        latency_ms: elapsed,
                        bytes: 0,
                        source_required: false,
                        timestamp: TelemetryData::now(),
                    }),
                }
            }
            None => PrimeEnvelope {
                status: ResponseStatus::Unknown,
                source_required: true,
                coverage: 0.0,
                result: None,
                missing: vec![format!("Entity '{}' not found", qualified_name)],
                provenance: Vec::new(),
                warnings: Vec::new(),
                telemetry: None,
            },
        }
    }
}
