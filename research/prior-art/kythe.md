# Kythe: Google's Semantic Code Graph — Prior Art Analysis

## Confidence Legend

| Label | Meaning |
|-------|---------|
| **FACT** | Verified by primary source (kythe.io docs, schema, storage model, whitepaper) |
| **OBSERVATION** | Directly observed in implementation or documented behavior |
| **HYPOTHESIS** | Proposed explanation requiring validation |
| **INFERENCE** | Deduced from evidence, marked as such |
| **OPEN QUESTION** | Explicitly unknown, needs research |

---

## 1. Overview and Motivation

**FACT**: Kythe is a "pluggable, (mostly) language-agnostic ecosystem for building tools that work with code" — [kythe.io](https://kythe.io), [README.adoc](https://github.com/kythe/kythe/blob/master/README.adoc).

**FACT**: Founded at Google to index their "enormous, multi-lingual internal codebase" — [kythe-overview.html](https://kythe.io/docs/kythe-overview.html#background).

**FACT**: Core premise: **hub-and-spoke model** reduces integration complexity from O(L×C×B) to O(L+C+B) for L languages, C clients, B build systems — [kythe-overview.html](https://kythe.io/docs/kythe-overview.html#goals-of-kythe).

**FACT**: Non-goals: writing compilers, replacing existing IRs, achieving UNCOL — [kythe-overview.html](https://kythe.io/docs/kythe-overview.html#non-goals-of-kythe).

**FACT**: "Interoperability should not be 'all-or-nothing'" — tools must handle incomplete data gracefully — [kythe-overview.html](https://kythe.io/docs/kythe-overview.html#goals-of-kythe).

---

## 2. Graph Data Model

### 2.1 Nodes as Vectors (VName)

**FACT**: Every node identified by **VName** (Vector-Name) — 5 fields — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#TermVName):

| Field | Purpose | Example |
|-------|---------|---------|
| `signature` | Analyzer-generated opaque ID | `com.google.Foo#bar()` |
| `corpus` | Source collection | `chromium`, `aosp`, `github.com/user/repo` |
| `root` | Corpus subset (branch, generated) | `third_party/openssl`, `generated` |
| `path` | File path relative to corpus+root | `src/main/java/Foo.java` |
| `language` | Language label | `java`, `c++`, `go`, `python` |

**FACT**: VName is extensible — add dimensions to resolve collisions — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#TermVName).

**FACT**: Kythe URI encodes VName: `kythe://corpus?lang=java?path=Foo.java#signature` — [kythe-uri-spec.html](https://kythe.io/docs/kythe-uri-spec.html).

### 2.2 Facts and Edges

**FACT**: **Fact** = (name, value) string pair; names path-structured: `/kythe/node/kind`, `/kythe/code` — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#TermFact).

**FACT**: **Entry** = (source_ticket, kind, target_ticket, fact_label, fact_value) — atomic storage unit — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#Entry).

**FACT**: If `kind` and `target` empty → node fact; else → edge fact — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#Entry).

---

## 3. Schema: Node Kinds and Edge Kinds

### 3.1 Node Kinds (from schema reference)

**FACT**: 25+ node kinds defined — [schema reference](https://kythe.io/docs/schema/index.html#_node_kinds):

| Kind | Purpose |
|------|---------|
| `anchor` | Source location (span in file) |
| `file` | Source file |
| `function` | Function/method |
| `record` | Class/struct/record |
| `interface` | Interface |
| `variable` | Variable/field |
| `constant` | Constant |
| `type` aliases: `tapp`, `tnominal`, `tbuiltin`, `tvar`, `talias`, `tsigma` |
| `name` | External identifier |
| `package` | Package/module |
| `process` | Build process |
| `diagnostic` | Error/warning |
| `doc` | Documentation |

### 3.2 Edge Kinds (50+ defined)

**FACT**: Key semantic edges — [schema reference](https://kythe.io/docs/schema/index.html#edge_kinds):

| Edge | Source → Target | Purpose |
|------|-----------------|---------|
| `defines` | anchor → semantic node | Anchor defines semantic entity |
| `defines/binding` | anchor → semantic node | Identifier binds to entity |
| `defines/implicit` | anchor → semantic node | Implicit definition (dtor, module) |
| `ref` | anchor → semantic node | Reference to entity |
| `ref/call` | anchor → function | Call site |
| `ref/call/direct` | anchor → function | Direct (non-virtual) call |
| `ref/implicit` | anchor → semantic node | Implicit reference |
| `childof` | any → semantic node | Containment (lexical) |
| `childof/context` | anchor → instantiation | Template instantiation context |
| `extends` | record/interface → type | Inheritance |
| `overrides` | function → function | Method override |
| `instantiates` | semantic node → tapp | Template/generic instantiation |
| `specializes` | semantic node → tapp | Template specialization |
| `aliases` | talias → type | Type alias |
| `aliases/root` | talias → type | Ultimate aliased type |
| `typed` | variable/function → type | Type of entity |
| `param.N` | tapp/function → type | Nth type parameter |
| `tparam.N` | function/record → tvar | Nth template parameter |
| `bounded/upper` | tvar → type | Upper bound |
| `bounded/lower` | tvar → type | Lower bound |
| `satisfies` | type → interface | Structural satisfaction (Go) |
| `generates` | semantic/file → semantic/file | Code generation |
| `denotes` | concrete → abstract | Concrete represents abstract |
| `imputes` | anchor → semantic node | Generated code attribution |
| `ref/writes` | anchor → variable | Write reference |
| `ref/writes/thunk` | anchor → variable | Indirect write |
| `influences` | variable → variable | Data flow (experimental) |
| `property/reads` | function → variable | Property read |
| `property/writes` | function → variable | Property write |
| `completedby` | declaration → definition | Declaration completed by definition |
| `documents` | anchor/doc → semantic node | Documentation comment |
| `ref/doc` | anchor → semantic node | Reference in documentation |
| `named` | semantic node → name | External name (JVM binary name) |
| `tagged` | anchor/file → diagnostic | Diagnostic attachment |
| `exports` | process → process | Build rule exports |

**FACT**: Reverse edges derived at serving time (e.g., `ref` → `%/kythe/edge/ref`) — [schema reference](https://kythe.io/docs/schema/index.html#edge_kinds).

---

## 4. Type System Modeling

**FACT**: Rich type representation via nodes and edges — [schema reference](https://kythe.io/docs/schema/index.html#_node_kinds):

| Type Node | Purpose |
|-----------|---------|
| `tapp` | Type application (e.g., `List<String>`) |
| `tnominal` | Nominal type (class/interface name) |
| `tbuiltin` | Builtin (int, string) |
| `tvar` | Type variable (generic parameter) |
| `talias` | Type alias (typedef, using) |
| `tsigma` | Existential/dependent type |

**FACT**: `param.N` edges on `tapp` encode type arguments in order — [schema reference](https://kythe.io/docs/schema/index.html#param).

**FACT**: `tparam.N` edges on generic declarations encode type parameters — [schema reference](https://kythe.io/docs/schema/index.html#tparam).

**FACT**: `bounded/upper` and `bounded/lower` on `tvar` encode constraints — [schema reference](https://kythe.io/docs/schema/index.html#boundedupper-or-boundedlower).

**FACT**: `instantiates` + `specializes` separate **instantiation** (monomorphization) from **specialization** (alternate template body) — critical for C++ — [schema reference](https://kythe.io/docs/schema/index.html#instantiates).

**INFERENCE**: This is the **most sophisticated cross-language type model** of the three systems — SCIP has only `SymbolKind`; LSIF has none.

---

## 5. Storage Model

### 5.1 Entry Format

**FACT**: Each entry: `(source_ticket, kind, target_ticket, fact_label, fact_value)` — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#Entry).

**FACT**: Standard entry order: Source, Kind, Target, Fact, Value (lexicographic) — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#Ordering).

**FACT**: Goals: simplicity, compactness (1NF), neutrality, portability, composability — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#goals).

**FACT**: Non-goals: query efficiency, schematization — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#non-goals).

### 5.2 Service Interface

**FACT**: Graph store interface: `Read(source, kind)`, `Write(source, updates)`, `Scan(target, kind, fact)`, `Shard(index, n)` — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#service-interface).

**FACT**: `Read` complexity proportional to return set; enables forward-graph traversal — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#reading).

**FACT**: `Scan` allows full-table scans for reverse lookups — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#scanning).

**FACT**: `Shard` by fingerprint of `source|kind` for MapReduce — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#sharding).

### 5.3 Implementations

**FACT**: LevelDB implementation (`leveldb/leveldb.go`) — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#graph-store-tools).

**FACT**: SQL schema (Tickets, Nodes, Edges tables) — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#sql).

**FACT**: Tools: `write_entries`, `read_entries`, `triples` (RDF), `directory_indexer` — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#graph-store-tools).

---

## 6. Indexing Pipeline

**FACT**: Build extractors → emit compilation database (KCD) → run indexers → emit Kythe entries → aggregate into graph store — [kythe-overview.html](https://kythe.io/docs/kythe-overview.html#what-kythe-provides).

**FACT**: Compilation extractors for: javac, Maven, CMake, Go, Bazel — [README.adoc](https://github.com/kythe/kythe/blob/master/README.adoc).

**FACT**: Indexers for: C++, Java, Go (in-tree); others via community — [kythe-overview.html](https://kythe.io/docs/kythe-overview.html#what-kythe-provides).

**FACT**: `.kzip` format packages compilation units for transport — [kythe-kzip.html](https://kythe.io/docs/kythe-kzip.html).

---

## 7. Cross-Language Representation

**FACT**: VName includes `language` field — enables language-specific subgraphs — [kythe-storage.html](https://kythe.io/docs/kythe-storage.html#TermVName).

**FACT**: Common edge kinds (`defines`, `ref`, `childof`, `extends`) shared across languages — [schema reference](https://kythe.io/docs/schema/index.html#edge_kinds).

**FACT**: Language-specific rules in schema (C++, Go, Java, Protobuf, Common Lisp) — [schema reference](https://kythe.io/docs/schema/index.html#language-specific-rules).

**FACT**: Cross-language linking via `generates`, `denotes`, `named` edges — [schema reference](https://kythe.io/docs/schema/index.html#generates).

**INFERENCE**: Kythe's approach: **common semantic core + language-specific extensions** — unlike SCIP's per-language schemes or LSIF's moniker translation.

---

## 8. Agent Reasoning Relevance

### 8.1 What Kythe Provides

| Capability | Kythe Support | Notes |
|------------|---------------|-------|
| Symbol lookup | **FACT** Yes | VName → entries |
| Definition → references | **FACT** Yes | `defines/binding` + reverse `ref` |
| Call graph | **FACT** Yes | `ref/call`, `ref/call/direct` |
| Type hierarchy | **FACT** Yes | `extends`, `instantiates`, `specializes` |
| Generic instantiation | **FACT** Yes | `tapp` + `param.N` + `instantiates` |
| Template specialization | **FACT** Yes | `specializes` distinct from `instantiates` |
| Type bounds/variance | **FACT** Yes | `bounded/upper`, `bounded/lower`, `tvar` |
| Data flow (experimental) | **FACT** Partial | `influences`, `ref/writes`, `ref/writes/thunk` |
| Control flow | **NO** | Not modeled |
| Cross-repo | **FACT** Yes | Corpus + root in VName |
| Code generation tracking | **FACT** Yes | `generates`, `denotes`, `imputes` |
| Build dependency graph | **FACT** Yes | `process` nodes + `depends` edges |
| Provenance | **PARTIAL** | Process nodes track build; no per-fact confidence |

### 8.2 Redundant or Compressible for Prime

**HYPOTHESIS**: `anchor` nodes (one per source span) are extremely numerous — could be compressed via range encoding (like SCIP's variable-length arrays).

**HYPOTHESIS**: `childof` edges for anchors are implicit from VName (corpus+path+root) — [schema reference](https://kythe.io/docs/schema/index.html#childof) notes this optimization.

**HYPOTHESIS**: `tapp` nodes for every type application create explosion — could use structural sharing / hash-consing.

**HYPOTHESIS**: Entry format (5-tuple) is verbose in raw form; columnar compression would help.

### 8.3 Missing for Agent Reasoning

**FACT**: No **control flow graph** (CFG) — Kythe focuses on semantic cross-references.

**FACT**: No **per-fact confidence/provenance** — all facts equal weight.

**FACT**: No **incremental invalidation metadata** in storage model.

**FACT**: No **agent-centric envelope** (coverage, status, source_required).

**OPEN QUESTION**: Kythe's `influences` edge is experimental — is it sufficient for dataflow-based impact analysis?

---

## 9. Comparison: Live LSP vs Persistent LSIF vs Compact Prime Artifact

| Dimension | Live LSP | LSIF (Persistent) | Kythe (Graph Store) | Prime (Target) |
|-----------|----------|-------------------|---------------------|----------------|
| **Data Model** | Request/response | LSP response graph | Semantic graph (nodes/edges/facts) | **Language-agnostic semantic graph** |
| **Symbol Identity** | None | Monikers (scheme-based) | **VName (corpus/root/path/lang/sig)** | **Content-addressed + semantic** |
| **Type System** | LSP types | None | **Full (tapp, tvar, bounds, instantiate)** | **Full + cross-lang** |
| **Cross-Language** | No | Moniker translation | **VName + common edges + generates** | **Unified semantic model** |
| **Storage** | Memory | NDJSON → SQLite | LevelDB / SQL / custom | **Custom binary + mmap** |
| **Query** | LSP methods | Graph traversal | Read/Scan/Shard API | **7 semantic tools + streaming** |
| **Incremental** | N/A | Document events | Per-compilation-unit | **Fine-grained + invalidation** |
| **Provenance** | No | Project vertex | Process nodes | **Per-fact + PrimeEnvelope** |

**INFERENCE**: Kythe is the **closest to Prime's vision** — a true language-agnostic semantic graph with rich type system. Prime adds: agent-centric API, mmap storage, confidence/provenance, compression, CRDT distribution.

---

## 10. Convergent Concepts Across SCIP, LSIF, Kythe

**FACT**: All three independently converge on:

1. **Symbol identity** — SCIP: string IDs; LSIF: monikers; **Kythe: VName (most complete)**
2. **Definitions + references** — Core in all three
3. **Source locations** — SCIP: ranges; LSIF: ranges; **Kythe: anchors (most granular)**
4. **Relationships** — All have extends, implements, references, calls
5. **Document/file as unit** — SCIP: Document; LSIF: document vertex; **Kythe: file node + anchors**
6. **Incremental updates** — SCIP: per-file; LSIF: events; **Kythe: per-compilation-unit**
7. **Persistent indexes** — All produce queryable artifacts
8. **Provenance** — SCIP: tool_name; LSIF: project; **Kythe: process nodes (most detailed)**

**INFERENCE**: Kythe's VName is the **most robust identity model** (includes corpus, root, path, language, signature). Prime should consider VName-inspired identity.

**INFERENCE**: Kythe's **type system modeling** (tapp, tvar, bounds, instantiates/specializes) is the **only one sufficient for cross-language generics reasoning**.

---

## 11. What Prime Should Borrow from Kythe

| Concept | Why | Confidence |
|---------|-----|------------|
| VName (corpus, root, path, language, signature) | Robust, extensible, cross-language identity | **FACT** |
| Fact/entry storage model | Simple, composable, portable, columnar-friendly | **FACT** |
| Rich type system (tapp, tvar, bounds, instantiates) | Essential for cross-language generic reasoning | **FACT** |
| Common semantic edges + language-specific extensions | Balances uniformity with expressiveness | **FACT** |
| `generates`/`denotes`/`imputes` for codegen | Tracks generated code provenance | **FACT** |
| Process nodes + depends edges | Build graph integration for impact analysis | **FACT** |
| Reverse-edge derivation at serve time | Storage efficiency (store forward only) | **FACT** |
| Sharding by source|kind | Enables distributed processing | **FACT** |
| Kythe URI for textual tickets | Human-readable, canonical, invertible | **FACT** |

---

## 12. What Prime Should NOT Borrow from Kythe

| Concept | Why Not | Confidence |
|---------|---------|------------|
| LevelDB/SQL as primary storage | Prime targets mmap + custom binary for agent latency | **INFERENCE** |
| Per-anchor nodes (extreme granularity) | Too verbose; SCIP's range arrays more compact | **OBSERVATION** |
| No per-fact confidence/provenance | PrimeEnvelope requires this | **FACT** |
| Experimental `influences` only | Prime needs full dataflow (reaching defs, slices) | **HYPOTHESIS** |
| Separate serving layer required | Prime wants direct mmap query | **INFERENCE** |
| No compression in storage model | Prime needs semantic compression | **FACT** |
| No CRDT/distribution | Prime targets distributed knowledge sharing | **FACT** |

---

## 13. Open Questions for Prime Research

1. **OPEN QUESTION**: Can Kythe's VName be adapted to **content-addressed identity** (Merkle DAG) while preserving cross-corpus resolution?

2. **OPEN QUESTION**: Kythe's `tapp` nodes explode combinatorially — can Prime use **structural sharing + hash-consing** to compress?

3. **OPEN QUESTION**: How to map Kythe's per-compilation-unit indexing to Prime's **file-level incremental invalidation** with cross-file dependencies?

4. **OPEN QUESTION**: Kythe's `influences` is experimental — what **dataflow facts** does Prime actually need for agent impact analysis?

5. **OPEN QUESTION**: Kythe's process nodes model build graph — should Prime include **build semantics** or only **source semantics**?

6. **OPEN QUESTION**: Kythe URI scheme is canonical — should Prime adopt it as **external reference format**?

---

## 14. Cross-References

- **SCIP**: See `scip.md` for Protobuf transmission format, document-centric design, string symbol IDs
- **LSIF**: See `lsif.md` for LSP response graph, monikers, ResultSet, streaming events
- **Prime SPECS**: See `SPECS/01-codebase-knowledge/`, `SPECS/03-scip/`, `SPECS/04-lsif/`, `SPECS/05-kythe/`

---

## Summary

Kythe is the **most semantically complete** of the three systems — a true language-agnostic semantic graph with VName identity, rich type modeling (generics, instantiation, specialization, bounds), code generation tracking, and build graph integration. Its storage model (entries) is simple and composable. For Prime, Kythe contributes the **core semantic architecture**: VName-inspired identity, fact/entry storage, type system, common edges, codegen/build tracking. Prime must add: **agent-centric envelope**, **mmap-optimized binary format**, **semantic compression**, **confidence/provenance per fact**, **incremental invalidation metadata**, and **distributed CRDT sync** — while possibly compressing Kythe's verbose anchor granularity.