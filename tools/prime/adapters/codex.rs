//! Codex Adapter - Optimized for Codex

use crate::adapters::{AgentAdapter, AdapterConfig, AgentAdapter, AgentResponse, AgentQuery, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder, AgentQuery, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder, AgentQuery, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder, AgentQuery, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder};
use crate::core::provider::AgentQuery;
use std::sync::Arc;
use anyhow::Result;

/// Codex Adapter - Optimized for Codex
pub struct CodexAdapter {
    generic: GenericAdapter,
}

impl CodexAdapter {
    pub fn new(agent_query: crate::core::provider::AgentQuery, config: AdapterConfig) -> Self {
        Self {
            generic: GenericAdapter::new(agent_query, config),
        }
    }
}

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn initialize(&mut self, config: AdapterConfig) -> anyhow::Result<()> {
        self.generic.initialize(config)
    }

    fn query(&self, query: &str) -> anyhow::Result<AgentResponse> {
        self.generic.query(query)
    }

    fn find_symbol(&self, qualified_name: &str) -> anyhow::Result<Option<SymbolInfo>> {
        self.generic.find_symbol(qualified_name)
    }

    fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.generic.find_symbols(name)
    }

    fn get_context(&self, qualified_name: &str) -> Option<Context> {
        self.generic.get_context(qualified_name)
    }

    fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_surrounding(qualified_name)
    }

    fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_callers(qualified_name)
    }

    fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_callees(qualified_name)
    }

    fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_dependencies(qualified_name)
    }

    fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_dependents(qualified_name)
    }

    fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        self.generic.context_builder(8192)
    }

    fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.generic.search_all(query)
    }
}

/// Codex Adapter - Optimized for Codex
pub struct CodexAdapter {
    generic: GenericAdapter,
}

impl CodexAdapter {
    pub fn new(agent_query: crate::core::provider::AgentQuery, config: AdapterConfig) -> Self {
        Self {
            generic: GenericAdapter::new(agent_query, config),
        }
    }
}

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn initialize(&mut self, config: AdapterConfig) -> anyhow::Result<()> {
        self.generic.initialize(config)
    }

    fn query(&self, query: &str) -> anyhow::Result<AgentResponse> {
        self.generic.query(query)
    }

    fn find_symbol(&self, qualified_name: &str) -> anyhow::Result<Option<SymbolInfo>> {
        self.generic.find_symbol(qualified_name)
    }

    fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.generic.find_symbols(name)
    }

    fn get_context(&self, qualified_name: &str) -> Option<Context> {
        self.generic.get_context(qualified_name)
    }

    fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_surrounding(qualified_name)
    }

    fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_callers(qualified_name)
    }

    fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_callees(qualified_name)
    }

    fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_dependencies(qualified_name)
    }

    fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_dependents(qualified_name)
    }

    fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        self.generic.context_builder(8192)
    }

    fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.generic.search_all(query)
    }
}

/// Codex Adapter - Optimized for Codex
pub struct CodexAdapter {
    generic: GenericAdapter,
}

impl CodexAdapter {
    pub fn new(agent_query: crate::core::provider::AgentQuery, config: AdapterConfig) -> Self {
        Self {
            generic: GenericAdapter::new(agent_query, config),
        }
    }
}

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn initialize(&mut self, config: AdapterConfig) -> anyhow::Result<()> {
        self.generic.initialize(config)
    }

    fn query(&self, query: &str) -> anyhow::Result<AgentResponse> {
        self.generic.query(query)
    }

    fn find_symbol(&self, qualified_name: &str) -> anyhow::Result<Option<SymbolInfo>> {
        self.generic.find_symbol(qualified_name)
    }

    fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.generic.find_symbols(name)
    }

    fn get_context(&self, qualified_name: &str) -> Option<Context> {
        self.generic.get_context(qualified_name)
    }

    fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_surrounding(qualified_name)
    }

    fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_callers(qualified_name)
    }

    fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_callees(qualified_name)
    }

    fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_dependencies(qualified_name)
    }

    fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.generic.get_dependents(qualified_name)
    }

    fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        self.generic.context_builder(8192)
    }

    fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.generic.search_all(query)
    }
}