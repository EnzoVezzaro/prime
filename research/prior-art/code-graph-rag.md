# Code-Graph-RAG Prior Art Analysis

## Confidence Legend

| Label | Meaning |
|-------|---------|
| **FACT** | Verified by primary source (source code, official docs, technical specs) |
| **OBSERVATION** | Directly observed from implementation artifacts |
| **HYPOTHESIS** | Proposed explanation requiring validation |
| **INFERENCE** | Deduced from evidence, marked as such |
| **OPEN QUESTION** | Explicitly unknown, needs research |

---

## Executive Summary

Code-Graph-RAG (vitali87/code-graph-rag) is a production-grade multi-language code knowledge graph system that parses 14+ languages via Tree-sitter, builds a unified property graph in Memgraph, and exposes an MCP server for agent integration. Its graph schema, relationship types, and incremental/runtimesync capabilities are directly relevant to Prime's research on compact language-agnostic knowledge representations.

---

## Graph Schema — Entity Types

**FACT** — From `docs/architecture/graph-schema.md` and `docs/architecture/language-support.md`

Code-Graph-RAG defines **19 node labels** with explicit properties:

| Node Label | Key Properties | Notes |
|------------|----------------|-------|
| `Project` | `name` | Root container |
| `Package` | `qualified_name`, `name`, `path`, `absolute_path` | |
| `Folder` | `path`, `name`, `absolute_path` | |
| `File` | `path`, `name`, `extension?`, `absolute_path` | |
| `Module` | `qualified_name`, `name`, `path`, `absolute_path`, `flow_covered?`, `generated?`, `generator?`, `start_line?`, `end_line?` | Python modules, Rust crates, etc. |
| `Class` | `qualified_name`, `name`, `modifiers[]`, `decorators[]`, `path`, `absolute_path`, `start_col?`, `start_line?`, `end_line?`, `docstring?`, `is_exported?` | |
| `Function` | Class props + `is_macro?`, `name_start_line?`, `name_start_col?` | Includes macros (Rust `macro_rules!`, C `#define`) |
| `Method` | Class props + `is_property?`, `overrides_external?`, `name_start_line?`, `name_start_col?` | |
| `Interface` | `qualified_name`, `name`, `path`, `absolute_path`, `modifiers?[]`, `decorators?[]`, `start_col?`, `start_line?`, `end_line?`, `docstring?`, `is_exported?` | |
| `Enum` | Same as Interface | |
| `Type` | Same as Interface, `path`/`absolute_path` optional | Type aliases, unions |
| `Union` | Same as Type | |
| `ModuleInterface` | `qualified_name`, `name`, `path`, `absolute_path`, `module_type` | Backpack/ML-style signatures |
| `ModuleImplementation` | `qualified_name`, `name`, `path`, `absolute_path`, `implements_module`, `module_type` | |
| `ExternalPackage` | `name` | Third-party deps |
| `ExternalModule` | `qualified_name`, `name`, `path` | Imported external modules |
| `Resource` | `qualified_name`, `name`, `kind` | Synthetic I/O nodes: `FILE`, `ENV`, `NETWORK`, `DATABASE`, `STDIN`, `STDOUT`, `STDERR`, `SOCKET` |
| `Pattern` | `qualified_name`, `name`, `message`, `start_line`, `end_line`, `path`, `snippet?` | ast-grep structural findings |
| `CodeSmell` | Same as Pattern | |
| `SecurityIssue` | Same as Pattern | |

**OBSERVATION** — Qualified name uniqueness is handled by suffixing `@<start_line>` for duplicate definitions (e.g., `pkg.module.store_embedding@161`). This preserves all overload/fallback variants.

---

## Graph Schema — Relationship Types

**FACT** — From `docs/architecture/graph-schema.md` (explicit relationship table)

| Source | Relationship | Target | Capture Group |
|--------|-------------|--------|---------------|
| Project/Package/Folder | `CONTAINS_PACKAGE` | Package | structural |
| Project/Package/Folder | `CONTAINS_FOLDER` | Folder | structural |
| Project/Package/Folder | `CONTAINS_FILE` | File | structural |
| Project/Package/Folder | `CONTAINS_MODULE` | Module | structural |
| Module/Function/Method/Class | `DEFINES` | Class/Function/Method/Enum/Interface/Type/Union/Module | structural |
| Class/Interface/Enum/Type/Union | `DEFINES_METHOD` | Method | structural |
| Module | `IMPORTS` | Module/ExternalModule | imports |
| Module | `EXPORTS` | Class/Function | exports |
| Module | `EXPORTS_MODULE` | ModuleInterface | exports |
| ModuleInterface | `IMPLEMENTS_MODULE` | ModuleImplementation | exports |
| Class/Interface/Function | `INHERITS` | Class/Interface/Function/ExternalModule | inheritance |
| Class/Enum | `IMPLEMENTS` | Interface/Class/Enum/ExternalModule | inheritance |
| Method/Function | `OVERRIDES` | Method | inheritance |
| ModuleImplementation | `IMPLEMENTS` | ModuleInterface | exports |
| Project | `DEPENDS_ON_EXTERNAL` | ExternalPackage | imports |
| Module/Function/Method | `CALLS` | Function/Method/Enum/Type | calls (default) |
| Module/Function/Method | `REFERENCES` | Function/Method/Class | calls |
| Module/Function/Method | `INSTANTIATES` | Class | calls |
| Module/Function/Method | `READS_FROM` | Resource | io (opt-in) |
| Module/Function/Method | `WRITES_TO` | Resource | io (opt-in) |
| Module/Function/Method/Resource | `FLOWS_TO` | Module/Function/Method/Resource | io (opt-in) |
| Module | `IMPLEMENTS_PATTERN` | Pattern | findings (opt-in) |
| Module | `HAS_SMELL` | CodeSmell | findings (opt-in) |
| Module | `HAS_VULNERABILITY` | SecurityIssue | findings (opt-in) |

**OBSERVATION** — `FLOWS_TO` carries `kind` (`resource`, `arg`, `return`) and `via` (`arg:<index>`, `kw:<name>`, `return`) edge properties enabling intra-procedural taint tracking across call boundaries. Three-verdict query (`FOUND`/`NO_FLOW`/`UNKNOWN`) uses per-module `flow_covered` boolean.

---

## Relationship Types Prime Currently Misses

**INFERENCE** — Comparing Code-Graph-RAG's 25 relationship types against Prime's current schema (per SPECS/agent-native-interface.md and prime-core Entity/Relation types):

| Missing in Prime | Code-Graph-RAG Type | Semantic Value |
|-----------------|---------------------|----------------|
| `EXPORTS` | Module → Class/Function | Public API surface |
| `EXPORTS_MODULE` | Module → ModuleInterface | Signature exports |
| `IMPLEMENTS_MODULE` | ModuleInterface → ModuleImplementation | ML-style functors |
| `OVERRIDES` | Method → Method | Polymorphic dispatch target |
| `IMPLEMENTS` (class→interface) | Class/Enum → Interface | Explicit interface conformance |
| `READS_FROM` / `WRITES_TO` | Callable → Resource | I/O side-effect modeling |
| `FLOWS_TO` (3 shapes) | Resource↔Resource, Caller→Callee, Callee→Caller | Taint/data-flow provenance |
| `DEPENDS_ON_EXTERNAL` | Project → ExternalPackage | Supply-chain tracking |
| `CONTAINS_*` hierarchy | Project→Package→Folder→File→Module | Physical containment |
| `DEFINES_METHOD` | Class → Method | Ownership distinct from `DEFINES` |
| `IMPLEMENTS_PATTERN` / `HAS_SMELL` / `HAS_VULNERABILITY` | Module → Pattern/CodeSmell/SecurityIssue | Structural findings integration |
| `REFERENCES` (non-call) | Callable → Callable/Class | Callback/passed-as-value tracking |
| `INSTANTIATES` | Callable → Class | Construction sites |

**HYPOTHESIS** — Prime's current relation set (per SPECS) focuses on `CALLS`, `DEPENDS_ON`, `CONTAINS`, `DEFINES`, `INHERITS`, `IMPLEMENTS`. The missing types above enable: dead-code detection (via `EXPORTS` + reachability), data-flow analysis (`FLOWS_TO`), supply-chain queries (`DEPENDS_ON_EXTERNAL`), framework-aware dispatch (`OVERRIDES`), and structural lint findings as first-class graph nodes.

---

## Language Adapters & AST Mappings

**FACT** — From `docs/architecture/language-support.md` and `docs/architecture/graph-schema.md#language-specific-ast-mappings`

Code-Graph-RAG uses **Tree-sitter as the universal backbone** with language-specific AST node type mappings auto-generated from grammar specs. 14 languages fully supported:

| Language | Extensions | Key AST Node Types Captured |
|----------|------------|----------------------------|
| Python | `.py` | `class_definition`, `function_definition` |
| TypeScript | `.ts`, `.mts`, `.cts` | `class_declaration`, `function_declaration`, `interface_declaration`, `type_alias_declaration`, `enum_declaration`, `method_definition`, `arrow_function` |
| TSX | `.tsx` | All TS + JSX elements |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` | `class`, `class_declaration`, `function_declaration`, `function_expression`, `arrow_function`, `method_definition`, `generator_function` |
| Rust | `.rs` | `function_item`, `struct_item`, `enum_item`, `trait_item`, `impl_item`, `macro_definition`, `closure_expression`, `type_item`, `union_item` |
| Go | `.go` | `function_declaration`, `method_declaration`, `type_spec`, `type_alias` |
| Java | `.java` | `class_declaration`, `interface_declaration`, `method_declaration`, `constructor_declaration`, `record_declaration`, `enum_declaration`, `annotation_type_declaration` |
| C | `.c` | `function_definition`, `struct_specifier`, `enum_specifier`, `union_specifier` |
| C++ | `.cpp`, `.h`, `.hpp`, ... | `class_specifier`, `function_definition`, `struct_specifier`, `template_declaration`, `lambda_expression`, `enum_specifier`, `union_specifier` |
| C# | `.cs` | `class_declaration`, `method_declaration`, `constructor_declaration`, `property_declaration`, `interface_declaration`, `record_declaration`, `struct_declaration`, `enum_declaration` |
| PHP | `.php` | `class_declaration`, `function_definition`, `method_declaration`, `interface_declaration`, `trait_declaration`, `enum_declaration`, `arrow_function`, `anonymous_function` |
| Lua | `.lua` | `function_declaration`, `function_definition` |
| Dart | `.dart` | `class_definition`, `constructor_signature`, `factory_constructor_signature`, `function_signature`, `getter_signature`, `setter_signature`, `mixin_declaration`, `extension_declaration` |
| Scala | `.scala`, `.sc` | `class_definition`, `object_definition`, `trait_definition`, `function_declaration`, `function_definition` (in dev) |

**OBSERVATION** — Ruby supported via **ast-grep pluggable tier** (YAML pattern file) emitting `Module`, `Function`, `Class` + import edges without hand-written parser.

**OBSERVATION** — C/C++ and C# use **hybrid frontends**: Tree-sitter backbone + libclang/Roslyn for semantic facts (macros, overload resolution, partial types, LINQ). Falls back to pure Tree-sitter if semantic tooling unavailable.

---

## Incremental Updates & Graph Sync

**FACT** — From `docs/guide/realtime-updates.md` (referenced), `realtime_updater.py`, and `server.json`

- **File-level invalidation**: On file change, re-parse affected file, compute diff of symbols/relationships, apply minimal graph updates
- **Batch ingestion**: `cgr start --repo-path --update-graph` processes changed files only
- **Graph sync**: Shared Memgraph instance across multiple repos; `--clean` wipes all projects
- **Real-time watcher**: `realtime_updater.py` monitors filesystem, triggers incremental re-index

**OBSERVATION** — No Merkle-tree or content-addressed invalidation described; relies on file mtime/size change detection.

---

## Dynamic Call Tracing (Runtime Overlay)

**FACT** — From README, `docs/guide/dynamic-tracing.md`, NEWS.md

- `cgr trace` runs test suites (Python, JVM, Node.js, .NET, PHP, Lua, Dart, Go, Rust, C/C++) and merges **actual runtime calls** as `CALLS` edges flagged `static_missed: true`
- Ingests production eBPF profiles (Parca, Pyroscope, OpenTelemetry) via `cgr trace convert --format ebpf`
- Dynamic edges carry: `dynamic`, `dynamic_call_count`, `dynamic_workloads`, `dynamic_workload_count`, `dynamic_receiver_types`
- Exposes dispatch through interfaces, virtual methods, function pointers, reflection, framework routing

**INFERENCE** — This is a **semantic overlay** on the static graph, not a replacement. Prime could adopt a similar layered approach: static graph + optional runtime provenance annotations.

---

## Structural Search & Replace (ast-grep Integration)

**FACT** — From README, NEWS.md, `docs/architecture/graph-schema.md`

- ast-grep patterns exposed as agent tools for **match and transform structure** across codebase
- Findings (`Pattern`, `CodeSmell`, `SecurityIssue`) become graph nodes with `IMPLEMENTS_PATTERN`, `HAS_SMELL`, `HAS_VULNERABILITY` edges
- Opt-in via `findings` capture group

---

## MCP Integration

**FACT** — From README and `docs/guide/mcp-server.md`

- Runs as MCP server (stdio transport) exposing graph query, edit, optimize tools
- Natural language → Cypher → graph results → response pipeline
- Tools: query, retrieve source, edit (AST-based surgical patching), optimize, dead-code detection, structural search/replace, dynamic tracing

---

## What Prime Should Borrow

| Feature | Rationale | Evidence |
|---------|-----------|----------|
| **Unified multi-language graph schema** | Single query surface across polyglot repos | FACT: 14 languages, shared node/rel labels |
| **`FLOWS_TO` with `kind`/`via` edge properties** | Compact taint provenance without full PDG | FACT: Three shapes, single relationship type |
| **`READS_FROM`/`WRITES_TO` + `Resource` nodes** | I/O side effects as queryable graph structure | FACT: 8 resource kinds, registry-extensible |
| **`OVERRIDES` relationship** | Precise polymorphic dispatch targets | FACT: Distinct from `INHERITS`/`IMPLEMENTS` |
| **`EXPORTS`/`EXPORTS_MODULE`** | Public API boundary detection | FACT: Module-level export tracking |
| **Three-verdict flow query (`FOUND`/`NO_FLOW`/`UNKNOWN`)** | Honest coverage-aware reachability | FACT: Per-module `flow_covered` boolean |
| **Dynamic call tracing overlay** | Runtime truth for virtual/reflective dispatch | FACT: `static_missed: true` flag, multi-runtime |
| **ast-grep findings as graph nodes** | Structural patterns as first-class knowledge | FACT: `Pattern`/`CodeSmell`/`SecurityIssue` nodes |
| **Hybrid frontend pattern** | Tree-sitter backbone + semantic layer for hard languages | FACT: C/C++ libclang, C# Roslyn, fallback to TS |
| **Qualified name disambiguation via `@line`** | Preserves overload/fallback variants | FACT: `pkg.module.fn@161` suffix scheme |

---

## What Prime Should NOT Borrow

| Feature | Rationale | Evidence |
|---------|-----------|----------|
| **Memgraph/property graph backend** | Prime targets compact binary/mmap storage, not general graph DB | INFERENCE: Prime's design goals favor succinct structures over Cypher |
| **Full Cypher query generation** | Prime's agent API is tool-based (7 semantic tools), not arbitrary graph query | FACT: PrimeEnvelope<T> tools vs. open Cypher |
| **Runtime eBPF ingestion pipeline** | Out of scope for static knowledge representation | INFERENCE: Prime focuses on static analysis |
| **YAML rule configuration for findings** | Prime's query primitives are fixed semantic tools | INFERENCE: Different abstraction level |
| **File-mtime-based incremental only** | Prime investigates Merkle-tree/content-addressed invalidation | HYPOTHESIS: Prime's research includes CRDT/Merkle approaches |

---

## Open Questions

1. **OPEN QUESTION**: Can Prime's compact binary format represent `FLOWS_TO` edge properties (`kind`, `via`) more space-efficiently than Memgraph's property graph? What's the bits-per-edge cost?

2. **OPEN QUESTION**: Code-Graph-RAG's `FLOWS_TO` is intra-procedural + one-level inter-procedural. Prime's research includes succinct data structures — could a compressed transitive closure (Elias-Fano encoded reachability) replace the fixpoint worklist?

3. **OPEN QUESTION**: The hybrid frontend (Tree-sitter + libclang/Roslyn) duplicates parsing effort. Could Prime's language-agnostic semantic model subsume the semantic layer via a unified "capability model" (per SPECS/15-language-agnosticism)?

4. **OPEN QUESTION**: Code-Graph-RAG's `DEFINES` from Function/Method (nested definitions) — does Prime's current `CONTAINS` hierarchy support this, or is a distinct `DEFINES_IN_SCOPE` relation needed?

5. **OPEN QUESTION**: The `Resource` node model (8 kinds, `resource::<KIND>::<identity>`) — can Prime's entity model absorb this without dedicated node types, using typed edges instead?

6. **OPEN QUESTION**: Three-verdict query requires per-module `flow_covered` boolean. Prime's coverage metadata (per SPECS/agent-native-interface.md) — is this granular enough, or does Prime need per-entity coverage?

7. **OPEN QUESTION**: `OVERRIDES` vs `INHERITS` distinction — Code-Graph-RAG uses both. Prime currently has `INHERITS`. Is `OVERRIDES` derivable from `INHERITS` + method name match, or does it require semantic analysis (vtable layout)?

8. **OPEN QUESTION**: Code-Graph-RAG captures `IMPLEMENTS` for Class→Interface AND Enum→Interface. Prime's relation types — does `IMPLEMENTS` cover both, or are they distinct?

---

## References

- [Code-Graph-RAG Graph Schema](https://github.com/vitali87/code-graph-rag/blob/main/docs/architecture/graph-schema.md) — Primary source for node/relationship definitions
- [Code-Graph-RAG Data-Flow Edges](https://github.com/vitali87/code-graph-rag/blob/main/docs/architecture/data-flow-edges.md) — Primary source for `FLOWS_TO` semantics
- [Code-Graph-RAG Language Support](https://github.com/vitali87/code-graph-rag/blob/main/docs/architecture/language-support.md) — Primary source for language matrix
- [Code-Graph-RAG README](https://github.com/vitali87/code-graph-rag) — Primary source for architecture overview, MCP, dynamic tracing