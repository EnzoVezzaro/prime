# Prime Internal Architecture Map — Current State

## Overview
Prime ingests repositories via Tree-sitter parsers, extracts symbols & relations into a language-agnostic KnowledgeGraph, stores it in compact binary with mmap, and exposes agent-facing queries via MCP tools. The semantic model is symbol+relation graph with provenance & confidence, already agent-optimized via PrimeEnvelope.

## Ingestion
- `prime-parser` (Tree-sitter based)
  - `Parser::parse_file` -> Tree-sitter parse -> `Extractor::extract`
  - Languages: Rust, TS, JS, Python, Go, Java, C, C++
  - Outputs `ParseResult` with `ExtractedEntity` + `ExtractedRelation`
  - `ProjectAnalyzer::analyze(root)` walks repo, filters excluded patterns, parallel parse

## Representation
- `prime-core`
  - `Entity { id, kind: SymbolKind, name, qualified_name, file_id, range, language, confidence, signature, documentation, children, relations }`
  - `Relation { from, to, kind: RelationKind, confidence, provenance: Provenance }`
  - `File`, `Module`, `Project`
  - `KnowledgeGraph { project, entities: IndexMap, files, modules, relations, indexes }`
  - Indexes: `NameIndex`, `FileIndex`, `RelationIndex` with RoaringBitmap for deps
  - Confidence: Exact/High/Medium/Low/Unknown → AgentConfidence Exact/Derived/Inferred/Unknown
  - Provenance: Declared/Discovered/Inferred/Memory/Stored

## Storage / Indexing
- `prime-index`
  - `StorageManager` with `StorageConfig { compress, compression_level, use_mmap }`
  - Canonical binary serialization via `serialize` + `compact_serialization` (zstd/lz4, FlatBuffers/bincode/rmp-serde)
  - `mmap` zero-copy access
  - `IncrementalIndexer` with file hash tracking, invalidation index, dependent file propagation

## Query / Context Assembly
- `prime-query`
  - `QueryEngine` with `find_by_name/prefix/search/get_context`
  - `ContextResult` with target, dependencies, dependents, callers, callees, tests, returns, may_throw
  - `ToolExecutor` implements 7 semantic tools
  - `PrimeEnvelope<T>` standard response: status, source_required, coverage, provenance, warnings, telemetry

## Agent Interface
- `prime-mcp` 7 tools via rmcp stdio:
  - prime_search, prime_lookup, prime_context, prime_relationships, prime_dependencies, prime_impact, prime_architecture
  - All return PrimeEnvelope<T>
- CLI commands: build, update, query, stats, check, deps, dependents, calls, export, serve, inspect, benchmark

## Source-free evaluation
- Confidence mapping and coverage metrics drive source_required flag
- AgentConfidence derived from internal Confidence
- Provenance records analyzer source

## Benchmark
- `prime-bench` measures derivation, artifact size, retrieval latency, knowledge accuracy
- Corpus config in `benchmarks/corpus/repositories.json`

## Extensibility points
- `SymbolKind`, `RelationKind` enums extensible
- `KnowledgeGraph::build_indexes` rebuildable
- `ToolExecutor` pluggable
- `prime-index::compact_serialization` for new formats
- Parser config allows new Tree-sitter grammars

## Current gaps vs AIRL hypothesis
- No explicit effect system (Pure/IO/Fail)
- No typed dataflow graph (nodes are symbols, not computations)
- No semantic patches with invertibility/preconditions
- No constraints validation layer
- No canonical content-addressed versioning
- No projections back to source from semantic ops
- Serialization optimized for storage, not token efficiency for LLMs

## What can be extended without breakage
- Add new RelationKind variants for effects/dataflow
- Add semantic metadata fields to Entity (inputs/outputs/effects/capabilities)
- Add Agent-Native IR layer on top of KnowledgeGraph
- Add secondary serialization formats for agent view
- Add query layer extensions for semantic navigation
- Experimental semantic patch engine alongside existing source editing
