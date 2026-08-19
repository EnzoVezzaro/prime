# research.md — Conduct Research for Prime

A reproducible procedure for conducting research in the Prime project,
following the principles from `init-promt.md`.

## Research Principles (from init-promt.md)

1. **Evidence over assumptions**
   - Never write "X is faster" without evidence.
   - Write: "Benchmark/source X reports...", "Our experiment indicates...", "Theoretical analysis suggests...", "This remains unverified."
   - Distinguish: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION.

2. **Primary sources first**
   - Prefer: official documentation, academic papers, technical specifications, original repositories, engineering papers, benchmark results, source code, original authors' technical writing.
   - Use secondary sources only when useful.
   - Inspect actual implementation rather than relying solely on marketing/documentation.

3. **Do not prematurely converge**
   - Do NOT decide that Prime should use: SQLite, RocksDB, DuckDB, Tree-sitter, protobuf, FlatBuffers, Cap'n Proto, mmap, a custom binary format, a graph database, vectors, embeddings.
   - Until research establishes why.
   - Every major technology choice must have a documented rationale.

4. **Research alternatives fairly**
   - For every major design area, investigate multiple approaches.
   - Example for Storage: SQLite, RocksDB, LMDB, DuckDB, custom binary, memory-mapped structures, columnar storage, immutable indexes.
   - Do not research only the technology you expect to use.

## Research Workflow

### 1. Define the Research Question
- Identify the specific research area from `SPECS/RESEARCH.md` or `init-promt.md` (e.g., "What compression techniques apply to codebase graphs?").
- Check `SPECS/findings/open-problems.md` and `research-gaps.md` for related open questions.
- Read the relevant `AGENTS.md` to understand scope and constraints.

### 2. Survey Prior Art
- Check `SPECS/prior-art/` for existing research on the topic.
- Search official repositories, papers, specifications.
- Use `acc search <query> --kind code` to find relevant implementations.
- Document findings in `SPECS/prior-art/<topic>.md` or update existing files.

### 3. Compare Alternatives
- Create or update comparison tables in `SPECS/prior-art/comparison.md`.
- Compare across dimensions: representation, purpose, storage, indexing, retrieval, scalability, language support, incremental updates, compression, random access, agent suitability, weaknesses, reusable components.
- Document tradeoffs explicitly.

### 4. Analyze Tradeoffs
- Use structured comparison matrices.
- Identify: what works, what doesn't, why.
- Record: evidence, alternatives, tradeoffs, hypotheses, unresolved questions.
- Update `SPECS/findings/technical-findings.md` and `key-findings.md`.

### 5. Identify Open Questions
- Record in `SPECS/findings/open-problems.md` and `research-gaps.md`.
- Be specific: what is unknown, what experiment would resolve it.
- Link to relevant research areas and prior art.

### 6. Update Documentation
- Update relevant SPECS/ files with findings.
- Update `SPECS/findings/` (executive-summary, key-findings, technical-findings, open-problems, research-gaps).
- Update `SPECS/references/` (papers, repositories, specifications, glossary).
- Update `SPECS/benchmarks/` if new benchmarks/methodology/datasets are relevant.

### 7. Validate
- `acc check` — ensure no broken references, forbidden deps, duplicate ownership.
- `acc graph` — confirm research area relationships match intent.
- Run any research validation tests.

### 7. Update Memory
- Record durable knowledge in `.acc-memory.md`:
  - Gotchas, tried-and-rejected, open questions, hypothesis updates.
  - `acc memory add <dir> "<text>"`

## Research Output Standards

### SPECS/ Files Must Contain:
- Clear research question/problem statement.
- Evidence with citations (URLs to official repos, papers, specs).
- Comparison tables where alternatives exist.
- Explicit labeling: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION.
- Tradeoffs analysis.
- Open questions and research gaps.
- No product design claims.

### Confidence Levels:
- **FACT**: Verified by primary source, reproducible.
- **OBSERVATION**: Directly observed, reproducible.
- **HYPOTHESIS**: Proposed explanation, requires validation.
- **INFERENCE**: Deduced from evidence, marked as such.
- **OPEN QUESTION**: Explicitly unknown, needs research.

## Examples of Research Areas (from init-promt.md)

01. Codebase Knowledge (parsing, symbols, types, relationships)
02. Code Property Graphs (CPG, Joern, AST, CFG, PDG)
03. SCIP (symbol identity, cross-language representation)
04. LSIF (graph representation, persistence, querying)
05. Tree-sitter (parsing architecture, incremental, error recovery)
06. Agent-Oriented Code Indexing (Graph-sitter, Sourcegraph, Cursor, etc.)
07. Information Retrieval (inverted indexes, semantic search, hybrid search)
08. Storage Systems (SQLite, RocksDB, LMDB, custom binary, columnar)
09. Binary Format Design (protobuf, FlatBuffers, Cap'n Proto, custom)
10. Compression (integer, graph, string, general)
11. Succinct Data Structures (rank/select, Roaring bitmaps, Elias-Fano)
12. Memory Mapping and I/O (mmap, page cache, SSD/NVMe behavior)
13. Large-Scale Codebases (100K-1M files, monorepos, generated code)
14. Incremental Analysis (parsing, indexing, invalidation, Merkle trees)
15. Language Agnosticism (universal semantic models, capability model)
16. Agent Context and Token Efficiency (context windows, progressive disclosure)
17. Reusable Open-Source Tools (parsers, analyzers, storage, search)
18. Academic Research (papers on code graphs, compression, etc.)
19. Benchmark Research (existing benchmarks, methodology, datasets)

Plus the expanded areas from init-promt.md:
- Information theory (entropy, rate-distortion, info bottleneck)
- Knowledge representation (entities, facts, relationships, predicates)
- Agent architecture (context manager, memory, tool layer, planning)
- Attention and transformer architecture (context caching, long-context degradation)
- Semantic hashing and content addressing (Merkle DAGs, IPFS-style)
- P2P and distributed knowledge (content-addressed shards, LAN sharing)
- Cryptography (integrity, provenance, Merkle proofs, searchable encryption)
- Compressed graph research (WebGraph, Zuckerli)
- Approximate data structures (Bloom filters, learned indexes)
- Grammar compression (Sequitur, Re-Pair, DAG compression)
- Knowledge deduplication (semantic patterns)
- CRDTs and distributed updates (for distributed Prime)