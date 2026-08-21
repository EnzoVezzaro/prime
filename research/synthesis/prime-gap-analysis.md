# Prime Gap Analysis: Current State vs. State of the Art

**Confidence:** OBSERVATION (Prime implementation), HYPOTHESIS (improvement potential)
**Primary Sources:** Prime codebase, prior-art analyses
**Last Updated:** August 2026

## Executive Summary

Prime's current implementation covers ~60% of the necessary primitives identified in the convergence analysis. Key gaps exist in relationship richness, incremental indexing, and cross-repository support. This analysis prioritizes gaps by impact and feasibility.

## Current Prime Implementation (Verified)

### Entity Types (prime-core/src/types.rs)

| Entity | Status | Coverage |
|--------|--------|----------|
| `Entity` with `SymbolKind` | ✅ Implemented | Function, Method, Class, Struct, Enum, Trait, Module, etc. |
| `File` | ✅ Implemented | Path, language, size |
| `Module` | ✅ Implemented | Qualified name, language, file |
| `Project` | ✅ Implemented | Name, root path, files, modules |

### Relationship Types (prime-core/src/types.rs)

| Relationship | Status | Notes |
|-------------|--------|-------|
| `Calls` | ✅ Implemented | Caller → Callee |
| `DependsOn` | ✅ Implemented | Module dependencies |
| `Contains` | ✅ Implemented | File → Entity |
| `Defines` | ✅ Implemented | Module → Entity |
| `Inherits` | ✅ Implemented | Class → Parent |
| `Implements` | ✅ Implemented | Class → Interface |
| `Imports` | ✅ Implemented | Module imports |
| `Exports` | ❌ Missing | Public API surface |
| `Overrides` | ❌ Missing | Method override chain |
| `FLOWS_TO` | ❌ Missing | Data flow / taint |
| `REFERENCES` (non-call) | ❌ Missing | Callback/passed-as-value |

### Storage (prime-index/src/)

| Component | Status | Notes |
|-----------|--------|-------|
| Binary serialization | ✅ Implemented | Custom format |
| mmap access | ✅ Implemented | Read-only + read-write |
| Compression (zstd, lz4) | ✅ Implemented | Configurable |
| Incremental updates | ⚠️ Partial | Basic structure exists |
| FlatBuffer storage | ✅ Implemented | Alternative backend |

### Query Engine (prime-index/src/query.rs)

| Capability | Status | Notes |
|-----------|--------|-------|
| Entity search by name | ✅ Implemented | Full-text + prefix |
| Entity lookup by qualified name | ✅ Implemented | Direct lookup |
| Relationship traversal | ✅ Implemented | Caller/callee, deps |
| Context building | ✅ Implemented | Progressive disclosure |
| Streaming queries | ✅ Implemented | Lazy evaluation |

### MCP Server (prime-mcp/)

| Tool | Status | Notes |
|------|--------|-------|
| `prime_search` | ✅ Implemented | Keyword search |
| `prime_lookup` | ✅ Implemented | Qualified name lookup |
| `prime_context` | ✅ Implemented | Knowledge neighborhood |
| `prime_relationships` | ✅ Implemented | Multi-dimensional traversal |
| `prime_dependencies` | ✅ Implemented | Dependency graph |
| `prime_impact` | ✅ Implemented | Change impact analysis |
| `prime_architecture` | ✅ Implemented | Architecture overview |

## Gap Analysis

### Critical Gaps (High Impact, Required)

| Gap | Impact | Priority | Notes |
|-----|--------|----------|-------|
| **Incremental indexing** | High | P0 | Full rebuild required for every change |
| **Cross-file symbol resolution** | High | P0 | Heuristic-based, not semantic |
| **Type system integration** | High | P0 | No type inference beyond annotations |
| **Call graph accuracy** | High | P0 | Syntactic calls ≠ resolved calls |

### Important Gaps (High Impact, Can Wait)

| Gap | Impact | Priority | Notes |
|-----|--------|----------|-------|
| **Relationship richness** | Medium-High | P1 | Missing OVERRIDES, EXPORTS, FLOWS_TO |
| **Cross-repository support** | Medium | P1 | No moniker system |
| **Semantic compression** | Medium | P1 | Using general-purpose compression only |
| **Multi-dimensional graph** | Medium | P1 | No CFG/PDG layers |

### Nice-to-Have Gaps (Lower Impact)

| Gap | Impact | Priority | Notes |
|-----|--------|----------|-------|
| **Full-text search (Lucene/tantivy)** | Medium | P2 | Current search is basic |
| **Streaming HTTP transport** | Low | P2 | Only stdio MCP currently |
| **Telemetry collection** | Low | P2 | Structure exists, not populated |
| **CRDT support** | Low | P3 | Planned, not started |

## Gap Prioritization Matrix

| Gap | Impact | Feasibility | ROI | Recommendation |
|-----|--------|-------------|-----|----------------|
| Incremental indexing | High | Medium | High | **Do first** |
| Type system | High | Low (language-specific) | Medium | **Delegate to analyzers** |
| Relationship richness | Medium-High | High | High | **Do second** |
| Cross-repo support | Medium | Medium | Medium | **Do third** |
| Semantic compression | Medium | Medium | Medium | **Do fourth** |
| Multi-dimensional graph | Medium | Low (complex) | Low | **Do fifth** |

## Specific Code Gaps

### 1. Missing Relationships in prime-core/src/types.rs

```rust
// Currently defined:
pub enum RelationKind {
    Calls,
    DependsOn,
    Contains,
    Defines,
    Inherits,
    Implements,
    Imports,
}

// Missing (should add):
// - Exports (Module → Entity)
// - Overrides (Method → Method)  
// - FLOWS_TO (with kind/via properties)
// - References (non-call references)
// - Instantiates (Callable → Class)
// - ReadsFrom/WritesTo (Callable → Resource)
```

### 2. Missing Incremental Invalidation

Current `IncrementalStorage` (prime-index/src/storage.rs) is a stub:
```rust
pub struct IncrementalStorage {
    base_path: PathBuf,
    // No change tracking, no invalidation logic
}
```

Should implement:
- Content-hash based invalidation (Merkle tree)
- File-level change detection
- Entity-level invalidation
- Relationship re-computation

### 3. Missing Cross-File Resolution

Current parser (prime-parser/) extracts symbols per-file. Cross-file resolution relies on:
- Qualified name matching (heuristic)
- Import statement parsing (partial)

Should implement:
- Symbol table with scope resolution
- Import/export graph construction
- Type-aware resolution (optional, via analyzer integration)

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| Prime covers ~60% of necessary primitives | **INFERENCE** (based on convergence analysis) |
| Incremental indexing is highest-impact gap | **HYPOTHESIS** (requires user study) |
| Type system should be delegated to analyzers | **HYPOTHESIS** (based on Code-Graph-RAG pattern) |
| Relationship richness is feasible to add | **OBSERVATION** (Code-Graph-RAG demonstrates) |
| Cross-repo support can wait | **INFERENCE** (most use cases are single-repo) |
