//! Prime Query - Agent-optimized query API

use crate::types::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Query options for controlling retrieval
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryOptions {
    pub max_results: usize,
    pub include_relations: bool,
    pub relation_kinds: Option<Vec<RelationKind>>,
    pub max_depth: usize,
    pub min_confidence: Confidence,
    pub include_documentation: bool,
    pub include_signatures: bool,
    pub token_budget: usize,
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
            token_budget: 8192,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entity: EntitySummary,
    pub relations: Vec<RelationSummary>,
    pub score: f32,
}

/// Lightweight entity summary for agent context
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub relation_counts: std::collections::HashMap<RelationKind, usize>,
}

/// Lightweight relation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationSummary {
    pub kind: RelationKind,
    pub target_id: EntityId,
    pub target_name: String,
    pub target_kind: SymbolKind,
    pub confidence: Confidence,
}

/// Query options for controlling retrieval
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryOptions {
    pub max_results: usize,
    pub include_relations: bool,
    pub relation_kinds: Option<Vec<RelationKind>>,
    pub max_depth: usize,
    pub min_confidence: Confidence,
    pub include_documentation: bool,
    pub include_signatures: bool,
    pub token_budget: usize,
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
            token_budget: 8192,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entity: EntitySummary,
    pub relations: Vec<RelationSummary>,
    pub score: f32,
}

/// Lightweight entity summary for agent context
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub relation_counts: std::collections::HashMap<RelationKind, usize>,
}

/// Lightweight relation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationSummary {
    pub kind: RelationKind,
    pub target_id: EntityId,
    pub target_name: String,
    pub target_kind: SymbolKind,
    pub confidence: Confidence,
}

/// Main query engine for agent-optimized retrieval
pub struct QueryEngine {
    graph: Arc<KnowledgeGraph>,
}

impl QueryEngine {
    pub fn new(graph: KnowledgeGraph) -> Self {
        Self {
            graph: Arc::new(graph),
        }
    }

    pub fn graph(&self) -> &KnowledgeGraph {
        &self.graph
    }

    /// Find entity by qualified name
    pub fn find_by_qualified(&self, name: &str) -> Option<EntityId> {
        self.graph.find_by_qualified(name)
    }

    /// Find entities by name (fuzzy)
    pub fn find_by_name(&self, name: &str, opts: &QueryOptions) -> Vec<EntitySummary> {
        self.graph
            .find_by_name(name)
            .into_iter()
            .filter_map(|id| self.graph.entities.get(&id).map(|e| self.to_summary(&id, e)))
            .filter(|s| s.confidence >= opts.min_confidence)
            .take(opts.max_results)
            .collect()
    }

    /// Find entities by prefix
    pub fn find_by_prefix(&self, prefix: &str, opts: &QueryOptions) -> Vec<EntitySummary> {
        if let Some(ids) = self.graph.find_by_prefix(prefix) {
            ids.into_iter()
                .filter_map(|id| self.graph.entities.get(&id).map(|e| self.to_summary(&id, e)))
                .filter(|s| s.confidence >= opts.min_confidence)
                .take(opts.max_results)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Search across all symbols
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
                    results.push(self.to_summary(&id, entity));
                }
            }
        }

        // Sort by relevance (exact name match > qualified match > doc match)
        results.sort_by(|a, b| {
            let a_exact = a.name.to_lowercase() == query_lowercase();
            let b_exact = b.name.to_lowercase() == query_lowercase();
            b_exact.cmp(&a_exact)
        });

        results.truncate(opts.max_results);
        results
    }

    /// Get full context for an entity
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

    /// Get surrounding context (callers, callees, deps, dependents)
    pub fn get_surrounding_context(&self, entity_id: EntityId, opts: &QueryOptions) -> Vec<EntitySummary> {
        let mut results = Vec::new();

        // Dependencies
        let deps = self.graph.dependencies(entity_id);
        for dep_id in deps {
            if let Some(entity) = self.graph.entities.get(&dep_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&dep_id, entity));
                }
            }
        }

        // Dependents
        let dependents = self.graph.dependents(entity_id);
        for dep_id in dependents {
            if let Some(entity) = self.graph.entities.get(&dep_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&dep_id, entity));
                }
            }
        }

        // Callers
        let callers = self.graph.callers(entity_id);
        for caller_id in callers {
            if let Some(entity) = self.graph.entities.get(&caller_id) {
                if entity.confidence >= opts.min_confidence {
                    results.push(self.to_summary(&caller_id, entity));
                }
            }
        }

        // Callees
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

    /// Get entity summary with relations
    fn to_summary(&self, id: &EntityId, entity: &Entity) -> EntitySummary {
        let mut relation_counts = std::collections::HashMap::new();
        if let Some(index) = &self.graph.relation_index {
            for (kind, targets) in index.outgoing.get(&entity.id).unwrap_or(&Vec::new()) {
                *relation_counts.entry(kind).or_insert(0) += targets.len();
            }
        }

        EntitySummary {
            id: entity.id,
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

    fn get_relations(&self, entity_id: EntityId, opts: &QueryOptions) -> Vec<RelationSummary> {
        let mut results = Vec::new();

        if let Some(index) = &self.graph.relation_index {
            for (kind, target_id) in index.outgoing.get(&entity_id).unwrap_or(&Vec::new()) {
                if opts.relation_kinds.as_ref().map(|k| k.contains(kind)).unwrap_or(true) {
                    if let Some(target) = self.graph.entities.get(target_id) {
                        if target.confidence >= opts.min_confidence {
                            results.push(RelationSummary {
                                kind,
                                target_id: *target_id,
                                target_name: target.name.clone(),
                                target_kind: target.kind,
                                confidence: Confidence::High,
                            });
                        }
                    }
                }

                for (kind, source_id) in index.incoming.get(&entity_id).unwrap_or(&Vec::new()) {
                    if opts.relation_kinds.as_ref().map(|k| k.contains(kind)).unwrap_or(true) {
                        if let Some(source) = self.graph.entities.get(source_id) {
                            if source.confidence >= opts.min_confidence {
                                results.push(RelationSummary {
                                    kind,
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
        }

        results
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

    pub fn add_entity(&mut self, entity_id: EntityId) -> Option<EntitySummary> {
        if self.included.contains_key(&entity_id) {
            return self.included.get(&entity_id).cloned();
        }

        if let Some(entity) = self.engine.graph.entities.get(&entity_id) {
            let summary = self.to_summary(&entity_id, entity);
            let token_cost = self.estimate_tokens(&summary);
            if self.used + token_cost > self.budget {
                return None;
            }

            self.used += token_cost;
            self.included.insert(entity_id, summary.clone());
            Some(summary)
        } else {
            None
        }
    }

    pub fn expand_context(&mut self, entity_id: EntityId) {
        let opts = QueryOptions::for_agent();
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

    pub fn get_included(&self) -> Vec<EntitySummary> {
        self.included.values().cloned().collect()
    }

    fn to_summary(&self, id: EntityId, entity: &Entity) -> EntitySummary {
        let mut relation_counts = std::collections::HashMap::new();
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
            relation_counts: HashMap::new(),
        }
    }

    fn estimate_tokens(&self, summary: &EntitySummary) -> usize {
        let mut tokens = 50;
        tokens += summary.name.len() / 4;
        tokens += summary.qualified_name.len() / 4;
        tokens += summary.signature.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += summary.documentation.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens
    }
}

/// Streaming query for large result sets
pub mod streaming {
    use super::*;
    use crate::types::*;

    pub struct StreamingQuery<'a> {
        engine: &'a QueryEngine,
        opts: QueryOptions,
        current_batch: Vec<EntityId>,
        batch_index: usize,
        batch_size: usize,
        total_processed: usize,
        max_results: usize,
    }

    impl<'a> StreamingQuery<'a> {
        pub fn new(engine: &'a QueryEngine, opts: QueryOptions) -> Self {
            let batch_size = 100;
            let max_results = opts.max_results;
            Self {
                engine,
                opts,
                current_batch: Vec::new(),
                batch_index: 0,
                batch_size,
                total_processed: 0,
                max_results,
            }
        }

        pub fn search(&mut self, query: &str) -> SearchStream {
            SearchStream::new(self)
        }

        pub fn find_by_name(&mut self, name: &str) -> NameStream {
            NameStream::new(self, name)
        }

        pub fn find_by_prefix(&mut self, prefix: &str) -> PrefixStream {
            PrefixStream::new(self, prefix)
        }

        pub fn context_stream(&mut self, entity_ids: Vec<EntityId>) -> ContextStream {
            ContextStream::new(self, entity_ids)
        }
    }

    pub struct SearchStream<'a> {
        query: &'a StreamingQuery<'a>,
        results: Vec<EntitySummary>,
        index: usize,
        query_str: String,
    }

    impl<'a> SearchStream<'a> {
        fn new(query: &'a StreamingQuery<'a>) -> Self {
            Self {
                query,
                results: Vec::new(),
                index: 0,
                query_str: String::new(),
            }
        }

        pub fn query(&mut self, q: &str) {
            self.query_str = q.to_string();
            self.results = self.query.engine.search(q, &self.query.opts);
            self.index = 0;
        }

        pub fn next(&mut self) -> Option<EntitySummary> {
            if self.index < self.results.len() {
                let result = self.results[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }

        pub fn remaining(&self) -> usize {
            self.results.len() - self.index
        }
    }

    pub struct NameStream<'a> {
        query: &'a StreamingQuery<'a>,
        results: Vec<EntitySummary>,
        index: usize,
    }

    impl<'a> NameStream<'a> {
        fn new(query: &'a StreamingQuery<'a>) -> Self {
            Self {
                query,
                results: Vec::new(),
                index: 0,
            }
        }

        pub fn name(&mut self, n: &str) {
            self.results = self.query.engine.find_by_name(n, &self.query.opts);
            self.index = 0;
        }

        pub fn next(&mut self) -> Option<EntitySummary> {
            if self.index < self.results.len() {
                let result = self.results[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    pub struct PrefixStream<'a> {
        query: &'a StreamingQuery<'a>,
        results: Vec<EntitySummary>,
        index: usize,
    }

    impl<'a> PrefixStream<'a> {
        fn new(query: &'a StreamingQuery<'a>) -> Self {
            Self {
                query,
                results: Vec::new(),
                index: 0,
            }
        }

        pub fn prefix(&mut self, p: &str) {
            self.results = self.query.engine.find_by_prefix(p, &self.query.opts);
            self.index = 0;
        }

        pub fn next(&mut self) -> Option<EntitySummary> {
            if self.index < self.results.len() {
                let result = self.results[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    pub struct ContextStream<'a> {
        query: &'a StreamingQuery<'a>,
        entity_ids: Vec<EntityId>,
        index: usize,
        opts: QueryOptions,
    }

    impl<'a> ContextStream<'a> {
        fn new(query: &'a StreamingQuery<'a>, entity_ids: Vec<EntityId>) -> Self {
            let opts = query.opts.clone();
            Self {
                query,
                entity_ids,
                index: 0,
                opts,
            }
        }

        pub fn next(&mut self) -> Option<QueryResult> {
            if self.index < self.entity_ids.len() {
                let entity_id = self.entity_ids[self.index];
                self.index += 1;
                self.query.engine.get_context(entity_id, &self.opts)
            } else {
                None
            }
        }
    }

    /// Batch processor for large-scale operations
    pub struct BatchProcessor {
        engine: Arc<QueryEngine>,
        batch_size: usize,
    }

    impl BatchProcessor {
        pub fn new(engine: Arc<QueryEngine>, batch_size: usize) -> Self {
            Self { engine, batch_size }
        }

        pub fn process_entities<F>(&self, entity_ids: Vec<EntityId>, mut f: F) -> anyhow::Result<()>
        where
            F: FnMut(&Entity) -> anyhow::Result<()>,
        {
            for chunk in entity_ids.chunks(self.batch_size) {
                for id in chunk {
                    if let Some(entity) = self.engine.graph.entities.get(id) {
                        f(entity)?;
                    }
                }
            }
            Ok(())
        }

        pub fn process_relations<F>(&self, entity_ids: Vec<EntityId>, mut f: F) -> anyhow::Result<()>
        where
            F: FnMut(&Relation) -> anyhow::Result<()>,
        {
            for chunk in entity_ids.chunks(self.batch_size) {
                for id in chunk {
                    if let Some(index) = &self.engine.graph.relation_index {
                        for (_, target) in index.outgoing.get(id).unwrap_or(&Vec::new()) {
                            if let Some(rel) = self.engine.graph.relations.iter().find(|r| r.from == *id && r.to == *target) {
                                f(rel)?;
                            }
                        }
                    }
                }
            }
        }

        pub fn process_parallel<F, R>(&self, entity_ids: Vec<EntityId>, f: F) -> Vec<R>
        where
            F: Fn(&Entity) -> R + Send + Sync,
            R: Send,
        {
            use rayon::prelude::*;
            entity_ids.par_iter()
                .filter_map(|id| self.engine.graph.entities.get(id))
                .map(f)
                .collect()
        }
    }
}