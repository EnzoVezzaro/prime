//! Prime MCP Server
//!
//! Exposes Prime's knowledge graph as an MCP server with 7 semantic tools.
//! Supports stdio transport for agent integration.

use prime_core::KnowledgeGraph;
use prime_index::tools::ToolExecutor;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock, Implementation, ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerInfo},
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ─── Tool Parameters ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeSearchParams {
    /// Search query
    query: String,
    /// Maximum results to return
    #[serde(default = "default_limit")]
    limit: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeLookupParams {
    /// Entity qualified name
    qualified_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeContextParams {
    /// Entity qualified name
    qualified_name: String,
    /// Depth for transitive relationships
    #[serde(default = "default_depth")]
    depth: usize,
    /// Token budget for the response
    #[serde(default = "default_token_budget")]
    token_budget: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeRelationshipsParams {
    /// Entity qualified name
    qualified_name: String,
    /// Dimensions to include: dependencies, callers, callees, dependents
    #[serde(default)]
    dimensions: Vec<String>,
    /// Scope: direct, transitive, all
    #[serde(default = "default_scope")]
    scope: String,
    /// Maximum results
    #[serde(default = "default_limit")]
    limit: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeDependenciesParams {
    /// Entity qualified name
    qualified_name: String,
    /// Scope: direct, transitive
    #[serde(default = "default_scope")]
    scope: String,
    /// Maximum depth
    #[serde(default = "default_depth")]
    depth: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeImpactParams {
    /// Entity qualified name
    qualified_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PrimeArchitectureParams {
    /// Subsystem or entity name (empty for project overview)
    #[serde(default)]
    name: String,
}

fn default_limit() -> usize { 20 }
fn default_depth() -> usize { 1 }
fn default_token_budget() -> usize { 8192 }
fn default_scope() -> String { "direct".to_string() }

// ─── MCP Server ─────────────────────────────────────────────────────────────────

/// The Prime MCP Server
#[derive(Clone)]
pub struct PrimeMcpServer {
    executor: Arc<ToolExecutor>,
    tool_router: ToolRouter<PrimeMcpServer>,
}

impl PrimeMcpServer {
    pub fn new(graph: KnowledgeGraph) -> Self {
        Self {
            executor: Arc::new(ToolExecutor::from_graph(graph)),
            tool_router: Self::tool_router(),
        }
    }

    /// Run the MCP server on stdio transport
    pub async fn serve_stdio(self) -> anyhow::Result<()> {
        let service = self.serve(stdio()).await?;
        service.waiting().await?;
        Ok(())
    }

    fn execute_tool(
        &self,
        intent: prime_core::ToolIntent,
        target: Option<String>,
        params: prime_core::ToolRequest,
    ) -> Result<CallToolResult, McpError> {
        let mut request = params;
        request.intent = intent;
        request.target = target;

        let json_value = self.executor.execute(&request);
        let text = serde_json::to_string_pretty(&json_value)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
    }
}

// ─── Tool Definitions ───────────────────────────────────────────────────────────

#[tool_router]
impl PrimeMcpServer {
    /// Search for entities (functions, classes, modules) by keyword.
    #[tool(description = "Search for entities (functions, classes, modules) in the codebase by keyword. Returns matching entities with their types, locations, and confidence scores.")]
    async fn prime_search(
        &self,
        Parameters(params): Parameters<PrimeSearchParams>,
    ) -> Result<CallToolResult, McpError> {
        self.execute_tool(
            prime_core::ToolIntent::Search,
            Some(params.query),
            prime_core::ToolRequest {
                limit: params.limit,
                ..Default::default()
            },
        )
    }

    /// Look up a specific entity by its fully-qualified name.
    #[tool(description = "Look up a specific entity by its fully-qualified name (e.g., 'pkg.module.Class.method'). Returns detailed information including signature, documentation, and relationships.")]
    async fn prime_lookup(
        &self,
        Parameters(params): Parameters<PrimeLookupParams>,
    ) -> Result<CallToolResult, McpError> {
        self.execute_tool(
            prime_core::ToolIntent::Lookup,
            Some(params.qualified_name),
            prime_core::ToolRequest::default(),
        )
    }

    /// Get the knowledge neighborhood for an entity.
    #[tool(description = "Get the knowledge neighborhood for an entity: its direct dependencies, callers, callees, and dependents. Returns a compact context suitable for understanding the entity's role.")]
    async fn prime_context(
        &self,
        Parameters(params): Parameters<PrimeContextParams>,
    ) -> Result<CallToolResult, McpError> {
        self.execute_tool(
            prime_core::ToolIntent::Context,
            Some(params.qualified_name),
            prime_core::ToolRequest {
                depth: params.depth,
                token_budget: params.token_budget,
                ..Default::default()
            },
        )
    }

    /// Get relationships for an entity across specified dimensions.
    #[tool(description = "Get relationships for an entity across specified dimensions: dependencies, callers, callees, or dependents. Supports direct or transitive scope.")]
    async fn prime_relationships(
        &self,
        Parameters(params): Parameters<PrimeRelationshipsParams>,
    ) -> Result<CallToolResult, McpError> {
        let scope = match params.scope.as_str() {
            "transitive" => prime_core::RelationScope::Transitive,
            "all" => prime_core::RelationScope::All,
            _ => prime_core::RelationScope::Direct,
        };
        self.execute_tool(
            prime_core::ToolIntent::Relationships,
            Some(params.qualified_name),
            prime_core::ToolRequest {
                dimensions: params.dimensions,
                scope,
                limit: params.limit,
                ..Default::default()
            },
        )
    }

    /// Get the dependency graph for an entity.
    #[tool(description = "Get the dependency graph for an entity. Shows what this entity depends on, with optional transitive resolution.")]
    async fn prime_dependencies(
        &self,
        Parameters(params): Parameters<PrimeDependenciesParams>,
    ) -> Result<CallToolResult, McpError> {
        let scope = match params.scope.as_str() {
            "transitive" => prime_core::RelationScope::Transitive,
            "all" => prime_core::RelationScope::All,
            _ => prime_core::RelationScope::Direct,
        };
        self.execute_tool(
            prime_core::ToolIntent::Dependencies,
            Some(params.qualified_name),
            prime_core::ToolRequest {
                scope,
                depth: params.depth,
                ..Default::default()
            },
        )
    }

    /// Analyze the impact of changing an entity.
    #[tool(description = "Analyze the impact of changing an entity. Returns directly affected entities and risk assessment.")]
    async fn prime_impact(
        &self,
        Parameters(params): Parameters<PrimeImpactParams>,
    ) -> Result<CallToolResult, McpError> {
        self.execute_tool(
            prime_core::ToolIntent::Impact,
            Some(params.qualified_name),
            prime_core::ToolRequest::default(),
        )
    }

    /// Get architecture overview for a subsystem or the whole project.
    #[tool(description = "Get architecture overview for a subsystem or the whole project. Returns modules, key entities, and dependency structure.")]
    async fn prime_architecture(
        &self,
        Parameters(params): Parameters<PrimeArchitectureParams>,
    ) -> Result<CallToolResult, McpError> {
        self.execute_tool(
            prime_core::ToolIntent::Architecture,
            Some(params.name),
            prime_core::ToolRequest::default(),
        )
    }
}

// ─── ServerHandler ──────────────────────────────────────────────────────────────

#[tool_handler]
impl ServerHandler for PrimeMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_protocol_version(ProtocolVersion::V_2026_07_28)
        .with_instructions(
            "Prime MCP Server — provides codebase knowledge graph queries. \
             7 semantic tools for exploring codebases: search, lookup, context, \
             relationships, dependencies, impact, and architecture."
                .to_string(),
        )
    }
}
