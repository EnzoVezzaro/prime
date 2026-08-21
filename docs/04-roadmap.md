---
title: Roadmap
---

# Roadmap

Prime is research-stage. No final format, graph model, storage design, retrieval API, or compression algorithm exists yet. Those decisions are intentionally postponed until research justifies them.

## Research Phases

```text
PHASE 1
Understand next-generation agents
            │
            ▼
PHASE 2
Understand codebase information
            │
            ▼
PHASE 3
Study existing representations
            │
            ▼
PHASE 4
Discover the minimum useful knowledge unit
            │
            ▼
PHASE 5
Research representation and compression
            │
            ▼
PHASE 6
Research retrieval and agent interaction
            │
            ▼
PHASE 7
Research scale and language agnosticism
            │
            ▼
PHASE 8
Build isolated experiments
            │
            ▼
PHASE 9
Benchmark agent tasks
            │
            ▼
PHASE 10
Synthesize findings
            │
            ▼
PHASE 11
Write the Prime specification
            │
            ▼
PHASE 12
Configure ACC integration
            │
            ▼
PHASE 13
Build Prime
```

**Implementation is the final phase, not the first.**

## Current Status

### ✅ Completed (Research Infrastructure)
- Research specification structure (68 markdown files across 13 directories)
- ACC skill configuration and integration
- Rust workspace with 7 crates
- Tree-sitter based multi-language parser (8 languages)
- Symbol and relationship extraction
- Compact binary storage format with compression (zstd, lz4)
- mmap-based zero-copy access
- Agent-optimized query API
- CLI with build, query, stats, check, deps, calls, export, serve, inspect, benchmark
- Benchmarks for parsing, indexing, queries, storage, incremental updates
- Agent-Native Interface: `PrimeEnvelope<T>` response envelope
- Semantic tool operations: 7 tools
- MCP server crate (prime-mcp) with rmcp 3.1
- Agent confidence mapping (exact, derived, inferred, unknown)

### 🔧 In Progress
- Completing incremental analysis and invalidation
- Adding more language support (C#, Swift, Kotlin, etc.)
- Improving cross-language relationship tracking
- Research documentation synthesis

### 📋 Planned (Post-Research)
- Streamable HTTP transport for remote MCP access
- Context handles for multi-step retrieval
- Telemetry collection and reporting
- Complete incremental analysis and invalidation
- CRDT support for distributed knowledge
- Semantic compression (grammar-based, pattern deduplication)
- Content-addressed storage with Merkle DAGs
- Distributed knowledge sharing
- Cryptographic provenance and integrity proofs

## Research Areas (from init-prompt.md)

| Area | Status |
|------|--------|
| 01. Codebase Knowledge (parsing, symbols, types, relationships) | 🔧 |
| 02. Code Property Graphs (CPG, Joern, AST, CFG, PDG) | 🔧 |
| 03. SCIP (symbol identity, cross-language representation) | 🔧 |
| 04. LSIF (graph representation, persistence, querying) | 🔧 |
| 05. Tree-sitter (parsing architecture, incremental parsing) | 🔧 |
| 06. Agent-Oriented Code Indexing | 🔧 |
| 07. Information Retrieval | 🔧 |
| 08. Storage Systems | 🔧 |
| 09. Binary Format Design | 🔧 |
| 10. Compression | 🔧 |
| 11. Succinct Data Structures | 📋 |
| 12. Memory Mapping and I/O | 🔧 |
| 13. Large-Scale Codebases | 📋 |
| 14. Incremental Analysis | 🔧 |
| 15. Language Agnosticism | 🔧 |
| 16. Agent Context and Token Efficiency | 🔧 |
| 17. Reusable Open-Source Tools | 🔧 |
| 18. Academic Research | 🔧 |
| 19. Benchmark Research | 🔧 |

## Milestones

| Milestone | Criteria |
|-----------|----------|
| M1: Research Synthesis | All 19 areas have evidence-based findings with confidence levels |
| M2: Minimum Knowledge Unit | Defined, validated against agent tasks |
| M3: Representation Spec | Format, schema, encoding specified with rationale |
| M4: Retrieval API Spec | 7 semantic tools specified with contracts |
| M5: Benchmark Suite | Agent task benchmarks running on 5+ repositories |
| M6: Specification v1.0 | Complete, evidence-based, ready for implementation |

## Next

- [Research: Agent Architecture](../research/agent-architecture.md)
- [Research: Codebase Knowledge](../research/codebase-knowledge.md)
- [Benchmark Methodology](../benchmarks/benchmark-methodology.md)