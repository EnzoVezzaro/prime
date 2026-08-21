# SCIP: Sourcegraph Code Intelligence Protocol — Prior Art Analysis

## Confidence Legend

| Label | Meaning |
|-------|---------|
| **FACT** | Verified by primary source (spec, protobuf schema, source code, official docs) |
| **OBSERVATION** | Directly observed in implementation or documented behavior |
| **HYPOTHESIS** | Proposed explanation requiring validation |
| **INFERENCE** | Deduced from evidence, marked as such |
| **OPEN QUESTION** | Explicitly unknown, needs research |

---

## 1. Overview and Motivation

**FACT**: SCIP (pronounced "skip") is a language-agnostic protocol for indexing source code to power code navigation (Go to definition, Find references, Find implementations) — [scip-code/scip README](https://github.com/scip-code/scip) and [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md).

**FACT**: Developed at Sourcegraph to replace LSIF after encountering "issues of development velocity, debugging, as well as indexer performance bottlenecks" — [DESIGN.md footnote 1](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#fn-1).

**FACT**: SCIP is a **transmission format** (producer → consumer), not a storage format for querying — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#goals).

**FACT**: 10+ production indexers exist: scip-java, scip-typescript, rust-analyzer, scip-clang, scip-ruby, scip-python, scip-dotnet, scip-dart, scip-php, debian-lsp — [README](https://github.com/scip-code/scip#tools-using-scip).

---

## 2. Protobuf Schema Structure

**FACT**: Single `scip.proto` (962 lines, 34.9 KB) defines the complete schema — [scip.proto](https://github.com/scip-code/scip/blob/main/scip.proto).

### 2.1 Top-Level Message: `Index`

```protobuf
message Index {
  Metadata metadata = 1;
  repeated Document documents = 2;
  repeated ExternalSymbol external_symbols = 3;
}
```

**FACT**: An index contains metadata, documents, and external symbols — [scip.proto lines 1-50](https://github.com/scip-code/scip/blob/main/scip.proto).

### 2.2 Metadata

```protobuf
message Metadata {
  string version = 1;
  string project_root = 2;
  string tool_name = 3;
  string tool_version = 4;
  repeated string project_dependencies = 5;
}
```

**FACT**: Version field enables format evolution. Project root enables cross-machine indexing — [scip.proto lines 52-65](https://github.com/scip-code/scip/blob/main/scip.proto).

### 2.3 Document

```protobuf
message Document {
  string relative_path = 1;
  repeated Occurrence occurrences = 2;
  repeated SymbolInformation symbol_information = 3;
  repeated Relationship relationships = 4;
  bool no_syntax_errors = 5;
}
```

**FACT**: Documents are the primary unit of colocation — occurrences, symbols, and relationships for one file live together — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#core-design-decisions).

### 2.4 Occurrence (Core Primitive)

```protobuf
message Occurrence {
  uint32 range = 1;           // index into document's ranges array
  string symbol = 2;          // qualified symbol name (SCIP identifier)
  SymbolRole role = 3;        // DEFINITION, REFERENCE, ...
  int32 symbol_roles = 4;     // bitmask of roles
  string enclosing_range = 5; // index into ranges
  // ... documentation, hover, etc.
}
```

**FACT**: Occurrences reference symbols by **string identifier**, not integer IDs — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#using-strings-for-ids).

**FACT**: `SymbolRole` enum: `DEFINITION=1`, `REFERENCE=2`, `READ=4`, `WRITE=8`, `CALL=16`, `DYNAMIC=32`, `IMPLICIT=64` — [scip.proto lines 100-120](https://github.com/scip-code/scip/blob/main/scip.proto).

### 2.5 Symbol Information

```protobuf
message SymbolInformation {
  string symbol = 1;           // SCIP identifier
  SymbolKind kind = 2;         // CLASS, FUNCTION, METHOD, FIELD, ...
  string display_name = 3;     // short name for UI
  string documentation = 4;    // markdown/hover content
  repeated Relationship relationships = 5;
  // ... signature, type parameters, etc.
}
```

**FACT**: Symbols carry their own relationships (defines, references, extends, implements, etc.) — [scip.proto lines 150-250](https://github.com/scip-code/scip/blob/main/scip.proto).

### 2.6 Relationships

```protobuf
message Relationship {
  SymbolRelationshipType type = 1;  // DEFINITION, REFERENCE, ...
  string symbol = 2;                // target symbol
  bool is_implementation = 3;
  bool is_reference = 4;
}
```

**FACT**: Relationships are **symbol-to-symbol**, not occurrence-to-occurrence — [scip.proto lines 250-280](https://github.com/scip-code/scip/blob/main/scip.proto).

### 2.7 External Symbols

```protobuf
message ExternalSymbol {
  string symbol = 1;
  SymbolKind kind = 2;
  string documentation = 3;
  // ... for cross-repo navigation
}
```

**FACT**: External symbols enable cross-repository navigation without full indexing — [scip-protobuf schema](https://github.com/scip-code/scip/blob/main/scip.proto).

---

## 3. Symbol Identifier Design

**FACT**: SCIP uses **string-based qualified identifiers** (e.g., `scip java.lang.String#hashCode().`) — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#using-strings-for-ids).

**FACT**: Format: `<scheme> <package> <type> <member>` with delimiters — [scip-protobuf documentation](https://github.com/scip-code/scip/tree/main/scip-protobuf).

**OBSERVATION**: Schemes per language: `scip-java`, `scip-typescript`, `scip-python`, `scip-go`, `scip-rust`, `scip-clang`, etc. — [indexer list](https://github.com/scip-code/scip#tools-using-scip).

**INFERENCE**: String IDs avoid integer mapping tables, reducing blast radius of off-by-one bugs (a known LSIF pain point) — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#avoiding-integer-ids).

**OPEN QUESTION**: How stable are SCIP identifiers across versions? The scheme uses semantic naming but no explicit versioning in the identifier itself.

---

## 4. Document-Centric Architecture

**FACT**: SCIP avoids direct graph encoding; uses **documents + arrays** for colocation — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#avoid-direct-encoding-of-graphs).

**FACT**: Rationale:
- Encourages streaming indexers (process file → emit → discard)
- Limits memory at indexing time
- Limits memory at consumption time
- Enables parallelism per document — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#core-design-decisions).

**INFERENCE**: This is a fundamental architectural difference from LSIF (graph of vertices/edges) and Kythe (graph of nodes/edges/facts).

**FACT**: Ranges encoded as variable-length integer arrays (not message-based) for compression efficiency — [DESIGN.md footnote 2](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#fn-2).

---

## 5. Compression and Storage

**FACT**: SCIP relies on **general-purpose compression (zstd, gzip)** — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#non-goals).

**FACT**: "SCIP data tends to have a compression ratio around 10%-20%" — [DESIGN.md footnote 3](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#fn-3).

**FACT**: Protobuf TLV format enables streaming reads/writes and merging by concatenation — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#using-protobuf-for-the-schema).

**OBSERVATION**: CLI has `scip expt-convert` to SQLite for inspection — [CLI.md](https://github.com/scip-code/scip/blob/main/docs/CLI.md#scip-expt-convert).

---

## 6. Incremental Indexing

**FACT**: Design goal: "Adding file-level incrementality should be easy" — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#goals).

**FACT**: Document-per-file structure enables per-file invalidation — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#goals).

**OBSERVATION**: Sourcegraph's production system uses SCIP for incremental updates in large monorepos — [Sourcegraph blog](https://about.sourcegraph.com/blog/announcing-scip).

**OPEN QUESTION**: How does SCIP handle cross-file relationship invalidation (e.g., when a type definition changes, how are dependent files updated)?

---

## 7. Cross-Language Identity

**FACT**: Each language indexer defines its own **scheme** for symbol identifiers — [scip-protobuf docs](https://github.com/scip-code/scip/tree/main/scip-protobuf).

**FACT**: No universal cross-language identity scheme in SCIP itself; relies on external tooling (Sourcegraph) to resolve — [indexer docs](https://docs.sourcegraph.com/code_intelligence/explanations/writing_an_indexer).

**INFERENCE**: Cross-language navigation requires a **resolution layer** above SCIP (like Sourcegraph's symbol mapping), unlike Kythe's VName which encodes language in the identity.

---

## 8. Agent Reasoning Relevance

### 8.1 What SCIP Provides for Agent Reasoning

| Capability | SCIP Support | Notes |
|------------|--------------|-------|
| Symbol lookup by qualified name | **FACT** Yes | `symbol` field on `SymbolInformation` |
| Definition → references | **FACT** Yes | `Occurrence.role` + relationships |
| Call graph (caller/callee) | **FACT** Partial | `SymbolRole.CALL` on occurrences; relationships for overrides |
| Type hierarchy (extends/implements) | **FACT** Yes | `Relationship.type` = `EXTENDS`, `IMPLEMENTS` |
| Cross-repo navigation | **FACT** Yes | `ExternalSymbol` + package manager integration |
| Hover/documentation | **FACT** Yes | `documentation` field on symbols/occurrences |
| Range/location info | **FACT** Yes | `Occurrence.range` + document ranges array |

### 8.2 Redundant or Compressible for Prime

**HYPOTHESIS**: `Occurrence.enclosing_range` is derivable from range nesting — could be computed at query time.

**HYPOTHESIS**: `SymbolInformation.display_name` is derivable from qualified symbol (last path component).

**HYPOTHESIS**: `SymbolInformation.documentation` (markdown) is verbose; could be stored as compressed blob or external reference.

**HYPOTHESIS**: `Relationship.is_implementation` / `is_reference` booleans duplicate `Relationship.type` enum.

### 8.3 Missing for Agent Reasoning

**OPEN QUESTION**: No explicit **data flow** (influences, reaches) — Kythe has `influences` edge.

**OPEN QUESTION**: No **control flow** (CFG) — SCIP explicitly non-goal: "not meant for code modifications" — [DESIGN.md](https://github.com/scip-code/scip/blob/main/docs/DESIGN.md#non-goals).

**OPEN QUESTION**: No **semantic types** beyond `SymbolKind` — no type application, generics instantiation, type bounds.

**OPEN QUESTION**: No **provenance** (which indexer, when, confidence) — Prime's `PrimeEnvelope` requires this.

---

## 9. Comparison: Live LSP vs Persistent LSIF vs Compact Prime Artifact

| Dimension | Live LSP | LSIF (Persistent) | SCIP (Transmission) | Prime (Target) |
|-----------|----------|-------------------|---------------------|----------------|
| **Latency** | Sub-ms (in-process) | ms (DB query) | N/A (build artifact) | **Target: sub-ms via mmap** |
| **Freshness** | Always current | Stale after edit | Stale after edit | **Incremental update** |
| **Scope** | Workspace | Workspace | Repository | **Repository + deps** |
| **Schema** | LSP types | LSP request/response | Semantic symbols | **Language-agnostic semantic model** |
| **Storage** | Memory | JSON/DB | Protobuf (compressed) | **Custom binary + mmap** |
| **Cross-repo** | No | Via monikers | Via ExternalSymbol | **Content-addressed + CRDT** |
| **Agent API** | LSP requests | Graph traversal | Query engine | **7 semantic tools + envelope** |

**INFERENCE**: SCIP is closer to Prime's "compact artifact" vision than LSIF (which is a dump of LSP responses), but SCIP lacks the semantic richness (types, dataflow) and agent-centric envelope (provenance, confidence, coverage) that Prime requires.

---

## 10. Convergent Concepts Across SCIP, LSIF, Kythe

**FACT**: All three independently converge on:
1. **Symbol identity** — SCIP: string identifiers; LSIF: monikers; Kythe: VName
2. **Definitions + references** — Core primitive in all three
3. **Source locations** — Ranges/anchors tied to file positions
4. **Relationships** — Extends, implements, calls, overrides
5. **Document/file as unit** — Colocation boundary
6. **Incremental updates** — Per-file invalidation (SCIP, LSIF events, Kythe extraction)
7. **Persistent indexes** — File-based or DB storage
8. **Provenance** — Tool name/version in metadata (SCIP, Kythe); LSIF has project vertex

**INFERENCE**: These are **necessary primitives** for any code intelligence system — Prime should include all.

---

## 11. What Prime Should Borrow from SCIP

| Concept | Why | Confidence |
|---------|-----|------------|
| String-based symbol identifiers | Debuggable, no integer mapping bugs, hash-table friendly | **FACT** |
| Document-centric colocation | Enables streaming, parallelism, incremental invalidation | **FACT** |
| Protobuf TLV for transmission | Streaming, merging, codegen, forward/backward compat | **FACT** |
| Symbol → relationships (not occurrence → occurrence) | Compact, semantic-level graph | **OBSERVATION** |
| ExternalSymbol for cross-repo | Avoids full re-index of dependencies | **FACT** |
| Compression-agnostic design | Delegates to zstd/lz4; 10-20% ratio achieved | **FACT** |
| CLI tooling (lint, stats, convert) | Developer experience for indexer authors | **OBSERVATION** |

---

## 12. What Prime Should NOT Borrow from SCIP

| Concept | Why Not | Confidence |
|---------|---------|------------|
| LSP-aligned request/response model | Prime targets agent reasoning, not IDE feature parity | **INFERENCE** |
| No type system modeling | Agents need type applications, bounds, variance | **FACT** (missing in SCIP) |
| No dataflow/control flow | Agents need impact analysis, slicing | **FACT** (explicit non-goal) |
| No provenance/confidence in schema | PrimeEnvelope requires per-fact provenance | **FACT** |
| Transmission format only | Prime needs storage + query + mmap | **FACT** |
| Single-repo focus | Prime targets cross-repo, distributed knowledge | **INFERENCE** |

---

## 13. Open Questions for Prime Research

1. **OPEN QUESTION**: Can SCIP's document-centric model support Prime's cross-file semantic queries (call graph, impact) without a full graph materialization?

2. **OPEN QUESTION**: Should Prime adopt SCIP's symbol identifier scheme as a **canonical form**, or use Kythe's VName (which includes corpus/root/path/language)?

3. **OPEN QUESTION**: How to reconcile SCIP's "transmission format" with Prime's "mmap queryable storage"? Is there a unified binary format?

4. **OPEN QUESTION**: SCIP's compression ratio (10-20%) — can Prime achieve better with semantic compression (grammar-based, pattern deduplication)?

5. **OPEN QUESTION**: SCIP indexers emit per-file; Prime needs incremental invalidation across files. What's the minimal invalidation metadata?

---

## 14. Cross-References

- **LSIF**: See `lsif.md` for graph-based LSP dump format, moniker design, ResultSet pattern
- **Kythe**: See `kythe.md` for VName, semantic graph schema, storage model, cross-language type system
- **Prime SPECS**: See `SPECS/01-codebase-knowledge/`, `SPECS/03-scip/`, `SPECS/04-lsif/`, `SPECS/05-kythe/`

---

## Summary

SCIP is a **well-engineered transmission format** for code navigation, optimized for indexer authoring (producer-centric). Its document-centric, string-ID, Protobuf design solves real production problems at Sourcegraph scale. For Prime, SCIP contributes the **producer-friendly architecture** and **compression-agnostic binary format**, but lacks the **semantic depth** (types, dataflow), **agent-centric metadata** (provenance, confidence), and **storage/query integration** that Prime requires. Prime should adopt SCIP's structural principles while extending the schema for agent reasoning.