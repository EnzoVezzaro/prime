---
title: Agent Architecture
---

# Agent Architecture Research

## Why It Matters

Prime is designed specifically around the architecture of modern and emerging coding agents. The agent is not a passive reader — it operates in a loop:

```
observe
    ↓
reason
    ↓
retrieve
    ↓
reason
    ↓
act
    ↓
observe
    ↓
...
```

Prime must optimize for the **information flow through that loop**.

## Key Agent Concepts

| Concept | Relevance to Prime |
|---------|-------------------|
| Model context | Prime must fit in context windows |
| Attention | Prime must surface what matters |
| Context windows | Prime must be smaller than source |
| Context caching | Prime should enable caching |
| Tool use | Prime exposes semantic tools |
| Agent memory | Prime integrates with `.acc-memory.md` |
| Retrieval loops | Prime minimizes loop iterations |
| Planning | Prime provides architectural context |
| Tool schemas | Prime returns structured results |
| Progressive disclosure | Prime supports depth/budget params |
| External memory | Prime is external memory |
| Failure modes | Prime handles uncertainty |
| Information overload | Prime filters to useful |

## What Agents Actually Do

### Current Agent Workflow (Without Prime)

```
User: "How does AuthService.login work?"
Agent:
  1. Search for "AuthService"
  2. Find login.ts
  3. Read login.ts
  4. Search for UserRepository
  5. Read UserRepository
  6. Search for PasswordVerifier
  7. Read PasswordVerifier
  8. Search for SessionStore
  9. Read SessionStore
  10. Synthesize answer
  → 9 tool calls, ~50KB source read
```

### With Prime

```
User: "How does AuthService.login work?"
Agent:
  1. prime_context("AuthService.login")
  2. Receives: calls, returns, throws, impact, confidence
  3. Synthesizes answer
  → 1 tool call, ~2KB structured result
```

## Agent Tool Requirements

| Tool | Purpose | Structured Output |
|------|---------|-------------------|
| `prime_search` | Find entities by keyword | Entity list with scores |
| `prime_lookup` | Exact entity by qualified name | Full entity detail |
| `prime_context` | Neighborhood (deps, callers, callees) | Context graph |
| `prime_relationships` | Cross-dimension relations | Relation list |
| `prime_dependencies` | Dependency graph | Graph subset |
| `prime_impact` | Change impact analysis | Affected set |
| `prime_architecture` | System overview | Module graph |

## Open Questions

- What is the optimal token budget per tool call?
- How should Prime handle streaming/progressive context?
- Should Prime expose streaming results for long retrievals?
- How do agents compose multiple Prime tool calls?
- What context formats do agents prefer (JSON, MessagePack, custom)?

## References

- [Aider repo map](https://aider.chat/2023/10/22/repomap.html) — compact structural maps for LLMs
- [Sourcegraph Cody](https://sourcegraph.com/cody) — code intelligence for agents
- [Cursor](https://cursor.sh/) — agent-native IDE
- [LangChain retrieval](https://python.langchain.com/docs/modules/data_connection/) — retrieval patterns
- [AutoGPT](https://github.com/Significant-Gravitas/AutoGPT) — agent loops