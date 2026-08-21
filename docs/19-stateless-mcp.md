---
title: Stateless MCP
---

# Stateless MCP

Prime's MCP implementation is deliberately stateless.

## Why Stateless?

```
Agent
  ↓ request (contains all context)
Prime
  ↓ response (self-contained)
```

Each request contains the information required to process it.

## Benefits

| Benefit | Explanation |
|---------|-------------|
| Horizontal scaling | Any instance can handle any request |
| Simple deployment | No sticky sessions, no session store |
| Caching | Responses cacheable at CDN/edge |
| Routing | Any load balancer works |
| Resilience | Instance failure = no session loss |
| Serverless | Works on Cloudflare Workers, Lambda, etc. |
| Multi-agent | No session conflicts between agents |
| Testing | Deterministic request/response |

## Request Context

Requests include:
```json
{
  "tool": "prime_context",
  "arguments": {
    "target": "AuthService.login",
    "depth": 2,
    "token_budget": 16384
  },
  "context": {
    "repository": "my-project",
    "artifact_version": "v0.3.1",
    "agent_id": "agent-123"
  }
}
```

No server-side session needed.

## When State Might Be Added

Deliberate application-level state only for:
- **Context handles** — multi-step retrieval (return handle, subsequent calls use handle)
- **Streaming** — chunked responses for large results
- **Subscriptions** — real-time updates on artifact changes

Each introduced explicitly with clear semantics.

## What Stays Stateless

- Tool execution
- Artifact loading (mmap is OS-managed)
- Query processing
- Result assembly

## Next

- [MCP](./mcp.md)
- [Structured Results](./structured-results.md)
- [Retrieval](./retrieval.md)