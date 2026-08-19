Reusable storage research:

- Storage engine libraries that could be adapted for Prime:
  - SQLite: Embedded database, zero-configuration, cross-platform
    - Usefulness: Prime could store knowledge in SQLite (SQL queries for agent retrieval)
    - Adaptation: Schema design for universal knowledge (symbols, relationships, confidence)
    - Tradeoff: Not optimized for Prime's access patterns (agent retrieval vs. general SQL queries)
  - RocksDB: LSM tree-based, high performance, configurable
    - Usefulness: Prime could use RocksDB as primary storage engine
    - Adaptation: Column family organization for symbol/relationship indexes
    - Tradeoff: LSM overhead (space amplification, write vs. read balance)
  - LMDB: Minimal-overhead mmap-based key-value store
    - Usefulness: Prime could use LMDB as lightweight storage engine
    - Adaptation: Key-value pairs for symbol→metadata, relationship lookups
    - Tradeoff: Limited by 64-bit address space for very large DBs, no built-in compaction
  - Custom storage (designed for Prime): Purpose-built storage engine
    - Usefulness: Complete control over data layout, index structures, compression
    - Adaptation: Optimized for Prime's access patterns (mmap, chunked compression, agent queries)
    - Tradeoff: Requires significant design effort, less tested, smaller ecosystem

- Storage interface abstraction:
  - Define uniform storage interface (regardless of backend):
    - put(key, value): Store key-value pair
    - get(key): Retrieve value by key
    - delete(key): Remove key-value pair
    - scan(prefix): Iterate over keys with common prefix
    - batch_put(batch): Store multiple key-value pairs efficiently
    - batch_get(batch): Retrieve multiple keys efficiently
  - Usefulness: Prime can swap storage backend without changing higher-level code
  - Tradeoff: Abstraction layer adds slight overhead, may not expose all backend-specific optimizations

- Index structure reuse:
  - Inverted index: Standard data structure (term → posting list) can be reused
  - Hash map: Standard symbol→knowledge mapping (O(1) lookup)
  - Roaring bitmap: Standard compressed bit set (efficient for degree sets, set operations)
  - Usefulness: Prime can reuse these standard structures rather than inventing from scratch
  - Adaptation: Wrap/integrate into Prime's knowledge storage format

- Prime storage engine requirements (from earlier research):
  1. Zero-copy/mmap compatibility for agent retrieval performance
  2. Incremental update support without full rewrite
  3. Compression integration (ZSTD on stored blocks)
  4. Content-addressed metadata for provenance
  5. Append-only with compaction for incremental updates
  6. Secondary indexes for agent query patterns (symbol lookups, relationship queries)
  7. Scalability: 100K to 5M+ entities
  8. Concurrency: Multiple agent queries simultaneously

- Storage backend selection matrix:

| Backend | Best For | Tradeoff |
|---------|----------|----------|
| SQLite | Small repos, prototyping, SQL-literate agents | Not optimized for Prime agent retrieval patterns |
| RocksDB | Medium/large repos, write-heavy, concurrent agents | LSM overhead (space), complex configuration |
| LMDB | Embedded, read-dominant, simple key-value needs | Limited address space, no built-in compaction |
| Custom binary | Prime-optimized, control over every aspect | Requires significant design effort, less tested |
| Columnar | Analytical agent queries (retrieve specific knowledge columns) | Not ideal for point navigation, incremental updates |

- Reusable storage components:
  - Storage engine interface (uniform API across backends)
  - SQLite adapter (schema + query layer for universal knowledge)
  - RocksDB adapter (column family organization, compaction settings)
  - LMDB adapter (key-value mappings, mmap integration)
  - Custom storage builder (Prime-optimized data layout, index structures)
  - Index wrapper (inverted index, hash map, Roaring bitmap integration)