Comparison tables for Prime research:

## Comparison of Code Intelligence Systems

| Feature | SCIP | LSIF | CPG/Joern | Tree-sitter | Graph-sitter | Sourcegraph | Agent Indexers |
|---------|------|------|-----------|-------------|--------------|-------------|----------------|
| **Purpose** | Language-agnostic protocol for code indexing | Standardized output format for language servers | Code property graph for cross-language analysis | Parser generator & incremental parsing | AST-pattern based code grep | Full code intelligence platform | Codebase indexing for agents |
| **Representation** | Protobuf messages | Vertices (symbols, refs, defs) + Edges (relationships) | Multi-layered graph (AST + CFG + DFG + PG) | Concrete syntax tree | AST-derived graph patterns | Graph + embeddings + symbols | Varies (symbols + refs + embeddings) |
| **Storage** | Protobuf binary, JSON, SQLite | JSON files (can be large) | Custom graph database (in-memory) | Parse tree (in-memory) | Graph patterns | Database-backed + embeddings | Varies (DB + index + embeddings) |
| **Indexing** | Protocol messages from indexers | Language server output | Graph database construction | Parser generation per language | Pattern-based indexing | Multi-language indexers | Index construction from parsers |
| **Retrieval** | Go-to-def, find refs, find impls | Definition/ref lookup, symbol search | CPGQL graph queries | Tree traversal, search | Pattern matching queries | Hybrid (db + graph + semantic) | Retrieval-based + pattern matching |
| **Scalability** | Good (protocol-level) | Limited (JSON file size) | Good (in-memory with indexes) | Good per-file, whole-repo needs aggregation | Pattern matching scale | Very good (distributed, cached) | Varies by implementation |
| **Language Support** | 15+ languages via indexers | Requires LSP per language | 15+ via frontends | 50+ grammars available | Pattern per language | All languages via LSPs | Varies (typically 5-15) |
| **Incremental Updates** | Supported (refresh/delete messages) | Requires re-indexing | Supported (content hashing) | Incremental per-file | Pattern update on change | Supported (workspace tracking) | Varies (content-hash based) |
| **Compression** | Protobuf (binary, compact) | JSON (can be verbose) | Graph DB (optimized storage) | CST (compact tree format) | Graph patterns (variable) | Custom (embedding + DB) | Varies |
| **Agent Suitability** | Good (structured data) | Good (definition/ref focus) | Good (graph queries) | Limited (CST, no semantics) | Good (pattern matching) | Very good (hybrid approach) | Designed for agents |
| **Weaknesses** | Requires indexer per language | Large JSON files, no native graph | Memory-heavy, steep learning curve | No semantic info, CST only | Pattern complexity at scale | Complex infrastructure needed | Varies by system |
| **Reusable Components** | Protobuf schema, indexer pattern | Vertex/edge model | CPG construction pipeline | Parser generator framework | Pattern matching DSL | Indexing architecture, LSP integration | Index patterns, retrieval strategies |

## Comparison of Storage Systems

| Feature | SQLite | RocksDB | LMDB | DuckDB | Custom Binary | Columnar |
|---------|--------|---------|------|--------|---------------|----------|
| **Read Performance** | Good point lookup | Excellent range scans | Excellent read-heavy | Excellent analytical | Optimized for workload | Excellent column scans |
| **Random Access** | Good (B-tree) | Good (LSM) | Excellent (B-tree) | Excellent (column) | Workload-dependent | Excellent (column pruning) |
| **Sequential Access** | Good | Good | Excellent | Excellent | Workload-dependent | Excellent (columnar) |
| **Memory Usage** | Moderate | Configurable | Low | Low to moderate | Minimal (custom) | Moderate to high (columnar) |
| **File Size** | Small to moderate | Larger (LSM overhead) | Small | Moderate | Minimal (custom) | Can be large (compressed) |
| **Write Complexity** | Low (auto-compact) | Medium (level management) | Low (WAL) | Medium (auto) | Custom (design choice) | Medium (batch write) |
| **Update Complexity** | Low | Medium (LSM tiers) | Low (append-only) | Medium | Custom | Medium (optimistic) |
| **Concurrency** | Good (WAL + journal) | Excellent (LSM) | Excellent (readers-unblocked) | Good | Custom | Varies (lock-free opt.) |
| **Portability** | Excellent | Excellent | Excellent | Excellent | Platform-dependent | Excellent |
| **mmap Compatibility** | Yes | Yes | Yes | Yes | Yes (design choice) | Yes (some formats) |
| **Scalability** | Good to 100K rows | Excellent (TB+) | Good to 100M rows | Excellent (analytical) | Custom (workload dep.) | Excellent (large datasets) |
| **Agent Suitability** | Good (embedded, widespread) | Good (high throughput) | Good (embedded) | Excellent (SQL-like) | Optimizable | Good (analytical queries) |

## Comparison of Compression Techniques

| Technique | Compression Ratio | CPU Cost | Random Access | Decompression Cost | I/O Reduction | Best For |
|-----------|-------------------|----------|---------------|--------------------|---------------|----------|
| **Varints** | Low (2-4x) | Very low | Direct access | None | Modest | Integer sequences, small values |
| **SIMD-BP128** | Medium (3-8x) | Low (SIMD) | With correction | Low | Good | Large integer arrays |
| **Stream VByte** | Medium (3-5x) | Low | With gaps | Low | Good | Variable-length integers |
| **PForDelta** | Good (3-6x) | Medium | With bucket access | Medium | Good | Batch processing |
| **Frame-of-Reference** | Good (3-5x) | Low | Requires base | Low | Good | Sorted integer sequences |
| **Elias coding** | Good (3-5x) | Low | Requires bit ops | Low | Good | General integer compression |
| **zstd** | Very Good (2-10x+) | Medium | Requires full decompress | Medium | Very Good | General purpose, large files |
| **lz4** | Good (2-4x) | Very low | Requires full decompress | Very low | Good | Real-time, streaming |
| **brotli** | Very Good (3-10x+) | Medium-High | Requires full decompress | Medium-High | Very Good | Web, text compression |
| **gzip** | Good (2-4x) | Low | Requires full decompress | Low | Good | Legacy, wide support |
| **lzma** | Very Good (5-10x+) | High | Requires full decompress | High | Very Good | Maximum compression |

## Comparison of Retrieval Techniques

| Technique | Lexical Search | Semantic Search | Structural Search | Graph Search | Ranking | Filtering | Faceting |
|-----------|---------------|-----------------|-------------------|--------------|---------|-----------|----------|
| **Inverted Index** | Yes | No (requires embeddings) | Limited | No | Yes (TF-IDF) | Yes | Yes |
| **Vector Search** | No | Yes (dense vectors) | Limited | No | Yes (similarity) | Limited | Limited |
| **Hybrid Search** | Yes | Yes | Yes | Yes | Yes (combined) | Yes (combined) | Yes (combined) |
| **Graph Search** | Limited | Limited | Yes (structure) | Yes (patterns) | Limited | Yes (traversal) | Limited |
| **Structural Search** | Limited | Limited | Yes (patterns) | Yes (subgraphs) | Limited | Yes (constraints) | Limited |

## Comparison of Indexing Structures

| Structure | Symbol Lookup | Relationship Lookup | Scalability | Memory footprint | Best Use Case |
|-----------|--------------|--------------------|-------------|------------------|---------------|
| **Inverted Index** | Excellent (O(1)) | Poor (scan required) | Good | Low-moderate | Keyword/search |
| **Hash Map** | Excellent (O(1)) | Poor (no relationships) | Good | Low | Direct symbol lookup |
| **Adjacency List** | Good | Excellent (local) | Good | Moderate | Local relationship traversal |
| **CSR/CSC** | Moderate | Excellent (global) | Fair | Moderate | Matrix-style relationships |
| **Graph Index** | Good | Excellent (pattern matching) | Good (limited) | Moderate-high | Complex relationship queries |
| **Roaring Bitmap** | Fair (set membership) | Fair (set operations) | Excellent | Low-moderate | Set-based filtering, ID sets |
| **Elias-Fano** | Good (rank/select) | Fair (derived) | Excellent | Very low | Compact ordinal sets |
| **Succinct Tree** | Good (hierarchical) | Good (path queries) | Good | Low | Hierarchical code structure |
| **Symbol Table** | Excellent (O(1)) | N/A | Excellent | Low | Direct symbol resolution |

## Comparison of Language Support

| Level | SCIP | LSIF | CPG/Joern | Tree-sitter | Sourcegraph | Prime Target |
|-------|------|------|-----------|-------------|-------------|--------------|
| **Level 1 - Parseable** | Yes (via indexers) | Yes (via LSP) | Yes (via frontends) | Yes (50+ grammars) | Yes (all LSPs) | Parse any language |
| **Level 2 - Semantically analyzable** | Partial (some languages) | Partial (LSP dependent) | Partial (frontend dep.) | No (CST only, no semantics) | Partial (advanced LSPs) | Parse + partial semantics |
| **Level 3 - Knowledge derivable** | Limited (few languages) | Limited (few languages) | Limited (few languages) | No (syntax only) | Limited (advanced) | Graceful degradation |
| **Cross-language relationships** | Yes (protocol-level) | Limited (per-LSP) | Yes (CPG unifies) | No (language-specific) | Yes (via knowledge graph) | Yes (core capability) |

## Comparison of Storage Dimensions for Prime

| Dimension | SQLite | RocksDB | LMDB | Custom Binary | Columnar | Prime Recommendation |
|-----------|--------|---------|------|---------------|----------|----------------------|
| **Small repos (5 files)** | Excellent | Overkill | Excellent | Excellent | Overkill | SQLite or custom binary |
| **Medium repos (500 files)** | Good | Good | Good | Good | Fair | SQLite or RocksDB |
| **Large repos (50K files)** | Fair | Good | Good | Good | Good | RocksDB or custom binary |
| **Monorepo (500K+ files)** | Poor | Good | Good | Good | Excellent | Custom binary or columnar |
| **Incremental updates** | Good | Good | Excellent | Design-dependent | Batch-dependent | Design for incremental |
| **Random access patterns** | Excellent | Good | Excellent | Optimizable | Excellent (pruning) | Optimized for agent queries |
| **Sequential access patterns** | Good | Good | Excellent | Design-dependent | Excellent (column pruning) | Depends on workload |
| **Memory efficiency** | Moderate | Configurable | Low (memory-mapped) | Minimal (custom) | Moderate-high | Custom for Prime workload |
| **Query patterns (agent)** | Good (SQL) | Good (range scans) | Good (point lookups) | Optimize for workload | Good (column pruning) | Optimize for agent retrieval |
| **File size on disk** | Small | Larger (LSM) | Small | Minimal | Variable (compressed) | Minimize useful representation |
| **Concurrency requirements** | Good | Excellent | Excellent (readers-unblocked) | Custom | Varies | Design for agent concurrency |
| **Portability requirements** | Excellent | Excellent | Excellent | Platform-dependent | Excellent | Maximize portability |

## Summary Comparison Tables

### Key Findings from Comparisons:

1. **No single system dominates** across all dimensions. Each has specific strengths:
   - SCIP: Excellent protocol design, language coverage via indexers
   - LSIF: Standardized format, but JSON overhead at scale
   - CPG/Joern: Richest semantic information, but memory-intensive
   - Tree-sitter: Best parser generation, but CST-only (no semantics)
   - Sourcegraph: Most complete feature set, but heavy infrastructure

2. **Storage tradeoffs**:
   - SQLite: Best for small-to-medium, embedded use cases
   - RocksDB/LMDB: Best for large-scale, write-heavy workloads
   - Custom binary: Best for Prime-specific optimization (control over format)
   - Columnar: Best for analytical queries, not ideal for agent retrieval

3. **Compression tradeoffs**:
   - SIMD-BP128 / zstd: Best balance of compression ratio / random access
   - Varints: Best for small integers, no random access needed
   - General rule: optimize for "smallest useful representation with fastest retrieval"

4. **Retrieval tradeoffs**:
   - Hybrid search (lexical + semantic) appears most promising for agents
   - Graph search excels at relationship queries but may be overkill for simple lookups
   - Inverted indexes remain fundamental for lexical agent queries

5. **Language agnosticism**:
   - Level 1 (parseable): All systems can achieve this
   - Level 2 (semantically analyzable): Only some systems (SCIP, Joern with frontends)
   - Level 3 (knowledge derivable): Very few systems currently support this
   - Cross-language relationships: Joern (CPG) and Sourcegraph have strongest support