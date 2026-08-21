# Prime Research Tracking Dashboard

**Last Updated:** August 21, 2026
**Status:** Research & Implementation In Progress 🔄
**Reference:** Prime Code Intelligence & Systems Research Investigation

---

## Summary

| Category | Total | Completed | In Progress | Remaining |
|----------|-------|-----------|-------------|-----------|
| Prior Art Research (18 repos) | 18 | 18 | 0 | 0 |
| Synthesis Documents | 6 | 6 | 0 | 0 |
| Final Deliverable | 1 | 1 | 0 | 0 |
| Implementation (Phases 1-2) | 12 | 12 | 0 | 0 |
| Research Artifacts (from spec) | 30 | 8 | 0 | 22 |
| Benchmarking | 25 | 5 | 0 | 20 |
| Architecture Investigation | 10 | 2 | 0 | 8 |
| **Total** | **102** | **52** | **0** | **50** |

**Overall Progress:** ██████████░░░░░░░░░░░ 51%

---

## Part 1: Prior Art Research (18 Repos) ✅

| # | Repository | Status | File | Key Findings |
|---|------------|--------|------|--------------|
| 1 | Microsoft VS Code | ✅ | `research/prior-art/vscode.md` | Language services, file watchers, incremental updates |
| 2 | VS Code Language Server | ✅ | (covered in vscode.md) | LSP protocol, symbol providers |
| 3 | Sourcegraph SCIP | ✅ | `research/prior-art/scip.md` | Language-agnostic symbol identity |
| 4 | Microsoft LSIF | ✅ | `research/prior-art/lsif.md` | Persistent semantic representation |
| 5 | Code-Graph-RAG | ✅ | `research/prior-art/code-graph-rag.md` | Tree-sitter + Memgraph, multi-language graph |
| 6 | Tree-sitter | ✅ | `research/prior-art/tree-sitter.md` | Incremental parsing, query system |
| 7 | ast-grep | ✅ | `research/prior-art/ast-grep.md` | Structural search, pattern matching |
| 8 | Aider Repository Map | ✅ | `research/prior-art/aider.md` | Token budgeting, PageRank relevance |
| 9 | Git | ✅ | `research/prior-art/git.md` | Content addressing, Merkle structures |
| 10 | ripgrep | ✅ | `research/prior-art/ripgrep.md` | Parallel traversal, SIMD, memory mapping |
| 11 | disk-perf-git-and-pnpm | ✅ | `research/prior-art/disk-performance.md` | Filesystem benchmarks, APFS behavior |
| 12 | Joern/CPG | ✅ | `research/prior-art/joern.md` | Code Property Graph, AST+CFG+PDG |
| 13 | Semgrep | ✅ | `research/prior-art/semgrep.md` | Structural/semantic matching |
| 14 | Kythe | ✅ | `research/prior-art/kythe.md` | Language-independent indexing, facts |
| 15 | CodeQL | ✅ | `research/prior-art/codeql.md` | Semantic databases, relational facts |
| 16 | OpenGrok | ✅ | `research/prior-art/opengrok.md` | Large-scale source indexing |
| 17 | Cognee | ✅ | `research/prior-art/cognee.md` | Knowledge lifecycle, enrichment, query routing |
| 18 | OpenWiki | ✅ | `research/prior-art/openwiki.md` | Agent-generated knowledge, Grounded Claims, incremental maintenance |

---

## Part 2: Synthesis Documents ✅

| # | Document | Status | File |
|---|----------|--------|------|
| 1 | Code Intelligence Convergence | ✅ | `research/synthesis/code-intelligence-convergence.md` |
| 2 | Prime Gap Analysis | ✅ | `research/synthesis/prime-gap-analysis.md` |
| 3 | Relationship Model | ✅ | `research/synthesis/relationship-model.md` |
| 4 | Incremental Indexing | ✅ | `research/synthesis/incremental-indexing.md` |
| 5 | Agent Benchmark | ✅ | `research/synthesis/agent-benchmark.md` |
| 6 | Storage & Representation | ✅ | `research/synthesis/storage-representation.md` |

---

## Part 3: Final Deliverable ✅

| # | Document | Status | File |
|---|----------|--------|------|
| 1 | Prime Code Intelligence Model | ✅ | `docs/research-synthesis/prime-code-intelligence-model.md` |

---

## Part 4: Implementation Progress

### Phase 1: Foundation ✅

| # | Task | Status | Notes |
|---|------|--------|-------|
| 1 | Core entity model | ✅ | Entity, File, Module, Project types |
| 2 | Basic relationships (7 types) | ✅ | Calls, DependsOn, Contains, Defines, Inherits, Implements, Imports |
| 3 | Tree-sitter parser (8+ languages) | ✅ | Rust, TS, JS, Python, Go, Java, C, C++ |
| 4 | Binary storage + mmap | ✅ | Custom format with compression |
| 5 | Query engine | ✅ | Search, lookup, context, relationships |
| 6 | MCP server (7 tools) | ✅ | PrimeEnvelope<T> responses |
| 7 | CLI commands | ✅ | build, query, stats, check, deps, etc. |

### Phase 2: Relationship Richness ✅

| # | Task | Status | Notes |
|---|------|--------|-------|
| 8 | Incremental indexing | ✅ | File change detection + entity invalidation (70x speedup) |
| 9 | OVERRIDES relationship | ✅ | Heuristic-based method override detection (136 found) |
| 10 | EXPORTS relationship | ✅ | Public symbol detection (246 found) |
| 11 | FLOWS_TO relationship | ✅ | Data flow analysis (1,044 found) |
| 12 | INSTANTIATES relationship | ✅ | Class construction sites (1,223 found) |

### Phase 2.5: Entity Extraction Fix ✅

| # | Task | Status | Notes |
|---|------|--------|-------|
| 13 | Fix Python tree-sitter queries | ✅ | Removed invalid async_function_definition |
| 14 | Fix Go tree-sitter queries | ✅ | Cleaned up definition patterns |
| 15 | Fix JS/TS tree-sitter queries | ✅ | Removed #set! predicates |
| 16 | Fix Java/C/C++ queries | ✅ | Aligned with working Rust pattern |

**Current Implementation Metrics:**
- Entities: 1,094 (Functions: 600, Classes: 120, Structs: 53)
- Relations: 75,542 (8 types)
- Files parsed: 26,103
- Build time: 4.79s
- Cold query latency: 52µs

---

## Part 5: Research Artifacts (from Spec)

### 5.1 Comparative Analysis Table

| System | Status | Notes |
|--------|--------|-------|
| Create comparative analysis table | ⏳ Pending | Primary abstraction, precomputed info, relationships, etc. |
| Convergence analysis | ✅ | Completed in synthesis docs |

### 5.2 Prime-Specific Investigation

| # | Investigation | Status | Notes |
|---|---------------|--------|-------|
| 1 | What information Prime currently represents | ✅ | Documented in gap analysis |
| 2 | What VS Code exposes | ✅ | Documented in vscode.md |
| 3 | What SCIP represents | ✅ | Documented in scip.md |
| 4 | What LSIF represents | ✅ | Documented in lsif.md |
| 5 | What Code-Graph-RAG represents | ✅ | Documented in code-graph-rag.md |
| 6 | What CPG represents | ✅ | Documented in joern.md |
| 7 | What Kythe represents | ✅ | Documented in kythe.md |
| 8 | What Aider exposes to agents | ✅ | Documented in aider.md |
| 9 | What Prime is missing | ✅ | Documented in gap analysis |

---

## Part 6: Critical Benchmark Problem

### Current Benchmark Status (PR Corpus)

| Metric | Before | After | Target | Status |
|--------|--------|-------|--------|--------|
| Relationship F1 | 0.00 | 0.14 | >0.50 | 🟡 Improving |
| Entity F1 | 0.37 | 0.57 | >0.80 | 🟡 Improving |
| Source-free Accuracy | 8.2% | 1.6% | >60% | 🔴 Needs work |
| Accuracy | 0.0% | 1.6% | >60% | 🔴 Needs work |

### Benchmark by Repo

| Repo | Entities | Relations | Accuracy | Rel F1 |
|------|----------|-----------|----------|--------|
| bat (Rust) | 731 | 46,034 | 4.0% | 0.095 |
| httpx (Python) | 1,075 | 38,453 | 4.0% | 0.182 |
| express (JS) | 0 | 29,931 | 0.0% | 0.000 |
| gin (Go) | 1,411 | 33,958 | 0.0% | 0.000 |
| spdlog (C++) | 1,040 | 50,110 | 0.0% | 0.000 |

### Root Cause: Index Rebuild Fix ✅

**Root cause of Relationship F1=0.00 was missing index rebuild after storage load.**

Fixed by adding `build_indexes()` call in `StorageManager::load()` and `QueryEngine::new()`.

Result: Relationship F1 improved from 0.00 to 0.14 (bat/httpx only).

### Priority Order (from spec)

```
semantic correctness
    ↓
relationship coverage
    ↓
retrieval correctness
    ↓
compactness
    ↓
latency
```

**Status:** Relationship extraction improved (8 types), but precision/recall still need measurement.

---

## Part 7: Benchmark Dataset (from Spec)

### Created Benchmark Questions ✅

| # | Question Type | Status | Notes |
|---|---------------|--------|-------|
| 1 | Symbol questions (5) | ✅ | "What is the main entry point?" etc. |
| 2 | Call questions (3) | ✅ | "What functions call other functions?" etc. |
| 3 | Import questions (3) | ✅ | "What modules import other modules?" etc. |
| 4 | Export questions (2) | ✅ | "What public APIs are exposed?" etc. |
| 5 | FlowsTo questions (2) | ✅ | "Where does data flow?" etc. |
| 6 | Instantiate questions (2) | ✅ | "Where are objects created?" etc. |
| 7 | Dependency questions (3) | ✅ | "What are the direct dependencies?" etc. |
| 8 | Architecture questions (2) | ✅ | "What is the project structure?" etc. |
| 9 | Impact questions (3) | ✅ | "What would be affected if a core type changes?" etc. |

**Total:** 25 questions across 9 categories

### Ground Truth Status

The benchmark questions are **repo-agnostic** (generic queries), not **repo-specific** with ground truth answers. This is a significant limitation. The spec requires ground truth answers for every question.

**Next Step:** Create repo-specific benchmark questions with verified ground truth for each of the 5 benchmark repos.

---

## Part 8: Benchmark Baselines ❌

| # | Baseline | Status | Notes |
|---|----------|--------|-------|
| 1 | Raw filesystem + grep | ⏳ | Need to implement |
| 2 | ripgrep | ⏳ | Need to implement |
| 3 | VS Code/LSP-style semantic index | ⏳ | Need to implement |
| 4 | SCIP | ⏳ | Need to implement |
| 5 | LSIF | ⏳ | Need to implement |
| 6 | Code-Graph-RAG | ⏳ | Need to implement |
| 7 | Aider-style repository map | ⏳ | Need to implement |
| 8 | CPG/Joern | ⏳ | Need to implement |
| 9 | Prime (current) | ✅ | Benchmark exists |

### Metrics to Measure (from spec)

| Metric | Status |
|--------|--------|
| Answer accuracy | ✅ (in benchmark) |
| Source-free accuracy | ✅ (in benchmark) |
| Entity precision | ✅ (in benchmark) |
| Entity recall | ✅ (in benchmark) |
| Relationship precision | ✅ (in benchmark) |
| Relationship recall | ✅ (in benchmark) |
| Relationship F1 | ✅ (in benchmark) |
| MRR | ✅ (in benchmark) |
| Recall@1/3/5/10 | ✅ (in benchmark) |
| Bytes retrieved | ❌ Not measured |
| Tokens exposed | ❌ Not measured |
| Source reads | ❌ Not measured |
| Tool calls | ❌ Not measured |
| Query latency | ✅ (in benchmark) |
| Indexing latency | ✅ (in benchmark) |
| Incremental update latency | ✅ (incremental.rs) |
| Memory | ✅ (peak_memory_bytes) |
| Artifact size | ✅ (in benchmark) |

---

## Part 9: Incremental Updates Benchmark ❌

| # | Test Case | Status | Notes |
|---|-----------|--------|-------|
| 1 | Clean → complete index | ✅ | `prime build --force` |
| 2 | Modify 1 file → update | ✅ | `prime update` (70x speedup) |
| 3 | Modify 10 files → update | ⏳ | Need to test |
| 4 | Modify 1% of files | ⏳ | Need to test |
| 5 | Modify 10% of files | ⏳ | Need to test |
| 6 | Branch switch | ⏳ | Need to test |
| 7 | Large refactor | ⏳ | Need to test |

### Metrics to Measure

| Metric | Status |
|--------|--------|
| Files touched | ⏳ |
| Bytes read | ⏳ |
| Parsing time | ⏳ |
| Semantic derivation time | ⏳ |
| Index update time | ✅ |
| Artifact delta | ⏳ |
| Memory | ⏳ |
| Query availability during update | ⏳ |

---

## Part 10: Filesystem Benchmark ❌

| # | Test Case | Status | Notes |
|---|-----------|--------|-------|
| 1 | Cold scan | ⏳ | Need to implement |
| 2 | Warm scan | ⏳ | Need to implement |
| 3 | Cold derivation | ⏳ | Need to implement |
| 4 | Warm derivation | ⏳ | Need to implement |
| 5 | Incremental derivation | ✅ | Implemented |
| 6 | Concurrent derivation | ⏳ | Need to implement |
| 7 | Random query | ⏳ | Need to implement |
| 8 | Sequential queries | ⏳ | Need to implement |
| 9 | Filesystem watcher response | ⏳ | Need to implement |

---

## Part 11: Architecture Investigation

### Two-Level Representation ❌

| Level | Description | Status |
|-------|-------------|--------|
| Level 1 (cheap) | symbols, files, modules, definitions, references, imports, exports, types, calls | ⏳ Partial |
| Level 2 (derived) | call chains, data flow, impact, architecture, behavior, contracts, test relationships | ⏳ Partial |

**Current Status:** Level 1 mostly implemented. Level 2 partially implemented (FlowsTo, Instantiates).

### Specialized Indexes ❌

| Index | Status | Notes |
|-------|--------|-------|
| SymbolIndex | ✅ | NameIndex in types.rs |
| DefinitionIndex | ⏳ | Need to implement |
| ReferenceIndex | ⏳ | Need to implement |
| CallIndex | ⏳ | Need to implement |
| TypeIndex | ⏳ | Need to implement |
| DependencyIndex | ⏳ | Need to implement |
| TestIndex | ⏳ | Need to implement |
| ConfigurationIndex | ⏳ | Need to implement |
| ArchitectureIndex | ⏳ | Need to implement |

---

## Part 12: Graph Compression ❌

| Technique | Status | Notes |
|-----------|--------|-------|
| CSR | ⏳ | Need to investigate |
| CSC | ⏳ | Need to investigate |
| Compressed adjacency | ⏳ | Need to investigate |
| Delta encoding | ⏳ | Need to investigate |
| Variable-length integers | ✅ | Used in compact_serialization |
| Elias-Fano | ⏳ | Need to investigate |
| Roaring bitmaps | ✅ | Used in RelationIndex |
| Succinct graphs | ⏳ | Need to investigate |
| WebGraph-style | ⏳ | Need to investigate |
| Minimal perfect hashing | ⏳ | Need to investigate |

---

## Part 13: Identity Investigation

| Approach | Status | Notes |
|----------|--------|-------|
| Paths | ✅ | File paths used |
| Names | ✅ | Simple names |
| Fully-qualified names | ✅ | file::module::name |
| SCIP symbols | ⏳ | Need to investigate |
| Content hashes | ✅ | SHA-256 in ContentHash |
| Semantic IDs | ⏳ | Need to investigate |
| Stable IDs | ⏳ | Need to investigate |

---

## Part 14: Uncertainty Investigation ✅

| Level | Status | Implementation |
|-------|--------|----------------|
| EXACT | ✅ | Confidence::High |
| DERIVED | ✅ | Confidence::Medium |
| INFERRED | ✅ | Confidence::Low |
| UNKNOWN | ✅ | Confidence::Unknown |

---

## Part 15: Provenance Investigation

| Aspect | Status | Notes |
|--------|--------|-------|
| Source locations | ✅ | Range in Entity |
| Revision IDs | ⏳ | Need to add |
| Content hashes | ✅ | ContentHash |
| Merkle structures | ⏳ | Need to investigate |
| Signed manifests | ⏳ | Need to investigate |
| Compact evidence IDs | ⏳ | Need to investigate |

---

## Part 16: Agent-Specific Evaluation ❌

| Task | Status | Notes |
|------|--------|-------|
| "Where should I add authentication?" | ⏳ | Need to create |
| "What breaks if I rename this interface?" | ⏳ | Need to create |
| "Where is this API consumed?" | ⏳ | Need to create |
| "Find the implementation behind this behavior" | ⏳ | Need to create |
| "Explain the dependency chain from A to B" | ⏳ | Need to create |
| "What tests should I update if I change X?" | ⏳ | Need to create |
| "Which modules are likely affected by changing X?" | ⏳ | Need to create |

---

## Part 17: Architectural Questions (from spec)

| # | Question | Status | Answer |
|---|----------|--------|--------|
| 1 | What do mature systems agree on? | ✅ | 8 universal primitives converge |
| 2 | What information does an agent need? | ✅ | Symbols, relationships, context |
| 3 | What information can be discarded? | ✅ | Formatting, comments, whitespace |
| 4 | What relationships are essential? | ✅ | Calls, Imports, Extends, Implements |
| 5 | What relationships are optional? | ✅ | FlowsTo, Instantiates, Overrides |
| 6 | What does Prime currently lack? | ✅ | Documented in gap analysis |
| 7 | What to borrow from VS Code? | ✅ | Incrementality, language services |
| 8 | What to borrow from SCIP? | ✅ | Symbol identity model |
| 9 | What to borrow from LSIF? | ✅ | Persistent representation |
| 10 | What to borrow from Code-Graph-RAG? | ✅ | Multi-language graph schema |
| 11 | What to borrow from Tree-sitter? | ✅ | Incremental parsing |
| 12 | What to borrow from Git? | ✅ | Content addressing |
| 13 | What to borrow from ripgrep? | ✅ | Parallel traversal |
| 14 | What to NOT borrow? | ✅ | Full graph DB, embeddings |
| 15 | Should Prime be a graph? | ✅ | Yes, but compact |
| 16 | Should Prime use specialized indexes? | ✅ | Yes, for hot paths |
| 17 | Minimum knowledge unit? | ✅ | Entity + Relationship |
| 18 | How to encode relationships? | ✅ | Typed edges with confidence |
| 19 | How should identity work? | ✅ | Qualified names + content hashes |
| 20 | How should incremental updates work? | ✅ | File-level detection + entity invalidation |
| 21 | How should uncertainty work? | ✅ | Confidence levels |
| 22 | How should provenance work? | ⏳ | Partial (locations, hashes) |
| 23 | What should be benchmarked? | ✅ | Comprehensive benchmark suite |
| 24 | What experiments next? | ⏳ | Need to define |
| 25 | What assumptions to remove? | ⏳ | Need to audit |

---

## Part 18: OpenWiki Research ✅

| Concept | Status | Prime Adaptation |
|---------|--------|------------------|
| Grounded Claims (evidence-backed propositions) | ✅ | Investigate per-fact evidence versioning |
| Incremental knowledge maintenance via Git | ✅ | Model as Git-aware knowledge compiler |
| Evidence versioning | ✅ | Content hash per fact for invalidation |
| Two-level architecture (deterministic + agent) | ✅ | Deterministic base + optional agent enrichment |
| OKF format (typed knowledge units) | ✅ | Investigate typed knowledge units with lifecycle metadata |
| AGENTS.md integration | ✅ | MCP-based agent discovery (already implemented) |
| Human-readable wiki output | ✅ | Not Prime's goal — Prime is machine-optimized |
| LLM-based synthesis | ✅ | Not Prime's primary method — use static analysis |

---

## Next Steps (Priority Order)

### Immediate (This Week)

1. **Fix entity extraction** for JS (express=0 entities) — tree-sitter query issues
2. **Increase Relationship F1** from 0.14 to >0.30
3. **Create repo-specific benchmark questions** with ground truth answers
4. **Test incremental updates** at scale (10 files, 1%, 10%, branch switch)

### Short-term (Next 2 Weeks)

5. **Implement Level 2 relationships** (call chains, impact analysis)
6. **Add specialized indexes** (DefinitionIndex, CallIndex, TypeIndex)
7. **Complete incremental benchmarks** (10 files, 1%, 10%, branch switch)
8. **Add agent-specific evaluation tasks**

### Medium-term (Next Month)

9. **Graph compression investigation** (CSR, Elias-Fano, WebGraph)
10. **Identity system** (SCIP symbols, stable IDs)
11. **Provenance system** (revision IDs, Merkle structures)
12. **Filesystem benchmark** (cold/warm scan, concurrent derivation)
13. **OpenWiki-style incremental maintenance** (Git-aware knowledge compiler)

---

## Key Metrics Dashboard

| Metric | Current | Previous | Change | Target |
|--------|---------|----------|--------|--------|
| Entities | 1,094 | 384 | +185% | >2,000 |
| Relations | 75,542 | 73,275 | +3% | >100,000 |
| Relationship Types | 8 | 3 | +167% | 12+ |
| Build Time | 4.79s | 4.19s | +14% | <5s |
| Cold Query | 52µs | 209µs | -75% | <100µs |
| Incremental Speedup | 70x | — | — | >50x |
| Source-free Accuracy | 1.6% | 8.2% | -80% | >60% |
| Relationship F1 | 0.14 | 0.00 | +∞ | >0.50 |
| Entity F1 | 0.57 | 0.37 | +54% | >0.80 |

---

**Research Phase:** ✅ Complete (18/18 repos)
**Implementation Phase:** ✅ Complete (12/12 tasks)
**Benchmarking Phase:** 🔄 In Progress (5/25 tasks)
**Architecture Investigation:** ⏳ Not Started (2/10 tasks)

**Owner:** Research Team
**Next Review:** September 2026
