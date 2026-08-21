//! Query API for agent-optimized retrieval

use prime_core::{KnowledgeGraph, Entity, Relation, EntityId, RelationKind, SymbolKind, Language, Confidence, Range};
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;

/// Query options for controlling retrieval
#[derive(Debug, Clone, Default)]
pub struct QueryOptions {
    pub max_results: usize,
    pub include_relations: bool,
    pub relation_kinds: Option<Vec<RelationKind>>,
    pub max_depth: usize,
    pub min_confidence: Confidence,
    pub include_documentation: bool,
    pub include_signatures: bool,
    pub token_budget: usize,  // Max tokens for agent context
}

impl QueryOptions {
    pub fn for_agent() -> Self {
        Self {
            max_results: 50,
            include_relations: true,
            max_depth: 2,
            min_confidence: Confidence::Medium,
            include_documentation: false,
            include_signatures: true,
            token_budget: 8192, // ~8KB context
            ..Default::default()
        }
    }

    pub fn for_exploration() -> Self {
        Self {
            max_results: 100,
            include_relations: true,
            max_depth: 3,
            min_confidence: Confidence::Low,
            include_documentation: true,
            include_signatures: true,
            token_budget: 32768,
            ..Default::default()
        }
    }
}

/// Query result for agent consumption
#[derive(Debug, Clone, Serialize)]
pub struct QueryResult {
    pub entity: EntitySummary,
    pub relations: Vec<RelationSummary>,
    pub score: f32,
}

/// Lightweight entity summary for agent context
#[derive(Debug, Clone, Serialize)]
pub struct EntitySummary {
    pub id: EntityId,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub language: Language,
    pub range: Option<Range>,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: Confidence,
    pub relation_counts: HashMap<RelationKind, usize>,
}

/// Lightweight relation summary
#[derive(Debug, Clone, Serialize)]
pub struct RelationSummary {
    pub kind: RelationKind,
    pub target_id: EntityId,
    pub target_name: String,
    pub target_kind: SymbolKind,
    pub confidence: Confidence,
}

/// Main query engine for agent-optimized retrieval
pub struct QueryEngine {
    pub graph: Arc<prime_core::KnowledgeGraph>,
}

impl QueryEngine {
    pub fn new(mut graph: prime_core::KnowledgeGraph) -> Self {
        // Build indexes if they weren't serialized
        if graph.name_index.is_none() || graph.relation_index.is_none() {
            graph.build_indexes();
        }
        Self {
            graph: Arc::new(graph),
        }
    }

    pub fn graph(&self) -> &prime_core::KnowledgeGraph {
        &self.graph
    }

    /// Find entity by qualified name
    pub fn find_by_qualified(&self, name: &str) -> Option<EntitySummary> {
        self.graph.find_by_qualified(name).and_then(|id| {
            self.graph.entities.get(&id).map(|e| self.to_summary(&id, e))
        })
    }

    /// Find entities by simple name
    pub fn find_by_name(&self, name: &str, opts: &QueryOptions) -> Vec<EntitySummary> {
        if let Some(ids) = self.graph.find_by_name(name) {
            ids.iter()
                .filter_map(|id| self.graph.entities.get(id).map(|e| self.to_summary(id, e)))
                .filter(|s| s.confidence >= opts.min_confidence)
                .take(opts.max_results)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find entities by prefix
    pub fn find_by_prefix(&self, prefix: &str, opts: &QueryOptions) -> Vec<EntitySummary> {
        if let Some(ids) = self.graph.find_by_prefix(prefix) {
            ids.iter()
                .filter_map(|id| self.graph.entities.get(id).map(|e| self.to_summary(id, e)))
                .filter(|s| s.confidence >= opts.min_confidence)
                .take(opts.max_results)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get entity with context (relations, etc.)
    pub fn get_context(&self, entity_id: EntityId, opts: &QueryOptions) -> Option<QueryResult> {
        self.graph.entities.get(&entity_id).map(|entity| {
            let relations = self.get_relations(entity_id, opts);
            QueryResult {
                entity: self.to_summary(&entity_id, entity),
                relations,
                score: 1.0,
            }
        })
    }

    /// Get surrounding context (callers, callees, dependencies, etc.)
    pub fn get_surrounding_context(&self, entity_id: EntityId, opts: &QueryOptions) -> Vec<EntitySummary> {
        let mut results = Vec::new();

        // Get direct dependencies
        let deps = self.graph.dependencies(entity_id);
        for dep_id in deps {
            if let Some(entity) = self.graph.entities.get(&dep_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&dep_id, entity));
                }
            }
        }

        // Get dependents
        let dependents = self.graph.dependents(entity_id);
        for dep_id in dependents {
            if let Some(entity) = self.graph.entities.get(&dep_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&dep_id, entity));
                }
            }
        }

        // Get callers
        let callers = self.graph.callers(entity_id);
        for caller_id in callers {
            if let Some(entity) = self.graph.entities.get(&caller_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&caller_id, entity));
                }
            }
        }

        // Get callees
        let callees = self.graph.callees(entity_id);
        for callee_id in callees {
            if let Some(entity) = self.graph.entities.get(&callee_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&callee_id, entity));
                }
            }
        }

        results.truncate(opts.max_results);
        results
    }

    /// Search by keyword across names and documentation
    pub fn search(&self, query: &str, opts: &QueryOptions) -> Vec<EntitySummary> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for (id, entity) in &self.graph.entities {
            let name_match = entity.name.to_lowercase().contains(&query_lower);
            let qual_match = entity.qualified_name.to_lowercase().contains(&query_lower);
            let doc_match = entity.documentation.as_ref()
                .map(|d| d.to_lowercase().contains(&query_lower))
                .unwrap_or(false);

            if name_match || qual_match || doc_match {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(id, entity));
                }
            }
        }

        // Sort by relevance (exact name match > qualified match > doc match)
        results.sort_by(|a, b| {
            let a_exact = a.name.to_lowercase() == query_lower;
            let b_exact = b.name.to_lowercase() == query_lower;
            b_exact.cmp(&a_exact)
        });

        results.truncate(opts.max_results);
        results
    }

    /// Get entity summary
    fn to_summary(&self, id: &EntityId, entity: &Entity) -> EntitySummary {
        let mut relation_counts = HashMap::new();
        for rel in &self.graph.relations {
            if rel.from == *id {
                *relation_counts.entry(rel.kind).or_insert(0) += 1;
            }
            if rel.to == *id {
                *relation_counts.entry(rel.kind).or_insert(0) += 1;
            }
        }

        EntitySummary {
            id: *id,
            kind: entity.kind,
            name: entity.name.clone(),
            qualified_name: entity.qualified_name.clone(),
            language: entity.language,
            range: Some(entity.range),
            signature: entity.signature.clone(),
            documentation: None, // Not included by default
            confidence: entity.confidence,
            relation_counts,
        }
    }

    fn get_relations(&self, entity_id: EntityId, opts: &QueryOptions) -> Vec<RelationSummary> {
        let mut results = Vec::new();

        if let Some(index) = &self.graph.relation_index {
            for (kind, target_id) in index.outgoing.get(&entity_id).unwrap_or(&Vec::new()) {
                if opts.relation_kinds.as_ref().map(|k| k.contains(kind)).unwrap_or(true) {
                    if let Some(target) = self.graph.entities.get(target_id) {
                        if target.confidence >= opts.min_confidence {
                            results.push(RelationSummary {
                                kind: *kind,
                                target_id: *target_id,
                                target_name: target.name.clone(),
                                target_kind: target.kind,
                                confidence: Confidence::High,
                            });
                        }
                    }
                }
            }

            for (kind, source_id) in index.incoming.get(&entity_id).unwrap_or(&Vec::new()) {
                if opts.relation_kinds.as_ref().map(|k| k.contains(kind)).unwrap_or(true) {
                    if let Some(source) = self.graph.entities.get(source_id) {
                        if source.confidence >= opts.min_confidence {
                            results.push(RelationSummary {
                                kind: *kind,
                                target_id: *source_id,
                                target_name: source.name.clone(),
                                target_kind: source.kind,
                                confidence: Confidence::High,
                            });
                        }
                    }
                }
            }
        }

        results
    }
}

impl Clone for QueryEngine {
    fn clone(&self) -> Self {
        Self {
            graph: Arc::clone(&self.graph),
        }
    }
}

/// Progressive context builder for token-efficient retrieval
pub struct ProgressiveContextBuilder {
    engine: Arc<QueryEngine>,
    budget: usize,
    used: usize,
    included: HashMap<EntityId, EntitySummary>,
}

impl ProgressiveContextBuilder {
    pub fn new(engine: Arc<QueryEngine>, budget: usize) -> Self {
        Self {
            engine,
            budget,
            used: 0,
            included: HashMap::new(),
        }
    }

    /// Add an entity and its immediate context
    pub fn add_entity(&mut self, entity_id: EntityId) -> Option<EntitySummary> {
        if self.included.contains_key(&entity_id) {
            return self.included.get(&entity_id).cloned();
        }

        if let Some(summary) = self.engine.graph.entities.get(&entity_id)
            .map(|e| self.to_summary(entity_id, e)) {

            let token_cost = self.estimate_tokens(&summary);
            if self.used + token_cost > self.budget {
                return None; // Budget exceeded
            }

            self.used += token_cost;
            self.included.insert(entity_id, summary.clone());
            Some(summary)
        } else {
            None
        }
    }

    /// Expand context with surrounding entities
    pub fn expand_context(&mut self, entity_id: EntityId) {
        let opts = super::QueryOptions::for_agent();
        let surrounding = self.engine.get_surrounding_context(entity_id, &opts);
        for summary in surrounding {
            if !self.included.contains_key(&summary.id) {
                let token_cost = self.estimate_tokens(&summary);
                if self.used + token_cost <= self.budget {
                    self.used += token_cost;
                    self.included.insert(summary.id, summary);
                }
            }
        }
    }

    /// Get all included entities
    pub fn get_included(&self) -> Vec<EntitySummary> {
        self.included.values().cloned().collect()
    }

    /// Estimate token cost for an entity summary
    fn to_summary(&self, id: EntityId, entity: &Entity) -> EntitySummary {
        let mut relation_counts: HashMap<RelationKind, usize> = HashMap::new();
        // Simplified - in real implementation would use graph indexes
        EntitySummary {
            id,
            kind: entity.kind,
            name: entity.name.clone(),
            qualified_name: entity.qualified_name.clone(),
            language: entity.language,
            range: Some(entity.range),
            signature: entity.signature.clone(),
            documentation: entity.documentation.clone(),
            confidence: entity.confidence,
            relation_counts,
        }
    }

    fn estimate_tokens(&self, summary: &EntitySummary) -> usize {
        // Rough estimation: ~4 chars per token
        let mut tokens = 0;
        tokens += summary.name.len() / 4;
        tokens += summary.qualified_name.len() / 4;
        tokens += summary.signature.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += summary.documentation.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += 50; // Base overhead
        tokens
    }
}