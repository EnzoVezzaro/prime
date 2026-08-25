//! Query API for agent-optimized retrieval

use prime_core::{Entity, EntityId, RelationKind, SymbolKind, Language, Confidence, Range};
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

/// Retrieval intent
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalIntent {
    Lookup,
    Prefix,
    Search,
    Callers,
    Callees,
    Dependents,
    Dependencies,
    Context,
    Unknown,
}

impl Default for RetrievalIntent {
    fn default() -> Self { RetrievalIntent::Unknown }
}

/// Retrieval metrics report
#[derive(Debug, Clone, Serialize, Default)]
pub struct RetrievalMetricsReport {
    pub query: String,
    pub precision_at_5: f32,
    pub recall_at_5: f32,
    pub ndcg_at_5: f32,
    pub mrr: f32,
    pub candidate_result_ratio: f32,
    pub evidence_density: f32,
    pub bytes_retrieved: usize,
    pub tokens_retrieved: usize,
}

/// Retrieval telemetry for explain mode
#[derive(Debug, Clone, Serialize, Default)]
pub struct RetrievalTelemetry {
    pub query: String,
    pub query_type: String,
    pub intent: String,
    pub candidate_count_before_filter: usize,
    pub candidate_count_after_filter: usize,
    pub final_count: usize,
    pub candidate_result_ratio: f32,
    pub evidence_nodes: usize,
    pub evidence_edges: usize,
    pub latency_ms: f32,
    pub note: String,
}

/// Candidate for retrieval pipeline
#[derive(Debug, Clone)]
pub struct Candidate {
    pub entity_id: EntityId,
    pub source: String,
    pub raw_score: u32,
    pub signals: CandidateSignals,
    pub final_score: f32,
}

#[derive(Debug, Clone, Default)]
pub struct CandidateSignals {
    pub name_exact_match: bool,
    pub name_prefix_match: bool,
    pub name_contains: bool,
    pub qualified_ends_with: bool,
    pub qualified_contains: bool,
    pub doc_contains: bool,
    pub confidence: Confidence,
}

#[derive(Debug, Clone)]
pub struct RankingPolicy {
    pub name_exact_match: f32,
    pub name_prefix_match: f32,
    pub name_contains: f32,
    pub qualified_ends_with: f32,
    pub qualified_contains: f32,
    pub doc_contains: f32,
    pub confidence_weight: f32,
}

impl RankingPolicy {
    fn for_intent(intent: &RetrievalIntent) -> Self {
        match intent {
            RetrievalIntent::Lookup => Self {
                name_exact_match: 100.0,
                name_prefix_match: 80.0,
                name_contains: 50.0,
                qualified_ends_with: 90.0,
                qualified_contains: 30.0,
                doc_contains: 5.0,
                confidence_weight: 10.0,
            },
            RetrievalIntent::Search => Self {
                name_exact_match: 100.0,
                name_prefix_match: 85.0,
                name_contains: 70.0,
                qualified_ends_with: 60.0,
                qualified_contains: 40.0,
                doc_contains: 10.0,
                confidence_weight: 5.0,
            },
            _ => Self {
                name_exact_match: 100.0,
                name_prefix_match: 85.0,
                name_contains: 70.0,
                qualified_ends_with: 60.0,
                qualified_contains: 40.0,
                doc_contains: 10.0,
                confidence_weight: 5.0,
            },
        }
    }

    fn score(&self, signals: &CandidateSignals) -> f32 {
        let mut s = 0.0;
        if signals.name_exact_match { s += self.name_exact_match; }
        if signals.name_prefix_match { s += self.name_prefix_match; }
        if signals.name_contains { s += self.name_contains; }
        if signals.qualified_ends_with { s += self.qualified_ends_with; }
        if signals.qualified_contains { s += self.qualified_contains; }
        if signals.doc_contains { s += self.doc_contains; }
        // confidence boost
        let conf_val = match signals.confidence {
            prime_core::Confidence::Exact => 1.0,
            prime_core::Confidence::High => 0.8,
            prime_core::Confidence::Medium => 0.5,
            prime_core::Confidence::Low => 0.2,
            prime_core::Confidence::Unknown => 0.0,
        };
        s += conf_val * self.confidence_weight;
        s
    }
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

fn classify_intent(query: &str, query_type: &str) -> RetrievalIntent {
    let q = query.to_lowercase();
    if query_type == "name" || q.contains("::") {
        return RetrievalIntent::Lookup;
    }
    if query_type == "prefix" {
        return RetrievalIntent::Prefix;
    }
    if q.starts_with("who calls") || q.contains(" calls ") || q.ends_with(" callers") {
        return RetrievalIntent::Callers;
    }
    if q.starts_with("what calls") || q.contains(" called by") {
        return RetrievalIntent::Callers;
    }
    if q.contains("callees") || (q.contains(" calls ") && q.contains("?")) {
        return RetrievalIntent::Callees;
    }
    if q.contains("dependents") {
        return RetrievalIntent::Dependents;
    }
    if q.contains("dependencies") || q.contains("depends on") {
        return RetrievalIntent::Dependencies;
    }
    if query_type == "context" {
        return RetrievalIntent::Context;
    }
    RetrievalIntent::Search
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

    pub fn get_context_with_telemetry(&self, entity_id: EntityId, opts: &QueryOptions) -> Option<(QueryResult, RetrievalTelemetry)> {
        use std::time::Instant;
        let start = Instant::now();
        let result = self.get_context(entity_id, opts)?;
        let surrounding = self.get_surrounding_context(entity_id, opts);
        let nodes = 1 + surrounding.len();
        let edges = result.relations.len();
        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;
        let telemetry = RetrievalTelemetry {
            query: format!("context:{:?}", entity_id),
            query_type: "context".to_string(),
            intent: "Context".to_string(),
            candidate_count_before_filter: 0,
            candidate_count_after_filter: nodes,
            final_count: nodes,
            candidate_result_ratio: 1.0,
            evidence_nodes: nodes,
            evidence_edges: edges,
            latency_ms,
            note: "Bounded context expansion with telemetry – progressive disclosure level 1".to_string(),
        };
        Some((result, telemetry))
    }

    pub fn get_progressive_context(&self, entity_id: EntityId, opts: &QueryOptions, level: u8) -> Option<QueryResult> {
        let result = self.get_context(entity_id, opts)?;
        if level >= 1 {
            let surrounding = self.get_surrounding_context(entity_id, opts);
            // Simplified: merge surrounding into result relations count
            // Full implementation would materialize nodes
            let _ = surrounding.len();
        }
        Some(result)
    }

    pub fn compute_metrics_stub(&self, query: &str, _results: &[EntitySummary]) -> RetrievalMetricsReport {
        RetrievalMetricsReport {
            query: query.to_string(),
            precision_at_5: 0.0,
            recall_at_5: 0.0,
            ndcg_at_5: 0.0,
            mrr: 0.0,
            candidate_result_ratio: 0.0,
            evidence_density: 0.0,
            bytes_retrieved: 0,
            tokens_retrieved: 0,
        }
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
    /// Uses index-based candidate generation + multi-signal scoring
    pub fn search(&self, query: &str, opts: &QueryOptions) -> Vec<EntitySummary> {
        let query_lower = query.to_lowercase();
        let query_tokens: Vec<&str> = query_lower.split_whitespace().collect();
        let is_multi_word = query_tokens.len() > 1;
        let mut candidates: Vec<(f32, EntityId)> = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // --- Stage 1: Index-based candidate generation (fast path) ---
        // 1a: Exact name match via index
        if let Some(ids) = self.graph.find_by_name(&query_lower) {
            for id in ids {
                if seen.insert(*id) {
                    candidates.push((1000.0, *id));
                }
            }
        }

        // 1b: Prefix match via index
        if let Some(ids) = self.graph.find_by_prefix(&query_lower) {
            for id in ids {
                if seen.insert(*id) {
                    candidates.push((900.0, *id));
                }
            }
        }

        // 1c: For multi-word queries, try individual tokens via index
        if is_multi_word {
            for token in &query_tokens {
                if token.len() < 2 { continue; }
                if let Some(ids) = self.graph.find_by_name(token) {
                    for id in ids {
                        if seen.insert(*id) {
                            candidates.push((500.0, *id));
                        }
                    }
                }
                if let Some(ids) = self.graph.find_by_prefix(token) {
                    for id in ids {
                        if seen.insert(*id) {
                            candidates.push((400.0, *id));
                        }
                    }
                }
            }
        }

        // 1e: For single-word queries, also try prefix index for broader coverage
        if !is_multi_word && query_lower.len() >= 2 {
            if let Some(ids) = self.graph.find_by_prefix(&query_lower) {
                for id in ids {
                    if seen.insert(*id) {
                        candidates.push((300.0, *id));
                    }
                }
            }
        }

        // 1d: Qualified name contains match via index (for path-like queries)
        if query.contains("::") || query.contains("/") {
            // Treat as qualified name lookup
            let qual_query = &query_lower;
            for (id, entity) in &self.graph.entities {
                if entity.confidence < opts.min_confidence { continue; }
                let qual_lower = entity.qualified_name.to_lowercase();
                if qual_lower.contains(qual_query) {
                    if seen.insert(*id) {
                        candidates.push((950.0, *id));
                    }
                }
            }
        }

        // --- Stage 2: Full scan fallback (only if index yielded very few results) ---
        if candidates.len() < 3 {
            for (id, entity) in &self.graph.entities {
                if entity.confidence < opts.min_confidence { continue; }
                if seen.contains(id) { continue; }

                let name_lower = entity.name.to_lowercase();
                let qual_lower = entity.qualified_name.to_lowercase();
                let doc_lower = entity.documentation.as_ref()
                    .map(|d| d.to_lowercase())
                    .unwrap_or_default();

                let score: f32 = if name_lower == query_lower {
                    1000.0
                } else if name_lower.starts_with(&query_lower) {
                    900.0
                } else if name_lower.contains(&query_lower) {
                    700.0
                } else if qual_lower.ends_with(&format!("::{}", query_lower)) {
                    600.0
                } else if qual_lower.contains(&query_lower) {
                    400.0
                } else if !doc_lower.is_empty() && doc_lower.contains(&query_lower) {
                    100.0
                } else {
                    0.0
                };

                if score > 0.0 {
                    seen.insert(*id);
                    candidates.push((score, *id));
                }
            }
        }

        // --- Stage 3: Score refinement with entity-type, path, and relationship signals ---
        let mut scored: Vec<(f32, EntityId)> = candidates.into_iter().map(|(base_score, id)| {
            let mut score = base_score;
            if let Some(entity) = self.graph.entities.get(&id) {
                // Bonus: name exactly matches query (exact match = highest confidence)
                if entity.name.to_lowercase() == query_lower {
                    score += 500.0;
                }
                // Bonus: qualified name ends with ::query (strong identifier match)
                if entity.qualified_name.to_lowercase().ends_with(&format!("::{}", query_lower)) {
                    score += 300.0;
                }
                // Bonus: path contains query tokens (file/module relevance)
                let path_lower = entity.qualified_name.to_lowercase();
                for token in &query_tokens {
                    if path_lower.contains(token) {
                        score += 50.0;
                    }
                }
                // Bonus: entity has outgoing relationships (richer entities are more useful)
                let outgoing = self.graph.all_outgoing(id);
                if !outgoing.is_empty() {
                    score += 100.0;
                }
                // Penalty: very long qualified names (likely internal/implementation details)
                let qual_depth = entity.qualified_name.matches("::").count();
                if qual_depth > 5 {
                    score -= 20.0 * (qual_depth as f32 - 5.0);
                }
            }
            (score, id)
        }).collect();

        // --- Stage 4: Sort and return ---
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                let name_a = self.graph.entities.get(&a.1).map(|e| e.name.as_str()).unwrap_or("");
                let name_b = self.graph.entities.get(&b.1).map(|e| e.name.as_str()).unwrap_or("");
                name_a.cmp(name_b)
            }));

        scored.into_iter()
            .take(opts.max_results)
            .filter_map(|(_, id)| self.graph.entities.get(&id).map(|e| self.to_summary(&id, e)))
            .collect()
    }

    /// Search with retrieval telemetry for explain mode
    pub fn search_with_telemetry(&self, query: &str, opts: &QueryOptions) -> (Vec<EntitySummary>, RetrievalTelemetry) {
        use std::time::Instant;
        let start = Instant::now();
        let query_lower = query.to_lowercase();
        let mut candidates: Vec<Candidate> = Vec::new();
        let mut candidate_before = 0usize;

        let intent = classify_intent(query, "search");
        let policy = RankingPolicy::for_intent(&intent);

        // Candidate Generation + Enrichment
        for (id, entity) in &self.graph.entities {
            if entity.confidence < opts.min_confidence {
                continue;
            }
            let name_lower = entity.name.to_lowercase();
            let qual_lower = entity.qualified_name.to_lowercase();
            let doc_lower = entity.documentation.as_ref()
                .map(|d| d.to_lowercase())
                .unwrap_or_default();

            let name_exact = name_lower == query_lower;
            let name_prefix = name_lower.starts_with(&query_lower) && !name_exact;
            let name_contains = name_lower.contains(&query_lower) && !name_prefix && !name_exact;
            let qual_ends = qual_lower.ends_with(&format!("::{}", query_lower));
            let qual_contains = qual_lower.contains(&query_lower) && !qual_ends;
            let doc_contains = !doc_lower.is_empty() && doc_lower.contains(&query_lower);

            let signals = CandidateSignals {
                name_exact_match: name_exact,
                name_prefix_match: name_prefix,
                name_contains,
                qualified_ends_with: qual_ends,
                qualified_contains: qual_contains,
                doc_contains,
                confidence: entity.confidence,
            };

            let raw_score: u32 = if signals.name_exact_match { 100 }
            else if signals.name_prefix_match { 90 }
            else if signals.name_contains { 70 }
            else if signals.qualified_ends_with { 60 }
            else if signals.qualified_contains { 40 }
            else if signals.doc_contains { 10 }
            else { 0 };

            if raw_score > 0 {
                candidate_before += 1;
                let final_score = policy.score(&signals);
                candidates.push(Candidate {
                    entity_id: *id,
                    source: "fulltext".to_string(),
                    raw_score,
                    signals,
                    final_score,
                });
            }
        }

        // Filtering step – already filtered by confidence, now dedup
        let mut seen = std::collections::HashSet::new();
        candidates.retain(|c| seen.insert(c.entity_id));

        let candidate_after = candidates.len();

        // Ranking – deterministic sort by final_score then name
        candidates.sort_by(|a, b| {
            b.final_score.partial_cmp(&a.final_score).unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    let name_a = self.graph.entities.get(&a.entity_id).map(|e| e.name.as_str()).unwrap_or("");
                    let name_b = self.graph.entities.get(&b.entity_id).map(|e| e.name.as_str()).unwrap_or("");
                    name_a.cmp(name_b)
                })
        });

        let top_ids: Vec<EntityId> = candidates.iter()
            .take(opts.max_results)
            .map(|c| c.entity_id)
            .collect();

        let results: Vec<EntitySummary> = top_ids.iter()
            .filter_map(|id| self.graph.entities.get(id).map(|e| self.to_summary(id, e)))
            .collect();

        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;
        let candidate_result_ratio = if results.is_empty() { 0.0 } else { candidate_after as f32 / results.len() as f32 };
        let intent_str = format!("{:?}", intent);
        let telemetry = RetrievalTelemetry {
            query: query.to_string(),
            query_type: "search".to_string(),
            intent: intent_str,
            candidate_count_before_filter: candidate_before,
            candidate_count_after_filter: candidate_after,
            final_count: results.len(),
            candidate_result_ratio,
            evidence_nodes: results.len(),
            evidence_edges: 0,
            latency_ms,
            note: "Candidate/ranking separation + intent-aware policy implemented".to_string(),
        };

        (results, telemetry)
    }

    /// Lookup with telemetry for explain mode
    pub fn lookup_with_telemetry(&self, query: &str, opts: &QueryOptions) -> (Vec<EntitySummary>, RetrievalTelemetry) {
        use std::time::Instant;
        let start = Instant::now();
        
        let intent = classify_intent(query, "name");
        let mut candidate_before = 0;
        let mut candidates = Vec::new();
        
        // Try exact qualified name match first
        if let Some(id) = self.graph.find_by_qualified(query) {
            candidate_before += 1;
            if let Some(entity) = self.graph.entities.get(&id) {
                candidates.push(self.to_summary(&id, entity));
            }
        }
        
        // Also try name match
        if let Some(ids) = self.graph.find_by_name(query) {
            for id in ids {
                candidate_before += 1;
                if let Some(entity) = self.graph.entities.get(id) {
                    if !candidates.iter().any(|c| c.id == *id) {
                        candidates.push(self.to_summary(id, entity));
                    }
                }
            }
        }
        
        let candidate_after = candidates.len();
        let results: Vec<EntitySummary> = candidates.into_iter().take(opts.max_results).collect();
        
        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;
        let telemetry = RetrievalTelemetry {
            query: query.to_string(),
            query_type: "name".to_string(),
            intent: format!("{:?}", intent),
            candidate_count_before_filter: candidate_before,
            candidate_count_after_filter: candidate_after,
            final_count: results.len(),
            candidate_result_ratio: if results.is_empty() { 0.0 } else { candidate_after as f32 / results.len() as f32 },
            evidence_nodes: results.len(),
            evidence_edges: 0,
            latency_ms,
            note: "Lookup by qualified name + simple name".to_string(),
        };
        
        (results, telemetry)
    }

    /// Prefix search with telemetry for explain mode
    pub fn prefix_with_telemetry(&self, query: &str, opts: &QueryOptions) -> (Vec<EntitySummary>, RetrievalTelemetry) {
        use std::time::Instant;
        let start = Instant::now();
        
        let intent = classify_intent(query, "prefix");
        let mut candidate_before = 0;
        let mut candidates = Vec::new();
        
        // Find entities by prefix
        if let Some(ids) = self.graph.find_by_prefix(query) {
            for id in ids {
                candidate_before += 1;
                if let Some(entity) = self.graph.entities.get(id) {
                    if entity.confidence >= opts.min_confidence {
                        candidates.push(self.to_summary(id, entity));
                    }
                }
            }
        }
        
        let candidate_after = candidates.len();
        let results: Vec<EntitySummary> = candidates.into_iter().take(opts.max_results).collect();
        
        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;
        let telemetry = RetrievalTelemetry {
            query: query.to_string(),
            query_type: "prefix".to_string(),
            intent: format!("{:?}", intent),
            candidate_count_before_filter: candidate_before,
            candidate_count_after_filter: candidate_after,
            final_count: results.len(),
            candidate_result_ratio: if results.is_empty() { 0.0 } else { candidate_after as f32 / results.len() as f32 },
            evidence_nodes: results.len(),
            evidence_edges: 0,
            latency_ms,
            note: "Prefix search with confidence filtering".to_string(),
        };
        
        (results, telemetry)
    }

    /// Context lookup with telemetry for explain mode
    pub fn context_with_telemetry(&self, query: &str, opts: &QueryOptions) -> (Vec<EntitySummary>, RetrievalTelemetry) {
        use std::time::Instant;
        let start = Instant::now();
        
        let intent = classify_intent(query, "context");
        let mut candidate_before = 0;
        let mut results = Vec::new();
        let mut evidence_edges = 0;
        
        // Find the entity
        if let Some(entity_id) = self.graph.find_by_qualified(query) {
            candidate_before += 1;
            if let Some(entity) = self.graph.entities.get(&entity_id) {
                results.push(self.to_summary(&entity_id, entity));
                
                // Get surrounding context
                let surrounding = self.get_surrounding_context(entity_id, opts);
                evidence_edges = surrounding.len();
                for s in surrounding {
                    if !results.iter().any(|r| r.id == s.id) {
                        results.push(s);
                    }
                }
            }
        }
        
        let candidate_after = results.len();
        results.truncate(opts.max_results);
        
        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;
        let telemetry = RetrievalTelemetry {
            query: query.to_string(),
            query_type: "context".to_string(),
            intent: format!("{:?}", intent),
            candidate_count_before_filter: candidate_before,
            candidate_count_after_filter: candidate_after,
            final_count: results.len(),
            candidate_result_ratio: if results.is_empty() { 0.0 } else { candidate_after as f32 / results.len() as f32 },
            evidence_nodes: results.len(),
            evidence_edges,
            latency_ms,
            note: "Context expansion with callers, callees, dependencies".to_string(),
        };
        
        (results, telemetry)
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
        let relation_counts: HashMap<RelationKind, usize> = HashMap::new();
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