# Agent-Native Interface — Implementation Tracking

**Status:** Phases 1–5 completed  
**Spec:** `docs/AGENT-NATIVE-INTERFACE.md` (full architecture)  
**Created:** 2026-08-19

---

## Implementation Phases

### Phase 1: Response Envelope — prime-core
**Status:** ✅ COMPLETED  
**Date:** 2026-08-19

Added to `prime-core/src/agent.rs`:

- [x] `ResponseStatus` enum: `complete`, `partial`, `unknown`, `unsupported`, `error`
- [x] `AgentConfidence` enum: `exact`, `derived`, `inferred`, `unknown`
- [x] `PrimeEnvelope<T>` struct with status, coverage, source_required, missing, provenance, warnings, telemetry
- [x] `TelemetryData` struct (tool, target, status, latency_ms, bytes, source_required, timestamp)
- [x] `AgentProvenance` struct
- [x] Helper methods: `complete()`, `partial()`, `unknown()`, `error()`, `with_telemetry()`, `is_sufficient()`, `envelope_tokens()`
- [x] `serde_json` re-exported from prime-core

### Phase 2: Semantic Tool Operations — prime-index
**Status:** ✅ COMPLETED  
**Date:** 2026-08-19

Added to `prime-index/src/tools.rs`:

- [x] `ToolExecutor` struct wrapping `QueryEngine`
- [x] `prime_search(query, limit) -> PrimeEnvelope<Vec<EntityDetail>>`
- [x] `prime_lookup(qualified_name) -> PrimeEnvelope<Option<EntityDetail>>`
- [x] `prime_context(qualified_name, depth, token_budget) -> PrimeEnvelope<Option<ContextResult>>`
- [x] `prime_relationships(qualified_name, dimensions, scope, limit) -> PrimeEnvelope<Vec<EntityDetail>>`
- [x] `prime_dependencies(qualified_name, scope, depth) -> PrimeEnvelope<Option<DependencyResult>>`
- [x] `prime_impact(qualified_name) -> PrimeEnvelope<Option<ImpactResult>>`
- [x] `prime_architecture(qualified_name) -> PrimeEnvelope<Option<ArchitectureResult>>`
- [x] Coverage calculation per operation
- [x] Source escalation detection (coverage < 0.8 → source_required = true)
- [x] Tool dispatch via `execute(ToolRequest) -> serde_json::Value`

### Phase 3: MCP Server — prime-mcp
**Status:** ✅ COMPLETED  
**Date:** 2026-08-19

Created `prime-mcp/` crate:

- [x] `prime-mcp/Cargo.toml` with rmcp 3.1, tokio, schemars
- [x] `PrimeMcpServer` struct implementing `ServerHandler`
- [x] 7 MCP tool definitions using `#[tool]` macro
- [x] JSON Schema via `#[derive(schemars::JsonSchema)]` on params
- [x] Stdio transport via `serve(stdio())`
- [x] Tool dispatch: MCP tool name → `ToolRequest` → `ToolExecutor.execute()` → JSON response
- [x] Server metadata: name, version, capabilities, instructions, protocol version

### Phase 4: CLI Updates — prime-cli
**Status:** ✅ COMPLETED  
**Date:** 2026-08-19

Added to `prime-cli/src/main.rs`:

- [x] `prime serve` — starts MCP server on stdio
- [x] `prime inspect <entity>` — shows entity context with agent envelope
- [x] `prime benchmark` — runs quick performance benchmark
- [x] Dependencies: prime-mcp, prime-query, tokio

### Phase 5: Agent Skill + AGENTS.md
**Status:** ✅ COMPLETED  
**Date:** 2026-08-19

- [x] Updated `.agents/skills/prime/SKILL.md` with MCP tool catalog, envelope docs, confidence levels
- [x] Updated `AGENTS.md` with architecture table, MCP tool list, quick start with serve/inspect/benchmark

---

### Phase 6: Telemetry + Context Handles
**Status:** PENDING  
**Priority:** LOW

#### Deliverables

- [ ] Optional telemetry recording in MCP server
- [ ] `ContextHandle` type for multi-step retrieval
- [ ] `prime_expand(handle, dimension)` operation

---

## Design Decisions

### D1: Tool Surface Size
**Decision:** 7 semantic tools (search, lookup, context, relationships, dependencies, impact, architecture).  
**Rationale:** §15 — tool count is an optimization variable. Start with semantic operations, measure, then consider consolidation.

### D2: MCP Transport
**Decision:** stdio (implemented), Streamable HTTP (planned).  
**Rationale:** §10-11 — stdio for local, HTTP for remote/hosted. SSE is legacy.

### D3: Stateless Requests
**Decision:** All Prime requests are stateless. Context handles are application-level.  
**Rationale:** §8-9 — no hidden protocol sessions. MCP 2026-07-28 supports this.

### D4: Response Envelope
**Decision:** Every tool result uses `PrimeEnvelope<T>`.  
**Rationale:** §19 — consistent status, coverage, provenance across all operations.

### D5: Confidence Mapping
**Decision:** Internal `Confidence` → `AgentConfidence` (exact, derived, inferred, unknown).  
**Rationale:** §44 — agents need to distinguish exact vs inferred knowledge.

---

## Open Questions

1. **Q1:** Should `prime_query` be a single unified tool or should we keep 7 separate tools? (§16)
2. **Q2:** What is the optimal default token budget for `prime_context`? (§21)
3. **Q3:** Should the MCP server auto-build the artifact on first request? (§32)
4. **Q4:** How should we handle artifact version mismatches? (§27)
5. **Q5:** Should telemetry be opt-in or opt-out by default? (§50)
