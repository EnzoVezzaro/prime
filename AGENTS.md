# prime

## Purpose

Prime is a research project investigating how software repositories can be transformed into extremely compact, language-agnostic knowledge representations optimized for machine analysis and AI-agent retrieval.

## Responsibilities

- Conduct evidence-based research on codebase knowledge representation
- Investigate prior art (SCIP, LSIF, CPG/Joern, Tree-sitter, agent indexers)
- Analyze compression, storage, indexing, retrieval tradeoffs
- Research language-agnostic semantic models
- Document findings with explicit confidence levels (FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION)

## Ownership

Owner: research team

## Inputs

- Source code repositories (any language, any scale)
- Academic papers and technical specifications
- Open-source tools and libraries (parsers, analyzers, storage engines)

## Outputs

- SPECS/ research specifications (evidence-based, not product)
- docs/ operational documentation
- ACC configuration for agent context awareness

## Dependencies

- agents-code-context (ACC CLI for agent context)
- Tree-sitter parsers (language analysis)
- Language-specific analyzers (rust-analyzer, tsserver, etc.)

## Constraints

- Evidence over assumptions: never claim without evidence
- Primary sources first: official docs, papers, specs, source code
- No premature convergence: don't decide technologies until research establishes why
- Research alternatives fairly: investigate multiple approaches per design area
- No product design: document evidence, alternatives, tradeoffs, hypotheses, open questions
- Distinguish clearly: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION

## Architecture

The Prime research repository has two distinct layers:

1. **SPECS/** — Research specifications: evidence-based findings, hypotheses, open questions, NO product design
2. **docs/** — Operational documentation: getting started, guidelines, contributing, navigation

The Rust implementation (`prime-rs/`) provides the reference implementation:

| Crate | Purpose |
|-------|---------|
| `prime-core` | Core types: Entity, Relation, KnowledgeGraph, PrimeEnvelope, agent types |
| `prime-parser` | Tree-sitter based multi-language parser |
| `prime-index` | Storage (binary, mmap), query engine, semantic tool operations |
| `prime-query` | Agent query API, progressive context building, streaming |
| `prime-mcp` | MCP server with 7 semantic tools (stdio transport) |
| `prime-cli` | CLI: build, query, serve, inspect, benchmark, etc. |
| `prime-bench` | Performance benchmarks |

The MCP server exposes 7 tools via Model Context Protocol:

| Tool | Description |
|------|-------------|
| `prime_search` | Search entities by keyword |
| `prime_lookup` | Look up entity by qualified name |
| `prime_context` | Get knowledge neighborhood (deps, callers, callees) |
| `prime_relationships` | Get relationships across dimensions |
| `prime_dependencies` | Get dependency graph |
| `prime_impact` | Analyze impact of changes |
| `prime_architecture` | Get architecture overview |

Every tool returns a `PrimeEnvelope<T>` with status, coverage, source_required, provenance, and warnings.

The ACC skill is configured to understand this repository structure and provide agent context for research navigation.

## Workflows

- See `.acc/config/workflows/feature.md` for adding a new research area.
- See `.acc/config/workflows/research.md` for conducting research.
- See `.acc/config/workflows/bugfix.md` for fixing tooling bugs.
- See `.acc/config/workflows/refactor.md` for restructuring research areas.
- See `.acc/config/workflows/release.md` for research milestone checklists.
- See `.acc/config/workflows/memory.md` for capturing durable knowledge.
- See `.acc/config/workflows/diagnostic.md` for diagnostic codes.
- See `.acc/config/workflows/security.md` for security-sensitive changes.

## Implementation Status

### ✅ Completed

- SPECS/ research specification structure (68 markdown files across 13 directories)
- ACC skill configuration and integration
- Rust workspace with 7 crates (prime-core, prime-parser, prime-index, prime-query, prime-mcp, prime-cli, prime-bench)
- Tree-sitter based multi-language parser (8 languages: Rust, TypeScript, JavaScript, Python, Go, Java, C, C++)
- Symbol and relationship extraction from source code
- Compact binary storage format with compression (zstd, lz4)
- mmap-based zero-copy access for fast agent retrieval
- Agent-optimized query API with progressive context building
- CLI with build, query, stats, check, deps, dependents, calls, export, serve, inspect, benchmark commands
- Benchmarks for parsing, indexing, queries, storage, incremental updates
- ACC skill fully configured and integrated
- Agent-Native Interface: PrimeEnvelope<T> response envelope with status, coverage, provenance, source_required
- Semantic tool operations: 7 tools (search, lookup, context, relationships, dependencies, impact, architecture)
- MCP server crate (prime-mcp) with rmcp 3.1 (MCP 2026-07-28 spec, stdio transport)
- Agent confidence mapping (exact, derived, inferred, unknown)

### 🔧 In Progress

- Completing incremental analysis and invalidation
- Adding more language support (C#, Swift, Kotlin, etc.)
- Improving cross-language relationship tracking

### 📋 Planned

- Streamable HTTP transport for remote MCP access
- Context handles for multi-step retrieval
- Telemetry collection and reporting
- Complete incremental analysis and invalidation
- Add CRDT support for distributed knowledge
- Implement semantic compression (grammar-based, pattern deduplication)
- Add content-addressed storage with Merkle DAGs
- Complete distributed knowledge sharing
- Add cryptographic provenance and integrity proofs

## Research Areas (per init-promt.md)

01. Codebase Knowledge (parsing, symbols, types, relationships)
02. Code Property Graphs (CPG, Joern, AST, CFG, PDG)
03. SCIP (symbol identity, cross-language representation)
04. LSIF (graph representation, persistence, querying)
05. Tree-sitter (parsing architecture, incremental parsing)
06. Agent-Oriented Code Indexing (Graph-sitter, Sourcegraph, Cursor, etc.)
07. Information Retrieval (inverted indexes, semantic search, hybrid search)
08. Storage Systems (SQLite, RocksDB, LMDB, custom binary, columnar)
09. Binary Format Design (protobuf, FlatBuffers, Cap'n Proto, custom)
10. Compression (integer, graph, string, general)
11. Succinct Data Structures (rank/select, Roaring bitmaps, Elias-Fano)
12. Memory Mapping and I/O (mmap, page cache, SSD/NVMe behavior)
13. Large-Scale Codebases (100K-1M+ files, monorepos)
14. Incremental Analysis (parsing, indexing, invalidation, Merkle trees)
15. Language Agnosticism (universal semantic models, capability model)
16. Agent Context and Token Efficiency (context windows, progressive disclosure)
17. Reusable Open-Source Tools (parsers, analyzers, storage, search)
18. Academic Research (papers on code graphs, compression, indexing)
19. Benchmark Research (existing benchmarks, methodology, datasets)

## Research Principles (from init-promt.md)

1. **Evidence over assumptions** — Never write "X is faster" without evidence. Write: "Benchmark/source X reports...", "Our experiment indicates...", "Theoretical analysis suggests...", "This remains unverified."
2. **Primary sources first** — Prefer official documentation, academic papers, technical specifications, original repositories, engineering papers, benchmark results, source code.
3. **Do not prematurely converge** — Do NOT decide on technologies (SQLite, RocksDB, graph DB, Tree-sitter, protobuf, etc.) until research establishes why.
4. **Research alternatives fairly** — For every major design area, investigate multiple approaches.
5. **No product design** — Document evidence, alternatives, tradeoffs, hypotheses, open questions. Do NOT decide Prime's final architecture.

## Confidence Levels

All research findings must distinguish:
- **FACT** — Verified by primary source, reproducible
- **OBSERVATION** — Directly observed, reproducible
- **HYPOTHESIS** — Proposed explanation, requires validation
- **INFERENCE** — Deduced from evidence, marked as such
- **OPEN QUESTION** — Explicitly unknown, needs research

## ACC Integration

The ACC skill is fully configured for Prime:
- Root AGENTS.md with project contract
- .acc/config/ with agents, workflows, standards, templates
- .acc-memory.md for durable knowledge
- 4 agent profiles: researcher, architect, documenter, reviewer
- 5 standards: architecture, coding, review, testing, tooling
- 8 workflows: feature, research, bugfix, refactor, release, memory, diagnostic, security
- 4 templates: agents.md, memory.md, warn.md

## Quick Start

```bash
# Build the project
cargo build --release --workspace

# Run the CLI
cargo run --bin prime -- --help

# Build knowledge graph for a project
cargo run --bin prime -- build --root /path/to/project --storage /path/to/storage

# Query the knowledge graph
cargo run --bin prime -- query "AuthService.login" --type search

# Show statistics
cargo run --bin prime -- stats

# Check for drift
cargo run --bin prime -- check
```

## Useful Commands

```bash
# Build with progress
cargo run --bin prime -- build --progress --root .

# Query with different formats
cargo run --bin prime -- query "AuthService" --format json
cargo run --bin prime -- query "AuthService" --format minimal
cargo run --bin prime -- query "AuthService" --type context

# Show dependencies
cargo run --bin prime -- deps "AuthService.login" --transitive

# Show call graph
cargo run --bin prime -- calls "AuthService.login" --direction both

# Export graph
cargo run --bin prime -- export --format dot --output ./graph.dot

# Start MCP server (stdio transport)
cargo run --bin prime -- serve

# Inspect an entity with agent envelope
cargo run --bin prime -- inspect "AuthService.login" --format json

# Run benchmark
cargo run --bin prime -- benchmark
```

## Research Methodology

See `.acc/config/workflows/research.md` for the complete research workflow:
1. Define the Research Question
2. Survey Prior Art
3. Compare Alternatives
4. Analyze Tradeoffs
5. Identify Open Questions
6. Update Documentation
7. Validate

## Important Notes

- This is a **research project**, not a product
- No final Prime format, graph model, storage design, retrieval API, or compression algorithm exists yet
- Those decisions are intentionally postponed until research justifies them
- The current goal is to understand the problem deeply enough to make decisions responsibly
- See `init-promt.md` for the original bootstrap specification