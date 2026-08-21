# LSIF: Language Server Index Format — Prior Art Analysis

## Confidence Legend

| Label | Meaning |
|-------|---------|
| **FACT** | Verified by primary source (specification.md, protocol.ts, lsif-node source) |
| **OBSERVATION** | Directly observed in implementation or documented behavior |
| **HYPOTHESIS** | Proposed explanation requiring validation |
| **INFERENCE** | Deduced from evidence, marked as such |
| **OPEN QUESTION** | Explicitly unknown, needs research |

---

## 1. Overview and Motivation

**FACT**: LSIF defines a standard format for language servers to dump workspace knowledge, enabling LSP requests **without running the language server** — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#language-server-index-format).

**FACT**: "The dump doesn't contain any program symbol information nor does the LSIF define any symbol semantics... The LSIF, therefore, doesn't define a symbol database" — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#motivation).

**FACT**: LSIF models **LSP request/response pairs** as a graph of vertices and edges — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#motivation).

**FACT**: Output format: **NDJSON** (newline-delimited JSON) — [lsif-node README](https://github.com/microsoft/lsif-node#how-to-run-the-tools).

**FACT**: Developed by Microsoft (TypeScript team); used by `lsif tsc` for TypeScript/JavaScript — [lsif-node](https://github.com/microsoft/lsif-node).

---

## 2. Graph Data Model

### 2.1 Vertices and Edges

**FACT**: Two vertex types: `vertex` (data) and `edge` (relationships) — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#motivation).

```json
{ "id": 1, "type": "vertex", "label": "document", "uri": "file:///...", "languageId": "typescript" }
{ "id": 2, "type": "vertex", "label": "foldingRangeResult", "result": [...] }
{ "id": 3, "type": "edge", "label": "textDocument/foldingRange", "outV": 1, "inV": 2 }
```

**FACT**: Edges are directed (`outV` → `inV`); `inVs` array for 1:N — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#motivation).

### 2.2 Vertex Labels (Core Types)

| Label | Purpose | Key Fields |
|-------|---------|------------|
| `document` | Source file | `uri`, `languageId`, `contents?` (base64) |
| `project` | Project context (tsconfig.json) | `resource`, `kind`, `contents?` |
| `range` | Source span | `start`, `end`, `tag?` |
| `resultSet` | Hub for shared results | (empty) |
| `hoverResult` | Hover content | `result: { contents, range? }` |
| `definitionResult` | Go to definition | — |
| `referenceResult` | Find references | — |
| `typeDefinitionResult` | Go to type definition | — |
| `implementationResult` | Go to implementation | — |
| `diagnosticResult` | Diagnostics | `result: Diagnostic[]` |
| `foldingRangeResult` | Folding ranges | `result: FoldingRange[]` |
| `documentLinkResult` | Document links | `result: DocumentLink[]` |
| `documentSymbolResult` | Document outline | `result: DocumentSymbol[]` |
| `moniker` | Symbol identity | `kind`, `scheme`, `identifier` |
| `$event` | Streaming boundaries | `kind: begin/end`, `scope`, `data` |

**FACT**: Full list in [protocol.ts](https://github.com/microsoft/lsif-node/blob/main/protocol/src/protocol.ts) (2119 lines).

---

## 3. Range-Based Position Model

**FACT**: Positions modeled as **ranges** (not points) — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#ranges).

**FACT**: Range containment rules:
1. Range ID contained in exactly one document
2. No two ranges equal
3. No overlapping ranges unless one contains the other — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#ranges).

**FACT**: Lookup algorithm: innermost containing range → follow `next` to `resultSet` → check for request edge — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#ranges).

### 3.1 Range Tags (Semantic Annotation)

**FACT**: Ranges carry semantic tags — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#request-textdocumentdocumentsymbol):

```typescript
interface DeclarationTag { type: 'declaration', text: string, kind: SymbolKind, fullRange: Range, detail?: string }
interface DefinitionTag  { type: 'definition', text: string, kind: SymbolKind, fullRange: Range, detail?: string }
interface ReferenceTag   { type: 'reference', text: string }
interface UnknownTag     { type: 'unknown', text: string }
```

**OBSERVATION**: This is LSIF's **only** symbol semantics — attached to ranges, not standalone symbols.

---

## 4. ResultSet Pattern (Deduplication Hub)

**FACT**: `ResultSet` vertices act as hubs to share results across ranges — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#resultset).

**FACT**: `next` edge chains: `range` → `resultSet` → (optional) another `resultSet` — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#resultset).

**FACT**: Lookup traverses `next` chain until finding requested edge — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#resultset).

**INFERENCE**: This avoids duplicating hover/definition/reference results for every occurrence of a symbol — major space savings.

---

## 5. LSP Request Coverage

### 5.1 Position-Dependent Requests (via ranges)

| LSP Request | LSIF Vertex | Edge Label | Result Items |
|-------------|-------------|------------|--------------|
| `textDocument/hover` | `hoverResult` | `textDocument/hover` | MarkupContent |
| `textDocument/definition` | `definitionResult` | `textDocument/definition` | `item` edges to ranges |
| `textDocument/declaration` | `declarationResult` | `textDocument/declaration` | `item` edges |
| `textDocument/typeDefinition` | `typeDefinitionResult` | `textDocument/typeDefinition` | `item` edges |
| `textDocument/references` | `referenceResult` | `textDocument/references` | `item` edges (property: definitions/references/referenceResults) |
| `textDocument/implementation` | `implementationResult` | `textDocument/implementation` | `item` edges (property: implementationResults) |

**FACT**: `referenceResult` uses `item.edge.property` to distinguish definitions vs references vs nested referenceResults — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#request-textdocumentreferences).

### 5.2 Document-Level Requests (direct from document)

| LSP Request | LSIF Vertex | Edge Label |
|-------------|-------------|------------|
| `textDocument/foldingRange` | `foldingRangeResult` | `textDocument/foldingRange` |
| `textDocument/documentLink` | `documentLinkResult` | `textDocument/documentLink` |
| `textDocument/documentSymbol` | `documentSymbolResult` | `textDocument/documentSymbol` |
| `textDocument/diagnostic` | `diagnosticResult` | `textDocument/diagnostic` |

---

## 6. Monikers: Cross-Project Symbol Identity

**FACT**: Monikers provide **stable, position-independent symbol handles** for cross-project linking — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#project-exports-and-external-imports-monikers).

```json
{ "id": 12, "type": "vertex", "label": "moniker", "kind": "export", "scheme": "tsc", "identifier": "lib/index:func" }
{ "id": 13, "type": "edge", "label": "moniker", "outV": 11, "inV": 12 }
```

**FACT**: Moniker structure:
- `kind`: `export` | `import` | `local`
- `scheme`: opaque namespace (e.g., `tsc`, `npm`, `maven`, `nuget`)
- `identifier`: scheme-specific string — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#project-exports-and-external-imports-monikers).

**FACT**: `nextMoniker` edge translates between schemes (e.g., `tsc` → `npm`) — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#project-exports-and-external-imports-monikers).

**INFERENCE**: This is a **practical, deployed** cross-language identity mechanism — used in production for npm/GitHub code navigation.

---

## 7. Streaming and Incremental Processing

**FACT**: `$event` vertices mark document/project boundaries for streaming consumers — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#events).

```json
{ "id": 5, "type": "vertex", "label": "$event", "kind": "begin", "scope": "document", "data": 4 }
{ "id": 53, "type": "vertex", "label": "$event", "kind": "end", "scope": "document", "data": 4 }
```

**FACT**: After `end` event for document X, no further data may reference X's ranges — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#events).

**FACT**: Multiple documents can be open simultaneously (interleaved begin/end) — [specification.md](https://github.com/microsoft/language-server-protocol/blob/main/indexFormat/specification.md#events).

**INFERENCE**: Enables **single-pass streaming import** into databases — consumers don't need to hold full dump in memory.

---

## 8. Storage and Tooling

**FACT**: `lsif-node` provides SQLite importer (`sqlite/` directory) — [lsif-node](https://github.com/microsoft/lsif-node/tree/main/sqlite).

**FACT**: Validation tools in `tooling/` — [lsif-node](https://github.com/microsoft/lsif-node/tree/main/tooling).

**FACT**: VS Code extension (`vscode-lsif-extension`) serves LSIF dumps for LSP requests — [lsif-node README](https://github.com/microsoft/lsif-node#lsif-extension).

---

## 9. Agent Reasoning Relevance

### 9.1 What LSIF Provides

| Capability | LSIF Support | Notes |
|------------|--------------|-------|
| Hover at position | **FACT** Yes | `hoverResult` via range → resultSet |
| Go to definition | **FACT** Yes | `definitionResult` with `item` edges |
| Find references | **FACT** Yes | `referenceResult` with property tagging |
| Go to implementation | **FACT** Yes | `implementationResult` |
| Type hierarchy | **PARTIAL** | Only via `definitionResult` chains; no explicit extends/implements |
| Call graph | **NO** | Not modeled |
| Data flow | **NO** | Not modeled |
| Cross-repo symbols | **FACT** Yes | Monikers with scheme translation |
| Document outline | **FACT** Yes | `documentSymbolResult` with range hierarchy |
| Diagnostics | **FACT** Yes | `diagnosticResult` |

### 9.2 Redundant or Compressible for Prime

**HYPOTHESIS**: Entire **vertex/edge graph structure** is verbose in JSON — each vertex repeats `id`, `type`, `label`. Binary encoding (like SCIP's Protobuf) would compress significantly.

**HYPOTHESIS**: `ResultSet` vertices are structural artifacts for graph deduplication — in a columnar/indexed store, this deduplication happens at storage layer.

**HYPOTHESIS**: `range` vertices with `tag` duplicate information also in `definitionResult`/`referenceResult` items.

**HYPOTHESIS**: `documentSymbolResult` hierarchy duplicates `range` parent/child relationships.

### 9.3 Missing for Agent Reasoning

**FACT**: No **symbol table** — LSIF explicitly avoids defining symbol semantics.

**FACT**: No **type system** — no generics, type parameters, bounds, variance.

**FACT**: No **call graph** — only `references` which includes call sites but not distinguished.

**FACT**: No **data flow** — no `influences`, `reaches`, `defines` edges.

**FACT**: No **provenance/confidence** — no per-fact metadata about source indexer, timestamp, reliability.

**FACT**: No **incremental invalidation metadata** beyond document-level events.

---

## 10. Comparison: Live LSP vs Persistent LSIF vs Compact Prime Artifact

| Dimension | Live LSP | LSIF (Persistent) | SCIP (Transmission) | Prime (Target) |
|-----------|----------|-------------------|---------------------|----------------|
| **Data Model** | Request/response | Graph of LSP responses | Semantic symbols + relationships | **Language-agnostic semantic graph** |
| **Symbol Semantics** | None (LSP) | None (range tags only) | SymbolInfo + relationships | **Rich semantic model** |
| **Cross-Repo** | No | Monikers (scheme-based) | ExternalSymbol | **Content-addressed + CRDT** |
| **Storage** | In-memory | NDJSON → SQLite | Protobuf (compressed) | **Custom binary + mmap** |
| **Incremental** | N/A | Document events | Per-file (design goal) | **Fine-grained + invalidation** |
| **Type System** | LSP types | None | SymbolKind only | **Full type applications** |
| **Agent API** | LSP methods | Graph traversal | Query engine + CLI | **7 semantic tools + envelope** |

**INFERENCE**: LSIF is a **persistence layer for LSP**, not a semantic code model. Prime's goals (agent reasoning, impact analysis, cross-language semantics) require going beyond LSP request/response modeling.

---

## 11. Convergent Concepts Across SCIP, LSIF, Kythe

**FACT**: All three independently converge on:
1. **Symbol identity** — SCIP: string IDs; LSIF: monikers; Kythe: VName
2. **Definitions + references** — Core in all three
3. **Source locations** — Ranges/anchors with file positions
4. **Relationships** — Extends, implements, references (LSIF via nested referenceResults)
5. **Document/file as unit** — LSIF: document vertex; SCIP: Document message; Kythe: file node
6. **Incremental updates** — LSIF: events; SCIP: per-file; Kythe: extraction per compilation unit
7. **Persistent indexes** — All produce file-based or DB artifacts
8. **Provenance** — LSIF: project vertex; SCIP: Metadata.tool_name; Kythe: process nodes

**INFERENCE**: These are **necessary primitives** — Prime must include all.

**OBSERVATION**: LSIF's **moniker scheme translation** (`nextMoniker`) is a practical solution to cross-ecosystem identity that SCIP lacks and Kythe handles via VName corpus/language fields.

---

## 12. What Prime Should Borrow from LSIF

| Concept | Why | Confidence |
|---------|-----|------------|
| Moniker scheme + identifier + translation | Proven cross-ecosystem identity (tsc↔npm, maven, nuget) | **FACT** |
| ResultSet deduplication pattern | Efficient sharing of results across occurrences | **FACT** |
| Range containment + innermost-match lookup | Precise position-to-semantic mapping | **FACT** |
| Streaming events (begin/end) | Enables single-pass incremental consumption | **FACT** |
| Document/project hierarchy | Natural scoping for configuration + containment | **FACT** |
| Range tags (declaration/definition/reference) | Lightweight semantic annotation on locations | **OBSERVATION** |

---

## 13. What Prime Should NOT Borrow from LSIF

| Concept | Why Not | Confidence |
|---------|---------|------------|
| LSP request/response as primary model | Prime targets agent reasoning, not IDE feature parity | **FACT** |
| NDJSON graph encoding | Verbose; binary/columnar far more efficient for agents | **INFERENCE** |
| No standalone symbol table | Agents need symbol-centric queries, not range-centric | **FACT** |
| No type system | Agents need generics, bounds, variance for impact analysis | **FACT** |
| No call graph / data flow | Agents need control/data flow for reasoning | **FACT** |
| Vertex/edge graph in storage | Columnar/mmap better for analytical queries | **INFERENCE** |
| No per-fact provenance | PrimeEnvelope requires coverage, confidence, source | **FACT** |

---

## 14. Open Questions for Prime Research

1. **OPEN QUESTION**: Can LSIF's moniker translation (`nextMoniker`) be generalized to a **universal symbol resolution protocol** for Prime?

2. **OPEN QUESTION**: Is the ResultSet pattern necessary in Prime's storage, or does columnar deduplication make it obsolete?

3. **OPEN QUESTION**: LSIF's range tags provide lightweight semantic annotation — should Prime adopt a similar **anchor-tag** model alongside full symbol graph?

4. **OPEN QUESTION**: How to map LSIF's document-event streaming to Prime's incremental invalidation with cross-file dependencies?

5. **OPEN QUESTION**: LSIF dumps are workspace-scoped; Prime targets repository+dependencies. What's the composition model?

---

## 15. Cross-References

- **SCIP**: See `scip.md` for Protobuf transmission format, document-centric design, string symbol IDs
- **Kythe**: See `kythe.md` for VName, semantic graph schema, storage model, type system
- **Prime SPECS**: See `SPECS/01-codebase-knowledge/`, `SPECS/03-scip/`, `SPECS/04-lsif/`, `SPECS/05-kythe/`

---

## Summary

LSIF is a **pragmatic persistence format for LSP responses**, not a semantic code model. Its genius is modeling exactly what IDEs need (hover, definition, references) as a queryable graph, with monikers solving cross-repo identity in practice. For Prime, LSIF contributes **moniker-based cross-ecosystem identity**, **ResultSet deduplication**, **streaming events**, and **range-tag semantics** — but Prime must transcend the LSP request/response paradigm to build a true **language-agnostic semantic graph** with types, dataflow, and agent-centric metadata.