---
title: MCP
---

# Model Context Protocol

MCP is the interface layer between agents and Prime. Prime itself remains independent from MCP.

## Architecture

```
LOCAL                          REMOTE                          HOSTED
Agent                          Agent                          Agent platform
  ↓                               ↓                               ↓
stdio                      Streamable HTTP                      MCP
  ↓                               ↓                               ↓
Prime                          Prime                          Prime
```

## Transport

| Transport | Use Case | Status |
|-----------|----------|--------|
| stdio | Local development, CLI agents | ✅ Implemented |
| Streamable HTTP | Remote servers, multi-agent | 📋 Planned |
| WebSocket | Real-time subscriptions | 📋 Planned |

## Prime MCP Server

The `prime-mcp` crate implements an MCP server exposing the 7 semantic tools.

### Tool Discovery

```json
{
  "tools": [
    { "name": "prime_search", "description": "Search entities by keyword" },
    { "name": "prime_lookup", "description": "Look up entity by qualified name" },
    { "name": "prime_context", "description": "Get knowledge neighborhood" },
    { "name": "prime_relationships", "description": "Get relationships across dimensions" },
    { "name": "prime_dependencies", "description": "Get dependency graph" },
    { "name": "prime_impact", "description": "Analyze impact of changes" },
    { "name": "prime_architecture", "description": "Get architecture overview" }
  ]
}
```

### Structured Results

All tools return `PrimeEnvelope<T>` with:
- `status`: complete | partial | error | not_found
- `coverage`: full | partial | unknown
- `source_required`: boolean
- `provenance`: confidence + evidence
- `warnings`: non-fatal issues
- `result`: typed payload

## Why Stateless MCP

Prime's MCP implementation is stateless:

```
Agent
  ↓ request (self-contained)
Prime
  ↓ response (complete)
```

Benefits:
- Horizontal scaling
- Simple deployment
- Caching at edge
- Serverless compatible
- Multiple agents
- Resilient to restarts

No application-level session state unless deliberately introduced.

## Authentication

MCP itself doesn't mandate auth. Prime delegates to transport layer:
- stdio: OS permissions
- HTTP: Bearer tokens, mTLS, OAuth
- Hosted: Platform auth

## Caching

Response caching via HTTP headers:
- `Cache-Control: max-age=60, stale-while-revalidate=300`
- ETags for conditional requests
- Cache invalidation on `prime build`

## Routing

Multi-tenant via path prefix:
```
POST /prime/{repository}/tools/call
```

## Open Questions

- Should Prime support MCP resources (read-only data)?
- How to handle large artifact streaming?
- MCP sampling integration for agent-driven derivation?
- Tool composition (agent calls multiple tools in one request)?

## Next

- [Stateless MCP](./stateless-mcp.md)
- [Structured Results](./structured-results.md)
- [Retrieval](./retrieval.md)