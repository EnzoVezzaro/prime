# Aider Repository Map — Prior Art Analysis

## Confidence Legend
- **FACT** — Verified by primary source (Aider blog, source code, documentation)
- **OBSERVATION** — Directly observed from source behavior or documented mechanics
- **HYPOTHESIS** — Proposed explanation requiring validation
- **INFERENCE** — Deduced from evidence, marked as such
- **OPEN QUESTION** — Explicitly unknown, needs research

---

## 1. Architecture Overview

### 1.1 Purpose (FACT)
Aider's repository map provides **code context** to LLMs within token budgets. It extracts symbol definitions via Tree-sitter, constructs a file-level dependency graph, applies PageRank-like relevance scoring, and emits a concise map of the most important symbols per file within a configurable token budget (default 1k tokens, controlled by `--map-tokens`) [aider.chat/2023/10/22/repomap.html](https://aider.chat/2023/10/22/repomap.html).

### 1.2 Pipeline Stages (OBSERVATION)
1. **Parse** — Tree-sitter parses source files into ASTs (via `py-tree-sitter-languages`, supporting 30+ languages)
2. **Extract** — Identify definitions (classes, functions, methods, types) and references (calls, imports) from AST
3. **Graph Construction** — Build file-level graph: nodes = files, edges = import/dependency relationships
4. **Rank** — Run PageRank-style algorithm on graph to score symbol importance by cross-file reference count
5. **Budget** — Select top-ranked symbols per file to fit token budget
6. **Emit** — Output concise map showing file paths, key symbols, and signatures

### 1.3 Token Budgeting (FACT)
- Default budget: **1,000 tokens** (`--map-tokens`)
- Budget applies to **entire repo map**, not per-file
- Symbols ranked globally, then selected greedily until budget exhausted
- Critical lines (signatures, class definitions) included; bodies elided with `⋮...`

---

## 2. Symbol Extraction & Representation

### 2.1 Tree-sitter Queries (FACT)
Aider uses **modified `tags.scm` queries** from upstream Tree-sitter grammars to extract:
- Function/method definitions (names, parameters, return types)
- Class/struct/interface definitions
- Type aliases, constants, variables
- Import/require statements for dependency edges

Languages supported: Python, JavaScript, TypeScript, Rust, Go, Java, C, C++, C#, Ruby, PHP, and 20+ more via `tree-sitter-language-pack` [aider.chat/docs/languages.html](https://aider.chat/docs/languages.html).

### 2.2 Reference Tracking (OBSERVATION)
Aider tracks **cross-file references** (imports, function calls) to build the dependency graph. The blog states: *"edges connect files which have dependencies"* and *"most often referenced by other portions of the code"* [aider.chat/2023/10/22/repomap.html](https://aider.chat/2023/10/22/repomap.html).

### 2.3 Output Format (OBSERVATION)
```
aider/coders/base_coder.py:
⋮...
│class Coder:
│    abs_fnames = None
⋮...
│    @classmethod
│    def create(
│        self,
│        main_model,
│        edit_format,
│        io,
│        skip_model_availabily_check=False,
│        **kwargs,
⋮...
│    def abs_root_path(self, path):
⋮...
│    def run(self, with_message=None):
⋮...
```

---

## 3. Graph Ranking Algorithm

### 3.1 PageRank-like Scoring (FACT)
Aider *"analyzes the full repo map using a graph ranking algorithm, computed on a graph where each source file is a node and edges connect files which have dependencies"* [aider.chat/2023/10/22/repomap.html](https://aider.chat/2023/10/22/repomap.html).

### 3.2 Ranking Signals (INFERENCE)
Based on blog description and typical PageRank adaptations:
- **In-degree** — Number of files importing/referencing this file
- **Out-degree** — Number of dependencies this file has
- **Symbol frequency** — How often symbols from this file are referenced elsewhere
- **Centrality** — Files bridging disparate modules score higher

### 3.3 Per-File Symbol Selection (OBSERVATION)
*"It only includes the most important identifiers, the ones which are most often referenced by other portions of the code"* — symbols ranked by reference count within the global graph, then top-N selected per file.

---

## 4. Incremental & Caching Behavior

### 4.1 Re-computation Trigger (OBSERVATION)
Repo map is **rebuilt on each interaction** (each user request). The blog describes it as sent *"along with each request from the user to make a code change."*

### 4.2 Caching (OPEN QUESTION)
- Does Aider cache parsed ASTs between interactions?
- Does it invalidate only changed files?
- Source suggests full rebuild: *"automatically identify and provide the needed code context"* per request.

### 4.3 Git Integration (FACT)
Aider *"automatically commits changes with sensible commit messages"* and respects `.gitignore` [aider.chat/docs/git.html](https://aider.chat/docs/git.html). Repo map likely uses Git for file discovery but not for incremental parsing.

---

## 5. Comparison: Aider Repo Map vs Prime Artifact

| Dimension | Aider Repo Map | Prime Knowledge Graph |
|-----------|----------------|----------------------|
| **Computation** | Per-interaction (ephemeral) | Precomputed, persistent |
| **Storage** | None (in-memory string) | Binary, mmap, compressed |
| **Granularity** | File + symbol signatures | Entity + relations + types |
| **Cross-lang** | Yes (Tree-sitter) | Yes (Tree-sitter) |
| **Ranking** | PageRank on file graph | TBD (research area 07, 11) |
| **Token budget** | Hard limit (1k default) | Progressive disclosure (PrimeEnvelope) |
| **Incremental** | Not documented | Planned (research area 14) |
| **Provenance** | None | PrimeEnvelope: exact/derived/inferred/unknown |
| **Query API** | None (LLM consumes map) | 7 semantic tools (MCP) |

---

## 6. What Prime Should BORROW

### 6.1 Tree-sitter Multi-language Extraction (FACT → BORROW)
- **Use Tree-sitter** as the parsing backbone — proven at scale (Aider, GitHub, VS Code)
- **Adopt `tags.scm` query pattern** for symbol extraction — language-agnostic, maintainable
- **Support 30+ languages** via `tree-sitter-language-pack` approach

### 6.2 Graph-Based Relevance Ranking (INFERENCE → BORROW)
- **File-level dependency graph** for coarse ranking
- **Symbol-level reference counting** for fine-grained importance
- **PageRank / Personalized PageRank** for context-aware relevance

### 6.3 Token-Budgeted Progressive Disclosure (FACT → BORROW)
- **Hard token budgets** map to Prime's `PrimeEnvelope.coverage` and `source_required`
- **Progressive detail**: signatures → full definitions → callers/callees
- **Agent-driven selection**: LLM requests more context via tools (`prime_context`, `prime_relationships`)

### 6.4 Signature-First Representation (OBSERVATION → BORROW)
- Emit **signatures + types** not bodies — matches Prime's `Entity` design (name, qualified_name, kind, signature, span)
- **Elide bodies** with `⋮...` — matches Prime's progressive context building

---

## 7. What Prime Should NOT Borrow

### 7.1 Per-Interaction Recomputation (FACT → AVOID)
- **Re-parsing entire repo every chat turn** is wasteful
- Prime **precomputes** persistent KnowledgeGraph; Aider rediscoveries per interaction
- **Key insight**: Prime can **precompute what repo-map systems rediscover every interaction**

### 7.2 File-Level Graph Only (OBSERVATION → AVOID)
- Aider's graph: **nodes = files**, edges = imports
- Prime needs **symbol-level graph**: entities + typed relations (calls, extends, imports, references)
- File-level loses precision for impact analysis, call graphs, precise navigation

### 7.3 No Persistence / No Incremental (OPEN QUESTION → AVOID)
- No evidence of incremental parsing or persistent index
- Prime **must** support incremental updates (research area 14)

### 7.4 Single Output Format (OBSERVATION → AVOID)
- Aider emits **one text format** for LLM consumption
- Prime needs **multiple views**: search, lookup, context, dependencies, impact, architecture
- PrimeEnvelope provides **structured, typed responses** with provenance

---

## 8. Precomputation Opportunity: The Core Thesis

### 8.1 The Redundancy Problem (INFERENCE)
```
Aider per-interaction cost:
  Parse all files → Extract symbols → Build graph → Rank → Budget → Emit
  × N interactions
  = N × O(repo_size)

Prime precomputation:
  Parse all files → Extract symbols → Build graph → Rank → Persist
  = 1 × O(repo_size) + N × O(query)
```

### 8.2 What Prime Precomputes That Aider Rediscovers (HYPOTHESIS)
| Artifact | Aider (per-turn) | Prime (once) |
|----------|------------------|--------------|
| AST per file | Re-parsed | Cached in binary storage |
| Symbol table | Re-extracted | Persisted (Entity[]) |
| Dependency graph | Rebuilt | Persisted (Relation[]) |
| PageRank scores | Recomputed | Persisted (or incrementally updated) |
| Token-budgeted map | Re-sliced | Served via `prime_context` with dynamic budget |

### 8.3 Incremental Invalidation Model (HYPOTHESIS)
- **Git-style content addressing**: Source file hash → semantic derivation hash → Prime artifact hash
- **Change detection**: `git diff` → affected files → re-parse only changed → update graph incrementally
- **Merkle DAG**: Prime artifacts form content-addressed DAG (research area 14, 18)

---

## 9. Open Questions

1. **OPEN QUESTION**: Does Aider cache Tree-sitter parse trees between runs? The blog implies full rebuild but source code may reveal caching.

2. **OPEN QUESTION**: What is the exact PageRank variant? Damping factor? Personalization vector (user's current file)?

3. **OPEN QUESTION**: How does Aider handle cross-language references (e.g., Python calling Rust via FFI, TypeScript importing JSON)?

4. **OPEN QUESTION**: Symbol disambiguation — how does Aider distinguish `foo()` in file A vs `foo()` in file B when both imported?

5. **OPEN QUESTION**: Scaling behavior — at what repo size does 1k token budget become insufficient? How does ranking degrade?

6. **OPEN QUESTION**: Does the graph ranking consider **semantic** relationships (inheritance, implementation) or only **syntactic** (imports, calls)?

7. **OPEN QUESTION**: Could Prime's persistent graph serve as a **shared backend** for multiple Aider-like agents simultaneously?

8. **OPEN QUESTION**: What is the latency of Aider's repo-map generation on 100K+ file repos? (Prime targets sub-second query on precomputed index)

---

## 10. Evidence Summary

| Claim | Evidence | Confidence |
|-------|----------|------------|
| Tree-sitter for multi-lang parsing | Blog, `py-tree-sitter-languages` dep | FACT |
| PageRank-style file graph ranking | Blog: "graph ranking algorithm" | FACT |
| 1k default token budget | Blog: `--map-tokens` default | FACT |
| Per-interaction rebuild | Blog: "sent along with each request" | OBSERVATION |
| Signature-only output | Blog example output | OBSERVATION |
| 30+ language support | Docs/languages page | FACT |
| Git integration for commits | Docs/git.html | FACT |
| No persistent index documented | Absence in blog/docs | INFERENCE |
| File-level (not symbol-level) graph | Blog: "each source file is a node" | FACT |

---

*Research conducted per Prime methodology: primary sources first, evidence over assumptions, distinguish confidence levels.*