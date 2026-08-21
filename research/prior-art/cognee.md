# Cognee — Knowledge Lifecycle & Retrieval Architecture

**Repository:** https://github.com/topoteretes/cognee  
**Documentation:** https://docs.cognee.ai/  
**Confidence:** OBSERVATION (based on documentation review)

## Overview

Cognee is not a direct architectural template for Prime. It is a knowledge-engineering system that explores:

```
ingestion
    ↓
knowledge construction
    ↓
enrichment
    ↓
retrieval
    ↓
feedback
    ↓
improvement
```

## Key Concepts

### Core Lifecycle

```
remember() → cognify() → improve() → recall()
```

For Prime, this maps to:

```
derive() → normalize() → enrich() → minimize() → index() → retrieve()
```

### DataPoints

Cognee's DataPoints are atomic knowledge units. Prime's equivalent is the Entity + Relationship pair.

### Tasks

Cognee Tasks are analysis passes. Prime's equivalent is the derivation pipeline stages.

### Pipelines

Cognee Pipelines orchestrate knowledge construction. Prime's equivalent is the derivation pipeline.

### Provenance Store

Cognee maintains provenance for all derived facts. Prime should maintain compact evidence/provenance.

### Graph Store

Cognee uses a graph store for structural relationships. Prime should use a relationship layer.

### Vector Store

Cognee uses a vector store for semantic similarity. Prime should use a semantic fallback/index.

### Auto-Routing

Cognee's `recall()` can automatically select a retrieval strategy based on the query. This is critical for Prime.

### Feedback

Cognee supports feedback and self-improvement mechanisms. Prime should use benchmark/agent feedback to improve representation.

## Three-Store Architecture

Cognee separates:

```
relational → provenance / metadata
vector → semantic similarity
graph → structural relationships
```

For Prime, this could become:

```
SYMBOL INDEX → entity lookup
RELATION INDEX → structural relationships
SEMANTIC INDEX → semantic fallback
```

## Query Routing

Cognee can route queries toward different retrieval strategies. For Prime:

```
"What calls AuthService.login?"
    → RELATION INDEX

"What is AuthService?"
    → SYMBOL INDEX

"How is authentication implemented?"
    → SEMANTIC INDEX

"What breaks if login changes?"
    → IMPACT INDEX

"Where should I modify this?"
    → STRUCTURAL + IMPACT
```

## Enrichment

Cognee's pipeline:

```
raw data → ingestion → graph construction → embeddings → enrichment → retrieval
```

For Prime, this could be multi-pass deterministic analysis:

```
SOURCE
  │
  ▼
PASS 0 — filesystem
  │
  ▼
PASS 1 — syntax
  │
  ▼
PASS 2 — symbols
  │
  ▼
PASS 3 — references
  │
  ▼
PASS 4 — relationships
  │
  ▼
PASS 5 — contracts
  │
  ▼
PASS 6 — architecture
  │
  ▼
PASS 7 — impact
  │
  ▼
MINIMIZATION
  │
  ▼
PRIME
```

**Critical distinction:** Prime's enrichment should primarily be deterministic/static analysis, not LLM-generated knowledge. For code, we have:

```
AST, LSP, compiler, type checker, symbol resolver,
call graph, imports, exports, tests, build system,
Git history, configuration
```

## Hypothesis: Prime as Knowledge Compiler

> Prime might be a knowledge compiler, not a knowledge database.

```
CODEBASE
    │
    ▼
SEMANTIC COMPILER
    │
    ▼
OPTIMIZATION
    │
    ▼
PRIME
```

Then the runtime is tiny:

```
PRIME → QUESTION → QUERY ROUTER → compact indexes → ANSWER
```

## What to Steal from Cognee

| Cognee idea | Prime adaptation | Keep? |
|-------------|-----------------|-------|
| `remember → recall → improve` | `derive → retrieve → refine` | **Yes** |
| DataPoints | Prime knowledge units | **Yes** |
| Tasks | analysis passes | **Yes** |
| Pipelines | derivation pipeline | **Yes** |
| Provenance store | compact evidence/provenance | **Yes** |
| Graph store | relationship layer | **Maybe** |
| Vector store | semantic fallback/index | **Maybe** |
| LLM entity extraction | code-aware semantic extraction | **Probably not by default** |
| Auto-routing | question → optimal Prime index | **Absolutely** |
| Feedback | benchmark/agent feedback → improve representation | **Yes** |
| Sessions | agent interaction layer | **Maybe, but outside core Prime** |
| Three independent databases | likely overkill for Prime | **No** |

## Critical Distinction

Cognee asks: *How do we turn information into useful AI memory?*

Prime asks: *How do we compile a software system into the minimum semantic information an agent needs?*

They overlap heavily, but Prime's opportunity is to take the knowledge lifecycle ideas from Cognee and combine them with the deterministic semantic machinery of VS Code/LSP/SCIP/Tree-sitter/compiler tooling, then optimize the resulting artifact like a systems/data-structure problem.

## Open Questions

1. Does Prime need all three stores (relational, vector, graph)?
2. Can query routing reduce bytes retrieved, latency, and tokens?
3. Can real agent usage tell Prime which derived facts are worth materializing?
4. Is demand-driven derivation better than exhaustive derivation?
5. Can deterministic analysis replace LLM extraction for code knowledge?

## References

- [Cognee Architecture](https://docs.cognee.ai/core-concepts/architecture)
- [Cognee Recall](https://docs.cognee.ai/core-concepts/main-operations/recall)
- [Cognee Remember](https://docs.cognee.ai/core-concepts/main-operations/remember)
- [How Cognee Builds AI Memory](https://www.cognee.ai/blog/fundamentals/how-cognee-builds-ai-memory)
