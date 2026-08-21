---
title: What is Prime?
---

# What is Prime?

Prime is a derived codebase knowledge artifact designed for agents.

```
source
    │
    ▼
derivation
    │
    ▼
artifact
    │
    ▼
retrieval
```

Prime transforms a codebase into a single universal knowledge artifact that an agent can read, navigate, and reason about.

## What Prime Is

- A **derived representation** of codebase knowledge
- **Language agnostic** — works across TypeScript, Python, Rust, Go, Java, C++, and more
- **Compact** — minimal useful representation, not a compressed copy
- **Indexed** — fast semantic retrieval for agent queries
- **Typed** — entities, relations, contracts with provenance and confidence
- **Agent-native** — semantic tools (`prime_search`, `prime_context`, `prime_dependencies`, ...)
- **MCP native** — exposed via Model Context Protocol (stdio, Streamable HTTP)

## What Prime Is Not

| Not | Why |
|-----|-----|
| A database | May use any storage; storage is not the product |
| A compiler | Does not transform or execute code |
| Source compression | Removes what agents don't need; preserves what they do |
| An LLM | No model weights; structured knowledge for agents to consume |
| A documentation generator | Produces machine-readable knowledge, not human docs |
| A replacement for source | Source remains authoritative; Prime is the fast path |

## The Core Distinction

> **Prime does not simply compress source code.**
>
> **Prime derives the smallest useful representation of codebase knowledge that allows an agent to answer questions without retrieving the underlying source whenever possible.**

This distinction is fundamental. Traditional approaches compress syntax. Prime distills semantics.

## The Agent's Problem

Today's agents repeatedly:
- Search files
- Read source
- Follow imports
- Parse declarations
- Inspect references
- Infer relationships
- Reconstruct context

For every question. Every session. Every agent.

Prime changes this: **derive once, answer many.**

## The Prime Thesis

```
SOURCE CODE
    │
    │ analyze (Tree-sitter, language servers, static analysis)
    ▼
DERIVED KNOWLEDGE
    │
    │ minimize (remove redundancy, preserve utility)
    ▼
PRIME ARTIFACT
    │
    │ query (semantic tools, MCP)
    ▼
AGENT
```

The source remains the authority. Prime is the derived fast path.

## Next Steps

- [The Problem](./the-problem.md) — Why current approaches fall short
- [Principles](./principles.md) — Research and design principles
- [Roadmap](./roadmap.md) — Where Prime is heading
- [Research: Agent Architecture](../research/agent-architecture.md)