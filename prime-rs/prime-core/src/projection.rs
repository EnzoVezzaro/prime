//! Prime Projection Engine
//!
//! This module implements the projection layer that converts a knowledge graph
// into a minimal PAR projection tailored to a specific query.

use crate::par::{ParDocument, ParVocabulary, ParAliases, ParFact, ParObject, ParMetadata, ParEntity, entity_to_par_entity};
use crate::{KnowledgeGraph, Entity, EntityId, Relation, RelationKind, SymbolKind, Confidence, Language, Range, Confidence as Conf};
use std::collections::{HashMap, HashSet};
use serde_json;

/// Projection configuration
#[derive(Debug, Clone)]
pub struct ProjectionConfig {
    /// Maximum number of entities to include
    pub max_entities: usize,
    /// Maximum number of facts to include
    pub max_facts: usize,
    /// Token budget for the projection
    pub token_budget: usize,
    /// Minimum confidence threshold
    pub min_confidence: crate::Confidence,
    /// Whether to include entity relations inline
    pub include_relations: bool,
    /// Whether to use aliases
    pub use_aliases: bool,
    /// Whether to include vocabulary
    pub include_vocabulary: bool,
    /// Query that triggered this projection
    pub query: Option<String>,
    /// Target entity for focused projections
    pub target_entity: Option<String>,
    /// Depth for transitive relationships
    pub max_depth: usize,
}

impl Default for ProjectionConfig {
    fn default() -> Self {
        Self {
            max_entities: 100,
            max_facts: 500,
            token_budget: 8192,
            min_confidence: crate::Confidence::Medium,
            include_relations: true,
            use_aliases: true,
            include_vocabulary: true,
            query: None,
            target_entity: None,
            max_depth: 2,
        }
    }
}

/// Projection Engine — Converts knowledge graph into PAR projections
pub struct ProjectionEngine {
    vocab: crate::par::ParVocabulary,
}

impl Default for ProjectionEngine {
    fn default() -> Self {
        Self {
            vocab: crate::par::ParVocabulary::default(),
        }
    }
}

impl ProjectionEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Project a knowledge graph into a PAR document based on a query
    pub fn project(
        &self,
        graph: &crate::KnowledgeGraph,
        config: ProjectionConfig,
    ) -> crate::par::ParDocument {
        let mut _aliases = crate::par::ParAliases::new();
        let mut _facts: Vec<crate::par::ParFact> = Vec::new();
        let mut _entity_map: HashMap<EntityId, crate::par::ParEntity> = HashMap::new();
        let mut _fact_count = 0;

        // Step 1: Determine relevant entities based on query
        let relevant_entities = self.find_relevant_entities(graph, &config);

        // Step 2: Create aliases for relevant entities
        let mut _aliases = crate::par::ParAliases::new();
        for entity_id in &relevant_entities {
            if let Some(entity) = graph.entities.get(&*entity_id) {
                crate::par::entity_to_par_entity(
                    entity,
                    entity_id,
                    &mut crate::par::ParAliases::new(),
                    &self.vocab,
                );
            }
        }

        // Step 3: Extract facts for relevant entities (placeholder)
        let _facts: Vec<crate::par::ParFact> = Vec::new();

        // Build the document
        let metadata = crate::par::ParMetadata {
            query: config.query.clone(),
            entity_count: relevant_entities.len(),
            fact_count: 0,
            token_estimate: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            vocabulary_version: "1.0".to_string(),
        };

        let _vocab = if config.include_vocabulary {
            Some(self.vocab.clone())
        } else {
            None
        };

        crate::par::ParDocument {
            version: "1.0".to_string(),
            vocabulary: if config.include_vocabulary { Some(self.vocab.clone()) } else { None },
            aliases: crate::par::ParAliases::new(),
            facts: Vec::new(),
            metadata: crate::par::ParMetadata {
                query: config.query.clone(),
                entity_count: relevant_entities.len(),
                fact_count: 0,
                token_estimate: 0,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                vocabulary_version: "1.0".to_string(),
            },
        }
    }

    /// Find relevant entities for a query
    fn find_relevant_entities(
        &self,
        graph: &crate::KnowledgeGraph,
        config: &ProjectionConfig,
    ) -> Vec<EntityId> {
        // If target entity specified, start from there and expand
        if let Some(target) = &config.target_entity {
            if let Some(target_id) = graph.find_by_qualified(target) {
                return self.expand_from_entity(graph, target, config.max_depth, config.max_entities);
            }
        }

        // If query specified, search for relevant entities
        if let Some(query) = &config.query {
            return self.search_relevant_entities(graph, query, config.max_entities);
        }

        // Default: return all entities up to max_entities, filtered by confidence
        graph.entities.keys()
            .filter(|id| {
                if let Some(entity) = graph.entities.get(*id) {
                    entity.confidence >= config.min_confidence
                } else {
                    false
                }
            })
            .take(config.max_entities)
            .cloned()
            .collect()
    }

    /// Expand from a target entity using graph traversal
    fn expand_from_entity(
        &self,
        graph: &crate::KnowledgeGraph,
        target: &str,
        max_depth: usize,
        max_entities: usize,
    ) -> Vec<EntityId> {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        let mut result = Vec::new();

        if let Some(start_id) = graph.find_by_qualified(target) {
            queue.push((start_id, 0));
            visited.insert(start_id);
        }

        while let Some((current_id, depth)) = queue.pop() {
            if depth > max_depth {
                continue;
            }

            result.push(current_id);

            if result.len() >= max_entities {
                break;
            }

            if depth < max_depth {
                // Add dependencies
                for dep_id in graph.dependencies(current_id) {
                    if visited.insert(dep_id) {
                        queue.push((dep_id, depth + 1));
                    }
                }
                // Add dependents
                for dep_id in graph.dependents(current_id) {
                    if visited.insert(dep_id) {
                        queue.push((dep_id, depth + 1));
                    }
                }
                // Add callers
                for caller_id in graph.callers(current_id) {
                    if visited.insert(caller_id) {
                        queue.push((caller_id, depth + 1));
                    }
                }
                // Add callees
                for callee_id in graph.callees(current_id) {
                    if visited.insert(callee_id) {
                        queue.push((callee_id, depth + 1));
                    }
                }
            }
        }

        result
    }

    /// Search for relevant entities by keyword
    fn search_relevant_entities(
        &self,
        graph: &crate::KnowledgeGraph,
        query: &str,
        max_entities: usize,
    ) -> Vec<EntityId> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for (id, entity) in &graph.entities {
            if entity.name.to_lowercase().contains(&query_lower)
                || entity.qualified_name.to_lowercase().contains(&query_lower)
                || entity.documentation.as_ref().map(|d| d.to_lowercase().contains(&query_lower)).unwrap_or(false)
            {
                if entity.confidence >= crate::Confidence::Medium {
                    results.push(*id);
                    if results.len() >= max_entities {
                        break;
                    }
                }
            }
        }

        results
    }

    /// Extract facts for a single entity (placeholder)
    fn _extract_entity_facts(
        &self,
        _entity: &crate::Entity,
        _entity_id: &EntityId,
        _graph: &crate::KnowledgeGraph,
        _config: &ProjectionConfig,
    ) -> Vec<crate::par::ParFact> {
        Vec::new()
    }

    /// Estimate token count for facts
    fn estimate_tokens(&self, facts: &[crate::par::ParFact]) -> usize {
        let mut tokens = 0;
        for fact in facts {
            tokens += fact.subject.len() / 4;
            tokens += fact.predicate.len() / 4;
            match &fact.object {
                crate::par::ParObject::Entity(e) => tokens += e.len() / 4,
                crate::par::ParObject::Literal(v) => tokens += v.to_string().len() / 4,
                crate::par::ParObject::Entities(es) => tokens += es.iter().map(|s| s.len() / 4).sum::<usize>(),
            }
            if let Some(c) = &fact.confidence { tokens += c.len() / 4; }
            if let Some(l) = &fact.location { tokens += l.len() / 4; }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection_config_default() {
        let config = crate::projection::ProjectionConfig::default();
        assert_eq!(config.max_entities, 100);
        assert_eq!(config.max_facts, 500);
        assert_eq!(config.token_budget, 8192);
    }
}