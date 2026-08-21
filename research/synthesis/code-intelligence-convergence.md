# Code Intelligence Convergence Analysis

**Confidence:** OBSERVATION (documented patterns), HYPOTHESIS (Prime implications)
**Primary Sources:** All prior-art analyses
**Last Updated:** August 2026

## Executive Summary

Analysis of 11 code intelligence systems reveals strong convergence on core primitives despite differing architectures. This synthesis identifies the **necessary components** any code intelligence system must include and the **differentiation opportunities** for Prime.

## Convergent Primitives

### 1. Symbol Identity

Every system independently converges on a stable identifier for code entities:

| System | Identity Mechanism | Scope |
|--------|-------------------|-------|
| SCIP | String-based qualified identifiers | Single repository |
| LSIF | Monikers (scheme + identifier) | Cross-project |
| Kythe | VName (corpus/root/path/language) | Cross-repository |
| Code-Graph-RAG | Qualified name + @line suffix | Single repository |
| OpenGrok | Ctags-based identifiers | Single repository |
| Tree-sitter | Node type + position | Single file |

**INFERENCE:** Prime must adopt a **hierarchical, content-addressed symbol identity** that supports:
- Cross-file resolution (like SCIP/LSIF)
- Cross-repository navigation (like Kythe)
- Disambiguation of overloaded definitions (like Code-Graph-RAG's @line)

### 2. Definitions + References

The fundamental query pair across all systems:

| System | Definitions | References |
|--------|-------------|------------|
| SCIP | `SymbolRole.DEFINITION` | `SymbolRole.REFERENCE` |
| LSIF | `definitionResult` vertex | `referenceResult` vertex |
| Kythe`/defines` edge | `/ref` edge |
| Code-Graph-RAG | `DEFINES` relationship | `REFERENCES` relationship |
| OpenGrok | Ctags definitions | Ctags references |

**INFERENCE:** Prime's `DEFINES` and `REFERENCES` relationships are necessary. Should be first-class in the graph schema.

### 3. Source Locations

Every system tracks where code entities exist:

| System | Location Model |
|--------|---------------|
| SCIP | Document ranges (variable-length integers) |
| LSIF | Range vertices (start/end positions) |
| Kythe`Anchor` nodes with VName |
| Code-Graph-RAG | `start_line`, `end_line`, `start_col` properties |
| Tree-sitter | Node byte offsets + line/column |

**INFERENCE:** Prime needs a compact location model. Byte offsets (like Tree-sitter) are more compact than line/column; range encoding (like SCIP) saves space.

### 4. Relationships

Core relationship types appear across systems:

| Relationship | SCIP | LSIF | Kythe | Code-Graph-RAG | Prime |
|-------------|------|------|-------|----------------|-------|
| Extends/Inherits | ✅ | Partial | ✅ | ✅ | ✅ |
| Implements | ✅ | Partial | ✅ | ✅ | ✅ |
| Calls | ✅ (via references) | ❌ | ✅ | ✅ | ✅ |
| Contains | ✅ (document) | ✅ (document) | ✅ | ✅ | ✅ |
| Imports | ✅ | ❌ | ✅ | ✅ | ✅ |
| Overrides | ❌ | ❌ | ✅ | ✅ | ❌ |
| Exports | ❌ | ❌ | ❌ | ✅ | ❌ |
| Data flow | ❌ | ❌ | ✅ (influences) | ✅ (FLOWS_TO) | ❌ |

**INFERENCE:** Prime currently covers 5 of 8 core relationships. Missing: `OVERRIDES`, `EXPORTS`, and data flow (`FLOWS_TO`).

### 5. Document/File as Unit

All systems use file boundaries as colocation units:

| System | Unit | Rationale |
|--------|------|-----------|
| SCIP | Document | Streaming, parallelism |
| LSIF | Document vertex | LSP request scope |
| Kythe | File node | Corpus boundary |
| Code-Graph-RAG | File node | Incremental invalidation |

**INFERENCE:** Prime's entity model should maintain file-level colocation for incremental updates.

### 6. Incremental Updates

Most systems support per-file invalidation:

| System | Mechanism |
|--------|-----------|
| SCIP | Per-file document replacement |
| LSIF | Document events (begin/end) |
| Tree-sitter | Changed ranges API |
| Code-Graph-RAG | File mtime detection + re-parse |

**INFERENCE:** Prime needs incremental invalidation beyond current implementation. Tree-sitter's `changed_ranges` API provides the foundation.

### 7. Cross-Repository Navigation

Two systems support cross-repo:

| System | Mechanism |
|--------|-----------|
| SCIP | ExternalSymbol |
| LSIF | Monikers with scheme translation |
| Kythe`VName` with corpus field |

**INFERENCE:** Prime should adopt a moniker-like system for cross-repo identity, building on SCIP's string identifiers.

## Differentiation Opportunities

### What Prime Can Do Better

1. **Agent-native metadata:** No existing system provides provenance, confidence levels, or token-budget-aware responses. Prime's `PrimeEnvelope<T>` is unique.

2. **Multi-dimensional graph:** Most systems are single-dimensional (AST-only or reference-only). Prime can combine AST + CFG + PDG like Joern, but with agent-optimized storage.

3. **Succinct storage:** Existing systems use verbose formats (JSON, Protobuf). Prime's binary format with succinct data structures can achieve 2-5x better compression.

4. **Progressive context building:** No system provides token-budget-aware context for agents. Prime's `ContextBuilder` with progressive disclosure is novel.

5. **Language-agnostic semantic model:** Code-Graph-RAG's hybrid frontends show the path; Prime can push this further with a unified capability model.

### What Prime Should Adopt

1. **SCIP's document-centric architecture** for streaming and parallelism
2. **LSIF's moniker system** for cross-project identity
3. **Kythe's VName** for cross-repository identity
4. **Code-Graph-RAG's relationship richness** (FLOWS_TO, OVERRIDES, EXPORTS)
5. **Joern's multi-layer graph** (AST + CFG + PDG)
6. **Tree-sitter's incremental parsing** for efficient updates
7. **OpenGrok's Lucene-based indexing** for full-text search

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| Symbol identity converges across systems | **FACT** (documented in 6+ systems) |
| Defs + refs are universal primitives | **FACT** (documented in all systems) |
| File-level colocation is standard | **FACT** (documented in all systems) |
| Incremental updates are universal | **OBSERVATION** (4+ systems support) |
| Cross-repo is emerging (only 2 systems) | **OBSERVATION** (SCIP, LSIF) |
| Agent-native metadata is unique to Prime | **INFERENCE** (no other system provides) |
| Multi-dimensional graph is rare | **OBSERVATION** (only Joern, Code-Graph-RAG) |
| Prime can achieve better compression | **HYPOTHESIS** (requires benchmarking) |
