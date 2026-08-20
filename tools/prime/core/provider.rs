//! Prime Provider - Abstraction over the Prime knowledge engine

use crate::types::*;
use crate::query::{QueryEngine, QueryOptions, QueryResult, EntitySummary, RelationSummary, ProgressiveContextBuilder};
use anyhow::Result;
use std::sync::Arc;

/// Trait for Prime knowledge providers
pub trait PrimeProvider: Send + Sync {
    /// Find entity by qualified name
    fn find_by_qualified(&self, name: &str) -> Option<EntityId>;

    /// Find entities by name (fuzzy)
    fn find_by_name(&self, name: &str, opts: &QueryOptions) -> Vec<EntitySummary>;

    /// Find entities by prefix
    fn find_by_prefix(&self, prefix: &str, opts: &QueryOptions) -> Vec<EntitySummary>;

    /// Search symbols by keyword
    fn search(&self, query: &str, opts: &QueryOptions) -> Vec<EntitySummary>;

    /// Get full context for an entity
    fn get_context(&self, entity_id: EntityId, opts: &QueryOptions) -> Option<QueryResult>;

    /// Get surrounding context (callers, callees, deps, dependents)
    fn get_surrounding_context(&self, entity_id: EntityId, opts: &QueryOptions) -> Vec<EntitySummary>;

    /// Get dependencies of an entity
    fn dependencies(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependents of an entity
    fn dependents(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callers of a function
    fn callers(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callees of a function
    fn callees(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependencies of an entity
    fn dependencies(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependents of an entity
    fn dependents(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callers of a function
    fn callers(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callees of a function
    fn callees(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependencies of an entity
    fn dependencies(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependents of an entity
    fn dependents(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callers of a function
    fn callers(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callees of a function
    fn callees(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependencies of an entity
    fn dependencies(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get dependents of an entity
    fn dependents(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callers of a function
    fn callers(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get callees of a function
    fn callees(&self, entity_id: EntityId) -> Vec<EntityId>;

    /// Get the underlying knowledge graph
    fn graph(&self) -> &KnowledgeGraph;
}

/// Agent-facing query interface
pub struct AgentQuery {
    engine: Arc<crate::query::QueryEngine>,
    default_opts: QueryOptions,
}

impl AgentQuery {
    pub fn new(graph: KnowledgeGraph) -> Self {
        let engine = Arc::new(QueryEngine::new(graph));
        let default_opts = QueryOptions::for_agent();
        Self {
            engine: Arc::new(QueryEngine::new(graph)),
            default_opts: QueryOptions::for_agent(),
        }
    }

    pub fn find_symbol(&self, qualified_name: &str) -> Option<SymbolInfo> {
        self.engine.find_by_qualified(qualified_name)
            .map(|summary| SymbolInfo::from_summary(summary))
    }

    pub fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.engine.find_by_name(name, &self.default_opts)
            .into_iter()
            .map(SymbolInfo::from_summary)
            .collect()
    }

    pub fn find_by_prefix(&self, prefix: &str) -> Vec<SymbolInfo> {
        self.engine.find_by_prefix(prefix, &self.default_opts)
            .into_iter()
            .map(SymbolInfo::from_summary)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<SymbolInfo> {
        self.engine.search(query, &self.default_opts)
            .into_iter()
            .map(SymbolInfo::from_summary)
            .collect()
    }

    pub fn get_context(&self, qualified_name: &str) -> Option<Context> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            self.engine.get_context(entity_id, &self.default_opts)
                .map(Context::from_result)
        } else {
            None
        }
    }

    pub fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            self.engine.get_surrounding_context(entity_id, &self.default_opts)
                .into_iter()
                .map(SymbolInfo::from_summary)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            self.engine.graph().callers(entity_id)
                .into_iter()
                .filter_map(|id| self.engine.graph().entities.get(&id))
                .map(SymbolInfo::from_entity)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            self.engine.graph().callees(entity_id)
                .into_iter()
                .filter_map(|id| self.engine.graph().entities.get(&id))
                .map(SymbolInfo::from_entity)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            self.engine.graph().dependencies(entity_id)
                .into_iter()
                .filter_map(|id| self.engine.graph().entities.get(&id))
                .map(SymbolInfo::from_entity)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            self.engine.graph().dependents(entity_id)
                .into_iter()
                .filter_map(|id| self.engine.graph().entities.get(&id))
                .map(SymbolInfo::from_entity)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        ProgressiveContextBuilder::new(self.engine.clone(), token_budget)
    }

    pub fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.engine.search(query, &self.default_opts)
            .into_iter()
            .map(SymbolInfo::from_summary)
            .collect()
    }
}

/// Symbol information for agent consumption
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolInfo {
    pub id: u64,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub language: Language,
    pub range: Option<Range>,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: Confidence,
    pub relation_counts: std::collections::HashMap<RelationKind, usize>,
}

impl SymbolInfo {
    fn from_summary(summary: EntitySummary) -> Self {
        Self {
            id: summary.id.0,
            kind: summary.kind,
            name: summary.name,
            qualified_name: summary.qualified_name,
            language: summary.language,
            range: summary.range,
            signature: summary.signature,
            documentation: summary.documentation,
            confidence: summary.confidence,
            relation_counts: summary.relation_counts,
        }
    }

    fn from_entity(entity: &crate::types::Entity) -> Self {
        Self {
            id: entity.id.0,
            kind: entity.kind,
            name: entity.name.clone(),
            qualified_name: entity.qualified_name.clone(),
            language: entity.language,
            range: Some(entity.range),
            signature: entity.signature.clone(),
            documentation: entity.documentation.clone(),
            confidence: entity.confidence,
            relation_counts: std::collections::HashMap::new(),
        }
    }
}

/// Context for a symbol (full context for agent)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Context {
    pub symbol: SymbolInfo,
    pub relations: Vec<RelationInfo>,
    pub token_count: usize,
}

impl Context {
    fn from_result(result: crate::query::QueryResult) -> Self {
        let token_count = Self::estimate_tokens(&result);
        Self {
            symbol: SymbolInfo::from_summary(result.entity),
            relations: result.relations.into_iter().map(RelationInfo::from_summary).collect(),
            token_count,
        }
    }

    fn estimate_tokens(result: &crate::query::QueryResult) -> usize {
        let mut tokens = 0;
        tokens += result.entity.name.len() / 4;
        tokens += result.entity.qualified_name.len() / 4;
        tokens += result.entity.signature.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += result.entity.documentation.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        for rel in &result.relations {
            tokens += rel.target_name.len() / 4;
        }
        tokens
    }
}

/// Relation information for agent
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelationInfo {
    pub kind: RelationKind,
    pub target_id: u64,
    pub target_name: String,
    pub target_kind: SymbolKind,
    pub confidence: Confidence,
}

impl RelationInfo {
    fn from_summary(summary: RelationSummary) -> Self {
        Self {
            kind: summary.kind,
            target_id: summary.target_id.0,
            target_name: summary.target_name,
            target_kind: summary.target_kind,
            confidence: summary.confidence,
        }
    }
}

/// Progressive context builder for token-efficient retrieval
pub struct ProgressiveContextBuilder {
    engine: std::sync::Arc<crate::query::QueryEngine>,
    budget: usize,
    used: usize,
    included: std::collections::HashMap<u64, SymbolInfo>,
}

impl ProgressiveContextBuilder {
    pub fn new(engine: std::sync::Arc<crate::query::QueryEngine>, budget: usize) -> Self {
        Self {
            engine,
            budget,
            used: 0,
            included: std::collections::HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, qualified_name: &str) -> Option<SymbolInfo> {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            if self.included.contains_key(&entity_id.0) {
                return self.included.get(&entity_id.0).cloned();
            }

            if let Some(entity) = self.engine.graph().entities.get(&entity_id) {
                let summary = prime_index::query::EntitySummary {
                    id: entity_id,
                    kind: entity.kind,
                    name: entity.name.clone(),
                    qualified_name: entity.qualified_name.clone(),
                    language: entity.language,
                    range: Some(entity.range),
                    signature: entity.signature.clone(),
                    documentation: entity.documentation.clone(),
                    confidence: entity.confidence,
                    relation_counts: std::collections::HashMap::new(),
                };

                let token_cost = self.estimate_tokens(&summary);
                if self.used + token_cost > self.budget {
                    return None;
                }

                let symbol = SymbolInfo::from_summary(summary);
                self.used += token_cost;
                self.included.insert(entity_id.0, symbol.clone());
                Some(symbol)
            } else {
                None
            }
        }

    pub fn expand(&mut self, qualified_name: &str) {
        if let Some(entity_id) = self.engine.graph().find_by_qualified(qualified_name) {
            let surrounding = self.engine.get_surrounding_context(entity_id, &prime_index::query::QueryOptions::for_agent());
            for summary in surrounding {
                if !self.included.contains_key(&summary.id.0) {
                    let token_cost = self.estimate_tokens(&summary);
                    if self.used + token_cost <= self.budget {
                        self.used += token_cost;
                        self.included.insert(summary.id.0, SymbolInfo::from_summary(summary));
                    }
                }
            }
        }
    }

    pub fn get_symbols(&self) -> Vec<SymbolInfo> {
        self.included.values().cloned().collect()
    }

    pub fn used_tokens(&self) -> usize {
        self.used
    }

    pub fn budget(&self) -> usize {
        self.budget
    }

    fn estimate_tokens(&self, summary: &prime_index::query::EntitySummary) -> usize {
        let mut tokens = 0;
        tokens += summary.name.len() / 4;
        tokens += summary.qualified_name.len() / 4;
        tokens += summary.signature.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += summary.documentation.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += 50;
        tokens
    }
}