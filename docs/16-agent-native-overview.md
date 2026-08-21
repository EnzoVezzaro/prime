---
title: Agent-Native Interface
---

# Agent-Native Interface

Prime exposes a semantic interface optimized for agent reasoning — not low-level graph primitives.

## Core Philosophy

**Bad (graph primitives):**
```
get_node(id)
get_edge(from, to)
scan_graph(pattern)
read_block(offset, len)
```

**Better (semantic tools):**
```
prime_search("AuthService")
prime_context("AuthService.login")
prime_dependencies("AuthService")
prime_impact("AuthService.login")
prime_architecture()
prime_relationships("User")
prime_lookup("AuthService.login")
```

## Why Semantic Tools?

| Graph Primitives | Semantic Tools |
|-----------------|----------------|
| Agent must know graph structure | Agent asks domain questions |
| Agent composes multiple calls | Single call returns useful context |
| No provenance | Every result tagged with confidence |
| No source escalation | `source_required` flag in envelope |
| Raw data | Structured, typed results |
| Agent does traversal logic | Prime does traversal, agent reasons |

## The Envelope

Every tool returns a `PrimeEnvelope<T>`:

```json
{
  "status": "complete",
  "coverage": "full",
  "source_required": false,
  "provenance": { "kind": "exact", "source": "static analysis" },
  "warnings": [],
  "result": { ... }
}
```

Fields:
- `status`: `complete` | `partial` | `error` | `not_found`
- `coverage`: `full` | `partial` | `unknown`
- `source_required`: `true` if agent should fetch source
- `provenance`: Confidence and evidence
- `warnings`: Non-fatal issues
- `result`: Tool-specific structured data

## Confidence Mapping

| Level | Meaning | When Used |
|-------|---------|-----------|
| `exact` | Verified in source | Static symbol resolution |
| `derived` | Computed from exact facts | Transitive closure, type inference |
| `inferred` | Probabilistic/heuristic | Dynamic dispatch, reflection |
| `unknown` | No evidence | Unanalyzed code, external deps |

## Next

- [Agent Tools](./agent-tools.md)
- [MCP](./mcp.md)
- [Structured Results](./structured-results.md)
- [Retrieval](./retrieval.md)
- [Source Escalation](./source-escalation.md)