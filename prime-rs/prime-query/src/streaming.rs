//! Streaming query results for large result sets

use prime_core::{KnowledgeGraph, Entity, EntityId, Relation, RelationKind, SymbolKind, Confidence, Range, Language};
use prime_index::query::{QueryEngine, QueryOptions};
use std::sync::Arc;

/// Streaming iterator for large query results
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

    /// Stream search results
    pub fn search(&mut self, query: &str) -> SearchStream {
        SearchStream::new(self)
    }

    /// Stream name matches
    pub fn find_by_name(&mut self, name: &str) -> NameStream {
        NameStream::new(self, name)
    }

    /// Stream prefix matches
    pub fn find_by_prefix(&mut self, prefix: &str) -> PrefixStream {
        PrefixStream::new(self, prefix)
    }

    /// Stream context for entities
    pub fn context_stream(&mut self, entity_ids: Vec<EntityId>) -> ContextStream {
        ContextStream::new(self, entity_ids)
    }
}

/// Stream of search results
pub struct SearchStream<'a> {
    query: &'a StreamingQuery<'a>,
    results: Vec<prime_index::query::EntitySummary>,
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

    pub fn next(&mut self) -> Option<prime_index::query::EntitySummary> {
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

/// Stream of name matches
pub struct NameStream<'a> {
    query: &'a StreamingQuery<'a>,
    results: Vec<prime_index::query::EntitySummary>,
    index: usize,
}

impl<'a> NameStream<'a> {
    fn new(query: &'a StreamingQuery<'a>, name: &str) -> Self {
        let mut stream = Self {
            query,
            results: Vec::new(),
            index: 0,
        };
        stream.name(name);
        stream
    }

    pub fn name(&mut self, n: &str) {
        self.results = self.query.engine.find_by_name(n, &self.query.opts);
        self.index = 0;
    }

    pub fn next(&mut self) -> Option<prime_index::query::EntitySummary> {
        if self.index < self.results.len() {
            let result = self.results[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// Stream of prefix matches
pub struct PrefixStream<'a> {
    query: &'a StreamingQuery<'a>,
    results: Vec<prime_index::query::EntitySummary>,
    index: usize,
}

impl<'a> PrefixStream<'a> {
    fn new(query: &'a StreamingQuery<'a>, prefix: &str) -> Self {
        let mut stream = Self {
            query,
            results: Vec::new(),
            index: 0,
        };
        stream.prefix(prefix);
        stream
    }

    pub fn prefix(&mut self, p: &str) {
        self.results = self.query.engine.find_by_prefix(p, &self.query.opts);
        self.index = 0;
    }

    pub fn next(&mut self) -> Option<prime_index::query::EntitySummary> {
        if self.index < self.results.len() {
            let result = self.results[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// Stream of context for entities
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

    pub fn next(&mut self) -> Option<prime_index::query::QueryResult> {
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
    engine: Arc<prime_index::query::QueryEngine>,
    batch_size: usize,
}

impl BatchProcessor {
    pub fn new(engine: Arc<prime_index::query::QueryEngine>, batch_size: usize) -> Self {
        Self { engine, batch_size }
    }

    /// Process entities in batches
    pub fn process_entities<F>(&self, entity_ids: Vec<EntityId>, mut f: F) -> anyhow::Result<()>
    where
        F: FnMut(&prime_core::Entity) -> anyhow::Result<()>,
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

    /// Process relations in batches
    pub fn process_relations<F>(&self, entity_ids: Vec<EntityId>, mut f: F) -> anyhow::Result<()>
    where
        F: FnMut(&prime_core::Relation) -> anyhow::Result<()>,
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
        Ok(())
    }

    /// Parallel batch processing
    pub fn process_parallel<F, R>(&self, entity_ids: Vec<EntityId>, f: F) -> Vec<R>
    where
        F: Fn(&prime_core::Entity) -> R + Send + Sync,
        R: Send,
    {
        use rayon::prelude::*;
        entity_ids.par_iter()
            .filter_map(|id| self.engine.graph.entities.get(id))
            .map(f)
            .collect()
    }
}