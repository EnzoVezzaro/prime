//! Generic Agent Adapter - Works with any agent that follows the skill protocol

use crate::core::provider::{AgentQuery, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder, AgentQuery};
use crate::adapters::{AgentAdapter, AdapterConfig, AgentResponse, ResponseFormat, ResponseMetadata, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder, AgentQuery, SymbolInfo, Context, RelationInfo, ProgressiveContextBuilder};
use std::collections::HashMap;
use anyhow::Result;
use std::sync::Arc;

/// Generic adapter (works with any agent that follows the skill protocol)
pub struct GenericAdapter {
    agent_query: crate::core::provider::AgentQuery,
    config: AdapterConfig,
}

impl GenericAdapter {
    pub fn new(agent_query: crate::core::provider::AgentQuery, config: AdapterConfig) -> Self {
        Self { agent_query, config }
    }
}

impl AgentAdapter for GenericAdapter {
    fn name(&self) -> &'static str {
        "generic"
    }

    fn initialize(&mut self, config: AdapterConfig) -> anyhow::Result<()> {
        self.config = config;
        Ok(())
    }

    fn query(&self, query: &str) -> anyhow::Result<AgentResponse> {
        let results = self.agent_query.search(query);
        Ok(AgentResponse {
            content: format!("Found {} results", results.len()),
            format: ResponseFormat::JSON,
            metadata: ResponseMetadata::default(),
        })
    }

    fn find_symbol(&self, qualified_name: &str) -> anyhow::Result<Option<SymbolInfo>> {
        Ok(self.agent_query.find_symbol(qualified_name))
    }

    fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.agent_query.find_symbols(name)
    }

    fn get_context(&self, qualified_name: &str) -> Option<Context> {
        self.agent_query.get_context(qualified_name)
    }

    fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_surrounding(qualified_name)
    }

    fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_callers(qualified_name)
    }

    fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_callees(qualified_name)
    }

    fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_dependencies(qualified_name)
    }

    fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_dependents(qualified_name)
    }

    fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        self.agent_query.context_builder(token_budget)
    }

    fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.agent_query.search_all(query)
    }
}

/// Generic adapter (works with any agent that follows the skill protocol)
pub struct GenericAdapter {
    agent_query: crate::core::provider::AgentQuery,
    config: AdapterConfig,
}

impl GenericAdapter {
    pub fn new(agent_query: crate::core::provider::AgentQuery, config: AdapterConfig) -> Self {
        Self { agent_query, config }
    }
}

impl AgentAdapter for GenericAdapter {
    fn name(&self) -> &'static str {
        "generic"
    }

    fn initialize(&mut self, config: AdapterConfig) -> anyhow::Result<()> {
        self.config = config;
        Ok(())
    }

    fn query(&self, query: &str) -> anyhow::Result<AgentResponse> {
        let results = self.agent_query.search(query);
        Ok(AgentResponse {
            content: format!("Found {} results", results.len()),
            format: ResponseFormat::JSON,
            metadata: ResponseMetadata::default(),
        })
    }

    fn find_symbol(&self, qualified_name: &str) -> anyhow::Result<Option<SymbolInfo>> {
        Ok(self.agent_query.find_symbol(qualified_name))
    }

    fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.agent_query.find_symbols(name)
    }

    fn get_context(&self, qualified_name: &str) -> Option<Context> {
        self.agent_query.get_context(qualified_name)
    }

    fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_surrounding(qualified_name)
    }

    fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_callers(qualified_name)
    }

    fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_callees(qualified_name)
    }

    fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_dependencies(qualified_name)
    }

    fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_dependents(qualified_name)
    }

    fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        self.agent_query.context_builder(token_budget)
    }

    fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.agent_query.search_all(query)
    }
}

/// Generic adapter (works with any agent that follows the skill protocol)
pub struct GenericAdapter {
    agent_query: crate::core::provider::AgentQuery,
    config: AdapterConfig,
}

impl GenericAdapter {
    pub fn new(agent_query: crate::core::provider::AgentQuery, config: AdapterConfig) -> Self {
        Self { agent_query, config }
    }
}

impl AgentAdapter for GenericAdapter {
    fn name(&self) -> &'static str {
        "generic"
    }

    fn initialize(&mut self, config: AdapterConfig) -> anyhow::Result<()> {
        self.config = config;
        Ok(())
    }

    fn query(&self, query: &str) -> anyhow::Result<AgentResponse> {
        let results = self.agent_query.search(query);
        Ok(AgentResponse {
            content: format!("Found {} results", results.len()),
            format: ResponseFormat::JSON,
            metadata: ResponseMetadata::default(),
        })
    }

    fn find_symbol(&self, qualified_name: &str) -> anyhow::Result<Option<SymbolInfo>> {
        Ok(self.agent_query.find_symbol(qualified_name))
    }

    fn find_symbols(&self, name: &str) -> Vec<SymbolInfo> {
        self.agent_query.find_symbols(name)
    }

    fn get_context(&self, qualified_name: &str) -> Option<Context> {
        self.agent_query.get_context(qualified_name)
    }

    fn get_surrounding(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_surrounding(qualified_name)
    }

    fn get_callers(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_callers(qualified_name)
    }

    fn get_callees(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_callees(qualified_name)
    }

    fn get_dependencies(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_dependencies(qualified_name)
    }

    fn get_dependents(&self, qualified_name: &str) -> Vec<SymbolInfo> {
        self.agent_query.get_dependents(qualified_name)
    }

    fn context_builder(&self, token_budget: usize) -> ProgressiveContextBuilder {
        self.agent_query.context_builder(token_budget)
    }

    fn search_all(&self, query: &str) -> Vec<SymbolInfo> {
        self.agent_query.search_all(query)
    }
}