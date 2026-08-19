Custom Storage research:

- Purpose-built storage for codebase knowledge:
  - Designed specifically for Prime's workload patterns
  - Tradeoffs: Optimization for specific access patterns vs generality

- Custom binary storage design considerations:
  - Entity representation: How symbols, relationships, and metadata are encoded
  - Index structure: Primary and secondary indexes for agent query patterns
  - Incremental update: How changes are applied without full rewrite
  - Compression integration: ZSTD or other compression on stored blocks
  - mmap compatibility: Deterministic layout for page fault-based random access
  - Append-only vs mutable: Append-only simplifies incremental updates, mutable enables in-place updates

- Append-only storage:
  - Write never-in-place, new version appended
  - Usefulness: High (incremental updates, time-travel, simplified concurrency)
  - Tradeoffs: Storage growth over time, compaction needed
  - Compaction strategies: Leveled compaction, tiered compaction, size-tiered
  - Usefulness: Managing storage growth while maintaining read performance

- Immutable snapshots:
  - Complete read-only version of knowledge artifact at a point in time
  - Usefulness: Agent snapshot comparison, reproducibility, incremental derivation
  - Tradeoffs: Storage (multiple versions), update requires new snapshot creation
  - Prime relevance: Core to incremental analysis research area

- Merkle trees:
  - Hash tree where leaves are hashes of data chunks, internal nodes are hashes of children
  - Usefulness: Content addressing, incremental invalidation, distributed sharing
  - Prime relevance: Research area (content addressing, provenance, incremental updates)
  - Incremental invalidation: Change in one file produces new root hash, agents can detect changes via root hash comparison

- Adjacency list storage:
  - Representing relationships (calls, references) as source→target pairs
  - Usefulness: Simple relationship representation
  - Tradeoffs: Inefficient for querying target→source (reverse lookups require full scan)
  - Prime relevance: Base structure, may augment with secondary indexes

- CSR/CSC (Compressed Sparse Row/Column) storage:
  - Matrix format for representing relationships (adjacency matrix compressed)
  - Usefulness: Efficient for certain matrix operations, graph algorithms
  - Tradeoffs: Static (rebuilding needed for modifications), not ideal for dynamic graphs
  - Prime relevance: Possible for static analysis of known-codebase snapshots

- LSM-tree (Log-Structured Merge-tree) storage:
  - Write-optimized: Writes to memtable, flushed to SSTables (sorted string tables)
  - Read-optimization: Multi-level index lookups (may require several disk reads)
  - Usefulness: High (used by RocksDB, LevelDB, SQLite WAL)
  - Tradeoffs: Write-amplification, read-amplification, storage amplification
  - Prime relevance: General-purpose storage engine candidate

- B-tree storage:
  - Balanced tree structure, all leaves at same depth
  - Usefulness: Predictable O(log n) performance, range queries
  - Tradeoffs: Write amplification (node splits), compared to LSM for write-heavy
  - Prime relevance: Point lookup optimization candidate

- Cached storage layer:
  - Fronting storage with memory cache (LRU, TTL-based)
  - Usefulness: Hot data in memory, cold data on disk
  - Tradeoffs: Cache coherence, memory overhead, staleness
  - Prime relevance: Agent query acceleration (hot symbols/relationships in memory)

- Content-addressed storage:
  - Data addressed by hash of content (not location)
  - Usefulness: Deduplication, integrity verification, distributed sharing
  - Prime relevance: Core research area (Merkle DAGs, CRDTs, distributed knowledge)
  - IPFS CID model: Content Identifiers as hash of content

- Shadow PGM (Probabilistic Graphics Model) storage:
  - Probabilistic data structures for compact representation
  - Usefulness: Approximate membership, sketching
  - Tradeoffs: False positives possible, deterministic guarantees lost
  - Prime relevance: Approximate query answering research area

- Sketch-based storage:
  - Bloom filters, Count-Min Sketches, HyperLogLog for approximate queries
  - Usefulness: Set membership, cardinality estimation, frequency estimation
  - Tradeoffs: Approximate (not exact), configurable false positive rate
  - Prime relevance: Research area (approximate membership queries, probabilistic indexes)

- Hybrid storage strategies for Prime:
  1. Hot/cold data partitioning: Frequently accessed knowledge in fast storage (SSD/memory), rarely accessed on durable storage
  2. Append-only with periodic compaction: Incremental updates, managed storage growth
  3. mmap-backed primary storage: Zero-copy access for agent retrieval, with custom binary format
  4. Index structures: Secondary indexes for agent query patterns (symbol lookups, relationship queries)
  5. Content-addressed metadata: Provenance, origin, revision tracking via hashes
  6. Hybrid columnar/custom: Columnar for agent query columns, custom binary for full entity storage

- Prime storage engine requirements:
  1. Zero-copy/mmap compatibility for agent retrieval performance
  2. Incremental update support without full rewrite
  3. Compression integration (ZSTD on stored blocks)
  4. Content-addressed metadata for provenance
  5. Append-only with compaction for incremental updates
  6. Secondary indexes for agent query patterns
  6. Scalability: 100K to 5M+ entities
  7. Concurrency: Multiple agent queries simultaneously