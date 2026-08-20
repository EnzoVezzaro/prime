//! Context building for agent consumption

use prime_core::{KnowledgeGraph, Entity, EntityId, Relation, RelationKind, SymbolKind, Confidence, Range, Language, File};
use prime_index::query::{QueryEngine, QueryOptions};
use std::collections::HashMap;
use std::sync::Arc;

/// Context builder for creating agent-optimized context
pub struct ContextBuilder {
    graph: Arc<KnowledgeGraph>,
    token_budget: usize,
    used: usize,
    entities: HashMap<u64, SymbolInfo>,
    relations: Vec<RelationInfo>,
}

impl ContextBuilder {
    pub fn new(graph: KnowledgeGraph, token_budget: usize) -> Self {
        Self {
            graph: Arc::new(graph),
            token_budget,
            used: 0,
            entities: HashMap::new(),
            relations: Vec::new(),
        }
    }

    /// Add a symbol by qualified name
    pub fn add_symbol(&mut self, qualified_name: &str) -> Option<SymbolInfo> {
        if let Some(entity_id) = self.graph.find_by_qualified(qualified_name) {
            if self.entities.contains_key(&entity_id.0) {
                return self.entities.get(&entity_id.0).cloned();
            }

            if let Some(entity) = self.graph.entities.get(&entity_id) {
                let symbol = SymbolInfo::from_entity(entity);
                let token_cost = self.estimate_tokens(&symbol);

                if self.used + token_cost > self.token_budget {
                    return None;
                }

                self.used += token_cost;
                self.entities.insert(entity_id.0, symbol.clone());
                Some(symbol)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Add multiple symbols
    pub fn add_symbols(&mut self, qualified_names: &[&str]) -> Vec<SymbolInfo> {
        let mut added = Vec::new();
        for name in qualified_names {
            if let Some(symbol) = self.add_symbol(name) {
                added.push(symbol);
            }
        }
        added
    }

    /// Expand context with surrounding symbols
    pub fn expand(&mut self, qualified_name: &str, depth: usize) {
        let mut current = vec![qualified_name.to_string()];
        for _ in 0..depth {
            let mut next = Vec::new();
            for name in &current {
                if let Some(entity_id) = self.graph.find_by_qualified(name) {
                    // Get dependencies
                    let deps = self.graph.dependencies(entity_id);
                    for dep_id in deps {
                        if let Some(entity) = self.graph.entities.get(&dep_id) {
                            let qn = entity.qualified_name.clone();
                            if !self.entities.contains_key(&dep_id.0) {
                                next.push(qn);
                            }
                        }
                    }

                    // Get dependents
                    let dependents = self.graph.dependents(entity_id);
                    for dep_id in dependents {
                        if let Some(entity) = self.graph.entities.get(&dep_id) {
                            let qn = entity.qualified_name.clone();
                            if !self.entities.contains_key(&dep_id.0) {
                                next.push(qn);
                            }
                        }
                    }

                    // Get callers/callees
                    let callers = self.graph.callers(entity_id);
                    for caller_id in callers {
                        if let Some(entity) = self.graph.entities.get(&caller_id) {
                            let qn = entity.qualified_name.clone();
                            if !self.entities.contains_key(&caller_id.0) {
                                next.push(qn);
                            }
                        }
                    }

                    let callees = self.graph.callees(entity_id);
                    for callee_id in callees {
                        if let Some(entity) = self.graph.entities.get(&callee_id) {
                            let qn = entity.qualified_name.clone();
                            if !self.entities.contains_key(&callee_id.0) {
                                next.push(qn);
                            }
                        }
                    }
                }
            }
            current = next;
        }
    }

    /// Get all included symbols
    pub fn get_symbols(&self) -> Vec<SymbolInfo> {
        self.entities.values().cloned().collect()
    }

    /// Get used token budget
    pub fn used_tokens(&self) -> usize {
        self.entities.values().map(|s| self.estimate_tokens(s)).sum()
    }

    /// Get remaining budget
    pub fn remaining_budget(&self) -> usize {
        self.token_budget.saturating_sub(self.used)
    }

    /// Build context package for agent
    pub fn build(self) -> AgentContext {
        AgentContext {
            symbols: self.entities,
            relations: self.relations,
            token_budget: self.token_budget,
            used: self.used,
        }
    }

    fn estimate_tokens(&self, symbol: &SymbolInfo) -> usize {
        let mut tokens = 50; // base overhead
        tokens += symbol.name.len() / 4;
        tokens += symbol.qualified_name.len() / 4;
        tokens += symbol.signature.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens += symbol.documentation.as_ref().map(|s| s.len() / 4).unwrap_or(0);
        tokens
    }
}

/// Symbol information for agent
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolInfo {
    pub id: u64,
    pub kind: prime_core::SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub language: prime_core::Language,
    pub range: Option<prime_core::Range>,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: prime_core::Confidence,
}

impl SymbolInfo {
    fn from_entity(entity: &prime_core::Entity) -> Self {
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
        }
    }
}

/// Relation information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelationInfo {
    pub kind: prime_core::RelationKind,
    pub target_id: u64,
    pub target_name: String,
    pub target_kind: prime_core::SymbolKind,
    pub confidence: prime_core::Confidence,
}

/// Final context package for agent
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentContext {
    pub symbols: HashMap<u64, SymbolInfo>,
    pub relations: Vec<RelationInfo>,
    pub token_budget: usize,
    pub used: usize,
}

impl AgentContext {
    pub fn symbols(&self) -> Vec<&SymbolInfo> {
        self.symbols.values().collect()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}